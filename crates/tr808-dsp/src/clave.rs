use dsp_common::{ad_envelope::ADEnvelope, svfilter::SVFilter};

/// TR-808 Clave — single resonator at 2500Hz, very short, no filter.
/// The simplest 808 voice.
pub struct Clave {
    resonator: SVFilter,
    amp_env: ADEnvelope,
    impulse_counter: u32,
    impulse_length: u32,
    pub level: f32,
}

impl Clave {
    pub fn new(sample_rate: f32) -> Self {
        let mut resonator = SVFilter::new(sample_rate);
        resonator.set_freq(2500.0);
        resonator.set_q(25.0); // high Q for woody ring

        let mut amp_env = ADEnvelope::new(sample_rate);
        amp_env.set_attack(0.0002);
        amp_env.set_decay(0.025); // 25ms

        Self {
            resonator, amp_env,
            impulse_counter: 0,
            impulse_length: (sample_rate * 0.0003) as u32,
            level: 0.6,
        }
    }

    pub fn trigger(&mut self) {
        self.amp_env.trigger();
        self.impulse_counter = self.impulse_length;
        self.resonator.clear();
    }

    pub fn process(&mut self) -> f32 {
        let impulse = if self.impulse_counter > 0 {
            self.impulse_counter -= 1; 0.8
        } else { 0.0 };
        let ring = self.resonator.process_bp(impulse);
        let env = self.amp_env.process();
        ring * env * self.level
    }
}
