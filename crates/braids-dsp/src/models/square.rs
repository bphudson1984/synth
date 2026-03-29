use super::OscModel;

pub struct SquareModel { phase: f32, sub_phase: f32, sr: f32 }

impl SquareModel {
    pub fn new(sr: f32) -> Self { Self { phase: 0.0, sub_phase: 0.0, sr } }
}

impl OscModel for SquareModel {
    fn render(&mut self, freq: f32, timbre: f32, color: f32, out: &mut [f32]) {
        let dt = freq / self.sr;
        let pw = 0.05 + timbre * 0.9;
        let sub_dt = dt * 0.5;
        for s in out.iter_mut() {
            self.phase += dt; if self.phase >= 1.0 { self.phase -= 1.0; }
            self.sub_phase += sub_dt; if self.sub_phase >= 1.0 { self.sub_phase -= 1.0; }
            let mut pulse = if self.phase < pw { 1.0 } else { -1.0 };
            pulse += poly_blep(self.phase, dt);
            pulse -= poly_blep((self.phase - pw + 1.0) % 1.0, dt);
            let mut sub = if self.sub_phase < 0.5 { 1.0 } else { -1.0 };
            sub += poly_blep(self.sub_phase, sub_dt);
            sub -= poly_blep((self.sub_phase + 0.5) % 1.0, sub_dt);
            *s = pulse * (1.0 - color * 0.5) + sub * color * 0.5;
        }
    }
    fn reset(&mut self) { self.phase = 0.0; self.sub_phase = 0.0; }
}

fn poly_blep(t: f32, dt: f32) -> f32 {
    if t < dt { let t = t / dt; 2.0 * t - t * t - 1.0 }
    else if t > 1.0 - dt { let t = (t - 1.0) / dt; t * t + 2.0 * t + 1.0 }
    else { 0.0 }
}
