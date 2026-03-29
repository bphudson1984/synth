use dsp_common::note_to_hz;
use crate::oscillator::Oscillator;
use crate::filter::DiodeLadder;
use crate::envelope::{FilterEnvelope, AmpEnvelope};
use crate::glide::Glide;

/// Monophonic TB-303 voice.
/// Signal chain: Oscillator → DiodeLadder Filter → Distortion → Amp → Output
pub struct AcidVoice {
    osc: Oscillator,
    filter: DiodeLadder,
    filter_env: FilterEnvelope,
    amp_env: AmpEnvelope,
    glide: Glide,
    sample_rate: f32,

    pub cutoff: f32,
    pub resonance: f32,
    pub env_mod: f32,
    pub decay: f32,
    pub accent_level: f32,
    pub distortion: f32,
    pub volume: f32,

    current_accent: bool,
    first_note: bool,
}

impl AcidVoice {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            osc: Oscillator::new(sample_rate),
            filter: DiodeLadder::new(sample_rate),
            filter_env: FilterEnvelope::new(sample_rate),
            amp_env: AmpEnvelope::new(sample_rate),
            glide: Glide::new(sample_rate),
            sample_rate,
            cutoff: 800.0, resonance: 0.4, env_mod: 0.6,
            decay: 0.3, accent_level: 0.5, distortion: 0.0, volume: 0.8,
            current_accent: false, first_note: true,
        }
    }

    pub fn set_waveform(&mut self, saw: bool) { self.osc.set_waveform(saw); }

    pub fn note_on(&mut self, note: u8, accent: bool, slide: bool) {
        let target_hz = note_to_hz(note);
        if slide && !self.first_note {
            self.glide.set_enabled(true);
            self.glide.set_target(target_hz);
        } else {
            self.glide.set_enabled(false);
            self.glide.jump_to(target_hz);
        }
        self.current_accent = accent;
        if accent { self.filter_env.set_decay(0.03); } else { self.filter_env.set_decay(self.decay); }
        self.filter_env.trigger();
        self.amp_env.trigger();
        self.first_note = false;
    }

    pub fn note_off(&mut self) { self.amp_env.gate_off(); }

    pub fn process(&mut self) -> f32 {
        let freq = self.glide.process();
        self.osc.set_frequency(freq);
        let osc_out = self.osc.process();

        let env_val = self.filter_env.process();
        let accent_boost = if self.current_accent { self.accent_level * 3000.0 } else { 0.0 };
        let effective_cutoff = (self.cutoff + self.env_mod * env_val * 4000.0 + accent_boost * env_val)
            .clamp(20.0, self.sample_rate * 0.45);
        let effective_reso = if self.current_accent {
            (self.resonance + self.accent_level * 0.15).min(1.0)
        } else {
            self.resonance
        };

        self.filter.set_cutoff(effective_cutoff);
        self.filter.set_resonance(effective_reso);
        let filtered = self.filter.process(osc_out);

        // Distortion: tanh waveshaper after filter
        let distorted = if self.distortion > 0.001 {
            let drive = 1.0 + self.distortion * 15.0;
            let wet = (filtered * drive).tanh() / drive.sqrt();
            filtered * (1.0 - self.distortion) + wet * self.distortion * 2.0
        } else {
            filtered
        };

        let amp = self.amp_env.process();
        let accent_vol = if self.current_accent { 1.0 + self.accent_level * 0.3 } else { 1.0 };
        distorted * amp * accent_vol * self.volume
    }
}
