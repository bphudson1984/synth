/// Pitch envelope that sweeps from a start frequency to an end frequency
/// with exponential decay. Used for 808 bass drum pitch chirp, tom pitch sweep, etc.
pub struct PitchEnvelope {
    current_hz: f32,
    target_hz: f32,
    coeff: f32,
    active: bool,
    sample_rate: f32,
}

impl PitchEnvelope {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            current_hz: 50.0,
            target_hz: 50.0,
            coeff: 0.999,
            active: false,
            sample_rate,
        }
    }

    /// Set the sweep: starts at start_hz, decays to end_hz over sweep_time seconds.
    pub fn set_sweep(&mut self, start_hz: f32, end_hz: f32, sweep_time: f32) {
        self.target_hz = end_hz;
        self.current_hz = start_hz;
        let t = sweep_time.max(0.0001);
        self.coeff = (-1.0 / (t * self.sample_rate)).exp();
    }

    pub fn trigger(&mut self, start_hz: f32) {
        self.current_hz = start_hz;
        self.active = true;
    }

    /// Process one sample, returns current frequency in Hz.
    pub fn process(&mut self) -> f32 {
        if self.active {
            self.current_hz = self.target_hz + (self.current_hz - self.target_hz) * self.coeff;
            if (self.current_hz - self.target_hz).abs() < 0.1 {
                self.current_hz = self.target_hz;
                self.active = false;
            }
        }
        self.current_hz
    }

    pub fn hz(&self) -> f32 { self.current_hz }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pitch_sweep_down() {
        let mut env = PitchEnvelope::new(44100.0);
        env.set_sweep(300.0, 50.0, 0.01);
        env.trigger(300.0);

        let start = env.process();
        assert!(start > 250.0, "Should start high, got {start}");

        // After 50ms, should be near target
        for _ in 0..2205 { env.process(); }
        let end = env.hz();
        assert!(end < 80.0, "Should sweep down to ~50Hz, got {end}");
    }
}
