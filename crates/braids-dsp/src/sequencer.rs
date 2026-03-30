pub const PAGE_SIZE: usize = 16;
pub const MAX_PAGES: usize = 8;
pub const MAX_STEPS: usize = PAGE_SIZE * MAX_PAGES;
pub const MAX_NOTES_PER_STEP: usize = 4;

#[derive(Clone, Copy)]
pub struct Step {
    pub notes: [u8; MAX_NOTES_PER_STEP],
    pub num_notes: u8,
    pub gate: bool,
    pub velocity: u8,    // 0-127
    pub gate_pct: u8,    // 0-100 (percentage of step length)
    pub probability: u8, // 0-100
    pub ratchet: u8,     // 1-4 (subdivisions)
    pub skip: bool,
}

impl Default for Step {
    fn default() -> Self {
        Self {
            notes: [48, 0, 0, 0], num_notes: 1, gate: false,
            velocity: 100, gate_pct: 75, probability: 100, ratchet: 1, skip: false,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum PlayState { Stopped, Playing }

pub enum LeadSeqEvent {
    NoteOn { notes: [u8; MAX_NOTES_PER_STEP], num_notes: u8, velocity: u8 },
    NoteOff,
}

pub struct LeadSequencer {
    pub steps: [Step; MAX_STEPS],
    pub length: usize,
    state: PlayState,
    current_step: usize,
    sample_counter: f32,
    samples_per_step: f32,
    gate_active: bool,
    gate_samples: f32,
    gate_counter: f32,
    // Ratchet state
    ratchet_count: u8,
    ratchet_idx: u8,
    ratchet_samples: f32,
    ratchet_counter: f32,
    ratchet_step: Step,
    // Pattern settings
    pub direction: u8,     // 0=fwd, 1=rev, 2=pingpong, 3=random
    pub swing: f32,        // 0.0-1.0
    pub time_div: u8,      // 0=1/4, 1=1/8, 2=1/16, 3=1/32
    ping_dir: i32,         // 1 or -1 for ping-pong
    bpm: f32,
    sample_rate: f32,
    rng: u32,
    step_is_even: bool,
}

impl LeadSequencer {
    pub fn new(sample_rate: f32) -> Self {
        let mut seq = Self {
            steps: [Step::default(); MAX_STEPS],
            length: PAGE_SIZE,
            state: PlayState::Stopped, current_step: 0,
            sample_counter: 0.0, samples_per_step: 0.0,
            gate_active: false, gate_samples: 0.0, gate_counter: 0.0,
            ratchet_count: 0, ratchet_idx: 0, ratchet_samples: 0.0,
            ratchet_counter: 0.0, ratchet_step: Step::default(),
            direction: 0, swing: 0.0, time_div: 2, // default 1/16
            ping_dir: 1, bpm: 120.0, sample_rate, rng: 54321,
            step_is_even: false,
        };
        seq.update_timing();
        seq
    }

    pub fn play(&mut self) {
        self.state = PlayState::Playing;
        self.current_step = if self.direction == 1 { self.length - 1 } else { 0 };
        self.sample_counter = 0.0;
        self.gate_active = false;
        self.gate_counter = 0.0;
        self.ratchet_count = 0;
        self.ping_dir = 1;
        self.step_is_even = false;
    }

    pub fn stop(&mut self) {
        self.state = PlayState::Stopped;
        self.gate_active = false;
        self.ratchet_count = 0;
    }

    pub fn set_bpm(&mut self, bpm: f32) { self.bpm = bpm.clamp(30.0, 300.0); self.update_timing(); }
    pub fn set_length(&mut self, len: usize) { self.length = len.clamp(PAGE_SIZE, MAX_STEPS); }
    pub fn current_step(&self) -> usize { self.current_step }
    pub fn is_playing(&self) -> bool { self.state == PlayState::Playing }

    fn update_timing(&mut self) {
        let beats_per_sec = self.bpm / 60.0;
        let quarter = self.sample_rate / beats_per_sec;
        self.samples_per_step = match self.time_div {
            0 => quarter,        // 1/4
            1 => quarter / 2.0,  // 1/8
            3 => quarter / 8.0,  // 1/32
            _ => quarter / 4.0,  // 1/16 (default)
        };
    }

    pub fn set_time_div(&mut self, div: u8) { self.time_div = div; self.update_timing(); }

    pub fn rotate(&mut self, dir: i32) {
        let len = self.length;
        if len == 0 { return; }
        let mut buf = [Step::default(); MAX_STEPS];
        for i in 0..len {
            let src = if dir > 0 {
                if i == 0 { len - 1 } else { i - 1 }
            } else {
                (i + 1) % len
            };
            buf[i] = self.steps[src];
        }
        for i in 0..len { self.steps[i] = buf[i]; }
    }

    fn rand(&mut self) -> u32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 17;
        self.rng ^= self.rng << 5;
        self.rng
    }

    fn advance_step(&mut self) {
        match self.direction {
            1 => { // reverse
                if self.current_step == 0 { self.current_step = self.length - 1; }
                else { self.current_step -= 1; }
            }
            2 => { // ping-pong
                let next = self.current_step as i32 + self.ping_dir;
                if next >= self.length as i32 {
                    self.ping_dir = -1;
                    self.current_step = if self.length >= 2 { self.length - 2 } else { 0 };
                } else if next < 0 {
                    self.ping_dir = 1;
                    self.current_step = if self.length >= 2 { 1 } else { 0 };
                } else {
                    self.current_step = next as usize;
                }
            }
            3 => { // random
                self.current_step = (self.rand() as usize) % self.length;
            }
            _ => { // forward
                self.current_step = (self.current_step + 1) % self.length;
            }
        }
        self.step_is_even = !self.step_is_even;
    }

    pub fn process(&mut self, events: &mut Vec<LeadSeqEvent>) {
        if self.state != PlayState::Playing { return; }

        // Ratchet processing — subdivided retriggering within a step
        if self.ratchet_count > 0 {
            self.ratchet_counter += 1.0;
            // Gate off within ratchet
            if self.gate_active {
                self.gate_counter += 1.0;
                if self.gate_counter >= self.gate_samples {
                    self.gate_active = false;
                    events.push(LeadSeqEvent::NoteOff);
                }
            }
            if self.ratchet_counter >= self.ratchet_samples {
                self.ratchet_counter -= self.ratchet_samples;
                self.ratchet_idx += 1;
                if self.ratchet_idx < self.ratchet_count {
                    // Retrigger
                    events.push(LeadSeqEvent::NoteOff);
                    self.gate_samples = self.ratchet_samples * (self.ratchet_step.gate_pct as f32 / 100.0);
                    self.gate_counter = 0.0;
                    self.gate_active = true;
                    events.push(LeadSeqEvent::NoteOn {
                        notes: self.ratchet_step.notes,
                        num_notes: self.ratchet_step.num_notes,
                        velocity: self.ratchet_step.velocity,
                    });
                } else {
                    self.ratchet_count = 0;
                }
            }
            // Don't process step advance while ratcheting (will happen naturally)
            if self.ratchet_count > 0 {
                self.sample_counter += 1.0;
                if self.sample_counter >= self.effective_step_samples() {
                    self.sample_counter -= self.effective_step_samples();
                    self.ratchet_count = 0; // force end ratchet on step boundary
                    self.trigger_current_step(events);
                    self.advance_step();
                }
                return;
            }
        }

        // Normal gate off
        if self.gate_active {
            self.gate_counter += 1.0;
            if self.gate_counter >= self.gate_samples {
                self.gate_active = false;
                events.push(LeadSeqEvent::NoteOff);
            }
        }

        // Step boundary
        self.sample_counter += 1.0;
        if self.sample_counter >= self.effective_step_samples() {
            self.sample_counter -= self.effective_step_samples();
            self.trigger_current_step(events);
            self.advance_step();
        }
    }

    fn effective_step_samples(&self) -> f32 {
        let base = self.samples_per_step;
        if self.step_is_even && self.swing > 0.0 {
            base + self.swing * base * 0.5
        } else {
            base
        }
    }

    fn trigger_current_step(&mut self, events: &mut Vec<LeadSeqEvent>) {
        let step = self.steps[self.current_step];
        if step.skip || !step.gate || step.num_notes == 0 { return; }

        // Probability check
        if step.probability < 100 {
            if (self.rand() % 100) >= step.probability as u32 { return; }
        }

        if step.ratchet > 1 {
            // Start ratchet sequence
            self.ratchet_count = step.ratchet;
            self.ratchet_idx = 0;
            self.ratchet_samples = self.samples_per_step / step.ratchet as f32;
            self.ratchet_counter = 0.0;
            self.ratchet_step = step;
        }

        self.gate_samples = if step.ratchet > 1 {
            (self.samples_per_step / step.ratchet as f32) * (step.gate_pct as f32 / 100.0)
        } else {
            self.samples_per_step * (step.gate_pct as f32 / 100.0)
        };
        self.gate_counter = 0.0;
        self.gate_active = true;
        events.push(LeadSeqEvent::NoteOn {
            notes: step.notes, num_notes: step.num_notes, velocity: step.velocity,
        });
    }

    pub fn clear(&mut self) {
        self.steps = [Step::default(); MAX_STEPS];
        self.length = PAGE_SIZE;
    }
}
