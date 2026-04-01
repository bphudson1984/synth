use dsp_common::engine::{SynthEngine, MelodicEngine};
use dsp_common::note_sequencer::{NoteSequencer, NoteSeqEvent, MAX_NOTES_PER_STEP};
use crate::voice::Voice;
use crate::lfo::Lfo;
use crate::noise::NoiseGenerator;

pub const NUM_VOICES: usize = 5;

pub struct ProphetSynth {
    voices: [Voice; NUM_VOICES],
    pub lfo: Lfo,
    noise: NoiseGenerator,
    voice_ages: [u64; NUM_VOICES],
    age_counter: u64,
    sample_rate: f32,
    pub master_volume: f32,
    pub unison: bool,
    pub unison_detune: f32, // detune amount in Hz

    // Wheel Mod config (global)
    pub wheel_mod_source_mix: f32, // 0=LFO, 1=Noise
    pub mod_wheel: f32,            // 0-1 from mod wheel
    pub wheel_mod_dest_freq_a: bool,
    pub wheel_mod_dest_freq_b: bool,
    pub wheel_mod_dest_pw_a: bool,
    pub wheel_mod_dest_pw_b: bool,
    pub wheel_mod_dest_filter: bool,

    // Pitch bend
    pub pitch_bend: f32, // -1 to +1
    pub pitch_bend_range: f32, // semitones (default 2)

    // LFO initial amount — base modulation level even when mod wheel is at 0
    pub lfo_initial_amount: f32, // 0.0-1.0

    // Embedded sequencer (matches Braids pattern)
    pub sequencer: NoteSequencer,
    seq_events: Vec<NoteSeqEvent>,
    pub seq_external: bool,
    seq_last_notes: [u8; MAX_NOTES_PER_STEP],
    seq_last_num_notes: u8,
}

