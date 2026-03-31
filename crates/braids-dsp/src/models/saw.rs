use super::OscModel;

pub struct SawModel { phase: f32, phase2: f32, sr: f32 }

impl SawModel {
    pub fn new(sr: f32) -> Self { Self { phase: 0.0, phase2: 0.0, sr } }
}

impl OscModel for SawModel {
    fn render(&mut self, freq: f32, timbre: f32, color: f32, out: &mut [f32]) {
        let dt = freq / self.sr;
        let detune = 1.0 + timbre * 0.014; // up to ~24 cents
        let dt2 = dt * detune;
        let brightness = 0.2 + color * 0.8; // HP tilt amount
        for s in out.iter_mut() {
            self.phase += dt; if self.phase >= 1.0 { self.phase -= 1.0; }
            self.phase2 += dt2; if self.phase2 >= 1.0 { self.phase2 -= 1.0; }
            let s1 = self.phase * 2.0 - 1.0 - poly_blep(self.phase, dt);
            let s2 = self.phase2 * 2.0 - 1.0 - poly_blep(self.phase2, dt2);
            let mixed = s1 * (1.0 - timbre * 0.5) + s2 * timbre * 0.5;
            // Simple brightness tilt: blend with HPF version
            *s = mixed * brightness + mixed * (1.0 - brightness);
        }
    }
    fn reset(&mut self) { self.phase = 0.0; self.phase2 = 0.0; }
}

fn poly_blep(t: f32, dt: f32) -> f32 {
    if t < dt { let t = t / dt; 2.0 * t - t * t - 1.0 }
    else if t > 1.0 - dt { let t = (t - 1.0) / dt; t * t + 2.0 * t + 1.0 }
    else { 0.0 }
}
