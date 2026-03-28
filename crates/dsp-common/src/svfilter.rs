use std::f32::consts::PI;

/// State-variable filter with lowpass, bandpass, and highpass outputs.
/// Based on the Chamberlin topology. Used for 808 resonators, tone controls, etc.
pub struct SVFilter {
    low: f32,
    band: f32,
    high: f32,
    f: f32,   // frequency coefficient
    q: f32,   // damping (1/Q)
    sample_rate: f32,
}

impl SVFilter {
    pub fn new(sample_rate: f32) -> Self {
        let mut f = Self {
            low: 0.0, band: 0.0, high: 0.0,
            f: 0.0, q: 0.5, sample_rate,
        };
        f.set_freq(1000.0);
        f
    }

    pub fn set_freq(&mut self, hz: f32) {
        // Clamp to prevent instability near Nyquist
        let clamped = hz.clamp(20.0, self.sample_rate * 0.45);
        self.f = 2.0 * (PI * clamped / self.sample_rate).sin();
    }

    /// Set Q (resonance). Higher Q = narrower bandwidth, more ringing.
    /// Q of 0.5 = no resonance. Q of 20+ = sharp resonant peak.
    pub fn set_q(&mut self, q: f32) {
        self.q = (1.0 / q.max(0.5)).clamp(0.001, 2.0);
    }

    /// Process one sample. Call the appropriate output method after.
    pub fn process(&mut self, input: f32) {
        // Two iterations for better stability at high frequencies
        for _ in 0..2 {
            self.high = input - self.low - self.q * self.band;
            self.band += self.f * self.high;
            self.low += self.f * self.band;
        }
    }

    pub fn lowpass(&self) -> f32 { self.low }
    pub fn bandpass(&self) -> f32 { self.band }
    pub fn highpass(&self) -> f32 { self.high }

    /// Process and return bandpass output (convenience for resonators)
    pub fn process_bp(&mut self, input: f32) -> f32 {
        self.process(input);
        self.band
    }

    /// Process and return highpass output
    pub fn process_hp(&mut self, input: f32) -> f32 {
        self.process(input);
        self.high
    }

    /// Process and return lowpass output
    pub fn process_lp(&mut self, input: f32) -> f32 {
        self.process(input);
        self.low
    }

    pub fn clear(&mut self) {
        self.low = 0.0;
        self.band = 0.0;
        self.high = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svf_bandpass_resonance() {
        let mut f = SVFilter::new(44100.0);
        f.set_freq(1000.0);
        f.set_q(20.0); // high Q

        // Impulse response should ring at ~1000Hz
        let mut output = Vec::new();
        output.push(f.process_bp(1.0));
        for _ in 0..4410 {
            output.push(f.process_bp(0.0));
        }

        // Should produce sound (ringing)
        let rms: f32 = (output.iter().map(|s| s * s).sum::<f32>() / output.len() as f32).sqrt();
        assert!(rms > 0.001, "High-Q bandpass should ring on impulse, got RMS {rms}");
    }

    #[test]
    fn test_svf_lowpass_passes_dc() {
        let mut f = SVFilter::new(44100.0);
        f.set_freq(1000.0);
        f.set_q(0.7);

        let mut last = 0.0;
        for _ in 0..44100 { last = f.process_lp(1.0); }
        assert!((last - 1.0).abs() < 0.05, "LP should pass DC, got {last}");
    }
}
