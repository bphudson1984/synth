#[derive(Clone, Copy, PartialEq)]
enum Stage { Idle, Attack, Decay, Sustain, Release }

pub struct Envelope {
    stage: Stage,
    value: f32,
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    sample_rate: f32,
}

impl Envelope {
    pub fn new(sample_rate: f32) -> Self {
        Self { stage: Stage::Idle, value: 0.0, attack: 0.01, decay: 0.1, sustain: 0.5, release: 0.1, sample_rate }
    }
    pub fn set_attack(&mut self, s: f32) { self.attack = s.max(0.0005); }
    pub fn set_decay(&mut self, s: f32) { self.decay = s.max(0.0005); }
    pub fn set_sustain(&mut self, l: f32) { self.sustain = l.clamp(0.0, 1.0); }
    pub fn set_release(&mut self, s: f32) { self.release = s.max(0.0005); }
    pub fn gate_on(&mut self) { self.stage = Stage::Attack; }
    pub fn gate_off(&mut self) { if self.stage != Stage::Idle { self.stage = Stage::Release; } }
    pub fn is_active(&self) -> bool { self.stage != Stage::Idle }

    pub fn process(&mut self) -> f32 {
        match self.stage {
            Stage::Idle => {}
            Stage::Attack => {
                let c = (-1.0 / (self.attack * self.sample_rate)).exp();
                self.value = 1.5 + (self.value - 1.5) * c;
                if self.value >= 1.0 { self.value = 1.0; self.stage = Stage::Decay; }
            }
            Stage::Decay => {
                let c = (-1.0 / (self.decay * self.sample_rate)).exp();
                self.value = self.sustain + (self.value - self.sustain) * c;
                if (self.value - self.sustain).abs() < 0.0001 { self.value = self.sustain; self.stage = Stage::Sustain; }
            }
            Stage::Sustain => { self.value = self.sustain; }
            Stage::Release => {
                let c = (-1.0 / (self.release * self.sample_rate)).exp();
                self.value *= c;
                if self.value < 0.0001 { self.value = 0.0; self.stage = Stage::Idle; }
            }
        }
        self.value
    }
}
