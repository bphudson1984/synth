/// MiniFreak-inspired arpeggiator with multiple modes, octave range,
/// gate length, swing, and hold.

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ArpMode {
    Off = 0,
    Up = 1,
    Down = 2,
    UpDown = 3,     // inclusive (repeats top/bottom)
    UpDownExcl = 4, // exclusive
    Random = 5,
    Order = 6,      // order played
}

impl ArpMode {
    pub fn from_u8(v: u8) -> Self {
        match v {
            1 => Self::Up,
            2 => Self::Down,
            3 => Self::UpDown,
            4 => Self::UpDownExcl,
            5 => Self::Random,
            6 => Self::Order,
            _ => Self::Off,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ArpDivision {
    Quarter = 0,      // 1/4
    Eighth = 1,       // 1/8
    Sixteenth = 2,    // 1/16
    ThirtySecond = 3, // 1/32
    DottedEighth = 4, // dotted 1/8
    TripletEighth = 5, // triplet 1/8
}

impl ArpDivision {
    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::Quarter,
            1 => Self::Eighth,
            2 => Self::Sixteenth,
            3 => Self::ThirtySecond,
            4 => Self::DottedEighth,
            5 => Self::TripletEighth,
            _ => Self::Eighth,
        }
    }

    /// Samples per step at given BPM and sample rate
    pub fn samples_per_step(&self, bpm: f32, sample_rate: f32) -> f32 {
        let beats_per_sec = bpm / 60.0;
        let quarter_samples = sample_rate / beats_per_sec;
        match self {
            Self::Quarter => quarter_samples,
            Self::Eighth => quarter_samples / 2.0,
            Self::Sixteenth => quarter_samples / 4.0,
            Self::ThirtySecond => quarter_samples / 8.0,
            Self::DottedEighth => quarter_samples * 0.75,
            Self::TripletEighth => quarter_samples / 3.0,
        }
    }
}

pub struct Arpeggiator {
    // Held notes (in order played for "Order" mode)
    held_notes: Vec<u8>,
    // Sorted version for Up/Down modes
    sorted_notes: Vec<u8>,
    // Current state
    step_counter: f32,
    current_index: i32,
    direction: i32, // 1 = up, -1 = down
    current_octave: u8,
    note_on_active: bool,
    gate_counter: f32,
    rng_state: u32,

    // Parameters
    pub mode: ArpMode,
    pub division: ArpDivision,
    pub bpm: f32,           // 30-300
    pub octaves: u8,        // 1-4
    pub gate: f32,          // 0.05-1.0 (percentage of step)
    pub swing: f32,         // 0-1 (0.5 = no swing)
    pub hold: bool,         // latch mode

