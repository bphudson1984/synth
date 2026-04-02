#[derive(Clone, Copy, PartialEq)]
enum Stage { Idle, Attack, Decay, Sustain, Release }

/// Volca Bass ADR envelope with sustain switch.
///
/// - Sustain OFF: Attack → Decay to zero (AD behavior)
/// - Sustain ON: Attack → hold at 1.0 until gate off → Release
/// - Decay and Release share the same time parameter
pub struct BassEnvelope {
    value: f32,
    stage: Stage,
    attack_coeff: f32,
    decay_coeff: f32,
    sample_rate: f32,
    pub sustain_on: bool,
}

impl BassEnvelope {
    pub fn new(sample_rate: f32) -> Self {
        let mut env = Self {
            value: 0.0,
            stage: Stage::Idle,
            attack_coeff: 0.0,
            decay_coeff: 0.0,
            sample_rate,
            sustain_on: false,
        };
        env.set_attack(0.005);
        env.set_decay_release(0.3);
        env
    }

    pub fn set_attack(&mut self, secs: f32) {
        self.attack_coeff = coeff(secs.clamp(0.0005, 10.0), self.sample_rate);
    }

    pub fn set_decay_release(&mut self, secs: f32) {
        self.decay_coeff = coeff(secs.clamp(0.01, 10.0), self.sample_rate);
    }

    pub fn trigger(&mut self) {
        self.stage = Stage::Attack;
    }

    pub fn gate_off(&mut self) {
        match self.stage {
            Stage::Attack | Stage::Sustain => {
                self.stage = Stage::Release;
            }
            Stage::Decay if self.sustain_on => {
                self.stage = Stage::Release;
            }
            _ => {}
        }
    }

    pub fn process(&mut self) -> f32 {
        match self.stage {
            Stage::Idle => {}
            Stage::Attack => {
                self.value += (1.2 - self.value) * (1.0 - self.attack_coeff);
                if self.value >= 1.0 {
                    self.value = 1.0;
                    if self.sustain_on {
                        self.stage = Stage::Sustain;
                    } else {
                        self.stage = Stage::Decay;
                    }
                }
            }
            Stage::Decay => {
                self.value *= self.decay_coeff;
                if self.value < 0.0001 {
                    self.value = 0.0;
                    self.stage = Stage::Idle;
                }
            }
            Stage::Sustain => {
                // Hold at 1.0 until gate_off
            }
            Stage::Release => {
                self.value *= self.decay_coeff;
                if self.value < 0.0001 {
                    self.value = 0.0;
                    self.stage = Stage::Idle;
                }
            }
        }
        self.value
    }
}

fn coeff(secs: f32, sr: f32) -> f32 {
    (-1.0 / (secs * sr)).exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_envelope_attack_reaches_peak() {
        let mut env = BassEnvelope::new(44100.0);
        env.set_attack(0.01);
        env.trigger();
        let mut peak = 0.0f32;
        for _ in 0..4410 {
            peak = peak.max(env.process());
        }
        assert!((peak - 1.0).abs() < 0.01, "Envelope should reach 1.0, got {peak}");
    }

    #[test]
    fn test_envelope_ad_mode() {
        let mut env = BassEnvelope::new(44100.0);
        env.sustain_on = false;
        env.set_attack(0.001);
        env.set_decay_release(0.1);
        env.trigger();
        // Process through attack + decay
        for _ in 0..44100 {
            env.process();
        }
        assert!(env.process() < 0.001, "AD envelope should decay to zero");
    }

    #[test]
    fn test_envelope_sustain_holds() {
        let mut env = BassEnvelope::new(44100.0);
        env.sustain_on = true;
        env.set_attack(0.001);
        env.trigger();
        // Process through attack into sustain
        for _ in 0..4410 {
            env.process();
        }
        // Should be holding at ~1.0
        let val = env.process();
        assert!(val > 0.95, "Sustain should hold at 1.0, got {val}");
    }

    #[test]
    fn test_envelope_release_after_sustain() {
        let mut env = BassEnvelope::new(44100.0);
        env.sustain_on = true;
        env.set_attack(0.001);
        env.set_decay_release(0.1);
        env.trigger();
        for _ in 0..4410 { env.process(); }
        env.gate_off();
        for _ in 0..44100 { env.process(); }
        assert!(env.process() < 0.001, "Should decay to zero after release");
    }
}
