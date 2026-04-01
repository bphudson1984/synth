use dsp_common::engine::{SynthEngine, MelodicEngine};
use crate::voice::AcidVoice;
use crate::sequencer::{AcidSequencer, AcidSeqEvent};

pub struct TB303 {
    pub voice: AcidVoice,
    pub sequencer: AcidSequencer,
    events: Vec<AcidSeqEvent>,
    pub master_volume: f32,
}

impl TB303 {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            voice: AcidVoice::new(sample_rate),
            sequencer: AcidSequencer::new(sample_rate),
            events: Vec::with_capacity(4),
            master_volume: 0.7,
        }
    }

    pub fn process(&mut self) -> f32 {
        self.events.clear();
        self.sequencer.process(&mut self.events);
        for event in &self.events {
            match event {
                AcidSeqEvent::NoteOn { note, accent, slide } => {
                    self.voice.note_on(*note, *accent, *slide);
                }
                AcidSeqEvent::NoteOff => { self.voice.note_off(); }
            }
        }
        self.voice.process() * self.master_volume
    }

    /// Play a note directly (for live playing outside the sequencer).
    /// Velocity > 100 triggers accent behavior.
    pub fn note_on(&mut self, note: u8, velocity: u8) {
        let accent = velocity > 100;
        self.voice.note_on(note, accent, false);
    }

    pub fn note_off(&mut self) {
        self.voice.note_off();
    }
}

impl SynthEngine for TB303 {
    fn process(&mut self) -> f32 { self.process() }

    fn set_param(&mut self, id: u32, value: f32) {
        match id {
            0 => self.voice.cutoff = value,
            1 => self.voice.resonance = value,
            2 => self.voice.env_mod = value,
            3 => self.voice.decay = value,
            4 => self.voice.accent_level = value,
            5 => self.voice.set_waveform(value < 0.5),
            6 => self.master_volume = value,
            7 => self.voice.distortion = value,
            _ => {}
        }
    }

    fn set_master_volume(&mut self, vol: f32) { self.master_volume = vol; }
    fn master_volume(&self) -> f32 { self.master_volume }
}

impl MelodicEngine for TB303 {
    fn note_on(&mut self, note: u8, velocity: u8) { self.note_on(note, velocity); }
    fn note_off(&mut self, _note: u8) { self.note_off(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(engine: &mut TB303, duration_secs: f32) -> Vec<f32> {
        let n = (44100.0 * duration_secs) as usize;
        (0..n).map(|_| engine.process()).collect()
    }

    fn setup() -> TB303 {
        TB303::new(44100.0)
    }

    #[test]
    fn test_sequencer_produces_sound() {
        let mut e = setup();
        e.sequencer.steps[0].gate = true;
        e.sequencer.steps[0].note = 48;
        e.sequencer.set_bpm(240.0);
        e.sequencer.play();

        let buf = render(&mut e, 0.5);
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
    }

    #[test]
    fn test_silence_when_stopped() {
        let mut e = setup();
        let buf = render(&mut e, 0.1);
        audio_test_harness::level::assert_silent(&buf, 0.001);
    }

    #[test]
    fn test_accent_is_louder() {
        let mut e1 = setup();
        e1.sequencer.steps[0].gate = true;
        e1.sequencer.steps[0].note = 48;
        e1.sequencer.steps[0].accent = false;
        e1.sequencer.length = 1;
        e1.sequencer.set_bpm(240.0);
        e1.sequencer.play();
        let buf_normal = render(&mut e1, 0.3);
        let rms_normal = audio_test_harness::level::rms(&buf_normal);

        let mut e2 = setup();
        e2.sequencer.steps[0].gate = true;
        e2.sequencer.steps[0].note = 48;
        e2.sequencer.steps[0].accent = true;
        e2.sequencer.length = 1;
        e2.sequencer.set_bpm(240.0);
        e2.sequencer.play();
        let buf_accent = render(&mut e2, 0.3);
        let rms_accent = audio_test_harness::level::rms(&buf_accent);

        assert!(
            rms_accent > rms_normal,
            "accent ({rms_accent:.4}) should be louder than normal ({rms_normal:.4})"
        );
    }

    #[test]
    fn test_master_volume() {
        let mut e1 = setup();
        e1.master_volume = 0.8;
        e1.sequencer.steps[0].gate = true;
        e1.sequencer.steps[0].note = 48;
        e1.sequencer.length = 1;
        e1.sequencer.set_bpm(240.0);
        e1.sequencer.play();
        let buf1 = render(&mut e1, 0.3);
        let rms1 = audio_test_harness::level::rms(&buf1);

        let mut e2 = setup();
        e2.master_volume = 0.2;
        e2.sequencer.steps[0].gate = true;
        e2.sequencer.steps[0].note = 48;
        e2.sequencer.length = 1;
        e2.sequencer.set_bpm(240.0);
        e2.sequencer.play();
        let buf2 = render(&mut e2, 0.3);
        let rms2 = audio_test_harness::level::rms(&buf2);

        assert!(
            rms1 > rms2 * 2.0,
            "vol=0.8 ({rms1:.4}) should be >2x louder than vol=0.2 ({rms2:.4})"
        );
    }

    #[test]
    fn test_cutoff_affects_tone() {
        // Verify that changing filter cutoff produces a different sound
        let mut e1 = setup();
        e1.voice.cutoff = 100.0;
        e1.voice.env_mod = 0.0; // disable filter envelope to isolate cutoff effect
        e1.sequencer.steps[0].gate = true;
        e1.sequencer.steps[0].note = 48;
        e1.sequencer.length = 1;
        e1.sequencer.set_bpm(240.0);
        e1.sequencer.play();
        let buf_dark = render(&mut e1, 0.3);

        let mut e2 = setup();
        e2.voice.cutoff = 12000.0;
        e2.voice.env_mod = 0.0;
        e2.sequencer.steps[0].gate = true;
        e2.sequencer.steps[0].note = 48;
        e2.sequencer.length = 1;
        e2.sequencer.set_bpm(240.0);
        e2.sequencer.play();
        let buf_bright = render(&mut e2, 0.3);

        // Different cutoff should produce different waveforms
        let corr = audio_test_harness::correlation::cross_correlation(&buf_dark, &buf_bright);
        assert!(
            corr < 0.99,
            "different cutoffs should produce different output (corr={corr:.4})"
        );
    }

    #[test]
    fn test_slide_changes_behavior() {
        let mut e = setup();
        e.sequencer.steps[0].gate = true;
        e.sequencer.steps[0].note = 36;
        e.sequencer.steps[0].slide = false;
        e.sequencer.steps[1].gate = true;
        e.sequencer.steps[1].note = 48;
        e.sequencer.steps[1].slide = true;
        e.sequencer.length = 2;
        e.sequencer.set_bpm(120.0);
        e.sequencer.play();

        let buf = render(&mut e, 1.0);
        // Should produce sound through both steps
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
    }
}
