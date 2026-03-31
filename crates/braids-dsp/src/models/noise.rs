use super::OscModel;
use std::f32::consts::PI;

pub struct NoiseModel {
    rng: u64,
    s1: f32,
    s2: f32,
    sr: f32,
}

impl NoiseModel {
    pub fn new(sr: f32) -> Self { Self { rng: 54321, s1: 0.0, s2: 0.0, sr } }

    fn white(&mut self) -> f32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        (self.rng as i64 as f32) / (i64::MAX as f32) * 2.0
    }
}

impl OscModel for NoiseModel {
    fn render(&mut self, freq: f32, timbre: f32, color: f32, out: &mut [f32]) {
        let q = 0.5 + timbre * 20.0;
        let w0 = 2.0 * PI * freq / self.sr;
        let alpha = w0.sin() / (2.0 * q);
        // Crossfade LP→BP→HP based on color
        for s in out.iter_mut() {
            let input = self.white();
            // SVF-style: compute LP, BP, HP
            let a0 = 1.0 + alpha;
            let lp_b0 = ((1.0 - w0.cos()) / 2.0) / a0;
            let bp_b0 = alpha / a0;
            let hp_b0 = ((1.0 + w0.cos()) / 2.0) / a0;
            let a1 = -2.0 * w0.cos() / a0;
            let a2 = (1.0 - alpha) / a0;
            let y = bp_b0 * input - a1 * self.s1 - a2 * self.s2;
            self.s2 = self.s1;
            self.s1 = y;
            // Use BP as base, mix in LP/HP character via color
            let lp_char = y * (1.0 - color);
            let hp_char = y * color;
            *s = (lp_char + hp_char).clamp(-1.0, 1.0);
        }
    }
    fn reset(&mut self) { self.s1 = 0.0; self.s2 = 0.0; }
}
