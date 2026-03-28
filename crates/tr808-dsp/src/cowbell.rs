use dsp_common::{ad_envelope::ADEnvelope, svfilter::SVFilter};

/// TR-808 Cowbell — two square oscillators (800Hz + 540Hz) through BPF,
/// with a two-stage envelope (fast initial decay, slow tail).
pub struct Cowbell {
    phase1: f32,
    phase2: f32,
    inc1: f32,  // 800 Hz
    inc2: f32,  // 540 Hz
    bpf: SVFilter,
    env_fast: ADEnvelope,
    env_slow: ADEnvelope,
    pub level: f32,
}

impl Cowbell {
    pub fn new(sample_rate: f32) -> Self {
        let mut bpf = SVFilter::new(sample_rate);
        bpf.set_freq(880.0);
        bpf.set_q(5.0);

        let mut env_fast = ADEnvelope::new(sample_rate);
        env_fast.set_attack(0.0003);
        env_fast.set_decay(0.05); // 50ms fast stage

        let mut env_slow = ADEnvelope::new(sample_rate);
        env_slow.set_attack(0.0003);
        env_slow.set_decay(0.5); // 500ms slow tail

        Self {
            phase1: 0.0, phase2: 0.0,
            inc1: 800.0 / sample_rate,
            inc2: 540.0 / sample_rate,
            bpf, env_fast, env_slow, level: 0.6,
        }
    }

    pub fn trigger(&mut self) {
        self.env_fast.trigger();
        self.env_slow.trigger();
    }

    pub fn process(&mut self) -> f32 {
        // Two square oscillators
        self.phase1 += self.inc1;
        if self.phase1 >= 1.0 { self.phase1 -= 1.0; }
        self.phase2 += self.inc2;
        if self.phase2 >= 1.0 { self.phase2 -= 1.0; }

        let sq1 = if self.phase1 < 0.5 { 1.0 } else { -1.0 };
        let sq2 = if self.phase2 < 0.5 { 1.0 } else { -1.0 };
        let mixed = (sq1 + sq2) * 0.5;

        // Bandpass filter
        let filtered = self.bpf.process_bp(mixed);

        // Two-stage envelope: fast attack/decay to 50%, then slow tail
        let fast = self.env_fast.process();
        let slow = self.env_slow.process();
        let env = fast * 0.5 + slow * 0.5; // blends the two stages

        filtered * env * self.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cb_produces_sound() {
        let mut cb = Cowbell::new(44100.0);
        cb.trigger();
        let buf: Vec<f32> = (0..4410).map(|_| cb.process()).collect();
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
    }
}
