use dsp_common::engine::{SynthEngine, MelodicEngine};
use dsp_common::note_sequencer::{NoteSequencer, NoteSeqEvent};
use crate::voice::{SampleSlot, SampleVoice, PlayMode};

pub const MAX_PADS: usize = 16;
const MAX_VOICES: usize = 32;

/// SP-404 inspired sample playback engine.
/// 16 pads, 32-voice polyphony, choke groups, per-pad pitch/volume/mode.
pub struct Sampler {
    pub pads: Vec<SampleSlot>,
    voices: Vec<SampleVoice>,
    pub sequencer: NoteSequencer,
    pub seq_external: bool,
    events: Vec<NoteSeqEvent>,
    pub master_volume: f32,
    sample_rate: f32,
    voice_counter: u64, // monotonic counter for voice stealing
}

impl Sampler {
    pub fn new(sample_rate: f32) -> Self {
        let mut pads = Vec::with_capacity(MAX_PADS);
        for _ in 0..MAX_PADS { pads.push(SampleSlot::new()); }
        let mut voices = Vec::with_capacity(MAX_VOICES);
        for _ in 0..MAX_VOICES { voices.push(SampleVoice::new(sample_rate)); }

        Self {
            pads,
            voices,
            sequencer: NoteSequencer::new(sample_rate),
            seq_external: false,
            events: Vec::with_capacity(4),
            master_volume: 0.7,
            sample_rate,
            voice_counter: 0,
        }
    }

    /// Load sample data into a pad slot.
    pub fn load_sample(&mut self, pad: usize, left: Vec<f32>, right: Vec<f32>, sample_rate: f32) {
        if pad < MAX_PADS {
            self.pads[pad].load(left, right, sample_rate);
        }
    }

    /// Trigger a pad — start playing its sample.
    pub fn trigger(&mut self, pad: u8) {
        let pad_idx = pad as usize;
        if pad_idx >= MAX_PADS || !self.pads[pad_idx].loaded { return; }

        let slot = &self.pads[pad_idx];

        // Choke group: release voices in the same group
        if slot.choke_group > 0 {
            for v in &mut self.voices {
                if v.playing && v.pad_index < MAX_PADS as u8 {
                    let other_pad = &self.pads[v.pad_index as usize];
                    if other_pad.choke_group == slot.choke_group {
                        v.release();
                    }
                }
            }
        }

        // Compute playback rate: pitch (semitones) + sample rate conversion
        let pitch_rate = 2.0f64.powf(slot.pitch as f64 / 12.0);
        let sr_ratio = slot.sample_rate as f64 / self.sample_rate as f64;
        let rate = pitch_rate * sr_ratio;

        // Allocate voice
        self.voice_counter += 1;
        let voice_idx = self.allocate_voice();
        let voice = &mut self.voices[voice_idx];
        voice.start(pad, slot.volume, rate, self.voice_counter);
    }

    /// Release a pad (for Gate mode).
    pub fn release_pad(&mut self, pad: u8) {
        for v in &mut self.voices {
            if v.playing && v.pad_index == pad {
                v.release();
            }
        }
    }

    /// Immediately stop all voices for a pad.
    pub fn stop_pad(&mut self, pad: u8) {
        for v in &mut self.voices {
            if v.playing && v.pad_index == pad {
                v.stop();
            }
        }
    }

    /// Set a per-pad parameter.
    pub fn set_pad_param(&mut self, pad: u8, param: u8, value: f32) {
        let idx = pad as usize;
        if idx >= MAX_PADS { return; }
        let slot = &mut self.pads[idx];
        match param {
            0 => slot.volume = value.clamp(0.0, 1.0),
            1 => slot.pitch = value.clamp(-24.0, 24.0),
            2 => slot.play_mode = PlayMode::from_u8(value as u8),
            3 => slot.choke_group = (value as u8).min(4),
            4 => slot.reverse = value > 0.5,
            _ => {}
        }
    }

