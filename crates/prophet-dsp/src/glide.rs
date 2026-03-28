/// Exponential portamento in log-frequency space.
pub struct Glide {
    current_hz: f32,
    target_hz: f32,
    rate: f32, // 0.0 = instant, higher = slower
    enabled: bool,
}

impl Glide {
    pub fn new() -> Self {
        Self {
            current_hz: 440.0,
            target_hz: 440.0,
            rate: 0.0,
            enabled: false,
        }
    }

    pub fn set_rate(&mut self, rate: f32) {
        self.rate = rate.max(0.0);
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_target(&mut self, hz: f32) {
        self.target_hz = hz.max(1.0);
        if !self.enabled || self.rate <= 0.0 {
            self.current_hz = self.target_hz;
        }
    }

    /// Process one sample in log-frequency space for constant time per octave.
    pub fn process(&mut self, sample_rate: f32) -> f32 {
        if !self.enabled || self.rate <= 0.0 {
            self.current_hz = self.target_hz;
            return self.current_hz;
        }

        // Work in log space for equal glide time per octave
        let current_log = self.current_hz.ln();
        let target_log = self.target_hz.ln();
        let coeff = (-1.0 / (self.rate * sample_rate)).exp();
        let new_log = target_log + (current_log - target_log) * coeff;
        self.current_hz = new_log.exp();
        self.current_hz
    }

    pub fn current(&self) -> f32 {
        self.current_hz
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glide_disabled_is_instant() {
        let mut g = Glide::new();
        g.set_enabled(false);
        g.set_target(440.0);
        let hz = g.process(44100.0);
        assert!((hz - 440.0).abs() < 0.01, "Disabled glide should jump instantly, got {hz}");
    }

    #[test]
    fn test_glide_slides_between_notes() {
        let mut g = Glide::new();
        g.set_enabled(true);
        g.set_rate(0.1); // 100ms glide
        g.set_target(220.0);
        // Run to settle
        for _ in 0..44100 {
            g.process(44100.0);
        }
        assert!((g.current() - 220.0).abs() < 1.0, "Should settle at target");

        // Now glide to 440Hz
        g.set_target(440.0);
        // After 10ms, should be partway
        for _ in 0..441 {
            g.process(44100.0);
        }
        let mid = g.current();
        assert!(
            mid > 220.0 && mid < 440.0,
            "Mid-glide should be between 220 and 440, got {mid}"
        );

        // After 1 second, should be at target
        for _ in 0..44100 {
            g.process(44100.0);
        }
        assert!((g.current() - 440.0).abs() < 1.0, "Should reach target, got {}", g.current());
    }

    #[test]
    fn test_glide_equal_time_per_octave() {
        // In log-freq space, glide should take the same time per octave
        let mut g = Glide::new();
        g.set_enabled(true);
        g.set_rate(0.05);
        g.set_target(220.0);
        for _ in 0..44100 { g.process(44100.0); }

        // Glide up one octave: 220 → 440
        g.set_target(440.0);
        let mut samples_1oct = 0;
        while (g.current() - 440.0).abs() > 1.0 && samples_1oct < 44100 {
            g.process(44100.0);
            samples_1oct += 1;
        }

        // Reset and glide two octaves: 220 → 880
        g.set_target(220.0);
        for _ in 0..44100 { g.process(44100.0); }
        g.set_target(880.0);
        let mut samples_2oct = 0;
        while (g.current() - 880.0).abs() > 1.0 && samples_2oct < 88200 {
            g.process(44100.0);
            samples_2oct += 1;
        }

        // 2 octaves should take roughly 2x as long
        let ratio = samples_2oct as f32 / samples_1oct as f32;
        assert!(
            ratio > 1.1 && ratio < 2.5,
            "2 octaves should take roughly 1-2.5x as long: {samples_1oct} vs {samples_2oct} (ratio={ratio:.2})"
        );
    }
}
