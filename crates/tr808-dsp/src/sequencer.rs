/// Step sequencer for the TR-808.
/// 16 steps per pattern, 13 instrument tracks, variable tempo.
/// Runs sample-accurate in the audio thread.

pub const NUM_STEPS: usize = 16;
pub const NUM_TRACKS: usize = 13; // One per 808 voice

#[derive(Clone, Copy, PartialEq)]
pub enum PlayState {
    Stopped,
    Playing,
}

#[derive(Clone, Copy)]
pub struct Step {
    pub active: bool,
    pub accent: bool,   // future: louder hit
    pub probability: f32, // 0-1, 1.0 = always plays (future: random muting)
}

impl Default for Step {
    fn default() -> Self {
        Self { active: false, accent: false, probability: 1.0 }
    }
}

#[derive(Clone)]
pub struct Pattern {
    pub tracks: [[Step; NUM_STEPS]; NUM_TRACKS],
    pub length: usize, // 1-16, pattern length (default 16)
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            tracks: [[Step::default(); NUM_STEPS]; NUM_TRACKS],
            length: NUM_STEPS,
        }
    }
}

pub struct Sequencer {
    pattern: Pattern,
    state: PlayState,
    current_step: usize,
    samples_per_step: f32,
    sample_counter: f32,
    sample_rate: f32,
    trigger_pending: bool,

    pub bpm: f32,       // 30-300
    pub swing: f32,     // 0-1
    time_div: u8,       // 0=1/4, 1=1/8, 2=1/16, 3=1/32
}

/// Events emitted by the sequencer each sample.
pub struct SeqEvent {
    pub voice: u8,      // which instrument (0-12)
    pub accent: bool,
}

impl Sequencer {
    pub fn new(sample_rate: f32) -> Self {
        let mut seq = Self {
            pattern: Pattern::default(),
            state: PlayState::Stopped,
            current_step: 0,
            samples_per_step: 0.0,
            sample_counter: 0.0,
            sample_rate,
            trigger_pending: false,
            bpm: 120.0,
            swing: 0.5,
            time_div: 2,
        };
        seq.update_timing();
        seq
    }

    fn update_timing(&mut self) {
        let beats_per_sec = self.bpm / 60.0;
        let div = match self.time_div { 0 => 1.0, 1 => 2.0, 2 => 4.0, _ => 8.0 };
        self.samples_per_step = self.sample_rate / beats_per_sec / div;
    }

    pub fn set_time_div(&mut self, div: u8) {
        self.time_div = div.min(3);
        self.update_timing();
    }

    pub fn play(&mut self) {
        self.state = PlayState::Playing;
        self.current_step = 0;
        self.sample_counter = 0.0;
        self.trigger_pending = true; // fire step 0 immediately
        self.update_timing();
    }

    pub fn stop(&mut self) {
        self.state = PlayState::Stopped;
        self.current_step = 0;
        self.sample_counter = 0.0;
        self.trigger_pending = false;
    }

    pub fn is_playing(&self) -> bool {
        self.state == PlayState::Playing
    }

    pub fn current_step(&self) -> usize {
        self.current_step
    }

    pub fn set_bpm(&mut self, bpm: f32) {
        self.bpm = bpm.clamp(30.0, 300.0);
        self.update_timing();
    }

    /// Set a step on/off for a given track.
    pub fn set_step(&mut self, track: usize, step: usize, active: bool) {
        if track < NUM_TRACKS && step < NUM_STEPS {
            self.pattern.tracks[track][step].active = active;
        }
    }

    /// Toggle a step on a given track.
    pub fn toggle_step(&mut self, track: usize, step: usize) {
        if track < NUM_TRACKS && step < NUM_STEPS {
            self.pattern.tracks[track][step].active = !self.pattern.tracks[track][step].active;
        }
    }

    /// Get step state.
    pub fn get_step(&self, track: usize, step: usize) -> bool {
        if track < NUM_TRACKS && step < NUM_STEPS {
            self.pattern.tracks[track][step].active
        } else {
            false
        }
    }

    /// Set pattern length (1-16).
    pub fn set_length(&mut self, length: usize) {
        self.pattern.length = length.clamp(1, NUM_STEPS);
    }

    /// Clear all steps on all tracks.
    pub fn clear(&mut self) {
        self.pattern = Pattern::default();
    }

