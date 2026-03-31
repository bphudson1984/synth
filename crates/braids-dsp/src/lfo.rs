pub struct Lfo {
    phase: f32,
    freq_hz: f32,
    sample_rate: f32,
}

impl Lfo {
    pub fn new(sample_rate: f32) -> Self {
        Self { phase: 0.0, freq_hz: 5.0, sample_rate }
    }
    pub fn set_frequency(&mut self, hz: f32) { self.freq_hz = hz.clamp(0.04, 20.0); }
    pub fn process(&mut self) -> f32 {
        self.phase += self.freq_hz / self.sample_rate;
        if self.phase >= 1.0 { self.phase -= 1.0; }
        if self.phase < 0.5 { self.phase * 4.0 - 1.0 } else { 3.0 - self.phase * 4.0 }
    }
}
