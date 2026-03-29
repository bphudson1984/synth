pub const NUM_STEPS: usize = 16;

#[derive(Clone, Copy)]
pub struct Step {
    pub note: u8,
    pub gate: bool,
    pub accent: bool,
    pub slide: bool,
}

impl Default for Step {
    fn default() -> Self { Self { note: 48, gate: false, accent: false, slide: false } }
}

#[derive(Clone, Copy, PartialEq)]
enum PlayState { Stopped, Playing }

pub enum AcidSeqEvent {
    NoteOn { note: u8, accent: bool, slide: bool },
    NoteOff,
}

pub struct AcidSequencer {
    pub steps: [Step; NUM_STEPS],
    pub length: usize,
    state: PlayState,
    current_step: usize,
    sample_counter: f32,
    samples_per_step: f32,
    gate_active: bool,
    gate_samples: f32,
    gate_counter: f32,
    bpm: f32,
    sample_rate: f32,
}

impl AcidSequencer {
    pub fn new(sample_rate: f32) -> Self {
        let mut seq = Self {
            steps: [Step::default(); NUM_STEPS], length: NUM_STEPS,
            state: PlayState::Stopped, current_step: 0,
            sample_counter: 0.0, samples_per_step: 0.0,
            gate_active: false, gate_samples: 0.0, gate_counter: 0.0,
            bpm: 120.0, sample_rate,
        };
        seq.update_timing();
        seq
    }

    pub fn play(&mut self) {
        self.state = PlayState::Playing;
        self.current_step = 0;
        self.sample_counter = 0.0;
        self.gate_active = false;
        self.gate_counter = 0.0;
    }

    pub fn stop(&mut self) {
        self.state = PlayState::Stopped;
        self.gate_active = false;
    }

    pub fn set_bpm(&mut self, bpm: f32) {
        self.bpm = bpm.clamp(30.0, 300.0);
        self.update_timing();
    }

    pub fn current_step(&self) -> usize { self.current_step }
    pub fn is_playing(&self) -> bool { self.state == PlayState::Playing }

    fn update_timing(&mut self) {
        let beats_per_sec = self.bpm / 60.0;
        self.samples_per_step = self.sample_rate / beats_per_sec / 4.0;
    }

    pub fn process(&mut self, events: &mut Vec<AcidSeqEvent>) {
        if self.state != PlayState::Playing { return; }

        if self.gate_active {
            self.gate_counter += 1.0;
            if self.gate_counter >= self.gate_samples {
                self.gate_active = false;
                events.push(AcidSeqEvent::NoteOff);
            }
        }

        self.sample_counter += 1.0;
        if self.sample_counter >= self.samples_per_step {
            self.sample_counter -= self.samples_per_step;
            let step = &self.steps[self.current_step];

            if step.gate {
                let next_idx = (self.current_step + 1) % self.length;
                let next_has_slide = self.steps[next_idx].gate && self.steps[next_idx].slide;
                self.gate_samples = if next_has_slide {
                    self.samples_per_step * 1.1
                } else {
                    self.samples_per_step * 0.75
                };
                self.gate_counter = 0.0;
                self.gate_active = true;
                events.push(AcidSeqEvent::NoteOn {
                    note: step.note, accent: step.accent, slide: step.slide,
                });
            } else if self.gate_active {
                self.gate_active = false;
                events.push(AcidSeqEvent::NoteOff);
            }

            self.current_step = (self.current_step + 1) % self.length;
        }
    }

    pub fn clear(&mut self) { self.steps = [Step::default(); NUM_STEPS]; }
}
