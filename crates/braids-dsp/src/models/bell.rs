use super::OscModel;
use std::f32::consts::TAU;

const NUM_PARTIALS: usize = 8;
const HARMONIC_RATIOS: [f32; NUM_PARTIALS] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
const INHARM_RATIOS: [f32; NUM_PARTIALS] = [1.0, 2.76, 4.07, 5.39, 6.28, 6.63, 7.61, 9.02];

pub struct BellModel {
    phases: [f32; NUM_PARTIALS],
    amps: [f32; NUM_PARTIALS],
    sr: f32,
}

impl BellModel {
    pub fn new(sr: f32) -> Self {
        Self { phases: [0.0; NUM_PARTIALS], amps: [0.0; NUM_PARTIALS], sr }
    }
}

impl OscModel for BellModel {
    fn render(&mut self, freq: f32, timbre: f32, color: f32, out: &mut [f32]) {
        let decay_base = 0.99990 + timbre * 0.00009; // slower decay with more timbre
        for s in out.iter_mut() {
            let mut sum = 0.0;
            for i in 0..NUM_PARTIALS {
                let ratio = HARMONIC_RATIOS[i] * (1.0 - color) + INHARM_RATIOS[i] * color;
                let dt = freq * ratio / self.sr;
                self.phases[i] += dt;
                if self.phases[i] >= 1.0 { self.phases[i] -= 1.0; }
                sum += (self.phases[i] * TAU).sin() * self.amps[i];
                // Per-partial decay: higher partials decay faster
                let partial_decay = decay_base.powf(1.0 + i as f32 * 0.5);
                self.amps[i] *= partial_decay;
            }
            *s = sum * 0.3;
        }
    }

    fn reset(&mut self) {
        self.phases = [0.0; NUM_PARTIALS];
        for i in 0..NUM_PARTIALS {
            self.amps[i] = 1.0 / (i as f32 + 1.0);
        }
    }
}