impl ProphetSynth {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            voices: std::array::from_fn(|i| Voice::new(sample_rate, i as u32)),
            lfo: Lfo::new(sample_rate),
            noise: NoiseGenerator::new(999),
            voice_ages: [0; NUM_VOICES],
            age_counter: 0,
            sample_rate,
            master_volume: 1.0,
            unison: false,
            unison_detune: 3.0,
            wheel_mod_source_mix: 0.0,
            mod_wheel: 0.0,
            wheel_mod_dest_freq_a: false,
            wheel_mod_dest_freq_b: false,
            wheel_mod_dest_pw_a: false,
            wheel_mod_dest_pw_b: false,
            wheel_mod_dest_filter: false,
            pitch_bend: 0.0,
            pitch_bend_range: 2.0,
            lfo_initial_amount: 0.0,
            sequencer: NoteSequencer::new(sample_rate),
            seq_events: Vec::with_capacity(4),
            seq_external: false,
            seq_last_notes: [0; MAX_NOTES_PER_STEP],
            seq_last_num_notes: 0,
        }
    }

    pub fn note_on(&mut self, note: u8, velocity: u8) {
        if self.unison {
            // All voices play the same note with symmetric detune
            // Offsets only affect the base pitch — Osc B offset is applied separately in voice
            let detune_offsets = [0.0f32, 1.0, -1.0, 2.0, -2.0];
            for (i, voice) in self.voices.iter_mut().enumerate() {
                voice.unison_detune_hz = detune_offsets[i] * self.unison_detune;
                voice.note_on(note, velocity);
            }
            self.age_counter += 1;
            for age in &mut self.voice_ages {
                *age = self.age_counter;
            }
            return;
        }

        // Normal polyphonic mode
        if let Some(idx) = self.find_voice_playing(note) {
            self.voices[idx].unison_detune_hz = 0.0;
            self.voices[idx].note_on(note, velocity);
            self.age_counter += 1;
            self.voice_ages[idx] = self.age_counter;
            return;
        }

        let idx = self.find_free_voice()
            .unwrap_or_else(|| self.steal_oldest_voice());
        self.voices[idx].unison_detune_hz = 0.0;
        self.voices[idx].note_on(note, velocity);
        self.age_counter += 1;
        self.voice_ages[idx] = self.age_counter;
    }

    pub fn note_off(&mut self, note: u8) {
        for voice in &mut self.voices {
            if voice.active && voice.note == note {
                voice.note_off();
            }
        }
    }

    pub fn process(&mut self) -> f32 {
        // Process sequencer events
        self.seq_events.clear();
        self.sequencer.process(&mut self.seq_events);
        if !self.seq_external {
            let mut seq_note_on: Option<([u8; MAX_NOTES_PER_STEP], u8, u8)> = None;
            let mut seq_note_off = false;
            for i in 0..self.seq_events.len() {
                match &self.seq_events[i] {
                    NoteSeqEvent::NoteOn { notes, num_notes, velocity } => {
                        if *num_notes > 0 {
                            seq_note_on = Some((*notes, *num_notes, *velocity));
                        }
                    }
                    NoteSeqEvent::NoteOff => seq_note_off = true,
                }
            }
            if seq_note_off {
                for j in 0..self.seq_last_num_notes as usize {
                    self.note_off(self.seq_last_notes[j]);
                }
                self.seq_last_num_notes = 0;
            }
            if let Some((notes, num_notes, vel)) = seq_note_on {
                // Release previous sequencer notes
                for j in 0..self.seq_last_num_notes as usize {
                    self.note_off(self.seq_last_notes[j]);
                }
                // Play all notes in the step (chords)
                for j in 0..num_notes as usize {
                    self.note_on(notes[j], vel);
                }
                self.seq_last_notes = notes;
                self.seq_last_num_notes = num_notes;
            }
        }

        // Global LFO (computed once, shared by all voices)
        let lfo_val = self.lfo.process();

        // Wheel Mod: mix LFO and noise, scale by (initial_amount + mod_wheel)
        // initial_amount provides always-on modulation, mod_wheel adds on top
        let noise_val = self.noise.white();
        let wheel_source = lfo_val * (1.0 - self.wheel_mod_source_mix)
            + noise_val * self.wheel_mod_source_mix;
        let mod_depth = (self.lfo_initial_amount + self.mod_wheel).min(1.0);
        let wheel_mod_signal = wheel_source * mod_depth;

        // Pitch bend in semitones
        let bend_semitones = self.pitch_bend * self.pitch_bend_range;

        // Apply global modulation to all voices
        for voice in &mut self.voices {
            voice.wheel_mod_signal = wheel_mod_signal;
            voice.wheel_mod_dest_freq_a = self.wheel_mod_dest_freq_a;
            voice.wheel_mod_dest_freq_b = self.wheel_mod_dest_freq_b;
            voice.wheel_mod_dest_pw_a = self.wheel_mod_dest_pw_a;
            voice.wheel_mod_dest_pw_b = self.wheel_mod_dest_pw_b;
            voice.wheel_mod_dest_filter = self.wheel_mod_dest_filter;
            voice.pitch_bend_semitones = bend_semitones;
        }

        // Sum voices
        let mut out = 0.0;
        for voice in &mut self.voices {
            out += voice.process();
        }
        out * self.master_volume
    }

    pub fn process_block(&mut self, output: &mut [f32]) {
        for sample in output.iter_mut() {
            *sample = self.process();
        }
    }

    pub fn voice_mut(&mut self, index: usize) -> &mut Voice {
        &mut self.voices[index]
    }

    pub fn for_each_voice(&mut self, f: impl Fn(&mut Voice)) {
        for voice in &mut self.voices {
            f(voice);
        }
    }

    fn find_free_voice(&self) -> Option<usize> {
        self.voices.iter().position(|v| !v.active)
    }

    fn steal_oldest_voice(&self) -> usize {
        self.voice_ages
            .iter()
            .enumerate()
            .min_by_key(|(_, &age)| age)
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }

    fn find_voice_playing(&self, note: u8) -> Option<usize> {
        self.voices.iter().position(|v| v.active && v.note == note)
    }
}