    /// Process one sample. Returns a list of voice triggers for this sample.
    /// Call this once per sample in the audio thread.
    pub fn process(&mut self, events: &mut Vec<SeqEvent>) {
        events.clear();

        if self.state != PlayState::Playing {
            return;
        }

        let mut should_trigger = self.trigger_pending;
        self.trigger_pending = false;

        if !should_trigger {
            self.sample_counter += 1.0;

            // Swing: odd steps (1, 3, 5...) are delayed
            let is_odd_step = self.current_step % 2 == 1;
            let swing_offset = if is_odd_step {
                (self.swing - 0.5) * self.samples_per_step * 0.7
            } else {
                0.0
            };
            let step_length = self.samples_per_step + swing_offset;

            if self.sample_counter >= step_length {
                self.sample_counter -= step_length;
                // Advance to next step
                self.current_step = (self.current_step + 1) % self.pattern.length;
                should_trigger = true;
            }
        }

        if should_trigger {
            // Collect triggers for current step
            for track in 0..NUM_TRACKS {
                let step = &self.pattern.tracks[track][self.current_step];
                if step.active {
                    events.push(SeqEvent {
                        voice: track as u8,
                        accent: step.accent,
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequencer_triggers_on_beat() {
        let mut seq = Sequencer::new(44100.0);
        seq.set_bpm(120.0);
        // BD on every beat (steps 0, 4, 8, 12)
        seq.set_step(0, 0, true);
        seq.set_step(0, 4, true);
        seq.set_step(0, 8, true);
        seq.set_step(0, 12, true);
        seq.play();

        let mut events = Vec::new();
        let mut bd_triggers = 0;

        // Run for 2 seconds (should be ~2 bars at 120bpm)
        for _ in 0..88200 {
            seq.process(&mut events);
            for ev in &events {
                if ev.voice == 0 { bd_triggers += 1; }
            }
        }

        // At 120 BPM, 16 steps per bar, 1 bar = 2 seconds.
        // BD on steps 0,4,8,12 = 4 per bar. 2 seconds = 1 bar = 4 triggers.
        // With 88200 samples = exactly 2 bars = 8 triggers.
        // Actually: 120 BPM = 8 sixteenths/sec. 88200 samples / 44100 = 2 sec = 16 sixteenths = 1 bar.
        // So 4 BD triggers per bar, 1 bar in 2 sec = 4 triggers.
        assert!(bd_triggers >= 3 && bd_triggers <= 5,
            "Expected ~4 BD triggers in 1 bar at 120bpm, got {bd_triggers}");
    }

    #[test]
    fn test_sequencer_stopped_no_events() {
        let mut seq = Sequencer::new(44100.0);
        seq.set_step(0, 0, true);
        // Don't call play()

        let mut events = Vec::new();
        let mut count = 0;
        for _ in 0..44100 {
            seq.process(&mut events);
            count += events.len();
        }
        assert_eq!(count, 0, "Stopped sequencer should produce no events");
    }

    #[test]
    fn test_toggle_step() {
        let mut seq = Sequencer::new(44100.0);
        assert!(!seq.get_step(0, 0));
        seq.toggle_step(0, 0);
        assert!(seq.get_step(0, 0));
        seq.toggle_step(0, 0);
        assert!(!seq.get_step(0, 0));
    }

    #[test]
    fn test_pattern_length() {
        let mut seq = Sequencer::new(44100.0);
        seq.set_bpm(240.0); // fast for testing
        seq.set_length(4); // only 4 steps
        seq.set_step(0, 0, true); // BD on step 0
        seq.play();

        let mut events = Vec::new();
        let mut bd_triggers = 0;

        // 1 second at 240 BPM with 4-step pattern
        // 240 BPM = 4 beats/sec = 16 sixteenths/sec
        // 4-step pattern cycles 4 times per second
        for _ in 0..44100 {
            seq.process(&mut events);
            for ev in &events {
                if ev.voice == 0 { bd_triggers += 1; }
            }
        }

        // 4 cycles per second × 1 BD per cycle = ~4 triggers
        assert!(bd_triggers >= 3 && bd_triggers <= 5,
            "Expected ~4 BD triggers with 4-step pattern at 240bpm, got {bd_triggers}");
    }
}
