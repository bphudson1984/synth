pub struct Glide {
    current_hz: f32,
    target_hz: f32,
    rate: f32,
    enabled: bool,
}

impl Glide {
    pub fn new() -> Self {
        Self { current_hz: 440.0, target_hz: 440.0, rate: 0.0, enabled: false }
    }
    pub fn set_rate(&mut self, rate: f32) { self.rate = rate.max(0.0); }
    pub fn set_enabled(&mut self, e: bool) { self.enabled = e; }
    pub fn set_target(&mut self, hz: f32) {
        self.target_hz = hz.max(1.0);
        if !self.enabled || self.rate <= 0.0 { self.current_hz = self.target_hz; }
    }
    pub fn process(&mut self, sample_rate: f32) -> f32 {
        if !self.enabled || self.rate <= 0.0 { self.current_hz = self.target_hz; return self.current_hz; }
        let cl = self.current_hz.ln();
        let tl = self.target_hz.ln();
        let c = (-1.0 / (self.rate * sample_rate)).exp();
        self.current_hz = (tl + (cl - tl) * c).exp();
        self.current_hz
    }
}
