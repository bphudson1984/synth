use super::OscModel;
use std::f32::consts::PI;

// 5 vowels, each with 3 formant frequencies (F1, F2, F3)
const FORMANTS: [[f32; 3]; 5] = [
    [800.0, 1150.0, 2900.0],  // A
    [400.0, 1600.0, 2700.0],  // E
    [350.0, 2300.0, 3200.0],  // I
    [450.0,  800.0, 2830.0],  // O
    [325.0,  700.0, 2530.0],  // U
];

struct BpfState { s1: f32, s2: f32 }

pub struct VowelModel {
    phase: f32,
    bpf: [BpfState; 3],
    sr: f32,
}

impl VowelModel {
    pub fn new(sr: f32) -> Self {
        Self { phase: 0.0, bpf: [BpfState{s1:0.0,s2:0.0}, BpfState{s1:0.0,s2:0.0}, BpfState{s1:0.0,s2:0.0}], sr }
    }

    fn process_bpf(state: &mut BpfState, input: f32, freq: f32, q: f32, sr: f32) -> f32 {
        let w0 = 2.0 * PI * freq / sr;
        let alpha = w0.sin() / (2.0 * q);
        let a0 = 1.0 + alpha;
        let b1 = (w0.sin()) / a0;
        let a1 = -2.0 * w0.cos() / a0;
        let a2 = (1.0 - alpha) / a0;
        let out = b1 * input - a1 * state.s1 - a2 * state.s2;
        state.s2 = state.s1;
        state.s1 = out;
        out
    }
}

impl OscModel for VowelModel {
    fn render(&mut self, freq: f32, timbre: f32, color: f32, out: &mut [f32]) {
        let dt = freq / self.sr;
        let vowel_pos = timbre * 4.0;
        let vi = (vowel_pos as usize).min(3);
        let vf = vowel_pos - vi as f32;
        let shift = 2.0f32.powf((color - 0.5) * 2.0);
        let mut formant_freqs = [0.0f32; 3];
        for f in 0..3 {
            formant_freqs[f] = (FORMANTS[vi][f] * (1.0 - vf) + FORMANTS[vi + 1][f] * vf) * shift;
            formant_freqs[f] = formant_freqs[f].clamp(100.0, self.sr * 0.45);
        }
        for s in out.iter_mut() {
            self.phase += dt; if self.phase >= 1.0 { self.phase -= 1.0; }
            let impulse = if self.phase < dt { 1.0 / dt.max(0.001) } else { 0.0 };
            let mut sum = 0.0;
            for f in 0..3 {
                sum += Self::process_bpf(&mut self.bpf[f], impulse, formant_freqs[f], 10.0, self.sr) * 0.33;
            }
            *s = sum.clamp(-1.0, 1.0);
        }
    }
    fn reset(&mut self) {
        self.phase = 0.0;
        for b in &mut self.bpf { b.s1 = 0.0; b.s2 = 0.0; }
    }
}
