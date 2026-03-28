pub struct NoiseGenerator {
    state: u64,
}

impl NoiseGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    /// Generate one sample of white noise in [-1, 1].
    pub fn white(&mut self) -> f32 {
        // xorshift64
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        // Map to [-1, 1]
        (self.state as i64 as f64 / i64::MAX as f64) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_noise_not_silent() {
        let mut gen = NoiseGenerator::new(42);
        let buf: Vec<f32> = (0..44100).map(|_| gen.white()).collect();
        audio_test_harness::level::assert_not_silent(&buf, 0.1);
    }

    #[test]
    fn test_white_noise_in_range() {
        let mut gen = NoiseGenerator::new(42);
        for _ in 0..100000 {
            let s = gen.white();
            assert!(s >= -1.0 && s <= 1.0, "Sample {s} out of range");
        }
    }

    #[test]
    fn test_white_noise_roughly_flat_spectrum() {
        let mut gen = NoiseGenerator::new(42);
        let buf: Vec<f32> = (0..441000).map(|_| gen.white()).collect();
        let spectrum = audio_test_harness::spectral::magnitude_spectrum(&buf);
        // Check that energy in low and high bands is within 6dB
        let n = spectrum.len();
        let low_avg: f32 = spectrum[100..1000].iter().sum::<f32>() / 900.0;
        let high_avg: f32 = spectrum[n / 4..n / 2].iter().sum::<f32>() / (n / 4) as f32;
        let db_diff = 20.0 * (high_avg / low_avg).log10();
        assert!(
            db_diff.abs() < 6.0,
            "White noise spectrum should be roughly flat, got {db_diff:.1}dB difference"
        );
    }
}
