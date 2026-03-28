#[derive(Clone, Copy, PartialEq)]
pub enum Stage {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

pub struct Envelope {
    stage: Stage,
    value: f32,
    attack: f32,  // seconds
    decay: f32,
    sustain: f32, // 0.0 to 1.0
    release: f32,
    sample_rate: f32,
}

impl Envelope {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            stage: Stage::Idle,
            value: 0.0,
            attack: 0.01,
            decay: 0.1,
            sustain: 0.5,
            release: 0.1,
            sample_rate,
        }
    }

    pub fn set_attack(&mut self, secs: f32) {
        self.attack = secs.max(0.0005);
    }

    pub fn set_decay(&mut self, secs: f32) {
        self.decay = secs.max(0.0005);
    }

    pub fn set_sustain(&mut self, level: f32) {
        self.sustain = level.clamp(0.0, 1.0);
    }

    pub fn set_release(&mut self, secs: f32) {
        self.release = secs.max(0.0005);
    }

    pub fn gate_on(&mut self) {
        // Retrigger from current value (don't reset to 0)
        self.stage = Stage::Attack;
    }

    pub fn gate_off(&mut self) {
        if self.stage != Stage::Idle {
            self.stage = Stage::Release;
        }
    }

    pub fn process(&mut self) -> f32 {
        match self.stage {
            Stage::Idle => {}
            Stage::Attack => {
                // CEM3310-style: RC charge toward 1.5 (overshoot target)
                // This creates the characteristic exponential curve
                let coeff = (-1.0 / (self.attack * self.sample_rate)).exp();
                self.value = 1.5 + (self.value - 1.5) * coeff;
                if self.value >= 1.0 {
                    self.value = 1.0;
                    self.stage = Stage::Decay;
                }
            }
            Stage::Decay => {
                let coeff = (-1.0 / (self.decay * self.sample_rate)).exp();
                self.value = self.sustain + (self.value - self.sustain) * coeff;
                if (self.value - self.sustain).abs() < 0.0001 {
                    self.value = self.sustain;
                    self.stage = Stage::Sustain;
                }
            }
            Stage::Sustain => {
                self.value = self.sustain;
            }
            Stage::Release => {
                let coeff = (-1.0 / (self.release * self.sample_rate)).exp();
                self.value *= coeff;
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

    fn run_samples(env: &mut Envelope, n: usize) -> Vec<f32> {
        (0..n).map(|_| env.process()).collect()
    }

    #[test]
    fn test_idle_is_zero() {
        let mut env = Envelope::new(44100.0);
        let buf = run_samples(&mut env, 100);
        audio_test_harness::level::assert_silent(&buf, 0.0001);
    }

    #[test]
    fn test_attack_reaches_peak() {
        let mut env = Envelope::new(44100.0);
        env.set_attack(0.01); // 10ms attack
        env.set_decay(0.5);
        env.set_sustain(0.5);
        env.set_release(0.5);
        env.gate_on();
        let buf = run_samples(&mut env, 4410); // 100ms — well past 10ms attack
        let peak = audio_test_harness::level::peak(&buf);
        assert!(
            (peak - 1.0).abs() < 0.05,
            "Attack should reach ~1.0, got {peak}"
        );
    }

    #[test]
    fn test_attack_is_exponential() {
        let mut env = Envelope::new(44100.0);
        env.set_attack(0.05); // 50ms
        env.set_decay(1.0);
        env.set_sustain(1.0);
        env.set_release(1.0);
        env.gate_on();
        let buf = run_samples(&mut env, 2205); // 50ms

        // Exponential attack: fast at start, slowing down
        // At 25% of attack time, should be above 50% of peak (exponential)
        // Linear would be at 25%
        // Exponential attack: faster at start than linear
        // At 50% of attack time, should be well above 50% (linear would be 50%)
        let half_idx = buf.len() / 2;
        assert!(
            buf[half_idx] > 0.5,
            "At 50% of attack time, value should be >0.5 (exponential), got {}",
            buf[half_idx]
        );
    }

    #[test]
    fn test_decay_to_sustain() {
        let mut env = Envelope::new(44100.0);
        env.set_attack(0.001);
        env.set_decay(0.05);
        env.set_sustain(0.5);
        env.set_release(1.0);
        env.gate_on();
        let buf = run_samples(&mut env, 44100); // 1 second
        // Should settle near sustain level
        let tail_avg: f32 = buf[22050..].iter().sum::<f32>() / (44100 - 22050) as f32;
        assert!(
            (tail_avg - 0.5).abs() < 0.05,
            "Should settle at sustain 0.5, got {tail_avg}"
        );
    }

    #[test]
    fn test_release_to_zero() {
        let mut env = Envelope::new(44100.0);
        env.set_attack(0.001);
        env.set_decay(0.01);
        env.set_sustain(0.8);
        env.set_release(0.05);
        env.gate_on();
        run_samples(&mut env, 4410); // settle to sustain
        env.gate_off();
        let buf = run_samples(&mut env, 44100); // long time after release
        // Check the very end — well past the release time
        let tail = &buf[22050..];
        audio_test_harness::level::assert_silent(tail, 0.001);
    }

    #[test]
    fn test_retrigger_from_midvalue() {
        let mut env = Envelope::new(44100.0);
        env.set_attack(0.001);
        env.set_decay(0.01);
        env.set_sustain(0.8);
        env.set_release(0.1);
        env.gate_on();
        run_samples(&mut env, 4410); // settle to sustain ~0.8
        env.gate_off();
        run_samples(&mut env, 2205); // release partway (50ms into 100ms release)
        let value_before = env.value();
        assert!(
            value_before > 0.1 && value_before < 0.8,
            "Should be mid-release, got {value_before}"
        );
        env.gate_on(); // retrigger
        let buf = run_samples(&mut env, 441); // 10ms
        // Should attack from current value, not reset to 0
        assert!(
            buf[0] >= value_before * 0.9,
            "Retrigger should start from current value {value_before}, but first sample is {}",
            buf[0]
        );
    }
}