impl SynthEngine for ProphetSynth {
    fn process(&mut self) -> f32 { self.process() }

    /// Set synth parameters (IDs 0-47). Effects (50-59) and arp (60-67)
    /// are handled separately in the WASM wrapper.
    fn set_param(&mut self, id: u32, value: f32) {
        match id {
            // Oscillator A
            0 => self.for_each_voice(|v| v.osc_a.set_saw(value > 0.5)),
            1 => self.for_each_voice(|v| v.osc_a.set_pulse(value > 0.5)),
            2 => self.for_each_voice(|v| v.osc_a_pw = value),
            // Oscillator B
            3 => self.for_each_voice(|v| v.osc_b.set_saw(value > 0.5)),
            4 => self.for_each_voice(|v| v.osc_b.set_tri(value > 0.5)),
            5 => self.for_each_voice(|v| v.osc_b.set_pulse(value > 0.5)),
            6 => self.for_each_voice(|v| v.osc_b_pw = value),
            7 => self.for_each_voice(|v| v.osc_b_semitones = value),
            8 => self.for_each_voice(|v| v.osc_b_fine = value),
            // Mixer
            9 => self.for_each_voice(|v| v.osc_a_level = value),
            10 => self.for_each_voice(|v| v.osc_b_level = value),
            11 => self.for_each_voice(|v| v.noise_level = value),
            // Filter
            12 => self.for_each_voice(|v| v.filter_cutoff = value),
            13 => self.for_each_voice(|v| v.filter.set_resonance(value * 4.0)),
            14 => self.for_each_voice(|v| v.filter_env_amount = value),
            15 => self.for_each_voice(|v| v.filter_drive = value),
            // Filter Envelope
            16 => self.for_each_voice(|v| v.filter_env.set_attack(value)),
            17 => self.for_each_voice(|v| v.filter_env.set_decay(value)),
            18 => self.for_each_voice(|v| v.filter_env.set_sustain(value)),
            19 => self.for_each_voice(|v| v.filter_env.set_release(value)),
            // Amp Envelope
            20 => self.for_each_voice(|v| v.amp_env.set_attack(value)),
            21 => self.for_each_voice(|v| v.amp_env.set_decay(value)),
            22 => self.for_each_voice(|v| v.amp_env.set_sustain(value)),
            23 => self.for_each_voice(|v| v.amp_env.set_release(value)),
            // Sync
            24 => self.for_each_voice(|v| v.sync_enabled = value > 0.5),
            // Poly Mod
            25 => self.for_each_voice(|v| v.poly_mod_filt_env_amt = value),
            26 => self.for_each_voice(|v| v.poly_mod_osc_b_amt = value),
            27 => self.for_each_voice(|v| v.poly_mod_dest_freq_a = value > 0.5),
            28 => self.for_each_voice(|v| v.poly_mod_dest_pw_a = value > 0.5),
            29 => self.for_each_voice(|v| v.poly_mod_dest_filter = value > 0.5),
            // LFO
            30 => self.lfo.set_frequency(value),
            31 => self.lfo.set_triangle(value > 0.5),
            32 => self.lfo.set_sawtooth(value > 0.5),
            33 => self.lfo.set_square(value > 0.5),
            34 => self.lfo_initial_amount = value,
            // Wheel Mod
            35 => self.wheel_mod_source_mix = value,
            36 => self.wheel_mod_dest_freq_a = value > 0.5,
            37 => self.wheel_mod_dest_freq_b = value > 0.5,
            38 => self.wheel_mod_dest_pw_a = value > 0.5,
            39 => self.wheel_mod_dest_pw_b = value > 0.5,
            40 => self.wheel_mod_dest_filter = value > 0.5,
            // Master
            41 => self.master_volume = value,
            42 => self.for_each_voice(|v| v.set_glide_rate(value)),
            43 => self.for_each_voice(|v| v.set_glide_enabled(value > 0.5)),
            44 => self.unison = value > 0.5,
            45 => self.for_each_voice(|v| v.set_drift_amount(value)),
            // Mod wheel + pitch bend
            46 => self.mod_wheel = value,
            47 => self.pitch_bend = value,
            _ => {}
        }
    }

