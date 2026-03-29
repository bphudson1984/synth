use std::f32::consts::PI;
use dsp_common::{ad_envelope::ADEnvelope, pitch_envelope::PitchEnvelope};

/// TR-909 Tom — sine VCO with pitch sweep. Punchier than 808.
pub struct Tom909 {
    phase: f32,
    pitch_env: PitchEnvelope,
    amp_env: ADEnvelope,
    base_freq: f32,
    sample_rate: f32,
    pub level: f32,
    pub tuning: f32,
}

impl Tom909 {
    pub fn new(sample_rate: f32, base_freq: f32, decay_secs: f32) -> Self {
        let sweep_start = base_freq * 1.2;
        let mut pitch_env = PitchEnvelope::new(sample_rate);
        pitch_env.set_sweep(sweep_start, base_freq, 0.008);

        let mut amp_env = ADEnvelope::new(sample_rate);
        amp_env.set_attack(0.0003);
        amp_env.set_decay(decay_secs);

        Self { phase: 0.0, pitch_env, amp_env, base_freq, sample_rate, level: 0.7, tuning: 0.5 }
    }

    pub fn low(sr: f32) -> Self { Self::new(sr, 100.0, 0.15) }
    pub fn mid(sr: f32) -> Self { Self::new(sr, 150.0, 0.1) }
    pub fn high(sr: f32) -> Self { Self::new(sr, 220.0, 0.08) }

    pub fn trigger(&mut self) {
        let freq = self.base_freq * (0.8 + self.tuning * 0.4);
        self.pitch_env.set_sweep(freq * 1.2, freq, 0.008);
        self.pitch_env.trigger(freq * 1.2);
        self.amp_env.trigger();
        self.phase = 0.0;
    }

    pub fn process(&mut self) -> f32 {
        let freq = self.pitch_env.process();
        self.phase += freq / self.sample_rate;
        if self.phase >= 1.0 { self.phase -= 1.0; }
        let sine = (self.phase * 2.0 * PI).sin();
        let env = self.amp_env.process();
        sine * env * self.level
    }
}
