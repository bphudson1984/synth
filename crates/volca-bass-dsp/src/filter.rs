use std::f32::consts::PI;

/// Fast tan approximation using Padé approximant.
#[inline(always)]
fn fast_tan(x: f32) -> f32 {
    let x2 = x * x;
    x * (1.0 + x2 * (1.0 / 3.0 + x2 * (2.0 / 15.0)))
}

/// Fast tanh approximation using rational function.
#[inline(always)]
pub fn fast_tanh(x: f32) -> f32 {
    let x2 = x * x;
    x * (27.0 + x2) / (27.0 + 9.0 * x2)
}

/// Korg MiniKorg 700S-style diode bridge lowpass filter.
///
/// 2-pole (12dB/oct) LPF with nonlinear diode saturation.
/// The diode bridge creates amplitude-dependent impedance — louder signals
/// shift the cutoff lower, producing the characteristic "breathing" effect.
///
/// Implementation: Chamberlin state-variable filter extracting LP output,
/// with tanh saturation on input (diode bridge) and bandpass feedback
/// (resonance with diode-limited self-oscillation). 2x oversampled.
pub struct DiodeBridgeLPF {
    low: f32,
    band: f32,
    sample_rate: f32,
    pub cutoff_hz: f32,
    pub resonance: f32,
    f: f32,   // frequency coefficient
    q: f32,   // damping (1/Q) — lower = more resonance
}

impl DiodeBridgeLPF {
    pub fn new(sample_rate: f32) -> Self {
        let mut filter = Self {
            low: 0.0,
            band: 0.0,
            sample_rate,
            cutoff_hz: 1000.0,
            resonance: 0.0,
            f: 0.0,
            q: 1.0,
        };
        filter.update_coeffs();
        filter
    }

    pub fn set_cutoff(&mut self, hz: f32) {
        self.cutoff_hz = hz.clamp(20.0, self.sample_rate * 0.45);
        self.update_coeffs();
    }

    pub fn set_resonance(&mut self, r: f32) {
        self.resonance = r.clamp(0.0, 1.0);
        // Map 0-1 to damping 2.0-0.01 (lower damping = more resonance)
        // At q < ~0.05 the filter self-oscillates
        self.q = 2.0 * (1.0 - r * 0.995);
    }

    fn update_coeffs(&mut self) {
        // 2x oversampled frequency coefficient
        self.f = 2.0 * (PI * self.cutoff_hz / (self.sample_rate * 2.0)).sin();
    }

    /// Process one sample with 2x internal oversampling.
    pub fn process(&mut self, input: f32) -> f32 {
        let f = self.f;
        let q = self.q;
        let mut sum = 0.0;

        // 2x oversampled
        for _ in 0..2 {
            // Diode bridge input saturation: amplitude-dependent impedance
            let x = fast_tanh(input * 1.2);

            // Chamberlin SVF with diode-saturated feedback
            // The tanh on the bandpass models diode bridge nonlinearity:
            // at high resonance the feedback is soft-limited, creating harmonically
            // rich self-oscillation rather than hard clipping
            let high = x - self.low - q * fast_tanh(self.band);
            self.band += f * high;
            self.low += f * self.band;

            sum += self.low;
        }

        sum * 0.5
    }

    pub fn clear(&mut self) {
        self.low = 0.0;
        self.band = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    fn sine_buf(freq: f32, sample_rate: f32, seconds: f32) -> Vec<f32> {
        let n = (sample_rate * seconds) as usize;
        (0..n).map(|i| (2.0 * PI * freq * i as f32 / sample_rate).sin()).collect()
    }

    #[test]
    fn test_filter_passes_low_frequencies() {
        let mut f = DiodeBridgeLPF::new(44100.0);
        f.set_cutoff(5000.0);
        f.set_resonance(0.0);
        let input = sine_buf(100.0, 44100.0, 0.2);
        let output: Vec<f32> = input.iter().map(|&s| f.process(s)).collect();
        let rms_in = (input.iter().map(|s| s * s).sum::<f32>() / input.len() as f32).sqrt();
        let rms_out = (output.iter().map(|s| s * s).sum::<f32>() / output.len() as f32).sqrt();
        // Low freq should pass through mostly unattenuated
        assert!(rms_out > rms_in * 0.5, "100Hz should pass through 5kHz LPF, got ratio {:.3}", rms_out / rms_in);
    }

    #[test]
    fn test_filter_attenuates_high_frequencies() {
        let mut f = DiodeBridgeLPF::new(44100.0);
        f.set_cutoff(500.0);
        f.set_resonance(0.0);
        let input = sine_buf(8000.0, 44100.0, 0.2);
        let output: Vec<f32> = input.iter().map(|&s| f.process(s)).collect();
        let rms_in = (input.iter().map(|s| s * s).sum::<f32>() / input.len() as f32).sqrt();
        let rms_out = (output.iter().map(|s| s * s).sum::<f32>() / output.len() as f32).sqrt();
        assert!(rms_out < rms_in * 0.3, "8kHz should be heavily attenuated by 500Hz LPF, got ratio {:.3}", rms_out / rms_in);
    }

    #[test]
    fn test_resonance_boosts_at_cutoff() {
        // With high resonance, signal at cutoff should be boosted
        let mut f_flat = DiodeBridgeLPF::new(44100.0);
        f_flat.set_cutoff(1000.0);
        f_flat.set_resonance(0.0);
        let input = sine_buf(1000.0, 44100.0, 0.3);
        let out_flat: Vec<f32> = input.iter().map(|&s| f_flat.process(s)).collect();
        let rms_flat = (out_flat[2000..].iter().map(|s| s * s).sum::<f32>() / (out_flat.len() - 2000) as f32).sqrt();

        let mut f_reso = DiodeBridgeLPF::new(44100.0);
        f_reso.set_cutoff(1000.0);
        f_reso.set_resonance(0.8);
        let out_reso: Vec<f32> = input.iter().map(|&s| f_reso.process(s)).collect();
        let rms_reso = (out_reso[2000..].iter().map(|s| s * s).sum::<f32>() / (out_reso.len() - 2000) as f32).sqrt();

        assert!(rms_reso > rms_flat * 1.2, "Resonance should boost signal at cutoff: flat={rms_flat:.4}, reso={rms_reso:.4}");
    }

    #[test]
    fn test_self_oscillation() {
        let mut f = DiodeBridgeLPF::new(44100.0);
        f.set_cutoff(1000.0);
        f.set_resonance(1.0);

        // Feed silence — filter should self-oscillate
        // Seed with tiny impulse
        f.process(0.01);
        let mut energy = 0.0f32;
        for _ in 0..44100 {
            let out = f.process(0.0);
            energy += out.abs();
        }
        assert!(energy > 1.0, "Filter should self-oscillate at max resonance, got energy {energy:.4}");
    }

    #[test]
    fn test_different_cutoffs_produce_different_output() {
        let input = sine_buf(440.0, 44100.0, 0.3);

        let mut f_low = DiodeBridgeLPF::new(44100.0);
        f_low.set_cutoff(200.0);
        let out_low: Vec<f32> = input.iter().map(|&s| f_low.process(s)).collect();

        let mut f_high = DiodeBridgeLPF::new(44100.0);
        f_high.set_cutoff(8000.0);
        let out_high: Vec<f32> = input.iter().map(|&s| f_high.process(s)).collect();

        let corr = audio_test_harness::correlation::cross_correlation(&out_low, &out_high);
        assert!(corr < 0.95, "Different cutoffs should produce different output (corr={corr:.3})");
    }
}
