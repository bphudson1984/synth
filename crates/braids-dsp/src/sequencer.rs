pub const PAGE_SIZE: usize = 16;
pub const MAX_PAGES: usize = 8;
pub const MAX_STEPS: usize = PAGE_SIZE * MAX_PAGES;
pub const MAX_NOTES_PER_STEP: usize = 4;

#[derive(Clone, Copy)]
pub struct Step {
    pub notes: [u8; MAX_NOTES_PER_STEP],
    pub num_notes: u8,
    pub gate: bool,
}

impl Default for Step {
    fn default() -> Self { Self { notes: [48, 0, 0, 0], num_notes: 1, gate: false } }
}

#[derive(Clone, Copy, PartialEq)]
enum PlayState { Stopped, Playing }

pub enum LeadSeqEvent {
    NoteOn { notes: [u8; MAX_NOTES_PER_STEP], num_notes: u8 },
    NoteOff,
}

pub struct LeadSequencer {
    pub steps: [Step; MAX_STEPS],
    pub length: usize, // active number of steps (multiple of 16)
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

impl LeadSequencer {
    pub fn new(sample_rate: f32) -> Self {
        let mut seq = Self {
            steps: [Step::default(); MAX_STEPS],
            length: PAGE_SIZE,
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

    pub fn stop(&mut self) { self.state = PlayState::Stopped; self.gate_active = false; }
    pub fn set_bpm(&mut self, bpm: f32) { self.bpm = bpm.clamp(30.0, 300.0); self.update_timing(); }
    pub fn set_length(&mut self, len: usize) { self.length = len.clamp(PAGE_SIZE, MAX_STEPS); }
    pub fn current_step(&self) -> usize { self.current_step }
    pub fn is_playing(&self) -> bool { self.state == PlayState::Playing }

    fn update_timing(&mut self) {
        self.samples_per_step = self.sample_rate / (self.bpm / 60.0) / 4.0;
    }

    pub fn process(&mut self, events: &mut Vec<LeadSeqEvent>) {
        if self.state != PlayState::Playing { return; }

        if self.gate_active {
            self.gate_counter += 1.0;
            if self.gate_counter >= self.gate_samples {
                self.gate_active = false;
                events.push(LeadSeqEvent::NoteOff);
            }
        }

        self.sample_counter += 1.0;
        if self.sample_counter >= self.samples_per_step {
            self.sample_counter -= self.samples_per_step;
            let step = &self.steps[self.current_step];

            if step.gate && step.num_notes > 0 {
                self.gate_samples = self.samples_per_step * 0.75;
                self.gate_counter = 0.0;
                self.gate_active = true;
                events.push(LeadSeqEvent::NoteOn { notes: step.notes, num_notes: step.num_notes });
            } else if self.gate_active {
                self.gate_active = false;
                events.push(LeadSeqEvent::NoteOff);
            }

            self.current_step = (self.current_step + 1) % self.length;
        }
    }

    pub fn clear(&mut self) {
        self.steps = [Step::default(); MAX_STEPS];
        self.length = PAGE_SIZE;
    }
}
