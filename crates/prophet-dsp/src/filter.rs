use std::f32::consts::PI;

pub struct LadderFilter {
    s: [f32; 4],       // 4 integrator states
    cutoff_hz: f32,
    resonance: f32,    // 0.0 to ~4.0
    sample_rate: f32,
}

impl LadderFilter {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            s: [0.0; 4],
            cutoff_hz: 20000.0,
            resonance: 0.0,
            sample_rate,
        }
    }

    pub fn set_cutoff(&mut self, hz: f32) {
        self.cutoff_hz = hz.clamp(20.0, self.sample_rate * 0.45);
    }

    pub fn set_resonance(&mut self, r: f32) {
        self.resonance = r.clamp(0.0, 4.0);
    }

    pub fn process(&mut self, input: f32) -> f32 {
        // Zavalishin TPT ladder filter
        // g = tan(pi * cutoff / sample_rate) — the integrator gain
        let g = (PI * self.cutoff_hz / self.sample_rate).tan();
        let a = g / (1.0 + g); // TPT 1-pole gain coefficient
        let b = 1.0 - a;       // state-to-output coefficient

        // Estimate y4 from current state (linear estimate for feedback resolution)
        // y_i = a * x_i + b * s_i  for each stage
        // y4 = a^4 * u + b * (a^3*s0 + a^2*s1 + a*s2 + s3)
        let a2 = a * a;
        let a3 = a2 * a;
        let a4 = a3 * a;

        let s_estimate = b * (a3 * self.s[0] + a2 * self.s[1] + a * self.s[2] + self.s[3]);
        let g_total = a4;

        // Solve: u = input - k * y4 = input - k * (g_total * u + s_estimate)
        let u = (input - self.resonance * s_estimate) / (1.0 + self.resonance * g_total);

        // Process through 4 cascaded TPT 1-pole lowpass stages
        let mut x = u;
        for i in 0..4 {
            // SSM2040-style saturation at each stage input
            // Scale so normal signal levels are mostly linear
            x = ssm2040_saturate(x);

            let v = (x - self.s[i]) * a;
            let y = v + self.s[i];
            self.s[i] = y + v;
            x = y;
        }

        x
    }
}

/// SSM2040-style soft saturation with slight asymmetry.
/// Scaled so signals in [-1, 1] pass mostly unchanged, larger signals compress.
fn ssm2040_saturate(x: f32) -> f32 {
    // Soft saturation: nearly linear at unity, compresses at high levels.
    // tanh(1/5)*5 = 0.197*5 = 0.985 → through 4 stages: 0.985^4 = 0.94
    // This provides enough nonlinearity for self-oscillation stability
    // while keeping passband gain close to unity.
    (x * 0.2).tanh() * 5.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn white_noise(n: usize) -> Vec<f32> {
        // Simple deterministic pseudo-random noise for testing
        let mut state: u64 = 12345;
        (0..n)
            .map(|_| {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                (state as i64 as f32) / (i64::MAX as f32) * 2.0
            })
            .collect()
    }

    fn process_buffer(filter: &mut LadderFilter, input: &[f32]) -> Vec<f32> {
        input.iter().map(|&s| filter.process(s)).collect()
    }

    #[test]
    fn test_lowpass_passes_dc() {
        let mut filter = LadderFilter::new(44100.0);
        filter.set_cutoff(1000.0);
        filter.set_resonance(0.0);

        // Feed DC signal, output should converge to the same value
        let input = vec![1.0f32; 44100];
        let output = process_buffer(&mut filter, &input);
        // Check last portion has settled near 1.0
        let tail_avg: f32 = output[40000..].iter().sum::<f32>() / 4100.0;
        assert!(
            (tail_avg - 1.0).abs() < 0.05,
            "DC should pass through, got {tail_avg}"
        );
    }

    #[test]
    fn test_lowpass_attenuates_high_freq() {
        let mut filter = LadderFilter::new(44100.0);
        filter.set_cutoff(500.0);
        filter.set_resonance(0.0);

        // 5000Hz sine — well above cutoff, should be heavily attenuated
        let sr = 44100.0;
        let input: Vec<f32> = (0..44100)
            .map(|i| (2.0 * PI * 5000.0 * i as f32 / sr).sin())
            .collect();
        let output = process_buffer(&mut filter, &input);

        let input_rms = audio_test_harness::level::rms(&input);
        let output_rms = audio_test_harness::level::rms(&output[4410..]); // skip transient
        let attenuation_db = 20.0 * (output_rms / input_rms).log10();
        // 5000Hz is ~3.3 octaves above 500Hz cutoff, at 24dB/oct = ~80dB attenuation
        // In practice, analog filters don't achieve full theoretical attenuation
        assert!(
            attenuation_db < -30.0,
            "5kHz should be heavily attenuated at 500Hz cutoff, got {attenuation_db:.1}dB"
        );
    }

    #[test]
    fn test_lowpass_slope_24db() {
        let mut filter = LadderFilter::new(44100.0);
        filter.set_cutoff(1000.0);
        filter.set_resonance(0.0);

        let noise = white_noise(441000); // 10 seconds for good resolution
        let output = process_buffer(&mut filter, &noise);

        // Measure slope between 2kHz and 8kHz (2 octaves above cutoff)
        audio_test_harness::spectral::assert_spectral_slope(
            &output[44100..], // skip initial transient
            44100.0,
            2000.0,
            8000.0,
            -24.0,   // expected: -24dB/octave for 4-pole
            6.0,     // tolerance
        );
    }

    #[test]
    fn test_self_oscillation() {
        let mut filter = LadderFilter::new(44100.0);
        filter.set_cutoff(1000.0);
        filter.set_resonance(4.0);

        // Feed near-silence with a tiny impulse to kick-start oscillation
        let mut input = vec![0.0f32; 44100];
        input[0] = 0.5;
        let output = process_buffer(&mut filter, &input);

        // After settling, should be oscillating at ~1000Hz
        let tail = &output[22050..];
        audio_test_harness::level::assert_not_silent(tail, 0.001);
        audio_test_harness::pitch::assert_pitch(tail, 44100.0, 1000.0, 50.0); // wide tolerance for self-osc
    }

    #[test]
    fn test_resonance_peak() {
        let mut filter = LadderFilter::new(44100.0);
        filter.set_cutoff(1000.0);
        filter.set_resonance(3.0);

        let noise = white_noise(441000);
        let output = process_buffer(&mut filter, &noise);

        // Should have a strong peak near 1000Hz
        let spectrum = audio_test_harness::spectral::magnitude_spectrum(&output[44100..]);
        let peaks = audio_test_harness::spectral::find_peaks(&spectrum, 44100.0, 1);
        assert!(
            (peaks[0].0 - 1000.0).abs() < 200.0,
            "Resonance peak at {:.0}Hz, expected ~1000Hz",
            peaks[0].0
        );
    }
}
