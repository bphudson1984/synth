use std::f32::consts::PI;

pub struct LadderFilter {
    s: [f32; 4],
    cutoff_hz: f32,
    resonance: f32,
    drive: f32,
    sample_rate: f32,
}

impl LadderFilter {
    pub fn new(sample_rate: f32) -> Self {
        Self { s: [0.0; 4], cutoff_hz: 20000.0, resonance: 0.0, drive: 1.0, sample_rate }
    }
    pub fn set_cutoff(&mut self, hz: f32) { self.cutoff_hz = hz.clamp(20.0, self.sample_rate * 0.45); }
    pub fn set_resonance(&mut self, r: f32) { self.resonance = r.clamp(0.0, 4.0); }

    pub fn process(&mut self, input: f32) -> f32 {
        let g = (PI * self.cutoff_hz / self.sample_rate).tan();
        let a = g / (1.0 + g);
        let b = 1.0 - a;
        let (a2, a3, a4) = (a * a, a * a * a, a * a * a * a);
        let s_est = b * (a3 * self.s[0] + a2 * self.s[1] + a * self.s[2] + self.s[3]);
        let u = (input - self.resonance * s_est) / (1.0 + self.resonance * a4);
        let mut x = u;
        for i in 0..4 {
            let scale = 0.2 * self.drive;
            x = (x * scale).tanh() / scale;
            let v = (x - self.s[i]) * a;
            let y = v + self.s[i];
            self.s[i] = y + v;
            x = y;
        }
        x
    }
}
