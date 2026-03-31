use super::OscModel;
use std::f32::consts::TAU;

const NUM_GRAINS: usize = 8;

struct Grain { phase: f32, freq: f32, life: u32, max_life: u32, active: bool }

pub struct CloudModel {
    grains: [Grain; NUM_GRAINS],
    spawn_counter: f32,
    rng: u64,
    sr: f32,
}

impl CloudModel {
    pub fn new(sr: f32) -> Self {
        const EMPTY: Grain = Grain { phase: 0.0, freq: 440.0, life: 0, max_life: 1, active: false };
        Self { grains: [EMPTY; NUM_GRAINS], spawn_counter: 0.0, rng: 98765, sr }
    }

    fn rand(&mut self) -> f32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        (self.rng as u32 as f32) / (u32::MAX as f32)
    }
}

impl OscModel for CloudModel {
    fn render(&mut self, freq: f32, timbre: f32, color: f32, out: &mut [f32]) {
        let density = 1.0 + timbre * 49.0; // 1-50 grains/sec
        let spawn_interval = self.sr / density;
        let scatter = color * 24.0; // up to 2 octaves of scatter (in semitones)
        let grain_dur = (self.sr * 0.05) as u32; // 50ms grains

        for s in out.iter_mut() {
            // Spawn new grains
            self.spawn_counter += 1.0;
            if self.spawn_counter >= spawn_interval {
                self.spawn_counter -= spawn_interval;
                let offset = self.rand() - 0.5;
                if let Some(g) = self.grains.iter_mut().find(|g| !g.active) {
                    g.freq = freq * 2.0f32.powf(offset * scatter / 12.0);
                    g.phase = 0.0;
                    g.life = 0;
                    g.max_life = grain_dur;
                    g.active = true;
                }
            }

            // Render active grains
            let mut sum = 0.0;
            for g in &mut self.grains {
                if !g.active { continue; }
                g.phase += g.freq / self.sr;
                if g.phase >= 1.0 { g.phase -= 1.0; }
                g.life += 1;
                if g.life >= g.max_life { g.active = false; continue; }
                // Triangular envelope
                let env_pos = g.life as f32 / g.max_life as f32;
                let env = if env_pos < 0.5 { env_pos * 2.0 } else { 2.0 - env_pos * 2.0 };
                sum += (g.phase * TAU).sin() * env;
            }
            *s = sum * 0.3;
        }
    }

    fn reset(&mut self) {
        for g in &mut self.grains { g.active = false; }
        self.spawn_counter = 0.0;
    }
}