    /// Process one stereo sample frame.
    pub fn process_stereo(&mut self) -> (f32, f32) {
        // Process sequencer
        if !self.seq_external {
            self.events.clear();
            self.sequencer.process(&mut self.events);
            // Collect trigger pads first to avoid borrow conflict
            let mut trigger_pads: [u8; 16] = [0; 16];
            let mut trigger_count = 0usize;
            for event in &self.events {
                if let NoteSeqEvent::NoteOn { notes, num_notes, .. } = event {
                    let n = (*num_notes as usize).min(notes.len());
                    for i in 0..n {
                        let pad = notes[i].saturating_sub(36);
                        if (pad as usize) < MAX_PADS && trigger_count < 16 {
                            trigger_pads[trigger_count] = pad;
                            trigger_count += 1;
                        }
                    }
                }
            }
            for i in 0..trigger_count {
                self.trigger(trigger_pads[i]);
            }
        }

        // Mix all active voices
        let mut mix_l = 0.0f32;
        let mut mix_r = 0.0f32;

        for voice in &mut self.voices {
            if !voice.playing { continue; }
            let pad_idx = voice.pad_index as usize;
            if pad_idx >= MAX_PADS { continue; }
            let (l, r) = voice.process(&self.pads[pad_idx]);
            mix_l += l;
            mix_r += r;
        }

        (mix_l * self.master_volume, mix_r * self.master_volume)
    }

    fn allocate_voice(&self) -> usize {
        // 1. Find an idle voice
        if let Some(i) = self.voices.iter().position(|v| !v.playing) {
            return i;
        }
        // 2. Steal the oldest voice
        self.voices.iter()
            .enumerate()
            .min_by_key(|(_, v)| v.start_order)
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
}

impl SynthEngine for Sampler {
    fn process(&mut self) -> f32 {
        let (l, r) = self.process_stereo();
        (l + r) * 0.5 // mono fallback
    }

    fn set_param(&mut self, id: u32, value: f32) {
        if id == 200 {
            self.master_volume = value.clamp(0.0, 1.0);
        } else if id < 128 {
            // Per-pad params: pad = id / 8, param = id % 8
            let pad = (id / 8) as u8;
            let param = (id % 8) as u8;
            self.set_pad_param(pad, param, value);
        }
    }

    fn set_master_volume(&mut self, vol: f32) { self.master_volume = vol; }
    fn master_volume(&self) -> f32 { self.master_volume }
}

impl MelodicEngine for Sampler {
    fn note_on(&mut self, note: u8, _velocity: u8) {
        let pad = note.saturating_sub(36);
        if (pad as usize) < MAX_PADS {
            self.trigger(pad);
        }
    }

