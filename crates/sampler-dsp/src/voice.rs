/// Hermite cubic interpolation for high-quality pitched sample playback.
#[inline(always)]
fn cubic(y0: f32, y1: f32, y2: f32, y3: f32, frac: f32) -> f32 {
    let c0 = y1;
    let c1 = 0.5 * (y2 - y0);
    let c2 = y0 - 2.5 * y1 + 2.0 * y2 - 0.5 * y3;
    let c3 = 0.5 * (y3 - y0) + 1.5 * (y1 - y2);
    ((c3 * frac + c2) * frac + c1) * frac + c0
}

// =====================================================================
// PLAY MODE
// =====================================================================

#[derive(Clone, Copy, PartialEq)]
pub enum PlayMode {
    OneShot, // plays to end, ignores note-off
    Gate,    // plays while held, release envelope on note-off
    Loop,    // loops until stopped
}

impl PlayMode {
    pub fn from_u8(v: u8) -> Self {
        match v {
            1 => PlayMode::Gate,
            2 => PlayMode::Loop,
            _ => PlayMode::OneShot,
        }
    }
}

// =====================================================================
// SAMPLE SLOT — holds PCM data + per-pad settings
// =====================================================================

pub struct SampleSlot {
    pub data_l: Vec<f32>,
    pub data_r: Vec<f32>,
    pub length: usize,
    pub sample_rate: f32,
    pub volume: f32,
    pub pitch: f32,        // semitones (-24 to +24)
    pub reverse: bool,
    pub play_mode: PlayMode,
    pub choke_group: u8,   // 0=none, 1-4
    pub loaded: bool,
}

impl SampleSlot {
    pub fn new() -> Self {
        Self {
            data_l: Vec::new(),
            data_r: Vec::new(),
            length: 0,
            sample_rate: 48000.0,
            volume: 1.0,
            pitch: 0.0,
            reverse: false,
            play_mode: PlayMode::OneShot,
            choke_group: 0,
            loaded: false,
        }
    }

    pub fn load(&mut self, left: Vec<f32>, right: Vec<f32>, sample_rate: f32) {
        self.length = left.len();
        self.data_l = left;
        self.data_r = right;
        self.sample_rate = sample_rate;
        self.loaded = true;
    }

    /// Read a stereo sample at a fractional position with cubic interpolation.
    #[inline]
    pub fn read(&self, pos: f64) -> (f32, f32) {
        let idx = pos as usize;
        let frac = (pos - idx as f64) as f32;

        let get_l = |i: usize| -> f32 {
            if i < self.length { self.data_l[i] } else { 0.0 }
        };
        let get_r = |i: usize| -> f32 {
            if i < self.length { self.data_r[i] } else { 0.0 }
        };

        let i0 = if idx > 0 { idx - 1 } else { 0 };
        let i1 = idx;
        let i2 = idx + 1;
        let i3 = idx + 2;

        let l = cubic(get_l(i0), get_l(i1), get_l(i2), get_l(i3), frac);
        let r = cubic(get_r(i0), get_r(i1), get_r(i2), get_r(i3), frac);
        (l, r)
    }

    /// Read reversed (position counts from end).
    #[inline]
    pub fn read_reverse(&self, pos: f64) -> (f32, f32) {
        let rev_pos = (self.length as f64 - 1.0 - pos).max(0.0);
        self.read(rev_pos)
    }
}

// =====================================================================
// SAMPLE VOICE — one playing instance with envelope
// =====================================================================

#[derive(Clone, Copy, PartialEq)]
enum EnvStage { Attack, Hold, Release, Idle }

/// Coefficient for exponential envelope segment.
fn env_coeff(time_secs: f32, sample_rate: f32) -> f32 {
    if time_secs <= 0.0 { return 0.0; }
    (-1.0 / (time_secs * sample_rate)).exp()
}

pub struct SampleVoice {
    pub position: f64,
    rate: f64,
    pub playing: bool,
    pub pad_index: u8,
    volume: f32,
    // Click-free AR envelope
    env_value: f32,
    env_stage: EnvStage,
    attack_coeff: f32,
    release_coeff: f32,
    // For voice stealing: monotonic counter
    pub start_order: u64,
}

