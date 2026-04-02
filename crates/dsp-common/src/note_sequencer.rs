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
    pub gate_pct: u16,   // percentage of step length; >100 sustains across multiple steps
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

pub enum NoteSeqEvent {
    NoteOn { notes: [u8; MAX_NOTES_PER_STEP], num_notes: u8, velocity: u8 },
    NoteOff,
}

pub struct NoteSequencer {
    pub steps: [Step; MAX_STEPS],
    pub length: usize,
    state: PlayState,
    current_step: usize,
    display_step: usize, // the step that was last triggered (for UI)
    sample_counter: f32,
    samples_per_step: f32,
    gate_active: bool,
    gate_samples: f32,
    gate_counter: f32,
    active_notes: [u8; MAX_NOTES_PER_STEP],
    active_num_notes: u8,
    trigger_pending: bool,
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

impl NoteSequencer {
    pub fn new(sample_rate: f32) -> Self {
        let mut seq = Self {
            steps: [Step::default(); MAX_STEPS],
            length: PAGE_SIZE,
            state: PlayState::Stopped, current_step: 0, display_step: 0,
            sample_counter: 0.0, samples_per_step: 0.0,
            gate_active: false, gate_samples: 0.0, gate_counter: 0.0,
            active_notes: [0; MAX_NOTES_PER_STEP], active_num_notes: 0,
            trigger_pending: false,
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
        self.display_step = self.current_step;
        self.sample_counter = 0.0;
        self.gate_active = false;
        self.gate_counter = 0.0;
        self.trigger_pending = true; // fire step 0 immediately
        self.ratchet_count = 0;
        self.ping_dir = 1;
        self.step_is_even = false;
    }

    pub fn stop(&mut self) {
        self.state = PlayState::Stopped;
        self.gate_active = false;
        self.ratchet_count = 0;
    }

    pub fn set_bpm(&mut self, bpm: f32) {
        let old_sps = self.samples_per_step;
        self.bpm = bpm.clamp(30.0, 300.0);
        self.update_timing();
        if old_sps > 0.0 {
            let ratio = self.sample_counter / old_sps;
            self.sample_counter = ratio * self.samples_per_step;
        }
    }
    pub fn set_length(&mut self, len: usize) { self.length = len.clamp(PAGE_SIZE, MAX_STEPS); }
    pub fn current_step(&self) -> usize { self.display_step }
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

    pub fn set_time_div(&mut self, div: u8) {
        let old_sps = self.samples_per_step;
        self.time_div = div;
        self.update_timing();
        // Scale counter proportionally to new rate so position within step is preserved
        if old_sps > 0.0 {
            let ratio = self.sample_counter / old_sps;
            self.sample_counter = ratio * self.samples_per_step;
        }
    }

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

    pub fn process(&mut self, events: &mut Vec<NoteSeqEvent>) {
        if self.state != PlayState::Playing { return; }

        // Ratchet processing — subdivided retriggering within a step
        if self.ratchet_count > 0 {
            self.ratchet_counter += 1.0;
            // Gate off within ratchet
            if self.gate_active {
                self.gate_counter += 1.0;
                if self.gate_counter >= self.gate_samples {
                    self.gate_active = false;
                    events.push(NoteSeqEvent::NoteOff);
                }
            }
            if self.ratchet_counter >= self.ratchet_samples {
                self.ratchet_counter -= self.ratchet_samples;
                self.ratchet_idx += 1;
                if self.ratchet_idx < self.ratchet_count {
                    // Retrigger
                    events.push(NoteSeqEvent::NoteOff);
                    self.gate_samples = self.ratchet_samples * (self.ratchet_step.gate_pct as f32 / 100.0);
                    self.gate_counter = 0.0;
                    self.gate_active = true;
                    events.push(NoteSeqEvent::NoteOn {
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
                    self.display_step = self.current_step;
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
                events.push(NoteSeqEvent::NoteOff);
            }
        }

        // Immediate trigger on play (step 0)
        let mut should_trigger = self.trigger_pending;
        self.trigger_pending = false;

        if !should_trigger {
            self.sample_counter += 1.0;
            if self.sample_counter >= self.effective_step_samples() {
                self.sample_counter -= self.effective_step_samples();
                should_trigger = true;
            }
        }

        if should_trigger {
            self.display_step = self.current_step;
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

    fn trigger_current_step(&mut self, events: &mut Vec<NoteSeqEvent>) {
        let step = self.steps[self.current_step];

        // Long gate still sustaining — don't let other steps interrupt it
        if self.gate_active && self.gate_counter < self.gate_samples {
            if !step.gate || step.num_notes == 0 {
                // Empty step: let the sustained note ring through
                return;
            }
            if self.active_notes == step.notes
                && self.active_num_notes == step.num_notes
            {
                // Same note: don't retrigger, let it ring
                return;
            }
            // Different note: cut the sustained gate and fall through to play it
        }

        // Release previous note if gate is still active
        if self.gate_active {
            self.gate_active = false;
            events.push(NoteSeqEvent::NoteOff);
        }

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
        self.active_notes = step.notes;
        self.active_num_notes = step.num_notes;
        events.push(NoteSeqEvent::NoteOn {
            notes: step.notes, num_notes: step.num_notes, velocity: step.velocity,
        });
    }

    pub fn clear(&mut self) {
        self.steps = [Step::default(); MAX_STEPS];
        self.length = PAGE_SIZE;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collect_events(seq: &mut NoteSequencer, num_samples: usize) -> Vec<NoteSeqEvent> {
        let mut all_events = Vec::new();
        let mut frame_events = Vec::new();
        for _ in 0..num_samples {
            frame_events.clear();
            seq.process(&mut frame_events);
            all_events.extend(frame_events.drain(..));
        }
        all_events
    }

    #[test]
    fn test_stopped_produces_no_events() {
        let mut seq = NoteSequencer::new(44100.0);
        seq.steps[0].gate = true;
        seq.steps[0].num_notes = 1;
        let events = collect_events(&mut seq, 44100);
        assert!(events.is_empty());
    }

    #[test]
    fn test_single_step_triggers_note_on() {
        let mut seq = NoteSequencer::new(44100.0);
        seq.steps[0].gate = true;
        seq.steps[0].notes[0] = 60;
        seq.steps[0].num_notes = 1;
        seq.steps[0].velocity = 100;
        seq.set_length(1);
        seq.set_bpm(120.0);
        seq.play();

        let events = collect_events(&mut seq, 44100);
        let note_ons: Vec<_> = events.iter().filter(|e| matches!(e, NoteSeqEvent::NoteOn { .. })).collect();
        assert!(!note_ons.is_empty(), "should have at least one NoteOn");
    }

    #[test]
    fn test_gate_off_step_skipped() {
        let mut seq = NoteSequencer::new(44100.0);
        seq.steps[0].gate = false;
        seq.steps[0].num_notes = 1;
        seq.set_length(1);
        seq.set_bpm(240.0);
        seq.play();

        let events = collect_events(&mut seq, 22050);
        let note_ons: Vec<_> = events.iter().filter(|e| matches!(e, NoteSeqEvent::NoteOn { .. })).collect();
        assert!(note_ons.is_empty(), "gate=false should produce no NoteOn");
    }

    #[test]
    fn test_note_off_after_gate() {
        let mut seq = NoteSequencer::new(44100.0);
        seq.steps[0].gate = true;
        seq.steps[0].notes[0] = 60;
        seq.steps[0].num_notes = 1;
        seq.steps[0].velocity = 100;
        seq.steps[0].gate_pct = 50;
        seq.set_length(1);
        seq.set_bpm(120.0);
        seq.play();

        let events = collect_events(&mut seq, 44100);
        let note_offs: Vec<_> = events.iter().filter(|e| matches!(e, NoteSeqEvent::NoteOff)).collect();
        assert!(!note_offs.is_empty(), "should have NoteOff after gate expires");
    }

    #[test]
    fn test_skip_step() {
        let mut seq = NoteSequencer::new(44100.0);
        seq.steps[0].gate = true;
        seq.steps[0].notes[0] = 60;
        seq.steps[0].num_notes = 1;
        seq.steps[0].skip = true;
        seq.set_length(1);
        seq.set_bpm(240.0);
        seq.play();

        let events = collect_events(&mut seq, 22050);
        let note_ons: Vec<_> = events.iter().filter(|e| matches!(e, NoteSeqEvent::NoteOn { .. })).collect();
        assert!(note_ons.is_empty(), "skip=true should produce no NoteOn");
    }

    #[test]
    fn test_velocity_passed_through() {
        let mut seq = NoteSequencer::new(44100.0);
        seq.steps[0].gate = true;
        seq.steps[0].notes[0] = 60;
        seq.steps[0].num_notes = 1;
        seq.steps[0].velocity = 77;
        seq.set_length(1);
        seq.set_bpm(240.0);
        seq.play();

        let events = collect_events(&mut seq, 22050);
        for e in &events {
            if let NoteSeqEvent::NoteOn { velocity, .. } = e {
                assert_eq!(*velocity, 77);
                return;
            }
        }
        panic!("no NoteOn event found");
    }

    #[test]
    fn test_multi_note_step() {
        let mut seq = NoteSequencer::new(44100.0);
        seq.steps[0].gate = true;
        seq.steps[0].notes = [60, 64, 67, 0];
        seq.steps[0].num_notes = 3;
        seq.steps[0].velocity = 100;
        seq.set_length(1);
        seq.set_bpm(240.0);
        seq.play();

        let events = collect_events(&mut seq, 22050);
        for e in &events {
            if let NoteSeqEvent::NoteOn { notes, num_notes, .. } = e {
                assert_eq!(*num_notes, 3);
                assert_eq!(notes[0], 60);
                assert_eq!(notes[1], 64);
                assert_eq!(notes[2], 67);
                return;
            }
        }
        panic!("no NoteOn event found");
    }

    #[test]
    fn test_direction_reverse() {
        let mut seq = NoteSequencer::new(44100.0);
        for i in 0..4 {
            seq.steps[i].gate = true;
            seq.steps[i].notes[0] = 60 + i as u8;
            seq.steps[i].num_notes = 1;
            seq.steps[i].velocity = 100;
        }
        seq.set_length(4);
        seq.direction = 1; // reverse
        seq.set_bpm(240.0);
        seq.play();

        // Collect note values in order
        let events = collect_events(&mut seq, 88200);
        let notes: Vec<u8> = events.iter().filter_map(|e| {
            if let NoteSeqEvent::NoteOn { notes, .. } = e { Some(notes[0]) } else { None }
        }).collect();

        // First note should be the last step (63), then descending
        assert!(notes.len() >= 4, "should have at least 4 notes, got {}", notes.len());
        assert_eq!(notes[0], 63, "reverse should start from last step");
    }

    #[test]
    fn test_clear_resets_steps() {
        let mut seq = NoteSequencer::new(44100.0);
        seq.steps[0].gate = true;
        seq.steps[0].notes[0] = 72;
        seq.steps[0].num_notes = 1;
        seq.clear();
        assert!(!seq.steps[0].gate);
    }

    #[test]
    fn test_bpm_affects_timing() {
        // Fast BPM should produce more events in same time period
        // Use 4 steps so we can count multiple loops
        let samples = 44100 * 3; // 3 seconds

        let mut seq_fast = NoteSequencer::new(44100.0);
        seq_fast.steps[0].gate = true;
        seq_fast.steps[0].num_notes = 1;
        seq_fast.steps[0].velocity = 100;
        seq_fast.steps[0].gate_pct = 25; // short gate so NoteOff happens quickly
        seq_fast.set_length(4); // 4 steps to cycle through
        seq_fast.set_bpm(240.0);
        seq_fast.play();
        let events_fast = collect_events(&mut seq_fast, samples);
        let ons_fast = events_fast.iter().filter(|e| matches!(e, NoteSeqEvent::NoteOn { .. })).count();

        let mut seq_slow = NoteSequencer::new(44100.0);
        seq_slow.steps[0].gate = true;
        seq_slow.steps[0].num_notes = 1;
        seq_slow.steps[0].velocity = 100;
        seq_slow.steps[0].gate_pct = 25;
        seq_slow.set_length(4);
        seq_slow.set_bpm(60.0);
        seq_slow.play();
        let events_slow = collect_events(&mut seq_slow, samples);
        let ons_slow = events_slow.iter().filter(|e| matches!(e, NoteSeqEvent::NoteOn { .. })).count();

        assert!(
            ons_fast > ons_slow,
            "240bpm ({ons_fast} notes) should trigger more than 60bpm ({ons_slow} notes)"
        );
    }
}
