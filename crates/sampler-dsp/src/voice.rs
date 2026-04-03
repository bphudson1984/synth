use crate::vocoder::Vocoder;

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
    pub pan: f32,          // -1 (left) to +1 (right)
    pub pitch: f32,        // semitones (-24 to +24)
    pub reverse: bool,
    pub play_mode: PlayMode,
    pub choke_group: u8,   // 0=none, 1-4
    pub attack: f32,       // seconds (0-2)
    pub release: f32,      // seconds (0-2)
    pub start_pct: f32,    // 0-1 (start point as fraction of sample)
    pub end_pct: f32,      // 0-1 (end point as fraction of sample)
    pub bit_depth: f32,    // 1-16 (16 = clean, lower = more crushed)
    // Vocoder settings (stored per-pad, applied to voice on trigger)
    pub vocoder_enabled: bool,
    pub vocoder_root_note: u8,     // MIDI note the sample is tuned to
    pub vocoder_carrier: u8,       // 0=saw, 1=square, 2=noise
    pub vocoder_bands: u8,         // 4-16
    pub vocoder_formant: f32,      // -12 to +12 semitones
    pub vocoder_mix: f32,          // 0-1
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
            pan: 0.0,
            pitch: 0.0,
            reverse: false,
            play_mode: PlayMode::OneShot,
            choke_group: 0,
            attack: 0.002,
            release: 0.005,
            start_pct: 0.0,
            end_pct: 1.0,
            bit_depth: 16.0,
            vocoder_enabled: false,
            vocoder_root_note: 60,
            vocoder_carrier: 0,
            vocoder_bands: 12,
            vocoder_formant: 0.0,
            vocoder_mix: 1.0,
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
    pan: f32,
    // AR envelope with configurable times
    env_value: f32,
    env_stage: EnvStage,
    attack_coeff: f32,
    release_coeff: f32,
    // Start/end bounds (in frames)
    start_frame: f64,
    end_frame: f64,
    play_mode: PlayMode,
    looping: bool,
    // Vocoder
    pub vocoder: Vocoder,
    // For voice stealing: monotonic counter
    pub start_order: u64,
    sample_rate: f32,
}

impl SampleVoice {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            position: 0.0,
            rate: 1.0,
            playing: false,
            pad_index: 0,
            volume: 1.0,
            pan: 0.0,
            env_value: 0.0,
            env_stage: EnvStage::Idle,
            attack_coeff: env_coeff(0.002, sample_rate),
            release_coeff: env_coeff(0.005, sample_rate),
            start_frame: 0.0,
            end_frame: 0.0,
            play_mode: PlayMode::OneShot,
            looping: false,
            vocoder: Vocoder::new(sample_rate),
            start_order: 0,
            sample_rate,
        }
    }

    pub fn start(&mut self, pad: u8, slot: &SampleSlot, pitch_rate: f64, order: u64) {
        self.pad_index = pad;
        self.volume = slot.volume;
        self.pan = slot.pan;
        self.rate = pitch_rate;
        self.start_frame = (slot.start_pct * slot.length as f32) as f64;
        self.end_frame = (slot.end_pct * slot.length as f32) as f64;
        self.position = self.start_frame;
        self.play_mode = slot.play_mode;
        self.looping = slot.play_mode == PlayMode::Loop;
        self.playing = true;
        self.env_value = 0.0;
        self.env_stage = EnvStage::Attack;
        self.attack_coeff = env_coeff(slot.attack, self.sample_rate);
        self.release_coeff = env_coeff(slot.release, self.sample_rate);
        // Configure vocoder from slot
        self.vocoder.enabled = slot.vocoder_enabled;
        if slot.vocoder_enabled {
            self.vocoder.carrier_wave = slot.vocoder_carrier;
            self.vocoder.set_num_bands(slot.vocoder_bands as usize);
            self.vocoder.set_formant_shift(slot.vocoder_formant);
            self.vocoder.mix = slot.vocoder_mix;
            self.vocoder.root_note = slot.vocoder_root_note;
            // Carrier pitch = the note being played (set by engine)
            self.vocoder.clear();
        }
        self.start_order = order;
    }

    pub fn release(&mut self) {
        // One-shot ignores release — plays to end
        if self.play_mode == PlayMode::OneShot { return; }
        if self.env_stage != EnvStage::Idle {
            self.env_stage = EnvStage::Release;
            self.looping = false;
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
        let (mut l, mut r) = if slot.reverse {
            slot.read_reverse(self.position)
        } else {
            slot.read(self.position)
        };

        // Bit crush: quantize to fewer bits (16 = clean, 1 = extreme)
        if slot.bit_depth < 15.9 {
            let steps = (2.0f32).powf(slot.bit_depth);
            l = (l * steps).round() / steps;
            r = (r * steps).round() / steps;
        }

        // Vocoder: process mono sum through vocoder, then re-pan
        if self.vocoder.enabled {
            let mono = (l + r) * 0.5;
            let vocoded = self.vocoder.process(mono);
            l = vocoded;
            r = vocoded;
        }

        // Advance position
        self.position += self.rate;

        // Check end of sample (use end_frame from start/end points)
        let end = if self.end_frame > self.start_frame { self.end_frame } else { slot.length as f64 };
        if self.position >= end {
            if self.looping {
                self.position = self.start_frame;
            } else {
                self.stop();
                let gain = self.volume * self.env_value;
                let pan_l = (1.0 - self.pan).min(1.0);
                let pan_r = (1.0 + self.pan).min(1.0);
                return (l * gain * pan_l, r * gain * pan_r);
            }
        }

        let gain = self.volume * self.env_value;
        // Constant-power-ish pan
        let pan_l = (1.0 - self.pan).min(1.0);
        let pan_r = (1.0 + self.pan).min(1.0);
        (l * gain * pan_l, r * gain * pan_r)
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
        voice.start(0, &slot, 1.0, 1);

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
        voice.start(0, &slot, 1.0, 1);

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
        voice.start(0, &slot, 1.0, 1);

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
        voice.start(0, &slot, 2.0, 1);

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
        voice.start(0, &slot, 1.0, 1);

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
