use super::OscModel;
use std::f32::consts::TAU;

pub struct FbfmModel { carrier_phase: f32, mod_phase: f32, prev: f32, sr: f32 }

impl FbfmModel {
    pub fn new(sr: f32) -> Self { Self { carrier_phase: 0.0, mod_phase: 0.0, prev: 0.0, sr } }
}

impl OscModel for FbfmModel {
    fn render(&mut self, freq: f32, timbre: f32, color: f32, out: &mut [f32]) {
        let mod_index = timbre * 8.0;
        let feedback = color * 3.0;
        let c_dt = freq / self.sr;
        let m_dt = freq * 2.0 / self.sr;
        for s in out.iter_mut() {
            self.mod_phase += m_dt; if self.mod_phase >= 1.0 { self.mod_phase -= 1.0; }
            self.carrier_phase += c_dt; if self.carrier_phase >= 1.0 { self.carrier_phase -= 1.0; }
            let modulator = (self.mod_phase * TAU).sin();
            let out_val = (self.carrier_phase * TAU + mod_index * modulator + feedback * self.prev).sin();
            self.prev = out_val;
            *s = out_val;
        }
    }
    fn reset(&mut self) { self.carrier_phase = 0.0; self.mod_phase = 0.0; self.prev = 0.0; }
}
