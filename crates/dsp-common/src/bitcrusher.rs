/// Bit-crusher effect — quantizes signal to N bits and optionally reduces sample rate.
/// Essential for the TR-909's digital cymbal/hi-hat character (6-bit PCM).
pub struct BitCrusher {
    bits: u8,
    sample_hold: f32,
    counter: f32,
    rate_factor: f32, // 1.0 = full rate, 0.5 = half rate
}

impl BitCrusher {
    pub fn new(bits: u8, rate_factor: f32) -> Self {
        Self { bits: bits.clamp(1, 16), sample_hold: 0.0, counter: 0.0, rate_factor }
    }

    pub fn set_bits(&mut self, bits: u8) { self.bits = bits.clamp(1, 16); }
    pub fn set_rate(&mut self, factor: f32) { self.rate_factor = factor.clamp(0.01, 1.0); }

    pub fn process(&mut self, input: f32) -> f32 {
        self.counter += self.rate_factor;
        if self.counter >= 1.0 {
            self.counter -= 1.0;
            // Quantize to N bits
            let levels = (1 << self.bits) as f32;
            let half = levels / 2.0;
            self.sample_hold = ((input * half).round() / half).clamp(-1.0, 1.0);
        }
        self.sample_hold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitcrusher_reduces_resolution() {
        let mut bc = BitCrusher::new(4, 1.0); // 4-bit = 16 levels
        // A smooth ramp should become stepped
        let input: Vec<f32> = (0..100).map(|i| i as f32 / 100.0).collect();
        let output: Vec<f32> = input.iter().map(|&s| bc.process(s)).collect();

        // Count unique output values — should be far fewer than 100
        let mut unique: Vec<f32> = output.clone();
        unique.sort_by(|a, b| a.partial_cmp(b).unwrap());
        unique.dedup();
        assert!(unique.len() < 20, "4-bit should have <=16 levels, got {} unique", unique.len());
    }

    #[test]
    fn test_bitcrusher_passthrough_at_16bit() {
        let mut bc = BitCrusher::new(16, 1.0);
        let input = 0.12345f32;
        let output = bc.process(input);
        assert!((output - input).abs() < 0.001, "16-bit should be near-transparent");
    }
}
