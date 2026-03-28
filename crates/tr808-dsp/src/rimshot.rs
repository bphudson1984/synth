use dsp_common::{ad_envelope::ADEnvelope, svfilter::SVFilter};

/// TR-808 Rim Shot — two inharmonic resonators, very short, saturated.
pub struct RimShot {
    osc1: SVFilter, // 1667 Hz
    osc2: SVFilter, // 455 Hz
    amp_env: ADEnvelope,
    impulse_counter: u32,
    impulse_length: u32,
    pub level: f32,
}

impl RimShot {
    pub fn new(sample_rate: f32) -> Self {
        let mut osc1 = SVFilter::new(sample_rate);
        osc1.set_freq(1667.0); osc1.set_q(10.0);
        let mut osc2 = SVFilter::new(sample_rate);
        osc2.set_freq(455.0); osc2.set_q(8.0);
        let mut amp_env = ADEnvelope::new(sample_rate);
        amp_env.set_attack(0.0002);
        amp_env.set_decay(0.01); // 10ms — very short

        Self {
            osc1, osc2, amp_env,
            impulse_counter: 0,
            impulse_length: (sample_rate * 0.0005) as u32,
            level: 0.7,
        }
    }

    pub fn trigger(&mut self) {
        self.amp_env.trigger();
        self.impulse_counter = self.impulse_length;
        self.osc1.clear(); self.osc2.clear();
    }

    pub fn process(&mut self) -> f32 {
        let impulse = if self.impulse_counter > 0 {
            self.impulse_counter -= 1; 1.0
        } else { 0.0 };

        let r1 = self.osc1.process_bp(impulse);
        let r2 = self.osc2.process_bp(impulse);
        let mixed = (r1 + r2 * 0.5).tanh(); // saturation for snap
        let env = self.amp_env.process();
        mixed * env * self.level
    }
}