    sample_rate: f32,
    swing_even: bool, // toggle for swing timing
}

impl Arpeggiator {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            held_notes: Vec::with_capacity(16),
            sorted_notes: Vec::with_capacity(16),
            step_counter: 0.0,
            current_index: 0,
            direction: 1,
            current_octave: 0,
            note_on_active: false,
            gate_counter: 0.0,
            rng_state: 12345,
            mode: ArpMode::Off,
            division: ArpDivision::Eighth,
            bpm: 120.0,
            octaves: 1,
            gate: 0.5,
            swing: 0.5,
            hold: false,
            sample_rate,
            swing_even: false,
        }
    }

    pub fn note_on(&mut self, note: u8) {
        if !self.held_notes.contains(&note) {
            self.held_notes.push(note);
            self.sorted_notes.push(note);
            self.sorted_notes.sort();
        }
        // Reset if first note
        if self.held_notes.len() == 1 {
            self.current_index = 0;
            self.current_octave = 0;
            self.direction = 1;
            self.step_counter = 0.0;
        }
    }

    pub fn note_off(&mut self, note: u8) {
        if self.hold { return; } // latch mode ignores note off
        self.held_notes.retain(|&n| n != note);
        self.sorted_notes.retain(|&n| n != note);
        if self.held_notes.is_empty() {
            self.current_index = 0;
            self.current_octave = 0;
        }
    }

    pub fn all_notes_off(&mut self) {
        self.held_notes.clear();
        self.sorted_notes.clear();
        self.current_index = 0;
        self.current_octave = 0;
    }

    /// Process one sample. Returns Some((note, velocity)) on a new note trigger,
    /// Some((note, 0)) on note off, or None if no event this sample.
    pub fn process(&mut self) -> Option<(u8, u8)> {
        if self.mode == ArpMode::Off || self.sorted_notes.is_empty() {
            return None;
        }

        let step_samples = self.division.samples_per_step(self.bpm, self.sample_rate);

        // Apply swing to even steps
        let swing_offset = if self.swing_even {
            (self.swing - 0.5) * step_samples * 0.5
        } else {
            0.0
        };
        let effective_step = step_samples + swing_offset;

        self.step_counter += 1.0;

        // Gate off
        if self.note_on_active {
            self.gate_counter += 1.0;
            let gate_duration = effective_step * self.gate;
            if self.gate_counter >= gate_duration {
                self.note_on_active = false;
                // Return note-off (velocity 0)
                return Some((0, 0)); // caller handles note-off
            }
        }

        // Step trigger
        if self.step_counter >= effective_step {
            self.step_counter -= effective_step;
            self.gate_counter = 0.0;
            self.swing_even = !self.swing_even;

            let note = self.get_current_note();
            self.advance_step();
            self.note_on_active = true;

            if let Some(base_note) = note {
                let transposed = base_note as u16 + (self.current_octave as u16 * 12);
                if transposed <= 127 {
                    return Some((transposed as u8, 100));
                }
            }
        }

        None
    }

    fn get_current_note(&self) -> Option<u8> {
        let notes = match self.mode {
            ArpMode::Order => &self.held_notes,
            _ => &self.sorted_notes,
        };
        if notes.is_empty() { return None; }
        let idx = (self.current_index as usize) % notes.len();
        Some(notes[idx])
    }

    fn advance_step(&mut self) {
        let notes = match self.mode {
            ArpMode::Order => &self.held_notes,
            _ => &self.sorted_notes,
        };
        let len = notes.len() as i32;
        if len == 0 { return; }

        match self.mode {
            ArpMode::Up | ArpMode::Order => {
                self.current_index += 1;
                if self.current_index >= len {
                    self.current_index = 0;
                    self.advance_octave();
                }
            }
            ArpMode::Down => {
                self.current_index -= 1;
                if self.current_index < 0 {
                    self.current_index = len - 1;
                    self.advance_octave();
                }
            }
            ArpMode::UpDown => {
                self.current_index += self.direction;
                if self.current_index >= len {
                    self.direction = -1;
                    self.current_index = len - 1;
                    if self.current_octave > 0 || self.octaves == 1 {
                        // At top, reverse
                    } else {
                        self.advance_octave();
                        self.current_index = 0;
                        self.direction = 1;
                    }
                } else if self.current_index < 0 {
                    self.direction = 1;
                    self.current_index = 0;
                    self.advance_octave_down();
                }
            }
            ArpMode::UpDownExcl => {
                self.current_index += self.direction;
                if self.current_index >= len {
                    self.direction = -1;
                    self.current_index = (len - 2).max(0);
                } else if self.current_index < 0 {
                    self.direction = 1;
                    self.current_index = 1.min(len - 1);
                    self.advance_octave();
                }
            }
            ArpMode::Random => {
                self.rng_state ^= self.rng_state << 13;
                self.rng_state ^= self.rng_state >> 17;
                self.rng_state ^= self.rng_state << 5;
                self.current_index = (self.rng_state % len as u32) as i32;
                // Random octave too
                if self.octaves > 1 {
                    self.current_octave = (self.rng_state / len as u32 % self.octaves as u32) as u8;
                }
            }
            ArpMode::Off => {}
        }
    }

    fn advance_octave(&mut self) {
        if self.octaves > 1 {
            self.current_octave += 1;
            if self.current_octave >= self.octaves {
                self.current_octave = 0;
            }
        }
    }

    fn advance_octave_down(&mut self) {
        if self.octaves > 1 {
            if self.current_octave == 0 {
                self.current_octave = self.octaves - 1;
            } else {
                self.current_octave -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arp_up_mode() {
        let mut arp = Arpeggiator::new(44100.0);
        arp.mode = ArpMode::Up;
        arp.bpm = 120.0;
        arp.division = ArpDivision::Sixteenth;
        arp.gate = 0.5;

        arp.note_on(60); // C
        arp.note_on(64); // E
        arp.note_on(67); // G

        // Collect note-on events over 1 second
        let mut notes = Vec::new();
        for _ in 0..44100 {
            if let Some((note, vel)) = arp.process() {
                if vel > 0 { notes.push(note); }
            }
        }

        // Should cycle C, E, G, C, E, G...
        assert!(notes.len() >= 6, "Should trigger multiple notes, got {}", notes.len());
        // First three should be C, E, G
        assert_eq!(notes[0], 60);
        assert_eq!(notes[1], 64);
        assert_eq!(notes[2], 67);
    }

    #[test]
    fn test_arp_off_is_silent() {
        let mut arp = Arpeggiator::new(44100.0);
        arp.mode = ArpMode::Off;
        arp.note_on(60);

        let mut events = 0;
        for _ in 0..44100 {
            if arp.process().is_some() { events += 1; }
        }
        assert_eq!(events, 0, "Off mode should produce no events");
    }

    #[test]
    fn test_arp_octave_range() {
        let mut arp = Arpeggiator::new(44100.0);
        arp.mode = ArpMode::Up;
        arp.bpm = 240.0;
        arp.division = ArpDivision::Sixteenth;
        arp.octaves = 2;
        arp.gate = 0.5;

        arp.note_on(60);

        let mut notes = Vec::new();
        for _ in 0..44100 {
            if let Some((note, vel)) = arp.process() {
                if vel > 0 { notes.push(note); }
            }
        }

        // Should have notes at 60 and 72 (one octave up)
        assert!(notes.contains(&60), "Should have base note");
        assert!(notes.contains(&72), "Should have octave-up note");
    }
}
