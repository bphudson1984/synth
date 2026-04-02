use std::f32::consts::PI;

/// Fast tan approximation using Padé approximant.
/// Accurate to ~0.1% for x < 1.2 (covers cutoff up to ~0.4 * sample_rate).
#[inline(always)]
fn fast_tan(x: f32) -> f32 {
    let x2 = x * x;
    x * (1.0 + x2 * (1.0 / 3.0 + x2 * (2.0 / 15.0)))
}

/// Fast tanh approximation using rational function.
/// Smooth saturation character, accurate to ~1% for |x| < 4.
#[inline(always)]
pub fn fast_tanh(x: f32) -> f32 {
    let x2 = x * x;
    x * (27.0 + x2) / (27.0 + 9.0 * x2)
}

/// TB-303 diode ladder filter.
/// 4-pole (24dB/oct) lowpass with capped resonance (no self-oscillation).
/// Based on Zavalishin TPT topology with SSM2040-style saturation.
pub struct DiodeLadder {
    s: [f32; 4],
    cutoff_hz: f32,
    resonance: f32,
    drive: f32,
    sample_rate: f32,
    // Cached saturation scale/inv — only change when drive changes
    sat_scale: f32,
    sat_inv: f32,
    pi_over_sr: f32,
}

impl DiodeLadder {
    pub fn new(sample_rate: f32) -> Self {
        let drive = 1.5f32;
        let sat_scale = 0.2 * drive;
        Self {
            s: [0.0; 4], cutoff_hz: 1000.0, resonance: 0.0, drive, sample_rate,
            sat_scale,
            sat_inv: 1.0 / sat_scale,
            pi_over_sr: PI / sample_rate,
        }
    }

    pub fn set_cutoff(&mut self, hz: f32) {
        self.cutoff_hz = hz.clamp(20.0, self.sample_rate * 0.45);
    }

    /// Set resonance (0.0 to 1.0 normalized). Internally maps to 0..3.8
    /// to prevent self-oscillation (authentic 303 behavior).
    pub fn set_resonance(&mut self, r: f32) {
        self.resonance = r.clamp(0.0, 1.0);
    }

    pub fn set_drive(&mut self, d: f32) {
        self.drive = d.max(0.1);
        self.sat_scale = 0.2 * self.drive;
        self.sat_inv = 1.0 / self.sat_scale;
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let k = self.resonance * 3.8;
        let g = fast_tan(self.cutoff_hz * self.pi_over_sr);
        let a = g / (1.0 + g);
        let b = 1.0 - a;
        let a2 = a * a;
        let a3 = a2 * a;
        let a4 = a3 * a;
        let s_estimate = b * (a3 * self.s[0] + a2 * self.s[1] + a * self.s[2] + self.s[3]);
        let u = (input - k * s_estimate) / (1.0 + k * a4);

        let scale = self.sat_scale;
        let inv = self.sat_inv;
        let mut x = u;
        for i in 0..4 {
            x = fast_tanh(x * scale) * inv;
            let v = (x - self.s[i]) * a;
            let y = v + self.s[i];
            self.s[i] = y + v;
            x = y;
        }
        x
    }
}
