use crate::{oscillator::Oscillator, filter::LadderFilter, envelope::Envelope, noise::NoiseGenerator, drift::DriftGenerator, glide::Glide, tuning};

pub struct Voice {
    pub osc_a: Oscillator,
    pub osc_b: Oscillator,
    pub filter: LadderFilter,
    pub filter_env: Envelope,
    pub amp_env: Envelope,
    noise: NoiseGenerator,
    drift_a: DriftGenerator,
    drift_b: DriftGenerator,
    glide: Glide,
    pub active: bool,
    pub note: u8,
    sample_rate: f32,

    // Mixer levels
    pub osc_a_level: f32,
    pub osc_b_level: f32,
    pub noise_level: f32,

    // Filter params
    pub filter_cutoff: f32,
    pub filter_env_amount: f32,
    pub filter_kbd_track: f32,

    // Hard sync
    pub sync_enabled: bool,

    // Poly Mod
    pub poly_mod_filt_env_amt: f32,
    pub poly_mod_osc_b_amt: f32,
    pub poly_mod_dest_freq_a: bool,
    pub poly_mod_dest_pw_a: bool,
    pub poly_mod_dest_filter: bool,

    // Wheel Mod (values set by synth engine from global LFO/noise + mod wheel)
    pub wheel_mod_signal: f32,
    pub wheel_mod_dest_freq_a: bool,
    pub wheel_mod_dest_freq_b: bool,
    pub wheel_mod_dest_pw_a: bool,
    pub wheel_mod_dest_pw_b: bool,
    pub wheel_mod_dest_filter: bool,

    // Pitch bend (semitones, set by synth engine)
    pub pitch_bend_semitones: f32,
}

impl Voice {
    pub fn new(sample_rate: f32, voice_id: u32) -> Self {
        Self {
            osc_a: Oscillator::new(sample_rate),
            osc_b: Oscillator::new(sample_rate),
            filter: LadderFilter::new(sample_rate),
            filter_env: Envelope::new(sample_rate),
            amp_env: Envelope::new(sample_rate),
            noise: NoiseGenerator::new(voice_id as u64 + 1),
            drift_a: DriftGenerator::new(voice_id as u64 * 2 + 100),
            drift_b: DriftGenerator::new(voice_id as u64 * 2 + 101),
            glide: Glide::new(),
            active: false,
            note: 0,
            sample_rate,
            osc_a_level: 0.0,
            osc_b_level: 0.0,
            noise_level: 0.0,
            filter_cutoff: 20000.0,
            filter_env_amount: 0.0,
            filter_kbd_track: 0.0,
            sync_enabled: false,
            poly_mod_filt_env_amt: 0.0,
            poly_mod_osc_b_amt: 0.0,
            poly_mod_dest_freq_a: false,
            poly_mod_dest_pw_a: false,
            poly_mod_dest_filter: false,
            wheel_mod_signal: 0.0,
            wheel_mod_dest_freq_a: false,
            wheel_mod_dest_freq_b: false,
            wheel_mod_dest_pw_a: false,
            wheel_mod_dest_pw_b: false,
            wheel_mod_dest_filter: false,
            pitch_bend_semitones: 0.0,
        }
    }

    pub fn set_glide_rate(&mut self, rate: f32) {
        self.glide.set_rate(rate);
    }

    pub fn set_glide_enabled(&mut self, enabled: bool) {
        self.glide.set_enabled(enabled);
    }

    pub fn set_drift_amount(&mut self, hz: f32) {
        self.drift_a.set_amount(hz);
        self.drift_b.set_amount(hz);
    }

    pub fn note_on(&mut self, note: u8, _velocity: u8) {
        self.note = note;
        self.active = true;
        let hz = tuning::note_to_hz(note);
        self.glide.set_target(hz);
        self.filter_env.gate_on();
        self.amp_env.gate_on();
    }

    pub fn note_off(&mut self) {
        self.filter_env.gate_off();
        self.amp_env.gate_off();
    }

