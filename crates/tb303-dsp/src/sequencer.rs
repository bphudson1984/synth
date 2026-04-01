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
    trigger_pending: bool,
    bpm: f32,
    sample_rate: f32,
    pub direction: u8,   // 0=forward, 1=reverse, 2=pingpong, 3=random
    pub swing: f32,      // 0-1
    time_div: u8,        // 0=1/4, 1=1/8, 2=1/16, 3=1/32
    step_is_even: bool,
    pingpong_forward: bool,
    rng_state: u32,
}

impl AcidSequencer {
    pub fn new(sample_rate: f32) -> Self {
        let mut seq = Self {
            steps: [Step::default(); NUM_STEPS], length: NUM_STEPS,
            state: PlayState::Stopped, current_step: 0,
            sample_counter: 0.0, samples_per_step: 0.0,
            gate_active: false, gate_samples: 0.0, gate_counter: 0.0,
            trigger_pending: false, bpm: 120.0, sample_rate,
            direction: 0, swing: 0.0, time_div: 2,
            step_is_even: true, pingpong_forward: true, rng_state: 12345,
        };
        seq.update_timing();
        seq
    }

    pub fn set_time_div(&mut self, div: u8) {
        self.time_div = div.min(3);
        self.update_timing();
    }

    pub fn play(&mut self) {
        self.state = PlayState::Playing;
        self.current_step = if self.direction == 1 { self.length.saturating_sub(1) } else { 0 };
        self.sample_counter = 0.0;
        self.gate_active = false;
        self.gate_counter = 0.0;
        self.trigger_pending = true;
        self.step_is_even = true;
        self.pingpong_forward = true;
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
        let div = match self.time_div { 0 => 1.0, 1 => 2.0, 2 => 4.0, _ => 8.0 };
        self.samples_per_step = self.sample_rate / beats_per_sec / div;
    }

    fn rand(&mut self) -> u32 {
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 17;
        self.rng_state ^= self.rng_state << 5;
        self.rng_state
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

        let mut should_trigger = self.trigger_pending;
        self.trigger_pending = false;

        if !should_trigger {
            self.sample_counter += 1.0;
            let effective_step = if self.step_is_even && self.swing > 0.0 {
                self.samples_per_step + self.swing * self.samples_per_step * 0.5
            } else {
                self.samples_per_step
            };
            if self.sample_counter >= effective_step {
                self.sample_counter -= effective_step;
                should_trigger = true;
            }
        }

        if should_trigger {
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

            self.advance_step();
        }
    }

    fn advance_step(&mut self) {
        self.step_is_even = !self.step_is_even;
        match self.direction {
            1 => { // reverse
                if self.current_step == 0 { self.current_step = self.length - 1; }
                else { self.current_step -= 1; }
            }
            2 => { // pingpong
                if self.pingpong_forward {
                    self.current_step += 1;
                    if self.current_step >= self.length {
                        self.current_step = self.length.saturating_sub(2).max(0);
                        self.pingpong_forward = false;
                    }
                } else {
                    if self.current_step == 0 {
                        self.current_step = 1.min(self.length - 1);
                        self.pingpong_forward = true;
                    } else {
                        self.current_step -= 1;
                    }
                }
            }
            3 => { // random
                self.current_step = (self.rand() as usize) % self.length;
            }
            _ => { // forward
                self.current_step = (self.current_step + 1) % self.length;
            }
        }
    }

    pub fn clear(&mut self) { self.steps = [Step::default(); NUM_STEPS]; }
}
