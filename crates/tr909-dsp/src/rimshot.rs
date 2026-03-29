use dsp_common::{ad_envelope::ADEnvelope, svfilter::SVFilter};

/// TR-909 Rim Shot — two pulse oscillators through BPF, very short.
pub struct RimShot909 {
    phase1: f32, phase2: f32,
    inc1: f32, inc2: f32,
    bpf: SVFilter,
    amp_env: ADEnvelope,
    pub level: f32,
}

impl RimShot909 {
    pub fn new(sample_rate: f32) -> Self {
        let mut bpf = SVFilter::new(sample_rate);
        bpf.set_freq(1800.0); bpf.set_q(8.0);
        let mut amp_env = ADEnvelope::new(sample_rate);
        amp_env.set_attack(0.0002); amp_env.set_decay(0.005); // 5ms

        Self {
            phase1: 0.0, phase2: 0.0,
            inc1: 500.0 / sample_rate, inc2: 800.0 / sample_rate,
            bpf, amp_env, level: 0.7,
        }
    }

    pub fn trigger(&mut self) { self.amp_env.trigger(); self.bpf.clear(); }

    pub fn process(&mut self) -> f32 {
        self.phase1 += self.inc1; if self.phase1 >= 1.0 { self.phase1 -= 1.0; }
        self.phase2 += self.inc2; if self.phase2 >= 1.0 { self.phase2 -= 1.0; }
        let sq1 = if self.phase1 < 0.5 { 1.0 } else { -1.0 };
        let sq2 = if self.phase2 < 0.5 { 1.0 } else { -1.0 };
        let filtered = self.bpf.process_bp((sq1 + sq2) * 0.5);
        let env = self.amp_env.process();
        (filtered * env).tanh() * self.level
    }
}
