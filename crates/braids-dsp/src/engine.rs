use dsp_common::note_to_hz;
use dsp_common::engine::{SynthEngine, MelodicEngine};
use crate::macro_osc::MacroOscillator;
use crate::models::OscMode;
use crate::envelope::Envelope;
use crate::lfo::Lfo;
use crate::filter::LadderFilter;
use crate::glide::Glide;
use dsp_common::note_sequencer::{NoteSequencer, NoteSeqEvent};

pub struct BraidsSynth {
    macro_osc: MacroOscillator,
    filter: LadderFilter,
    pub amp_env: Envelope,
    pub filter_env: Envelope,
    lfo: Lfo,
    glide: Glide,
    pub sequencer: NoteSequencer,
    events: Vec<NoteSeqEvent>,
    sample_rate: f32,

    pub current_note: u8,
    gate: bool,
    velocity: f32,

    pub timbre: f32,
    pub color: f32,
    pub filter_cutoff: f32,
    pub filter_resonance: f32,
    pub filter_env_amt: f32,
    pub lfo_amount: f32,
    pub lfo_dest: u8,
    pub master_volume: f32,
    pub seq_external: bool, // when true, sequencer ticks but engine doesn't play its notes
}

impl BraidsSynth {
    pub fn new(sample_rate: f32) -> Self {
        let mut s = Self {
            macro_osc: MacroOscillator::new(sample_rate),
            filter: LadderFilter::new(sample_rate),
            amp_env: Envelope::new(sample_rate),
            filter_env: Envelope::new(sample_rate),
            lfo: Lfo::new(sample_rate),
            glide: Glide::new(),
            sequencer: NoteSequencer::new(sample_rate),
            events: Vec::with_capacity(4),
            sample_rate,
            current_note: 0, gate: false, velocity: 0.8,
            timbre: 0.5, color: 0.5,
            filter_cutoff: 8000.0, filter_resonance: 0.2, filter_env_amt: 3000.0,
            lfo_amount: 0.0, lfo_dest: 0,
            master_volume: 0.6, seq_external: false,
        };
        s.amp_env.set_attack(0.005);
        s.amp_env.set_decay(0.2);
        s.amp_env.set_sustain(0.8);
        s.amp_env.set_release(0.3);
        s.filter_env.set_attack(0.005);
        s.filter_env.set_decay(0.3);
        s.filter_env.set_sustain(0.3);
        s.filter_env.set_release(0.3);
        s
    }

    pub fn set_mode(&mut self, mode_id: u8) {
        self.macro_osc.set_mode(OscMode::from_u8(mode_id));
    }

    pub fn set_lfo_rate(&mut self, hz: f32) { self.lfo.set_frequency(hz); }

    pub fn set_glide_time(&mut self, secs: f32) {
        if secs > 0.001 { self.glide.set_enabled(true); self.glide.set_rate(secs); }
        else { self.glide.set_enabled(false); }
    }

    pub fn note_on(&mut self, note: u8, velocity: u8) {
        let hz = note_to_hz(note);
        self.glide.set_target(hz);
        if !self.gate {
            self.amp_env.gate_on();
            self.filter_env.gate_on();
            self.macro_osc.reset_active();
        }
        self.current_note = note;
        self.velocity = velocity as f32 / 127.0;
        self.gate = true;
    }

    pub fn note_off(&mut self, note: u8) {
        if note == self.current_note {
            self.gate = false;
            self.amp_env.gate_off();
            self.filter_env.gate_off();
        }
    }

    pub fn process(&mut self) -> f32 {
        // Process sequencer events
        self.events.clear();
        self.sequencer.process(&mut self.events);
        // When seq_external is true, sequencer still ticks for step reporting
        // but we don't play notes — JS arp handles playback
        if !self.seq_external {
            let mut seq_note_on: Option<(u8, u8)> = None;
            let mut seq_note_off = false;
            for i in 0..self.events.len() {
                match &self.events[i] {
                    NoteSeqEvent::NoteOn { notes, num_notes, velocity } => {
                        if *num_notes > 0 { seq_note_on = Some((notes[0], *velocity)); }
                    }
                    NoteSeqEvent::NoteOff => seq_note_off = true,
                }
            }
            if seq_note_off { let n = self.current_note; self.note_off(n); }
            if let Some((note, vel)) = seq_note_on { self.note_on(note, vel); }
        }

        if !self.gate && !self.amp_env.is_active() {
            return 0.0;
        }

        let lfo_val = self.lfo.process();
        let lfo_mod = lfo_val * self.lfo_amount;

        let mut eff_timbre = self.timbre;
        let mut eff_color = self.color;
        let mut pitch_mod = 0.0f32;
        let mut cutoff_mod = 0.0f32;
        match self.lfo_dest {
            1 => eff_timbre = (self.timbre + lfo_mod * 0.5).clamp(0.0, 1.0),
            2 => eff_color = (self.color + lfo_mod * 0.5).clamp(0.0, 1.0),
            3 => cutoff_mod = lfo_mod * 4000.0,
            4 => pitch_mod = lfo_mod * 12.0,
            _ => {}
        }

        let base_hz = self.glide.process(self.sample_rate);
        let freq = base_hz * 2.0f32.powf(pitch_mod / 12.0);

        let mut buf = [0.0f32; 1];
        self.macro_osc.render(freq, eff_timbre, eff_color, &mut buf);
        let osc_out = buf[0];

        let filt_env = self.filter_env.process();
        let eff_cutoff = (self.filter_cutoff + filt_env * self.filter_env_amt + cutoff_mod)
            .clamp(20.0, self.sample_rate * 0.45);
        self.filter.set_cutoff(eff_cutoff);
        self.filter.set_resonance(self.filter_resonance * 4.0);
        let filtered = self.filter.process(osc_out);

        let amp = self.amp_env.process();
        filtered * amp * self.velocity * self.master_volume
    }
}

