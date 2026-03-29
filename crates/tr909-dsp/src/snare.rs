use std::f32::consts::PI;
use dsp_common::{NoiseGenerator, ad_envelope::ADEnvelope, svfilter::SVFilter};

/// TR-909 Snare — sine body + BPF noise. Brighter and tighter than 808.
pub struct SnareDrum909 {
    phase: f32,
    body_freq: f32,
    noise: NoiseGenerator,
    noise_bpf: SVFilter,   // BPF at ~4000Hz (808 is 2749Hz)
    body_env: ADEnvelope,
    noise_env: ADEnvelope,
    sample_rate: f32,

    pub level: f32,
    pub tone: f32,   // 0-1: body vs noise balance
    pub snappy: f32, // 0-1: noise decay
}

impl SnareDrum909 {
    pub fn new(sample_rate: f32) -> Self {
        let mut noise_bpf = SVFilter::new(sample_rate);
        noise_bpf.set_freq(4000.0); // brighter than 808's 2749Hz
        noise_bpf.set_q(2.5);

        let mut body_env = ADEnvelope::new(sample_rate);
        body_env.set_attack(0.0003);
        body_env.set_decay(0.08); // tighter than 808

        let mut noise_env = ADEnvelope::new(sample_rate);
        noise_env.set_attack(0.0003);
        noise_env.set_decay(0.12);

        Self {
            phase: 0.0, body_freq: 190.0,
            noise: NoiseGenerator::new(909),
            noise_bpf, body_env, noise_env, sample_rate,
            level: 0.8, tone: 0.5, snappy: 0.5,
        }
    }

    pub fn trigger(&mut self) {
        let noise_decay = 0.05 + self.snappy * 0.15; // 50-200ms
        self.noise_env.set_decay(noise_decay);
        self.body_env.trigger();
        self.noise_env.trigger();
        self.phase = 0.0;
    }

    pub fn process(&mut self) -> f32 {
        // Sine body
        self.phase += self.body_freq / self.sample_rate;
        if self.phase >= 1.0 { self.phase -= 1.0; }
        let body = (self.phase * 2.0 * PI).sin();
        let body_val = self.body_env.process();

        // Noise through BPF
        let raw_noise = self.noise.white();
        let filtered_noise = self.noise_bpf.process_bp(raw_noise);
        let noise_val = self.noise_env.process();

        // Mix by tone control
        let body_mix = body * body_val * (1.0 - self.tone * 0.5);
        let noise_mix = filtered_noise * noise_val * (0.5 + self.tone * 0.5);

        (body_mix * 0.5 + noise_mix * 0.8) * self.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sd909_produces_sound() {
        let mut sd = SnareDrum909::new(44100.0);
        sd.trigger();
        let buf: Vec<f32> = (0..4410).map(|_| sd.process()).collect();
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
    }

    #[test]
    fn test_sd909_decays() {
        let mut sd = SnareDrum909::new(44100.0);
        sd.trigger();
        let buf: Vec<f32> = (0..44100).map(|_| sd.process()).collect();
        audio_test_harness::level::assert_silent(&buf[22050..], 0.01);
    }
}
