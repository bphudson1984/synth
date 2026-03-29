use super::OscModel;

const NUM_OSCS: usize = 7;
const DETUNE_OFFSETS: [f32; NUM_OSCS] = [-3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0];
const AMP_WEIGHTS: [f32; NUM_OSCS] = [0.5, 0.7, 0.9, 1.0, 0.9, 0.7, 0.5];

pub struct SwarmModel {
    phases: [f32; NUM_OSCS],
    hp_state: f32,
    sr: f32,
}

impl SwarmModel {
    pub fn new(sr: f32) -> Self { Self { phases: [0.0; NUM_OSCS], hp_state: 0.0, sr } }
}

impl OscModel for SwarmModel {
    fn render(&mut self, freq: f32, timbre: f32, color: f32, out: &mut [f32]) {
        let max_detune = timbre * 0.06; // up to ~1 semitone spread
        let hp_coeff = 1.0 - (color * 0.15); // higher color = more HP
        for s in out.iter_mut() {
            let mut sum = 0.0;
            for i in 0..NUM_OSCS {
                let detune = 1.0 + DETUNE_OFFSETS[i] * max_detune / 3.0;
                let dt = freq * detune / self.sr;
                self.phases[i] += dt;
                if self.phases[i] >= 1.0 { self.phases[i] -= 1.0; }
                let saw = self.phases[i] * 2.0 - 1.0 - poly_blep(self.phases[i], dt);
                sum += saw * AMP_WEIGHTS[i];
            }
            sum /= 4.0; // normalize
            // Simple 1-pole HP
            let hp_out = sum - self.hp_state;
            self.hp_state = self.hp_state * hp_coeff + sum * (1.0 - hp_coeff);
            *s = if color > 0.01 { sum * (1.0 - color) + hp_out * color } else { sum };
        }
    }
    fn reset(&mut self) { self.phases = [0.0; NUM_OSCS]; self.hp_state = 0.0; }
}

fn poly_blep(t: f32, dt: f32) -> f32 {
    if t < dt { let t = t / dt; 2.0 * t - t * t - 1.0 }
    else if t > 1.0 - dt { let t = (t - 1.0) / dt; t * t + 2.0 * t + 1.0 }
    else { 0.0 }
}
