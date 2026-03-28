pub struct Lfo {
    phase: f32,
    freq_hz: f32,
    sample_rate: f32,
    tri_on: bool,
    saw_on: bool,
    square_on: bool,
}

impl Lfo {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            phase: 0.0,
            freq_hz: 5.0,
            sample_rate,
            tri_on: false,
            saw_on: false,
            square_on: false,
        }
    }

    pub fn set_frequency(&mut self, hz: f32) {
        self.freq_hz = hz.clamp(0.04, 20.0);
    }

    pub fn set_triangle(&mut self, on: bool) { self.tri_on = on; }
    pub fn set_sawtooth(&mut self, on: bool) { self.saw_on = on; }
    pub fn set_square(&mut self, on: bool) { self.square_on = on; }

    /// Process one sample. Returns the LFO output.
    pub fn process(&mut self) -> f32 {
        self.phase += self.freq_hz / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        let mut out = 0.0;

        if self.tri_on {
            // Bipolar triangle: -1 to +1
            out += if self.phase < 0.5 {
                self.phase * 4.0 - 1.0
            } else {
                3.0 - self.phase * 4.0
            };
        }

        if self.saw_on {
            // Unipolar sawtooth: 0 to 1
            out += self.phase * 2.0 - 1.0;
        }

        if self.square_on {
            // Unipolar square
            out += if self.phase < 0.5 { 1.0 } else { -1.0 };
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(lfo: &mut Lfo, duration_secs: f32) -> Vec<f32> {
        let n = (lfo.sample_rate * duration_secs) as usize;
        (0..n).map(|_| lfo.process()).collect()
    }

    #[test]
    fn test_lfo_triangle_frequency() {
        let mut lfo = Lfo::new(44100.0);
        lfo.set_frequency(5.0);
        lfo.set_triangle(true);
        let buf = render(&mut lfo, 2.0);
        // At 5Hz, 2 seconds = 10 full cycles.
        // Count positive-to-negative zero crossings
        let crossings = count_zero_crossings(&buf);
        // 10 cycles = 10 zero crossings (pos→neg)
        assert!(
            (crossings as f32 - 10.0).abs() <= 1.0,
            "Expected ~10 zero crossings for 5Hz/2s, got {crossings}"
        );
    }

    #[test]
    fn test_lfo_triangle_bipolar() {
        let mut lfo = Lfo::new(44100.0);
        lfo.set_frequency(5.0);
        lfo.set_triangle(true);
        let buf = render(&mut lfo, 1.0);
        let peak = audio_test_harness::level::peak(&buf);
        assert!(peak > 0.9, "Triangle should reach near ±1, peak={peak}");
        let has_negative = buf.iter().any(|&s| s < -0.5);
        assert!(has_negative, "Triangle should be bipolar");
    }

    fn count_zero_crossings(buf: &[f32]) -> usize {
        buf.windows(2)
            .filter(|w| w[0] >= 0.0 && w[1] < 0.0)
            .count()
    }

    #[test]
    fn test_lfo_square_frequency() {
        let mut lfo = Lfo::new(44100.0);
        lfo.set_frequency(5.0);
        lfo.set_square(true);
        let buf = render(&mut lfo, 2.0);
        let crossings = count_zero_crossings(&buf);
        assert!(
            (crossings as f32 - 10.0).abs() <= 1.0,
            "Expected ~10 zero crossings for 5Hz/2s, got {crossings}"
        );
    }
}
