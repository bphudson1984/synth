use std::f32::consts::PI;

/// TB-303 diode ladder filter.
/// 4-pole (24dB/oct) lowpass with capped resonance (no self-oscillation).
/// Based on Zavalishin TPT topology with SSM2040-style saturation.
pub struct DiodeLadder {
    s: [f32; 4],
    cutoff_hz: f32,
    resonance: f32,
    drive: f32,
    sample_rate: f32,
}

impl DiodeLadder {
    pub fn new(sample_rate: f32) -> Self {
        Self { s: [0.0; 4], cutoff_hz: 1000.0, resonance: 0.0, drive: 1.5, sample_rate }
    }

    pub fn set_cutoff(&mut self, hz: f32) {
        self.cutoff_hz = hz.clamp(20.0, self.sample_rate * 0.45);
    }

    /// Set resonance (0.0 to 1.0 normalized). Internally maps to 0..3.8
    /// to prevent self-oscillation (authentic 303 behavior).
    pub fn set_resonance(&mut self, r: f32) {
        self.resonance = r.clamp(0.0, 1.0);
    }

    pub fn set_drive(&mut self, d: f32) { self.drive = d.max(0.1); }

    pub fn process(&mut self, input: f32) -> f32 {
        let k = self.resonance * 3.8;
        let g = (PI * self.cutoff_hz / self.sample_rate).tan();
        let a = g / (1.0 + g);
        let b = 1.0 - a;
        let a2 = a * a;
        let a3 = a2 * a;
        let a4 = a3 * a;
        let s_estimate = b * (a3 * self.s[0] + a2 * self.s[1] + a * self.s[2] + self.s[3]);
        let u = (input - k * s_estimate) / (1.0 + k * a4);

        let mut x = u;
        for i in 0..4 {
            x = saturate(x, self.drive);
            let v = (x - self.s[i]) * a;
            let y = v + self.s[i];
            self.s[i] = y + v;
            x = y;
        }
        x
    }
}

fn saturate(x: f32, drive: f32) -> f32 {
    let scale = 0.2 * drive;
    let inv = 1.0 / scale;
    (x * scale).tanh() * inv
}
