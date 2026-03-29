use super::OscModel;
use std::f32::consts::PI;

const TABLE_SIZE: usize = 256;
const NUM_TABLES: usize = 4;

pub struct WavetableModel {
    phase: f32,
    sr: f32,
    tables: [[f32; TABLE_SIZE]; NUM_TABLES],
}

impl WavetableModel {
    pub fn new(sr: f32) -> Self {
        let mut tables = [[0.0f32; TABLE_SIZE]; NUM_TABLES];
        // Table 0: organ (harmonics 1-8)
        for i in 0..TABLE_SIZE {
            let p = i as f32 / TABLE_SIZE as f32;
            for h in 1..=8 { tables[0][i] += (p * h as f32 * 2.0 * PI).sin() / h as f32; }
        }
        // Table 1: odd harmonics (hollow)
        for i in 0..TABLE_SIZE {
            let p = i as f32 / TABLE_SIZE as f32;
            for h in (1..=15).step_by(2) { tables[1][i] += (p * h as f32 * 2.0 * PI).sin() / h as f32; }
        }
        // Table 2: bright saw partials
        for i in 0..TABLE_SIZE {
            let p = i as f32 / TABLE_SIZE as f32;
            for h in 1..=16 { tables[2][i] += (p * h as f32 * 2.0 * PI).sin() * (-0.1 * h as f32).exp(); }
        }
        // Table 3: metallic (inharmonic)
        for i in 0..TABLE_SIZE {
            let p = i as f32 / TABLE_SIZE as f32;
            let ratios = [1.0, 2.76, 4.07, 5.39, 6.28, 7.61];
            for (j, r) in ratios.iter().enumerate() {
                tables[3][i] += (p * r * 2.0 * PI).sin() / (j as f32 + 1.0);
            }
        }
        // Normalize all tables
        for t in &mut tables {
            let mx = t.iter().map(|x| x.abs()).fold(0.0f32, f32::max).max(0.001);
            for s in t.iter_mut() { *s /= mx; }
        }
        Self { phase: 0.0, sr, tables }
    }
}

impl OscModel for WavetableModel {
    fn render(&mut self, freq: f32, timbre: f32, color: f32, out: &mut [f32]) {
        let dt = freq / self.sr;
        let pos = timbre * (TABLE_SIZE - 1) as f32;
        let table_f = color * (NUM_TABLES - 1) as f32;
        let t_idx = (table_f as usize).min(NUM_TABLES - 2);
        let t_frac = table_f - t_idx as f32;
        for s in out.iter_mut() {
            self.phase += dt; if self.phase >= 1.0 { self.phase -= 1.0; }
            let idx_f = self.phase * TABLE_SIZE as f32;
            let idx = idx_f as usize % TABLE_SIZE;
            let frac = idx_f - idx as f32;
            let next = (idx + 1) % TABLE_SIZE;
            let v1 = self.tables[t_idx][idx] * (1.0 - frac) + self.tables[t_idx][next] * frac;
            let v2 = self.tables[t_idx + 1][idx] * (1.0 - frac) + self.tables[t_idx + 1][next] * frac;
            *s = v1 * (1.0 - t_frac) + v2 * t_frac;
        }
    }
    fn reset(&mut self) { self.phase = 0.0; }
}