    pub fn process(&mut self) -> f32 {
        if !self.active {
            return 0.0;
        }

        // Process envelopes
        let filter_env_val = self.filter_env.process();
        let amp_env_val = self.amp_env.process();

        if !self.amp_env.is_active() {
            self.active = false;
            return 0.0;
        }

        // Glide (portamento)
        let base_hz = self.glide.process(self.sample_rate);

        // Pitch bend
        let bend_factor = 2.0f32.powf(self.pitch_bend_semitones / 12.0);
        let bent_hz = base_hz * bend_factor;

        // Per-voice drift
        let drift_a = self.drift_a.process(self.sample_rate);
        let drift_b = self.drift_b.process(self.sample_rate);

        // Poly Mod signal (per-voice)
        let poly_mod = filter_env_val * self.poly_mod_filt_env_amt
            + self.osc_b.triangle_output() * self.poly_mod_osc_b_amt;

        // Wheel Mod contributions
        let wm = self.wheel_mod_signal;
        let wm_freq_a = if self.wheel_mod_dest_freq_a { wm * bent_hz * 0.1 } else { 0.0 };
        let wm_freq_b = if self.wheel_mod_dest_freq_b { wm * bent_hz * 0.1 } else { 0.0 };
        let wm_filter = if self.wheel_mod_dest_filter { wm * 5000.0 } else { 0.0 };

        // Set oscillator frequencies with drift
        self.osc_b.set_frequency(bent_hz + drift_b + wm_freq_b);

        // FM offset for Osc A from Poly Mod
        let fm_offset = if self.poly_mod_dest_freq_a {
            poly_mod * bent_hz
        } else {
            0.0
        };

        // PW modulation from Poly Mod and Wheel Mod
        if self.poly_mod_dest_pw_a {
            let base_pw = 0.5;
            self.osc_a.set_pulse_width((base_pw + poly_mod * 0.4).clamp(0.01, 0.99));
        }
        if self.wheel_mod_dest_pw_a {
            let current = if self.poly_mod_dest_pw_a {
                // Already set above, read it back approximately
                (0.5 + poly_mod * 0.4).clamp(0.01, 0.99)
            } else {
                0.5
            };
            self.osc_a.set_pulse_width((current + wm * 0.3).clamp(0.01, 0.99));
        }
        if self.wheel_mod_dest_pw_b {
            self.osc_b.set_pulse_width((0.5 + wm * 0.3).clamp(0.01, 0.99));
        }

        // Process Osc B first (sync master + poly mod source)
        let osc_b_out = self.osc_b.process();

        // Hard sync
        if self.sync_enabled && self.osc_b.wrapped() {
            self.osc_a.sync_reset();
        }

        // Process Osc A with drift + FM + wheel mod
        self.osc_a.set_frequency(bent_hz + drift_a + wm_freq_a);
        let osc_a_out = self.osc_a.process_with_fm(fm_offset);

        let noise_out = self.noise.white();

        // Mixer
        let mixed = osc_a_out * self.osc_a_level
            + osc_b_out * self.osc_b_level
            + noise_out * self.noise_level;

        // Filter
        let note_hz = tuning::note_to_hz(self.note);
        let kbd_offset = (note_hz - 261.63) * self.filter_kbd_track;
        let poly_mod_filter = if self.poly_mod_dest_filter { poly_mod * 5000.0 } else { 0.0 };
        let cutoff = (self.filter_cutoff
            + filter_env_val * self.filter_env_amount
            + poly_mod_filter
            + wm_filter
            + kbd_offset)
            .clamp(20.0, 20000.0);
        self.filter.set_cutoff(cutoff);
        let filtered = self.filter.process(mixed);

        // VCA
        filtered * amp_env_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render_voice(voice: &mut Voice, duration_secs: f32) -> Vec<f32> {
        let n = (voice.sample_rate * duration_secs) as usize;
        (0..n).map(|_| voice.process()).collect()
    }

    #[test]
    fn test_voice_produces_sound() {
        let mut v = Voice::new(44100.0, 0);
        v.osc_a.set_saw(true);
        v.osc_a_level = 1.0;
        v.filter_cutoff = 20000.0;
        v.amp_env.set_attack(0.001);
        v.amp_env.set_sustain(1.0);
        v.note_on(69, 127); // A4
        let buf = render_voice(&mut v, 0.2);
        audio_test_harness::level::assert_not_silent(&buf, 0.05);
    }

    #[test]
    fn test_voice_respects_note() {
        let mut v1 = Voice::new(44100.0, 0);
        v1.osc_a.set_saw(true);
        v1.osc_a_level = 1.0;
        v1.filter_cutoff = 20000.0;
        v1.amp_env.set_attack(0.001);
        v1.amp_env.set_sustain(1.0);
        v1.note_on(69, 127); // A4 = 440Hz
        let buf_a4 = render_voice(&mut v1, 0.5);

        let mut v2 = Voice::new(44100.0, 1);
        v2.osc_a.set_saw(true);
        v2.osc_a_level = 1.0;
        v2.filter_cutoff = 20000.0;
        v2.amp_env.set_attack(0.001);
        v2.amp_env.set_sustain(1.0);
        v2.note_on(60, 127); // C4 = 261.63Hz
        let buf_c4 = render_voice(&mut v2, 0.5);

        audio_test_harness::pitch::assert_pitch(&buf_a4, 44100.0, 440.0, 5.0);
        audio_test_harness::pitch::assert_pitch(&buf_c4, 44100.0, 261.63, 5.0);
    }

    #[test]
    fn test_amp_envelope_shapes_output() {
        let mut v = Voice::new(44100.0, 0);
        v.osc_a.set_saw(true);
        v.osc_a_level = 1.0;
        v.filter_cutoff = 20000.0;
        v.amp_env.set_attack(0.01);
        v.amp_env.set_decay(0.05);
        v.amp_env.set_sustain(0.0);
        v.amp_env.set_release(0.001);
        v.note_on(69, 127);
        let buf = render_voice(&mut v, 0.5);

        // Start should be louder than end (decay to 0 sustain)
        let early_rms = audio_test_harness::level::rms(&buf[441..2205]); // 10-50ms
        let late_rms = audio_test_harness::level::rms(&buf[22050..]); // 500ms+
        assert!(
            early_rms > late_rms * 5.0,
            "Early ({early_rms}) should be much louder than late ({late_rms})"
        );
    }

    #[test]
    fn test_filter_envelope_moves_cutoff() {
        // With high filter env amount and low base cutoff, attack should be bright
        let mut v = Voice::new(44100.0, 0);
        v.osc_a.set_saw(true);
        v.osc_a_level = 1.0;
        v.filter_cutoff = 200.0; // low base cutoff
        v.filter_env_amount = 10000.0; // large env amount
        v.filter_env.set_attack(0.001);
        v.filter_env.set_decay(0.2);
        v.filter_env.set_sustain(0.0);
        v.filter_env.set_release(0.001);
        v.amp_env.set_attack(0.001);
        v.amp_env.set_sustain(1.0);
        v.note_on(69, 127);
        let buf = render_voice(&mut v, 1.0);

        // Early part should have more high-frequency content than late part
        // Use equal-sized windows for fair comparison
        let window_size = 4410; // 100ms
        let early = &buf[44..44 + window_size];
        let late = &buf[22050..22050 + window_size];

        let early_rms = audio_test_harness::level::rms(early);
        let late_rms = audio_test_harness::level::rms(late);

        // With filter env decayed to 0 sustain and base cutoff at 200Hz,
        // the late portion should be much quieter (heavily filtered)
        assert!(
            early_rms > late_rms * 1.5,
            "Early should be brighter/louder (early_rms={early_rms:.4}, late_rms={late_rms:.4})"
        );
    }

    #[test]
    fn test_mixer_levels() {
        // osc_a_level = 0 should silence osc_a contribution
        let mut v = Voice::new(44100.0, 0);
        v.osc_a.set_saw(true);
        v.osc_a_level = 0.0; // muted
        v.osc_b_level = 0.0;
        v.noise_level = 0.0;
        v.filter_cutoff = 20000.0;
        v.amp_env.set_attack(0.001);
        v.amp_env.set_sustain(1.0);
        v.note_on(69, 127);
        let buf = render_voice(&mut v, 0.1);
        audio_test_harness::level::assert_silent(&buf, 0.001);
    }

    #[test]
    fn test_hard_sync_in_voice() {
        let mut v = Voice::new(44100.0, 0);
        v.osc_a.set_saw(true);
        v.osc_a_level = 1.0;
        v.osc_b_level = 0.0;
        v.filter_cutoff = 20000.0;
        v.amp_env.set_attack(0.001);
        v.amp_env.set_sustain(1.0);
        v.sync_enabled = true;
        v.note_on(60, 127); // C4
        // Set osc_a to non-harmonic ratio for audible sync effect
        v.osc_a.set_frequency(tuning::note_to_hz(67)); // G4 (~392Hz)
        let buf = render_voice(&mut v, 0.5);
        // Fundamental should be at osc_b frequency (C4 ~261Hz)
        audio_test_harness::pitch::assert_pitch(&buf, 44100.0, 261.63, 10.0);
    }

    #[test]
    fn test_poly_mod_fm() {
        // Route Osc B to Osc A frequency — should create FM sidebands
        let mut v = Voice::new(44100.0, 0);
        v.osc_a.set_saw(true);
        v.osc_b.set_tri(true); // triangle output used as mod source
        v.osc_a_level = 1.0;
        v.osc_b_level = 0.0; // don't hear osc B directly
        v.filter_cutoff = 20000.0;
        v.amp_env.set_attack(0.001);
        v.amp_env.set_sustain(1.0);
        v.poly_mod_osc_b_amt = 2.0; // strong modulation
        v.poly_mod_dest_freq_a = true;
        v.note_on(69, 127);
        let buf_fm = render_voice(&mut v, 0.5);

        // Compare to no poly mod
        let mut v2 = Voice::new(44100.0, 1);
        v2.osc_a.set_saw(true);
        v2.osc_a_level = 1.0;
        v2.filter_cutoff = 20000.0;
        v2.amp_env.set_attack(0.001);
        v2.amp_env.set_sustain(1.0);
        v2.note_on(69, 127);
        let buf_plain = render_voice(&mut v2, 0.5);

        let corr = audio_test_harness::correlation::cross_correlation(&buf_fm, &buf_plain);
        assert!(corr < 0.9, "Poly Mod FM should change the sound (corr={corr:.3})");
    }

    #[test]
    fn test_note_off_releases() {
        let mut v = Voice::new(44100.0, 0);
        v.osc_a.set_saw(true);
        v.osc_a_level = 1.0;
        v.filter_cutoff = 20000.0;
        v.amp_env.set_attack(0.001);
        v.amp_env.set_sustain(1.0);
        v.amp_env.set_release(0.01);
        v.note_on(69, 127);
        render_voice(&mut v, 0.1); // sustain
        v.note_off();
        let buf = render_voice(&mut v, 0.5);
        // End should be silent
        let tail = &buf[22050..];
        audio_test_harness::level::assert_silent(tail, 0.001);
    }
}
