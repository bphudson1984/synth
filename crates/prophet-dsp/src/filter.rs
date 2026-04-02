use std::f32::consts::PI;

pub struct LadderFilter {
    s: [f32; 4],       // 4 integrator states
    cutoff_hz: f32,
    resonance: f32,    // 0.0 to ~4.0
    drive: f32,        // 1.0 = clean, higher = more saturation
    sample_rate: f32,
    // Cached coefficients — avoid tan() per sample
    a: f32,
    b: f32,
    a2: f32,
    a3: f32,
    a4: f32,
}

impl LadderFilter {
    pub fn new(sample_rate: f32) -> Self {
        let mut f = Self {
            s: [0.0; 4],
            cutoff_hz: 20000.0,
            resonance: 0.0,
            drive: 1.0,
            sample_rate,
            a: 0.0, b: 0.0, a2: 0.0, a3: 0.0, a4: 0.0,
        };
        f.update_coeffs();
        f
    }

    fn update_coeffs(&mut self) {
        let g = (PI * self.cutoff_hz / self.sample_rate).tan();
        self.a = g / (1.0 + g);
        self.b = 1.0 - self.a;
        self.a2 = self.a * self.a;
        self.a3 = self.a2 * self.a;
        self.a4 = self.a3 * self.a;
    }

    pub fn set_cutoff(&mut self, hz: f32) {
        let new_hz = hz.clamp(20.0, self.sample_rate * 0.45);
        if new_hz != self.cutoff_hz {
            self.cutoff_hz = new_hz;
            self.update_coeffs();
        }
    }

    pub fn set_resonance(&mut self, r: f32) {
        self.resonance = r.clamp(0.0, 4.0);
    }

    pub fn set_drive(&mut self, d: f32) {
        self.drive = d.max(0.1);
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let a = self.a;
        let b = self.b;

        let s_estimate = b * (self.a3 * self.s[0] + self.a2 * self.s[1] + a * self.s[2] + self.s[3]);
        let g_total = self.a4;

        // Solve: u = input - k * y4 = input - k * (g_total * u + s_estimate)
        let u = (input - self.resonance * s_estimate) / (1.0 + self.resonance * g_total);

        // Process through 4 cascaded TPT 1-pole lowpass stages
        let mut x = u;
        for i in 0..4 {
            // SSM2040-style saturation at each stage input
            // Scale so normal signal levels are mostly linear
            x = ssm2040_saturate(x, self.drive);

            let v = (x - self.s[i]) * a;
            let y = v + self.s[i];
            self.s[i] = y + v;
            x = y;
        }

        x
    }
}

/// Fast tanh approximation using rational function.
/// Smooth saturation character, accurate to ~1% for |x| < 4.
#[inline(always)]
fn fast_tanh(x: f32) -> f32 {
    let x2 = x * x;
    x * (27.0 + x2) / (27.0 + 9.0 * x2)
}

/// SSM2040-style soft saturation with slight asymmetry.
/// Scaled so signals in [-1, 1] pass mostly unchanged, larger signals compress.
fn ssm2040_saturate(x: f32, drive: f32) -> f32 {
    // Drive scales the input into the tanh curve earlier.
    // drive=1.0: clean (same as before). drive=2.0+: more grit.
    let scale = 0.2 * drive;
    let inv = 1.0 / scale;
    fast_tanh(x * scale) * inv
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
