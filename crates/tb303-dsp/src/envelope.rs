#[derive(Clone, Copy, PartialEq)]
enum Stage { Idle, Attack, Decay }

/// TB-303 filter envelope (MEG): fast attack, variable decay, no sustain.
pub struct FilterEnvelope {
    value: f32,
    stage: Stage,
    attack_coeff: f32,
    decay_coeff: f32,
    sample_rate: f32,
}

impl FilterEnvelope {
    pub fn new(sample_rate: f32) -> Self {
        let mut env = Self {
            value: 0.0, stage: Stage::Idle,
            attack_coeff: 0.0, decay_coeff: 0.0, sample_rate,
        };
        env.attack_coeff = coeff(0.003, sample_rate);
        env.set_decay(0.3);
        env
    }

    pub fn set_decay(&mut self, secs: f32) {
        self.decay_coeff = coeff(secs.clamp(0.03, 3.0), self.sample_rate);
    }

    pub fn trigger(&mut self) { self.stage = Stage::Attack; }

    pub fn process(&mut self) -> f32 {
        match self.stage {
            Stage::Idle => {}
            Stage::Attack => {
                self.value += (1.2 - self.value) * (1.0 - self.attack_coeff);
                if self.value >= 1.0 { self.value = 1.0; self.stage = Stage::Decay; }
            }
            Stage::Decay => {
                self.value *= self.decay_coeff;
                if self.value < 0.0001 { self.value = 0.0; self.stage = Stage::Idle; }
            }
        }
        self.value
    }
}

/// TB-303 amp envelope (VEG): fast attack, long fixed decay.
pub struct AmpEnvelope {
    value: f32,
    stage: Stage,
    attack_coeff: f32,
    decay_coeff: f32,
}

impl AmpEnvelope {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            value: 0.0, stage: Stage::Idle,
            attack_coeff: coeff(0.003, sample_rate),
            decay_coeff: coeff(3.0, sample_rate),
        }
    }

    pub fn trigger(&mut self) { self.stage = Stage::Attack; }

    pub fn gate_off(&mut self) {
        if self.stage != Stage::Idle { self.stage = Stage::Decay; }
    }

    pub fn process(&mut self) -> f32 {
        match self.stage {
            Stage::Idle => {}
            Stage::Attack => {
                self.value += (1.2 - self.value) * (1.0 - self.attack_coeff);
                if self.value >= 1.0 { self.value = 1.0; self.stage = Stage::Decay; }
            }
            Stage::Decay => {
                self.value *= self.decay_coeff;
                if self.value < 0.0001 { self.value = 0.0; self.stage = Stage::Idle; }
            }
        }
        self.value
    }
}

fn coeff(secs: f32, sr: f32) -> f32 { (-1.0 / (secs * sr)).exp() }
