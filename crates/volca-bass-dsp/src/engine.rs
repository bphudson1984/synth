use dsp_common::engine::{SynthEngine, MelodicEngine};
use dsp_common::note_sequencer::{NoteSequencer, NoteSeqEvent, MAX_STEPS};
use crate::voice::BassVoice;

pub struct VolcaBass {
    pub voice: BassVoice,
    pub sequencer: NoteSequencer,
    pub seq_external: bool,
    events: Vec<NoteSeqEvent>,
    pub master_volume: f32,
}

impl VolcaBass {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            voice: BassVoice::new(sample_rate),
            sequencer: NoteSequencer::new(sample_rate),
            seq_external: false,
            events: Vec::with_capacity(4),
            master_volume: 0.7,
        }
    }

    pub fn process(&mut self) -> f32 {
        if !self.seq_external {
            self.events.clear();
            self.sequencer.process(&mut self.events);
            for event in &self.events {
                match event {
                    NoteSeqEvent::NoteOn { notes, num_notes, velocity } => {
                        let n = (*num_notes as usize).min(notes.len());
                        if n > 0 {
                            self.voice.note_on(notes[0], *velocity);
                        }
                    }
                    NoteSeqEvent::NoteOff => {
                        self.voice.note_off();
                    }
                }
            }
        }
        self.voice.process() * self.master_volume
    }

    pub fn note_on(&mut self, note: u8, velocity: u8) {
        self.voice.note_on(note, velocity);
    }

    pub fn note_off(&mut self) {
        self.voice.note_off();
    }
}

impl SynthEngine for VolcaBass {
    fn process(&mut self) -> f32 { self.process() }

    fn set_param(&mut self, id: u32, value: f32) {
        match id {
            0 => self.voice.osc_pitch[0] = value,
            1 => self.voice.osc_pitch[1] = value,
            2 => self.voice.osc_pitch[2] = value,
            3 => self.voice.set_osc_waveform(0, value < 0.5),
            4 => self.voice.set_osc_waveform(1, value < 0.5),
            5 => self.voice.set_osc_waveform(2, value < 0.5),
            6 => self.voice.osc_active[0] = value > 0.5,
            7 => self.voice.osc_active[1] = value > 0.5,
            8 => self.voice.osc_active[2] = value > 0.5,
            9 => self.voice.cutoff = value,
            10 => self.voice.resonance = value,
            11 => self.voice.eg_intensity = value,
            12 => self.voice.envelope.set_attack(value),
            13 => self.voice.envelope.set_decay_release(value),
            14 => self.voice.envelope.sustain_on = value > 0.5,
            15 => self.voice.eg_to_vca = value > 0.5,
            16 => self.voice.set_lfo_rate(value),
            17 => self.voice.lfo_intensity = value,
            18 => self.voice.set_lfo_wave(value > 0.5),
            19 => self.voice.lfo_to_pitch = value > 0.5,
            20 => self.voice.lfo_to_cutoff = value > 0.5,
            21 => self.voice.lfo_to_amp = value > 0.5,
            22 => self.voice.group_mode = if value > 0.5 { 1 } else { 0 },
            23 => self.master_volume = value,
            _ => {}
        }
    }

    fn set_master_volume(&mut self, vol: f32) { self.master_volume = vol; }
    fn master_volume(&self) -> f32 { self.master_volume }
}

impl MelodicEngine for VolcaBass {
    fn note_on(&mut self, note: u8, velocity: u8) { self.note_on(note, velocity); }
    fn note_off(&mut self, _note: u8) { self.note_off(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(engine: &mut VolcaBass, duration_secs: f32) -> Vec<f32> {
        let n = (44100.0 * duration_secs) as usize;
        (0..n).map(|_| engine.process()).collect()
    }

    #[test]
    fn test_sequencer_produces_sound() {
        let mut e = VolcaBass::new(44100.0);
        e.sequencer.steps[0].gate = true;
        e.sequencer.steps[0].notes[0] = 36;
        e.sequencer.steps[0].num_notes = 1;
        e.sequencer.set_bpm(240.0);
        e.sequencer.play();

        let buf = render(&mut e, 0.5);
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
    }

    #[test]
    fn test_silence_when_stopped() {
        let mut e = VolcaBass::new(44100.0);
        let buf = render(&mut e, 0.1);
        audio_test_harness::level::assert_silent(&buf, 0.001);
    }

    #[test]
    fn test_master_volume() {
        let mut e1 = VolcaBass::new(44100.0);
        e1.master_volume = 0.8;
        e1.sequencer.steps[0].gate = true;
        e1.sequencer.steps[0].notes[0] = 36;
        e1.sequencer.steps[0].num_notes = 1;
        e1.sequencer.set_bpm(240.0);
        e1.sequencer.play();
        let rms1 = audio_test_harness::level::rms(&render(&mut e1, 0.3));

        let mut e2 = VolcaBass::new(44100.0);
        e2.master_volume = 0.2;
        e2.sequencer.steps[0].gate = true;
        e2.sequencer.steps[0].notes[0] = 36;
        e2.sequencer.steps[0].num_notes = 1;
        e2.sequencer.set_bpm(240.0);
        e2.sequencer.play();
        let rms2 = audio_test_harness::level::rms(&render(&mut e2, 0.3));

        assert!(rms1 > rms2 * 2.0, "vol=0.8 ({rms1:.4}) should be >2x louder than vol=0.2 ({rms2:.4})");
    }

    #[test]
    fn test_cutoff_affects_tone() {
        let mut e1 = VolcaBass::new(44100.0);
        e1.voice.cutoff = 100.0;
        e1.voice.eg_intensity = 0.0;
        e1.sequencer.steps[0].gate = true;
        e1.sequencer.steps[0].notes[0] = 36;
        e1.sequencer.steps[0].num_notes = 1;
        e1.sequencer.set_bpm(240.0);
        e1.sequencer.play();
        let buf_dark = render(&mut e1, 0.3);

        let mut e2 = VolcaBass::new(44100.0);
        e2.voice.cutoff = 12000.0;
        e2.voice.eg_intensity = 0.0;
        e2.sequencer.steps[0].gate = true;
        e2.sequencer.steps[0].notes[0] = 36;
        e2.sequencer.steps[0].num_notes = 1;
        e2.sequencer.set_bpm(240.0);
        e2.sequencer.play();
        let buf_bright = render(&mut e2, 0.3);

        let corr = audio_test_harness::correlation::cross_correlation(&buf_dark, &buf_bright);
        assert!(corr < 0.99, "different cutoffs should produce different output (corr={corr:.4})");
    }

    #[test]
    fn test_direct_note_on() {
        let mut e = VolcaBass::new(44100.0);
        e.note_on(48, 100);
        let buf = render(&mut e, 0.2);
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
    }
}
