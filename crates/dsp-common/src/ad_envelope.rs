/// Simple Attack-Decay envelope for drum synthesis.
/// No sustain — reaches peak then decays to zero.
pub struct ADEnvelope {
    value: f32,
    stage: Stage,
    attack_coeff: f32,
    decay_coeff: f32,
    sample_rate: f32,
}

#[derive(Clone, Copy, PartialEq)]
enum Stage { Idle, Attack, Decay }

impl ADEnvelope {
    pub fn new(sample_rate: f32) -> Self {
        let mut env = Self {
            value: 0.0,
            stage: Stage::Idle,
            attack_coeff: 0.0,
            decay_coeff: 0.0,
            sample_rate,
        };
        env.set_attack(0.001);
        env.set_decay(0.1);
        env
    }

    /// Set attack time in seconds (typically 0.0005-0.01 for drums)
    pub fn set_attack(&mut self, secs: f32) {
        let t = secs.max(0.0001);
        self.attack_coeff = (-1.0 / (t * self.sample_rate)).exp();
    }

    /// Set decay time in seconds
    pub fn set_decay(&mut self, secs: f32) {
        let t = secs.max(0.0001);
        self.decay_coeff = (-1.0 / (t * self.sample_rate)).exp();
    }

    pub fn trigger(&mut self) {
        self.stage = Stage::Attack;
        // Don't reset value — allows retriggering from current position
    }

    pub fn process(&mut self) -> f32 {
        match self.stage {
            Stage::Idle => {}
            Stage::Attack => {
                // Exponential rise toward 1.2 (overshoot target for fast attack)
                self.value = 1.2 + (self.value - 1.2) * self.attack_coeff;
                if self.value >= 1.0 {
                    self.value = 1.0;
                    self.stage = Stage::Decay;
                }
            }
            Stage::Decay => {
                self.value *= self.decay_coeff;
                if self.value < 0.0001 {
                    self.value = 0.0;
                    self.stage = Stage::Idle;
                }
            }
        }
        self.value
    }

    pub fn value(&self) -> f32 { self.value }
    pub fn is_active(&self) -> bool { self.stage != Stage::Idle }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ad_reaches_peak() {
        let mut env = ADEnvelope::new(44100.0);
        env.set_attack(0.001);
        env.set_decay(0.1);
        env.trigger();

        let mut peak = 0.0f32;
        for _ in 0..4410 {
            let v = env.process();
            peak = peak.max(v);
        }
        assert!((peak - 1.0).abs() < 0.05, "Should reach ~1.0, got {peak}");
    }

    #[test]
    fn test_ad_decays_to_zero() {
        let mut env = ADEnvelope::new(44100.0);
        env.set_attack(0.001);
        env.set_decay(0.05);
        env.trigger();

        // Run for 500ms
        for _ in 0..22050 { env.process(); }
        assert!(env.value() < 0.001, "Should decay to ~0, got {}", env.value());
    }

    #[test]
    fn test_ad_idle_is_zero() {
        let mut env = ADEnvelope::new(44100.0);
        for _ in 0..100 { env.process(); }
        assert_eq!(env.value(), 0.0);
    }
}
