use super::OscModel;
use dsp_common::{DelayLine, OnePole};

pub struct PluckModel {
    delay: DelayLine,
    lpf: OnePole,
    sr: f32,
    rng: u64,
    excited: bool,
}

impl PluckModel {
    pub fn new(sr: f32) -> Self {
        Self {
            delay: DelayLine::new((sr / 20.0) as usize), // enough for ~20Hz
            lpf: OnePole::new(),
            sr,
            rng: 12345,
            excited: false,
        }
    }

    fn noise(&mut self) -> f32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        (self.rng as i64 as f32) / (i64::MAX as f32) * 2.0
    }
}

impl OscModel for PluckModel {
    fn render(&mut self, freq: f32, timbre: f32, color: f32, out: &mut [f32]) {
        let delay_samples = self.sr / freq.max(20.0);
        // Damping: timbre 0 = bright/long, timbre 1 = dull/short
        let cutoff = 200.0 + (1.0 - timbre) * 10000.0;
        self.lpf.set_freq(cutoff, self.sr);
        // Pluck position affects excitation
        let excite_len = (delay_samples * (0.1 + color * 0.8)) as u32;

        if self.excited {
            // Fill delay with noise burst
            for i in 0..excite_len.min(delay_samples as u32) {
                let n = self.noise() * 0.8;
                self.delay.write(n);
                // Also need to advance reading
                let _ = self.delay.read(delay_samples);
            }
            self.excited = false;
        }

        for s in out.iter_mut() {
            let delayed = self.delay.read(delay_samples);
            let filtered = self.lpf.process(delayed);
            self.delay.write(filtered * 0.998); // slight loss
            *s = delayed;
        }
    }

    fn reset(&mut self) {
        self.delay.clear();
        self.lpf.clear();
        self.excited = true;
    }
}
