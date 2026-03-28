use dsp_common::{ad_envelope::ADEnvelope, pitch_envelope::PitchEnvelope, svfilter::SVFilter};

/// TR-808 Tom — single bridged-T resonator with pitch sweep.
/// Used for Low Tom, Mid Tom, and Hi Tom (different base frequencies).
pub struct Tom {
    resonator: SVFilter,
    pitch_env: PitchEnvelope,
    amp_env: ADEnvelope,
    base_freq: f32,
    impulse_counter: u32,
    impulse_length: u32,
    sample_rate: f32,
    pub level: f32,
    pub tuning: f32, // 0-1, adjusts base frequency ±20%
}

impl Tom {
    pub fn new(sample_rate: f32, base_freq: f32, decay_secs: f32) -> Self {
        let mut resonator = SVFilter::new(sample_rate);
        resonator.set_freq(base_freq);
        resonator.set_q(15.0);

        let sweep_start = base_freq * 1.15; // 15% higher at attack
        let mut pitch_env = PitchEnvelope::new(sample_rate);
        pitch_env.set_sweep(sweep_start, base_freq, 0.008);

        let mut amp_env = ADEnvelope::new(sample_rate);
        amp_env.set_attack(0.0005);
        amp_env.set_decay(decay_secs);

        Self {
            resonator, pitch_env, amp_env,
            base_freq,
            impulse_counter: 0,
            impulse_length: (sample_rate * 0.001) as u32,
            sample_rate, level: 0.7, tuning: 0.5,
        }
    }

    pub fn low(sample_rate: f32) -> Self { Self::new(sample_rate, 90.0, 0.2) }
    pub fn mid(sample_rate: f32) -> Self { Self::new(sample_rate, 135.0, 0.13) }
    pub fn high(sample_rate: f32) -> Self { Self::new(sample_rate, 185.0, 0.1) }

    pub fn trigger(&mut self) {
        let freq = self.base_freq * (0.8 + self.tuning * 0.4); // ±20%
        let sweep_start = freq * 1.15;
        self.pitch_env.set_sweep(sweep_start, freq, 0.008);
        self.pitch_env.trigger(sweep_start);
        self.amp_env.trigger();
        self.impulse_counter = self.impulse_length;
        self.resonator.clear();
    }

    pub fn process(&mut self) -> f32 {
        let freq = self.pitch_env.process();
        self.resonator.set_freq(freq);

        let impulse = if self.impulse_counter > 0 {
            self.impulse_counter -= 1; 0.7
        } else { 0.0 };

        let resonated = self.resonator.process_bp(impulse);
        let env = self.amp_env.process();
        resonated * env * self.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tom_produces_sound() {
        let mut tom = Tom::low(44100.0);
        tom.trigger();
        let buf: Vec<f32> = (0..4410).map(|_| tom.process()).collect();
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
    }

    #[test]
    fn test_toms_different_pitches() {
        let mut lt = Tom::low(44100.0);
        let mut ht = Tom::high(44100.0);
        lt.trigger(); ht.trigger();
        let buf_lt: Vec<f32> = (0..4410).map(|_| lt.process()).collect();
        let buf_ht: Vec<f32> = (0..4410).map(|_| ht.process()).collect();
        let corr = audio_test_harness::correlation::cross_correlation(&buf_lt, &buf_ht);
        assert!(corr < 0.8, "LT and HT should sound different (corr={corr:.3})");
    }
}