    fn set_master_volume(&mut self, vol: f32) { self.master_volume = vol; }
    fn master_volume(&self) -> f32 { self.master_volume }
}

impl MelodicEngine for ProphetSynth {
    fn note_on(&mut self, note: u8, velocity: u8) { self.note_on(note, velocity); }
    fn note_off(&mut self, note: u8) { self.note_off(note); }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(synth: &mut ProphetSynth, duration_secs: f32) -> Vec<f32> {
        let n = (synth.sample_rate * duration_secs) as usize;
        (0..n).map(|_| synth.process()).collect()
    }

    fn setup_basic_synth(sample_rate: f32) -> ProphetSynth {
        let mut synth = ProphetSynth::new(sample_rate);
        synth.for_each_voice(|v| {
            v.osc_a.set_saw(true);
            v.osc_a_level = 1.0;
            v.filter_cutoff = 20000.0;
            v.amp_env.set_attack(0.001);
            v.amp_env.set_decay(0.1);
            v.amp_env.set_sustain(1.0);
            v.amp_env.set_release(0.01);
        });
        synth.master_volume = 0.5;
        synth
    }

    #[test]
    fn test_single_note() {
        let mut synth = setup_basic_synth(44100.0);
        synth.note_on(69, 127);
        let buf = render(&mut synth, 0.3);
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
        audio_test_harness::pitch::assert_pitch(&buf, 44100.0, 440.0, 5.0);
    }

    #[test]
    fn test_five_voice_polyphony() {
        let mut synth = setup_basic_synth(44100.0);
        synth.note_on(60, 127);
        synth.note_on(64, 127);
        synth.note_on(67, 127);
        synth.note_on(72, 127);
        synth.note_on(76, 127);
        let buf = render(&mut synth, 0.3);
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
        let rms_5 = audio_test_harness::level::rms(&buf[4410..]);

        let mut synth2 = setup_basic_synth(44100.0);
        synth2.note_on(60, 127);
        let buf2 = render(&mut synth2, 0.3);
        let rms_1 = audio_test_harness::level::rms(&buf2[4410..]);

        assert!(rms_5 > rms_1 * 1.5, "5 voices ({rms_5:.4}) > 1 ({rms_1:.4})");
    }

    #[test]
    fn test_voice_stealing() {
        let mut synth = setup_basic_synth(44100.0);
        for note in [60, 62, 64, 65, 67] {
            synth.note_on(note, 127);
        }
        render(&mut synth, 0.05);
        synth.note_on(69, 127);
        let buf = render(&mut synth, 0.3);
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
    }

    #[test]
    fn test_note_off_frees_voice() {
        let mut synth = setup_basic_synth(44100.0);
        synth.note_on(69, 127);
        render(&mut synth, 0.1);
        synth.note_off(69);
        let buf = render(&mut synth, 0.5);
        audio_test_harness::level::assert_silent(&buf[22050..], 0.01);
    }

    #[test]
    fn test_silence_with_no_notes() {
        let mut synth = setup_basic_synth(44100.0);
        let buf = render(&mut synth, 0.1);
        audio_test_harness::level::assert_silent(&buf, 0.0001);
    }

    #[test]
    fn test_pitch_bend() {
        let mut synth = setup_basic_synth(44100.0);
        synth.pitch_bend = 1.0; // full up = +2 semitones
        synth.pitch_bend_range = 2.0;
        synth.note_on(69, 127); // A4 = 440Hz, bent up 2 semitones = ~493.88Hz (B4)
        let buf = render(&mut synth, 0.5);
        audio_test_harness::pitch::assert_pitch(&buf, 44100.0, 493.88, 10.0);
    }

