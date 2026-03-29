/// TB-303 fixed-time exponential portamento (~60ms constant-time).
pub struct Glide {
    current_hz: f32,
    target_hz: f32,
    coeff: f32,
    enabled: bool,
}

impl Glide {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            current_hz: 220.0, target_hz: 220.0,
            coeff: (-3.0 / (0.06 * sample_rate)).exp(), // 60ms default
            enabled: false,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) { self.enabled = enabled; }

    pub fn set_target(&mut self, hz: f32) {
        self.target_hz = hz.max(1.0);
        if !self.enabled { self.current_hz = self.target_hz; }
    }

    pub fn jump_to(&mut self, hz: f32) {
        self.current_hz = hz.max(1.0);
        self.target_hz = self.current_hz;
    }

    pub fn process(&mut self) -> f32 {
        if !self.enabled {
            self.current_hz = self.target_hz;
            return self.current_hz;
        }
        let current_log = self.current_hz.ln();
        let target_log = self.target_hz.ln();
        self.current_hz = (target_log + (current_log - target_log) * self.coeff).exp();
        self.current_hz
    }
}
