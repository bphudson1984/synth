use super::OscModel;
use std::f32::consts::TAU;

pub struct FmModel { carrier_phase: f32, mod_phase: f32, sr: f32 }

impl FmModel {
    pub fn new(sr: f32) -> Self { Self { carrier_phase: 0.0, mod_phase: 0.0, sr } }
}

const RATIOS: [f32; 8] = [0.5, 1.0, 1.5, 2.0, 3.0, 4.0, 5.0, 7.0];

impl OscModel for FmModel {
    fn render(&mut self, freq: f32, timbre: f32, color: f32, out: &mut [f32]) {
        let mod_index = timbre * 8.0;
        // Interpolate between discrete ratios
        let ri = color * 7.0;
        let ri_floor = (ri as usize).min(6);
        let ri_frac = ri - ri_floor as f32;
        let ratio = RATIOS[ri_floor] * (1.0 - ri_frac) + RATIOS[ri_floor + 1] * ri_frac;
        let c_dt = freq / self.sr;
        let m_dt = freq * ratio / self.sr;
        for s in out.iter_mut() {
            self.mod_phase += m_dt; if self.mod_phase >= 1.0 { self.mod_phase -= 1.0; }
            self.carrier_phase += c_dt; if self.carrier_phase >= 1.0 { self.carrier_phase -= 1.0; }
            *s = (self.carrier_phase * TAU + mod_index * (self.mod_phase * TAU).sin()).sin();
        }
    }
    fn reset(&mut self) { self.carrier_phase = 0.0; self.mod_phase = 0.0; }
}
