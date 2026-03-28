use dsp_common::{NoiseGenerator, OnePole, ad_envelope::ADEnvelope};

/// TR-808 Maracas — highpass-filtered white noise with very short envelope.
pub struct Maracas {
    noise: NoiseGenerator,
    hpf: OnePole,
    amp_env: ADEnvelope,
    sample_rate: f32,
    pub level: f32,
}

impl Maracas {
    pub fn new(sample_rate: f32) -> Self {
        let mut hpf = OnePole::new();
        hpf.set_freq(7000.0, sample_rate);

        let mut amp_env = ADEnvelope::new(sample_rate);
        amp_env.set_attack(0.001);
        amp_env.set_decay(0.03); // 30ms

        Self { noise: NoiseGenerator::new(303), hpf, amp_env, sample_rate, level: 0.5 }
    }

    pub fn trigger(&mut self) {
        self.amp_env.trigger();
    }

    pub fn process(&mut self) -> f32 {
        let raw = self.noise.white();
        let hp = raw - self.hpf.process(raw); // simple HPF
        let env = self.amp_env.process();
        hp * env * self.level
    }
}
