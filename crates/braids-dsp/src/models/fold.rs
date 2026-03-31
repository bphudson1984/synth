use super::OscModel;
use std::f32::consts::PI;

pub struct FoldModel { phase: f32, sr: f32 }

impl FoldModel {
    pub fn new(sr: f32) -> Self { Self { phase: 0.0, sr } }
}

impl OscModel for FoldModel {
    fn render(&mut self, freq: f32, timbre: f32, color: f32, out: &mut [f32]) {
        let dt = freq / self.sr;
        let gain = 1.0 + timbre * 7.0; // 1x to 8x fold
        for s in out.iter_mut() {
            self.phase += dt; if self.phase >= 1.0 { self.phase -= 1.0; }
            // Input shape: sine → triangle → saw
            let input = if color < 0.5 {
                let t = color * 2.0;
                let sine = (self.phase * 2.0 * PI).sin();
                let tri = if self.phase < 0.5 { self.phase * 4.0 - 1.0 } else { 3.0 - self.phase * 4.0 };
                sine * (1.0 - t) + tri * t
            } else {
                let t = (color - 0.5) * 2.0;
                let tri = if self.phase < 0.5 { self.phase * 4.0 - 1.0 } else { 3.0 - self.phase * 4.0 };
                let saw = self.phase * 2.0 - 1.0;
                tri * (1.0 - t) + saw * t
            };
            // Triangle fold waveshaper
            let folded = input * gain;
            *s = (folded * PI * 0.5).sin(); // smooth fold
        }
    }
    fn reset(&mut self) { self.phase = 0.0; }
}
