/// TB-303 oscillator: single VCO, sawtooth OR square (50% duty).
/// PolyBLEP antialiased.
pub struct Oscillator {
    phase: f32,
    freq_hz: f32,
    sample_rate: f32,
    use_saw: bool,
}

impl Oscillator {
    pub fn new(sample_rate: f32) -> Self {
        Self { phase: 0.0, freq_hz: 440.0, sample_rate, use_saw: true }
    }

    pub fn set_frequency(&mut self, hz: f32) { self.freq_hz = hz.max(1.0); }
    pub fn set_waveform(&mut self, saw: bool) { self.use_saw = saw; }

    pub fn process(&mut self) -> f32 {
        let dt = self.freq_hz / self.sample_rate;
        self.phase += dt;
        if self.phase >= 1.0 { self.phase -= 1.0; }

        if self.use_saw {
            let mut saw = self.phase * 2.0 - 1.0;
            saw -= poly_blep(self.phase, dt);
            saw
        } else {
            let mut sq = if self.phase < 0.5 { 1.0 } else { -1.0 };
            sq += poly_blep(self.phase, dt);
            sq -= poly_blep((self.phase + 0.5) % 1.0, dt);
            sq
        }
    }
}

fn poly_blep(t: f32, dt: f32) -> f32 {
    if t < dt {
        let t = t / dt;
        2.0 * t - t * t - 1.0
    } else if t > 1.0 - dt {
        let t = (t - 1.0) / dt;
        t * t + 2.0 * t + 1.0
    } else {
        0.0
    }
}
