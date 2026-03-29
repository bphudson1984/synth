use super::OscModel;
use std::f32::consts::PI;

pub struct MorphModel { phase: f32, phase2: f32, sr: f32 }

impl MorphModel {
    pub fn new(sr: f32) -> Self { Self { phase: 0.0, phase2: 0.0, sr } }
}

impl OscModel for MorphModel {
    fn render(&mut self, freq: f32, timbre: f32, color: f32, out: &mut [f32]) {
        let dt = freq / self.sr;
        let detune = 1.0 + color * 0.012;
        let dt2 = dt * detune;
        for s in out.iter_mut() {
            self.phase += dt; if self.phase >= 1.0 { self.phase -= 1.0; }
            self.phase2 += dt2; if self.phase2 >= 1.0 { self.phase2 -= 1.0; }
            let saw = self.phase * 2.0 - 1.0;
            let tri = if self.phase < 0.5 { self.phase * 4.0 - 1.0 } else { 3.0 - self.phase * 4.0 };
            let sine = (self.phase * 2.0 * PI).sin();
            let sq = if self.phase < 0.5 { 1.0 } else { -1.0 };
            // Morph: 0..0.33 saw→tri, 0.33..0.66 tri→sine, 0.66..1.0 sine→square
            let t = timbre * 3.0;
            let w1 = if t < 1.0 { saw * (1.0 - t) + tri * t }
                else if t < 2.0 { tri * (2.0 - t) + sine * (t - 1.0) }
                else { sine * (3.0 - t) + sq * (t - 2.0).min(1.0) };
            let w2 = {
                let saw2 = self.phase2 * 2.0 - 1.0;
                let tri2 = if self.phase2 < 0.5 { self.phase2 * 4.0 - 1.0 } else { 3.0 - self.phase2 * 4.0 };
                let sine2 = (self.phase2 * 2.0 * PI).sin();
                let sq2 = if self.phase2 < 0.5 { 1.0 } else { -1.0 };
                if t < 1.0 { saw2 * (1.0 - t) + tri2 * t }
                else if t < 2.0 { tri2 * (2.0 - t) + sine2 * (t - 1.0) }
                else { sine2 * (3.0 - t) + sq2 * (t - 2.0).min(1.0) }
            };
            *s = w1 * 0.6 + w2 * 0.4;
        }
    }
    fn reset(&mut self) { self.phase = 0.0; self.phase2 = 0.0; }
}