impl SynthEngine for BraidsSynth {
    fn process(&mut self) -> f32 { self.process() }

    fn set_param(&mut self, id: u32, value: f32) {
        match id {
            0 => self.set_mode(value as u8),
            1 => self.timbre = value,
            2 => self.color = value,
            3 => self.filter_cutoff = value,
            4 => self.filter_resonance = value,
            5 => self.filter_env_amt = value,
            6 => self.amp_env.set_attack(value),
            7 => self.amp_env.set_decay(value),
            8 => self.amp_env.set_sustain(value),
            9 => self.amp_env.set_release(value),
            10 => self.set_lfo_rate(value),
            11 => self.lfo_amount = value,
            12 => self.lfo_dest = value as u8,
            13 => self.master_volume = value,
            14 => self.set_glide_time(value),
            15 => self.filter_env.set_attack(value),
            16 => self.filter_env.set_decay(value),
            17 => self.filter_env.set_sustain(value),
            18 => self.filter_env.set_release(value),
            _ => {}
        }
    }

    fn set_master_volume(&mut self, vol: f32) { self.master_volume = vol; }
    fn master_volume(&self) -> f32 { self.master_volume }
}

impl MelodicEngine for BraidsSynth {
    fn note_on(&mut self, note: u8, velocity: u8) { self.note_on(note, velocity); }
    fn note_off(&mut self, note: u8) { self.note_off(note); }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(synth: &mut BraidsSynth, duration_secs: f32) -> Vec<f32> {
        let sr = 44100.0;
        let n = (sr * duration_secs) as usize;
        (0..n).map(|_| synth.process()).collect()
    }

    fn setup() -> BraidsSynth {
        let mut s = BraidsSynth::new(44100.0);
        s.master_volume = 0.6;
        s.filter_cutoff = 12000.0;
        s
    }

    #[test]
    fn test_note_on_produces_sound() {
        let mut s = setup();
        s.note_on(69, 127);
        let buf = render(&mut s, 0.3);
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
    }

    #[test]
    fn test_silence_when_idle() {
        let mut s = setup();
        let buf = render(&mut s, 0.1);
        audio_test_harness::level::assert_silent(&buf, 0.001);
    }

    #[test]
    fn test_note_off_decays_to_silence() {
        let mut s = setup();
        s.amp_env.set_release(0.01);
        s.note_on(69, 127);
        render(&mut s, 0.1);
        s.note_off(69);
        let buf = render(&mut s, 0.5);
        audio_test_harness::level::assert_silent(&buf[22050..], 0.01);
    }

    #[test]
    fn test_velocity_affects_amplitude() {
        let mut s1 = setup();
        s1.note_on(69, 127);
        let buf_loud = render(&mut s1, 0.2);
        let rms_loud = audio_test_harness::level::rms(&buf_loud[4410..]);

        let mut s2 = setup();
        s2.note_on(69, 40);
        let buf_quiet = render(&mut s2, 0.2);
        let rms_quiet = audio_test_harness::level::rms(&buf_quiet[4410..]);

        assert!(
            rms_loud > rms_quiet * 1.5,
            "vel=127 ({rms_loud:.4}) should be louder than vel=40 ({rms_quiet:.4})"
        );
    }

    #[test]
    fn test_master_volume_scales_output() {
        let mut s1 = setup();
        s1.master_volume = 0.8;
        s1.note_on(60, 100);
        let buf1 = render(&mut s1, 0.2);
        let rms1 = audio_test_harness::level::rms(&buf1[4410..]);

        let mut s2 = setup();
        s2.master_volume = 0.2;
        s2.note_on(60, 100);
        let buf2 = render(&mut s2, 0.2);
        let rms2 = audio_test_harness::level::rms(&buf2[4410..]);

        assert!(
            rms1 > rms2 * 2.0,
            "vol=0.8 ({rms1:.4}) should be >2x louder than vol=0.2 ({rms2:.4})"
        );
    }

    #[test]
    fn test_sequencer_triggers_notes() {
        let mut s = setup();
        // Program one step with a note
        s.sequencer.steps[0].gate = true;
        s.sequencer.steps[0].notes[0] = 60;
        s.sequencer.steps[0].num_notes = 1;
        s.sequencer.steps[0].velocity = 100;
        s.sequencer.set_length(1);
        s.sequencer.set_bpm(240.0); // fast for testing
        s.sequencer.play();

        let buf = render(&mut s, 0.5);
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
    }

    #[test]
    fn test_seq_external_suppresses_notes() {
        let mut s = setup();
        s.seq_external = true;
        s.sequencer.steps[0].gate = true;
        s.sequencer.steps[0].notes[0] = 60;
        s.sequencer.steps[0].num_notes = 1;
        s.sequencer.steps[0].velocity = 100;
        s.sequencer.set_length(1);
        s.sequencer.set_bpm(240.0);
        s.sequencer.play();

        let buf = render(&mut s, 0.5);
        audio_test_harness::level::assert_silent(&buf, 0.001);
    }

    #[test]
    fn test_different_modes_produce_sound() {
        for mode in 0..13u8 {
            let mut s = setup();
            s.set_mode(mode);
            s.note_on(60, 100);
            let buf = render(&mut s, 0.2);
            audio_test_harness::level::assert_not_silent(
                &buf,
                0.001,
            );
        }
    }

    #[test]
    fn test_glide_between_notes() {
        let mut s = setup();
        s.set_glide_time(0.1);
        s.note_on(60, 100);
        render(&mut s, 0.2);
        s.note_on(72, 100);
        let buf = render(&mut s, 0.05);
        // Should still produce sound while gliding
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
    }
}
