//! Shared DSP primitives used by both the Prophet-5 synth and TR-808 drum machine.

pub mod svfilter;
pub mod ad_envelope;
pub mod pitch_envelope;

use std::f32::consts::PI;

// =====================================================================
// DELAY LINE — Circular buffer with interpolated reads
// =====================================================================

pub struct DelayLine {
    buffer: Vec<f32>,
    mask: usize,
    write_pos: usize,
}

impl DelayLine {
    pub fn new(max_samples: usize) -> Self {
        let size = max_samples.next_power_of_two();
        Self { buffer: vec![0.0; size], mask: size - 1, write_pos: 0 }
    }

    pub fn write(&mut self, sample: f32) {
        self.buffer[self.write_pos] = sample;
        self.write_pos = (self.write_pos + 1) & self.mask;
    }

    pub fn tap(&self, offset: usize) -> f32 {
        let idx = (self.write_pos.wrapping_sub(1).wrapping_sub(offset)) & self.mask;
        self.buffer[idx]
    }

    pub fn read(&self, delay_samples: f32) -> f32 {
        let d = delay_samples.max(0.0);
        let idx = d as usize;
        let frac = d - idx as f32;
        let a = self.tap(idx);
        let b = self.tap(idx + 1);
        a + (b - a) * frac
    }

    pub fn read_cubic(&self, delay_samples: f32) -> f32 {
        let d = delay_samples.max(1.0);
        let idx = d as usize;
        let frac = d - idx as f32;
        let y0 = self.tap(idx.wrapping_sub(1));
        let y1 = self.tap(idx);
        let y2 = self.tap(idx + 1);
        let y3 = self.tap(idx + 2);
        let c0 = y1;
        let c1 = 0.5 * (y2 - y0);
        let c2 = y0 - 2.5 * y1 + 2.0 * y2 - 0.5 * y3;
        let c3 = 0.5 * (y3 - y0) + 1.5 * (y1 - y2);
        ((c3 * frac + c2) * frac + c1) * frac + c0
    }

    pub fn clear(&mut self) { self.buffer.fill(0.0); self.write_pos = 0; }
}

// =====================================================================
// ONE-POLE FILTER
// =====================================================================

pub struct OnePole {
    pub state: f32,
    coeff: f32,
}

impl OnePole {
    pub fn new() -> Self { Self { state: 0.0, coeff: 0.5 } }

    pub fn set_freq(&mut self, hz: f32, sample_rate: f32) {
        self.coeff = (-2.0 * PI * hz / sample_rate).exp();
    }

    pub fn set_coeff(&mut self, c: f32) { self.coeff = c.clamp(0.0, 0.9999); }

    pub fn process(&mut self, input: f32) -> f32 {
        self.state = input * (1.0 - self.coeff) + self.state * self.coeff;
        self.state
    }

    pub fn clear(&mut self) { self.state = 0.0; }
}

// =====================================================================
// ALLPASS FILTER — Schroeder allpass for reverb diffusion
// =====================================================================

pub struct AllPass {
    delay: DelayLine,
    delay_samples: usize,
    gain: f32,
}

impl AllPass {
    pub fn new(delay_samples: usize, gain: f32) -> Self {
        Self { delay: DelayLine::new(delay_samples + 1), delay_samples, gain }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let delayed = self.delay.tap(self.delay_samples);
        let v = input - self.gain * delayed;
        self.delay.write(v);
        delayed + self.gain * v
    }

    pub fn set_gain(&mut self, g: f32) { self.gain = g; }
    pub fn clear(&mut self) { self.delay.clear(); }
}

// =====================================================================
// NOISE GENERATOR — xorshift64
// =====================================================================

pub struct NoiseGenerator {
    state: u64,
}

impl NoiseGenerator {
    pub fn new(seed: u64) -> Self {
        Self { state: if seed == 0 { 1 } else { seed } }
    }

    pub fn white(&mut self) -> f32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        (self.state as i64 as f64 / i64::MAX as f64) as f32
    }
}

// =====================================================================
// TUNING
// =====================================================================

pub fn note_to_hz(note: u8) -> f32 {
    440.0 * 2.0f32.powf((note as f32 - 69.0) / 12.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay_line_tap() {
        let mut dl = DelayLine::new(128);
        dl.write(1.0); dl.write(2.0); dl.write(3.0);
        assert!((dl.tap(0) - 3.0).abs() < 0.001);
        assert!((dl.tap(1) - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_one_pole() {
        let mut lp = OnePole::new();
        lp.set_freq(100.0, 44100.0);
        let mut last = 0.0;
        for _ in 0..4410 { last = lp.process(1.0); }
        assert!(last > 0.9);
    }

    #[test]
    fn test_noise_range() {
        let mut gen = NoiseGenerator::new(42);
        for _ in 0..10000 {
            let s = gen.white();
            assert!(s >= -1.0 && s <= 1.0);
        }
    }

    #[test]
    fn test_note_to_hz() {
        assert!((note_to_hz(69) - 440.0).abs() < 0.01);
    }
}
