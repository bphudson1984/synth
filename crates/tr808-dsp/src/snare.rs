use dsp_common::{NoiseGenerator, OnePole, ad_envelope::ADEnvelope, svfilter::SVFilter};

/// TR-808 Snare Drum — two bridged-T oscillators (238Hz + 476Hz) + HP-filtered noise.
pub struct SnareDrum {
    osc1: SVFilter,       // 238 Hz resonator
    osc2: SVFilter,       // 476 Hz resonator (octave above)
    noise: NoiseGenerator,
    noise_hpf: OnePole,   // HPF at ~2749Hz for sizzle
    tone_env: ADEnvelope, // tonal component envelope
    noise_env: ADEnvelope,// noise component envelope
    impulse_counter: u32,
    impulse_length: u32,
    sample_rate: f32,

    pub level: f32,
    pub tone: f32,   // 0-1 crossfade between osc1 and osc2
    pub snappy: f32, // 0-1 noise amount and decay
}

impl SnareDrum {
    pub fn new(sample_rate: f32) -> Self {
        let mut osc1 = SVFilter::new(sample_rate);
        osc1.set_freq(238.0); osc1.set_q(15.0);

        let mut osc2 = SVFilter::new(sample_rate);
        osc2.set_freq(476.0); osc2.set_q(12.0);

        let mut noise_hpf = OnePole::new();
        noise_hpf.set_freq(2749.0, sample_rate);

        let mut tone_env = ADEnvelope::new(sample_rate);
        tone_env.set_attack(0.0005);
        tone_env.set_decay(0.12);

        let mut noise_env = ADEnvelope::new(sample_rate);
        noise_env.set_attack(0.0005);
        noise_env.set_decay(0.15);

        Self {
            osc1, osc2, noise: NoiseGenerator::new(808),
            noise_hpf, tone_env, noise_env,
            impulse_counter: 0,
            impulse_length: (sample_rate * 0.001) as u32,
            sample_rate,
            level: 0.8, tone: 0.5, snappy: 0.5,
        }
    }

    pub fn trigger(&mut self) {
        let noise_decay = 0.05 + self.snappy * 0.2; // 50-250ms
        self.noise_env.set_decay(noise_decay);
        self.tone_env.trigger();
        self.noise_env.trigger();
        self.impulse_counter = self.impulse_length;
        self.osc1.clear();
        self.osc2.clear();
    }

    pub fn process(&mut self) -> f32 {
        let impulse = if self.impulse_counter > 0 {
            self.impulse_counter -= 1; 0.7
        } else { 0.0 };

        // Tonal: two resonators mixed by tone control
        let t1 = self.osc1.process_bp(impulse);
        let t2 = self.osc2.process_bp(impulse);
        let tonal = t1 * (1.0 - self.tone) + t2 * self.tone;
        let tone_val = self.tone_env.process();

        // Noise: highpass filtered white noise
        let raw_noise = self.noise.white();
        let hp_noise = raw_noise - self.noise_hpf.process(raw_noise); // simple HPF
        let noise_val = self.noise_env.process();

        // Mix tonal and noise
        let mixed = tonal * tone_val * 0.6 + hp_noise * noise_val * self.snappy * 0.8;

        mixed * self.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sd_produces_sound() {
        let mut sd = SnareDrum::new(44100.0);
        sd.trigger();
        let buf: Vec<f32> = (0..4410).map(|_| sd.process()).collect();
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
    }

    #[test]
    fn test_sd_decays() {
        let mut sd = SnareDrum::new(44100.0);
        sd.trigger();
        let buf: Vec<f32> = (0..44100).map(|_| sd.process()).collect();
        let tail = &buf[22050..];
        audio_test_harness::level::assert_silent(tail, 0.01);
    }
}
