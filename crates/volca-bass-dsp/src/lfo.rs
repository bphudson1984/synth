/// Multi-destination LFO: triangle or square, free-running.
/// Output is bipolar (-1.0 to +1.0).
pub struct LFO {
    phase: f32,
    rate_hz: f32,
    sample_rate: f32,
    pub use_square: bool,
}

impl LFO {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            phase: 0.0,
            rate_hz: 5.0,
            sample_rate,
            use_square: false,
        }
    }

    pub fn set_rate(&mut self, hz: f32) {
        self.rate_hz = hz.clamp(0.1, 1000.0);
    }

    pub fn process(&mut self) -> f32 {
        self.phase += self.rate_hz / self.sample_rate;
        if self.phase >= 1.0 { self.phase -= 1.0; }

        if self.use_square {
            if self.phase < 0.5 { 1.0 } else { -1.0 }
        } else {
            // Triangle: -1 to +1
            if self.phase < 0.5 {
                self.phase * 4.0 - 1.0
            } else {
                3.0 - self.phase * 4.0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lfo_triangle_range() {
        let mut lfo = LFO::new(44100.0);
        lfo.set_rate(10.0);
        for _ in 0..44100 {
            let val = lfo.process();
            assert!(val >= -1.0 && val <= 1.0, "LFO out of range: {val}");
        }
    }

    #[test]
    fn test_lfo_square_values() {
        let mut lfo = LFO::new(44100.0);
        lfo.set_rate(10.0);
        lfo.use_square = true;
        for _ in 0..44100 {
            let val = lfo.process();
            assert!(val == 1.0 || val == -1.0, "Square LFO should be ±1, got {val}");
        }
    }
}