    fn note_off(&mut self, note: u8) {
        let pad = note.saturating_sub(36);
        if (pad as usize) < MAX_PADS {
            self.release_pad(pad);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    fn sine_sample(freq: f32, duration: f32, sr: f32) -> (Vec<f32>, Vec<f32>) {
        let len = (sr * duration) as usize;
        let data: Vec<f32> = (0..len).map(|i| (2.0 * PI * freq * i as f32 / sr).sin()).collect();
        (data.clone(), data)
    }

    #[test]
    fn test_trigger_produces_sound() {
        let mut s = Sampler::new(44100.0);
        let (l, r) = sine_sample(440.0, 0.5, 44100.0);
        s.load_sample(0, l, r, 44100.0);
        s.trigger(0);

        let mut has_audio = false;
        for _ in 0..4410 {
            let (l, _) = s.process_stereo();
            if l.abs() > 0.01 { has_audio = true; break; }
        }
        assert!(has_audio, "Triggered sample should produce audio");
    }

    #[test]
    fn test_silence_without_trigger() {
        let mut s = Sampler::new(44100.0);
        let (l, r) = sine_sample(440.0, 0.5, 44100.0);
        s.load_sample(0, l, r, 44100.0);

        let mut energy = 0.0f32;
        for _ in 0..4410 {
            let (l, _) = s.process_stereo();
            energy += l.abs();
        }
        assert!(energy < 0.01, "Should be silent without trigger");
    }

    #[test]
    fn test_choke_group() {
        let mut s = Sampler::new(44100.0);
        let (l, r) = sine_sample(440.0, 1.0, 44100.0);
        s.load_sample(0, l.clone(), r.clone(), 44100.0);
        s.load_sample(1, l, r, 44100.0);
        s.pads[0].choke_group = 1;
        s.pads[1].choke_group = 1;

        s.trigger(0);
        // Let it play a bit
        for _ in 0..2000 { s.process_stereo(); }
        assert!(s.voices.iter().any(|v| v.playing && v.pad_index == 0), "Pad 0 should be playing");

        // Trigger pad 1 in same choke group
        s.trigger(1);
        // Process a few frames for the release to take effect
        for _ in 0..44100 { s.process_stereo(); }
        // Pad 0's voice should have been choked (released + faded)
        let pad0_playing = s.voices.iter().any(|v| v.playing && v.pad_index == 0);
        assert!(!pad0_playing, "Pad 0 should be choked by pad 1");
    }

    #[test]
    fn test_polyphony() {
        let mut s = Sampler::new(44100.0);
        for i in 0..4 {
            let (l, r) = sine_sample(220.0 * (i + 1) as f32, 0.5, 44100.0);
            s.load_sample(i, l, r, 44100.0);
        }

        // Trigger all 4
        for i in 0..4 { s.trigger(i as u8); }

        let playing = s.voices.iter().filter(|v| v.playing).count();
        assert_eq!(playing, 4, "Should have 4 voices playing");
    }

    #[test]
    fn test_sequencer_triggers_pads() {
        let mut s = Sampler::new(44100.0);
        let (l, r) = sine_sample(440.0, 0.3, 44100.0);
        s.load_sample(0, l, r, 44100.0);

        // Set step 0 to trigger pad 0 (MIDI note 36)
        s.sequencer.steps[0].gate = true;
        s.sequencer.steps[0].notes[0] = 36;
        s.sequencer.steps[0].num_notes = 1;
        s.sequencer.set_bpm(240.0);
        s.sequencer.play();

        let mut has_audio = false;
        for _ in 0..44100 {
            let (l, _) = s.process_stereo();
            if l.abs() > 0.01 { has_audio = true; break; }
        }
        assert!(has_audio, "Sequencer should trigger sample playback");
    }

    #[test]
    fn test_unloaded_pad_silent() {
        let mut s = Sampler::new(44100.0);
        s.trigger(5); // pad 5 has no sample loaded
        let mut energy = 0.0f32;
        for _ in 0..4410 {
            let (l, _) = s.process_stereo();
            energy += l.abs();
        }
        assert!(energy < 0.01, "Unloaded pad should be silent");
    }

    #[test]
    fn test_master_volume() {
        let mut s = Sampler::new(44100.0);
        let (l, r) = sine_sample(440.0, 0.5, 44100.0);
        s.load_sample(0, l, r, 44100.0);

        s.master_volume = 0.8;
        s.trigger(0);
        let mut rms1 = 0.0f32;
        for _ in 0..4410 { let (l, _) = s.process_stereo(); rms1 += l * l; }
        rms1 = (rms1 / 4410.0).sqrt();

        // Reset
        let (l, r) = sine_sample(440.0, 0.5, 44100.0);
        let mut s2 = Sampler::new(44100.0);
        s2.load_sample(0, l, r, 44100.0);
        s2.master_volume = 0.2;
        s2.trigger(0);
        let mut rms2 = 0.0f32;
        for _ in 0..4410 { let (l, _) = s2.process_stereo(); rms2 += l * l; }
        rms2 = (rms2 / 4410.0).sqrt();

        assert!(rms1 > rms2 * 2.0, "vol=0.8 ({rms1:.4}) should be >2x louder than vol=0.2 ({rms2:.4})");
    }
}