impl SampleVoice {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            position: 0.0,
            rate: 1.0,
            playing: false,
            pad_index: 0,
            volume: 1.0,
            env_value: 0.0,
            env_stage: EnvStage::Idle,
            attack_coeff: env_coeff(0.002, sample_rate),  // 2ms attack
            release_coeff: env_coeff(0.005, sample_rate),  // 5ms release
            start_order: 0,
        }
    }

    pub fn start(&mut self, pad: u8, volume: f32, pitch_rate: f64, order: u64) {
        self.pad_index = pad;
        self.volume = volume;
        self.rate = pitch_rate;
        self.position = 0.0;
        self.playing = true;
        self.env_value = 0.0;
        self.env_stage = EnvStage::Attack;
        self.start_order = order;
    }

    pub fn release(&mut self) {
        if self.env_stage != EnvStage::Idle {
            self.env_stage = EnvStage::Release;
        }
    }

    pub fn stop(&mut self) {
        self.playing = false;
        self.env_stage = EnvStage::Idle;
        self.env_value = 0.0;
    }

    /// Process one stereo sample of this voice. Returns (left, right).
    #[inline]
    pub fn process(&mut self, slot: &SampleSlot) -> (f32, f32) {
        if !self.playing { return (0.0, 0.0); }

        // Envelope
        match self.env_stage {
            EnvStage::Attack => {
                self.env_value += (1.1 - self.env_value) * (1.0 - self.attack_coeff);
                if self.env_value >= 1.0 {
                    self.env_value = 1.0;
                    self.env_stage = EnvStage::Hold;
                }
            }
            EnvStage::Hold => {}
            EnvStage::Release => {
                self.env_value *= self.release_coeff;
                if self.env_value < 0.0001 {
                    self.stop();
                    return (0.0, 0.0);
                }
            }
            EnvStage::Idle => return (0.0, 0.0),
        }

        // Read sample
        let (l, r) = if slot.reverse {
            slot.read_reverse(self.position)
        } else {
            slot.read(self.position)
        };

        // Advance position
        self.position += self.rate;

        // Check end of sample
        if self.position >= slot.length as f64 {
            match slot.play_mode {
                PlayMode::OneShot | PlayMode::Gate => {
                    self.stop();
                    return (l * self.volume * self.env_value, r * self.volume * self.env_value);
                }
                PlayMode::Loop => {
                    self.position -= slot.length as f64;
                }
            }
        }

        let gain = self.volume * self.env_value;
        (l * gain, r * gain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_slot(length: usize, sample_rate: f32) -> SampleSlot {
        let mut slot = SampleSlot::new();
        // Fill with a simple ramp for testing
        let data: Vec<f32> = (0..length).map(|i| i as f32 / length as f32).collect();
        slot.load(data.clone(), data, sample_rate);
        slot
    }

    fn make_sine_slot(freq: f32, duration_secs: f32, sample_rate: f32) -> SampleSlot {
        let length = (sample_rate * duration_secs) as usize;
        let data: Vec<f32> = (0..length)
            .map(|i| (2.0 * std::f32::consts::PI * freq * i as f32 / sample_rate).sin())
            .collect();
        let mut slot = SampleSlot::new();
        slot.load(data.clone(), data, sample_rate);
        slot
    }

    #[test]
    fn test_voice_plays_and_stops() {
        let slot = make_sine_slot(440.0, 0.1, 44100.0);
        let mut voice = SampleVoice::new(44100.0);
        voice.start(0, 1.0, 1.0, 1);

        let mut has_audio = false;
        for _ in 0..4410 {
            let (l, _) = voice.process(&slot);
            if l.abs() > 0.01 { has_audio = true; }
        }
        assert!(has_audio, "Voice should produce audio during playback");

        // After the sample ends (0.1s = 4410 samples), voice should stop
        for _ in 0..1000 {
            voice.process(&slot);
        }
        assert!(!voice.playing, "Voice should stop after one-shot sample ends");
    }

    #[test]
    fn test_voice_loops() {
        let mut slot = make_sine_slot(440.0, 0.01, 44100.0); // very short
        slot.play_mode = PlayMode::Loop;
        let mut voice = SampleVoice::new(44100.0);
        voice.start(0, 1.0, 1.0, 1);

        // Process way more samples than the buffer length
        for _ in 0..44100 {
            voice.process(&slot);
        }
        assert!(voice.playing, "Looping voice should still be playing");
    }

    #[test]
    fn test_voice_gate_release() {
        let mut slot = make_sine_slot(440.0, 1.0, 44100.0);
        slot.play_mode = PlayMode::Gate;
        let mut voice = SampleVoice::new(44100.0);
        voice.start(0, 1.0, 1.0, 1);

        // Play for a bit
        for _ in 0..4410 {
            voice.process(&slot);
        }
        assert!(voice.playing, "Gate voice should play while held");

        // Release
        voice.release();
        for _ in 0..44100 {
            voice.process(&slot);
        }
        assert!(!voice.playing, "Gate voice should stop after release");
    }

    #[test]
    fn test_pitch_shift() {
        let slot = make_sine_slot(440.0, 0.5, 44100.0);
        let mut voice = SampleVoice::new(44100.0);
        // 2x rate = plays in half the time
        voice.start(0, 1.0, 2.0, 1);

        let mut count = 0;
        while voice.playing && count < 100000 {
            voice.process(&slot);
            count += 1;
        }
        // At 2x rate, 0.5s sample should finish in ~0.25s = ~11025 samples (plus envelope)
        assert!(count < 12000, "2x pitch should finish faster, took {count} samples");
    }

    #[test]
    fn test_reverse_playback() {
        let slot = make_slot(1000, 44100.0);
        let mut voice = SampleVoice::new(44100.0);
        voice.start(0, 1.0, 1.0, 1);

        // Forward: first samples should be near 0 (ramp starts at 0)
        let (l_fwd, _) = slot.read(10.0);

        // Reverse: reading at position 10 should give near-end values
        let (l_rev, _) = slot.read_reverse(10.0);

        assert!(l_fwd < l_rev, "Reverse should read from end: fwd={l_fwd:.3} rev={l_rev:.3}");
    }

    #[test]
    fn test_cubic_interpolation_smooth() {
        let slot = make_sine_slot(440.0, 0.1, 44100.0);
        // Read at fractional positions — should be smooth
        let (a, _) = slot.read(100.0);
        let (b, _) = slot.read(100.5);
        let (c, _) = slot.read(101.0);
        // b should be between a and c (roughly)
        assert!(b.is_finite(), "Interpolated value should be finite");
        assert!((b - a).abs() < 0.1, "Interpolation should be smooth");
    }
}