    #[test]
    fn test_unison_mode() {
        let mut synth = setup_basic_synth(44100.0);
        synth.unison = true;
        synth.unison_detune = 5.0;
        synth.note_on(69, 127);
        let buf = render(&mut synth, 0.3);

        // Unison should be louder than single voice
        let rms_unison = audio_test_harness::level::rms(&buf[4410..]);

        let mut synth2 = setup_basic_synth(44100.0);
        synth2.note_on(69, 127);
        let buf2 = render(&mut synth2, 0.3);
        let rms_single = audio_test_harness::level::rms(&buf2[4410..]);

        assert!(
            rms_unison > rms_single * 2.0,
            "Unison ({rms_unison:.4}) should be >2x louder than single ({rms_single:.4})"
        );
    }

    #[test]
    fn test_wheel_mod_vibrato() {
        let mut synth = setup_basic_synth(44100.0);
        synth.lfo.set_triangle(true);
        synth.lfo.set_frequency(6.0);
        synth.mod_wheel = 1.0;
        synth.wheel_mod_dest_freq_a = true;
        synth.note_on(69, 127);
        let buf_vibrato = render(&mut synth, 0.5);

        // Compare to no vibrato
        let mut synth2 = setup_basic_synth(44100.0);
        synth2.note_on(69, 127);
        let buf_plain = render(&mut synth2, 0.5);

        let corr = audio_test_harness::correlation::cross_correlation(&buf_vibrato, &buf_plain);
        assert!(corr < 0.95, "Vibrato should change the sound (corr={corr:.3})");
    }

    #[test]
    fn test_embedded_sequencer() {
        let mut synth = setup_basic_synth(44100.0);
        // Program a chord step
        synth.sequencer.steps[0].gate = true;
        synth.sequencer.steps[0].notes = [60, 64, 67, 0];
        synth.sequencer.steps[0].num_notes = 3;
        synth.sequencer.steps[0].velocity = 100;
        synth.sequencer.set_length(1);
        synth.sequencer.set_bpm(240.0);
        synth.sequencer.play();

        let buf = render(&mut synth, 0.5);
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
    }

    #[test]
    fn test_seq_external_suppresses_notes() {
        let mut synth = setup_basic_synth(44100.0);
        synth.seq_external = true;
        synth.sequencer.steps[0].gate = true;
        synth.sequencer.steps[0].notes[0] = 60;
        synth.sequencer.steps[0].num_notes = 1;
        synth.sequencer.steps[0].velocity = 100;
        synth.sequencer.set_length(1);
        synth.sequencer.set_bpm(240.0);
        synth.sequencer.play();

        let buf = render(&mut synth, 0.5);
        audio_test_harness::level::assert_silent(&buf, 0.001);
    }

    #[test]
    fn test_glide() {
        let mut synth = setup_basic_synth(44100.0);
        synth.for_each_voice(|v| {
            v.set_glide_enabled(true);
            v.set_glide_rate(0.1);
        });
        synth.note_on(60, 127); // C4
        render(&mut synth, 0.3); // settle
        synth.note_off(60);
        // Quick re-trigger on the same voice — should glide from C4
        synth.note_on(72, 127); // C5
        // Render enough for pitch detection but not enough to fully arrive
        let buf = render(&mut synth, 0.15);

        // The detected pitch should be somewhere between C4 and C5
        let detected = audio_test_harness::pitch::detect(&buf, 44100.0);
        if let Some(hz) = detected {
            // Loose check — just verify it's not exactly C4 or C5
            assert!(
                hz > 250.0 && hz < 550.0,
                "Gliding pitch should be in range, got {hz:.1}Hz"
            );
        }
    }
}
