use super::{AllPass, DelayLine, OnePole};
use std::f32::consts::PI;

/// Dattorro plate reverb.
/// Based on Jon Dattorro's "Effect Design Part 1" (1997).
/// Mono input → stereo output with lush, dense reverb tail.
pub struct PlateReverb {
    // Input processing
    bandwidth: OnePole,
    input_diffusion: [AllPass; 4],

    // Tank — two cross-coupled channels
    tank_ap1_l: AllPass,
    tank_delay1_l: DelayLine,
    tank_damp_l: OnePole,
    tank_ap2_l: AllPass,
    tank_delay2_l: DelayLine,

    tank_ap1_r: AllPass,
    tank_delay1_r: DelayLine,
    tank_damp_r: OnePole,
    tank_ap2_r: AllPass,
    tank_delay2_r: DelayLine,

    // Tank LFO for modulation (prevents metallic ringing)
    tank_lfo_phase: f32,

    // Pre-delay
    predelay: DelayLine,
    predelay_samples: usize,

    // Parameters
    pub decay: f32,     // 0-0.99
    pub damping: f32,   // 0-1 (1 = no damping, 0 = heavy damping)
    pub size: f32,      // 0.5-2.0 (scales delay lengths)
    pub mix: f32,       // 0-1

    sample_rate: f32,
    lfo_inc: f32,       // cached 1.0 / sample_rate
    last_damping: f32,  // track damping changes

    // Delay length constants (scaled to 48kHz from Dattorro's 29761Hz)
    dl1_l: usize,
    dl2_l: usize,
    dl1_r: usize,
    dl2_r: usize,
}

impl PlateReverb {
    pub fn new(sample_rate: f32) -> Self {
        let scale = sample_rate / 29761.0;

        // Input diffusion allpasses (Dattorro coefficients)
        let id1 = AllPass::new((142.0 * scale) as usize, 0.75);
        let id2 = AllPass::new((107.0 * scale) as usize, 0.75);
        let id3 = AllPass::new((379.0 * scale) as usize, 0.625);
        let id4 = AllPass::new((277.0 * scale) as usize, 0.625);

        // Tank delay lengths
        let dl1_l = (672.0 * scale) as usize;
        let dl2_l = (1800.0 * scale) as usize;
        let dl1_r = (908.0 * scale) as usize;
        let dl2_r = (2656.0 * scale) as usize;

        // Tank allpasses
        let tap1_l = AllPass::new((908.0 * scale) as usize, -0.7);
        let tap2_l = AllPass::new((4217.0 * scale) as usize, 0.5);
        let tap1_r = AllPass::new((672.0 * scale) as usize, -0.7);
        let tap2_r = AllPass::new((3163.0 * scale) as usize, 0.5);

        let mut bandwidth = OnePole::new();
        bandwidth.set_coeff(0.15); // input bandwidth ~0.85

        Self {
            bandwidth,
            input_diffusion: [id1, id2, id3, id4],
            tank_ap1_l: tap1_l,
            tank_delay1_l: DelayLine::new(dl1_l + 64),
            tank_damp_l: OnePole::new(),
            tank_ap2_l: tap2_l,
            tank_delay2_l: DelayLine::new(dl2_l + 64),
            tank_ap1_r: tap1_r,
            tank_delay1_r: DelayLine::new(dl1_r + 64),
            tank_damp_r: OnePole::new(),
            tank_ap2_r: tap2_r,
            tank_delay2_r: DelayLine::new(dl2_r + 64),
            tank_lfo_phase: 0.0,
            predelay: DelayLine::new(4800), // up to 100ms
            predelay_samples: 480, // 10ms default
            decay: 0.7,
            damping: 0.7,
            size: 1.0,
            mix: 0.25,
            sample_rate,
            lfo_inc: 1.0 / sample_rate,
            last_damping: -1.0,
            dl1_l, dl2_l, dl1_r, dl2_r,
        }
    }

    /// Process mono input, returns (left, right) stereo reverb.
    pub fn process(&mut self, input: f32) -> (f32, f32) {
        // Pre-delay
        self.predelay.write(input);
        let predelayed = self.predelay.tap(self.predelay_samples);

        // Input bandwidth filter
        let bw = self.bandwidth.process(predelayed);

        // Input diffusion (4 allpasses in series)
        let mut diffused = bw;
        for ap in &mut self.input_diffusion {
            diffused = ap.process(diffused);
        }

        // Tank LFO (slow modulation prevents metallic ringing)
        self.tank_lfo_phase += self.lfo_inc;
        if self.tank_lfo_phase >= 1.0 { self.tank_lfo_phase -= 1.0; }

        // Update damping only when parameter changes
        if self.damping != self.last_damping {
            self.last_damping = self.damping;
            let damp_coeff = 1.0 - self.damping * 0.7;
            self.tank_damp_l.set_coeff(damp_coeff);
            self.tank_damp_r.set_coeff(damp_coeff);
        }

        // Tank: cross-coupled channels
        // Read from end of other channel's delay2 (cross-coupling)
        let cross_l = self.tank_delay2_r.tap(self.dl2_r.saturating_sub(1));
        let cross_r = self.tank_delay2_l.tap(self.dl2_l.saturating_sub(1));

        // Left channel
        let tank_in_l = diffused + cross_l * self.decay;
        let ap1_l = self.tank_ap1_l.process(tank_in_l);
        self.tank_delay1_l.write(ap1_l);
        let d1_l = self.tank_delay1_l.tap(self.dl1_l.saturating_sub(1));
        let damped_l = self.tank_damp_l.process(d1_l) * self.decay;
        let ap2_l = self.tank_ap2_l.process(damped_l);
        self.tank_delay2_l.write(ap2_l);

        // Right channel
        let tank_in_r = diffused + cross_r * self.decay;
        let ap1_r = self.tank_ap1_r.process(tank_in_r);
        self.tank_delay1_r.write(ap1_r);
        let d1_r = self.tank_delay1_r.tap(self.dl1_r.saturating_sub(1));
        let damped_r = self.tank_damp_r.process(d1_r) * self.decay;
        let ap2_r = self.tank_ap2_r.process(damped_r);
        self.tank_delay2_r.write(ap2_r);

        // Output taps — extract from multiple points for decorrelation
        let out_l = self.tank_delay1_l.tap(self.dl1_l / 3)
            + self.tank_delay1_l.tap(self.dl1_l * 2 / 3)
            - self.tank_delay2_r.tap(self.dl2_r / 2)
            + self.tank_delay2_l.tap(self.dl2_l / 4);

        let out_r = self.tank_delay1_r.tap(self.dl1_r / 3)
            + self.tank_delay1_r.tap(self.dl1_r * 2 / 3)
            - self.tank_delay2_l.tap(self.dl2_l / 2)
            + self.tank_delay2_r.tap(self.dl2_r / 4);

        // Scale output
        let wet_l = out_l * 0.15;
        let wet_r = out_r * 0.15;

        // Mix
        let left = input * (1.0 - self.mix) + wet_l * self.mix;
        let right = input * (1.0 - self.mix) + wet_r * self.mix;

        (left, right)
    }

    /// Process stereo input — sums to mono for reverb, preserves stereo dry signal.
    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        let mono_in = (left + right) * 0.5;
        // Get wet-only reverb (temporarily set mix to 1.0)
        let saved_mix = self.mix;
        self.mix = 1.0;
        let (wet_l, wet_r) = self.process(mono_in);
        self.mix = saved_mix;
        // Blend: keep stereo dry image, add reverb wet
        let out_l = left * (1.0 - self.mix) + wet_l * self.mix;
        let out_r = right * (1.0 - self.mix) + wet_r * self.mix;
        (out_l, out_r)
    }

    pub fn clear(&mut self) {
        self.predelay.clear();
        self.bandwidth.clear();
        for ap in &mut self.input_diffusion { ap.clear(); }
        self.tank_ap1_l.clear(); self.tank_delay1_l.clear();
        self.tank_damp_l.clear(); self.tank_ap2_l.clear(); self.tank_delay2_l.clear();
        self.tank_ap1_r.clear(); self.tank_delay1_r.clear();
        self.tank_damp_r.clear(); self.tank_ap2_r.clear(); self.tank_delay2_r.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverb_tail() {
        let mut reverb = PlateReverb::new(44100.0);
        reverb.decay = 0.8;
        reverb.mix = 1.0;

        // Send impulse
        reverb.process(1.0);

        // After 0.5s of silence, there should still be energy (reverb tail)
        let mut tail_energy = 0.0f32;
        for _ in 0..22050 {
            let (l, r) = reverb.process(0.0);
            tail_energy += l.abs() + r.abs();
        }
        assert!(tail_energy > 0.1, "Reverb should have a tail, got energy {tail_energy}");
    }

    #[test]
    fn test_reverb_stereo() {
        let mut reverb = PlateReverb::new(44100.0);
        reverb.decay = 0.7;
        reverb.mix = 1.0;

        // Send impulse and collect L/R
        reverb.process(1.0);
        let mut left = Vec::new();
        let mut right = Vec::new();
        for _ in 0..4410 {
            let (l, r) = reverb.process(0.0);
            left.push(l);
            right.push(r);
        }

        let corr = audio_test_harness::correlation::cross_correlation(&left, &right);
        assert!(corr < 0.95, "Reverb should produce stereo decorrelation (corr={corr:.3})");
    }

    #[test]
    fn test_reverb_decay_affects_tail() {
        // Short decay
        let mut short = PlateReverb::new(44100.0);
        short.decay = 0.3;
        short.mix = 1.0;
        short.process(1.0);
        let mut short_energy = 0.0f32;
        for _ in 0..44100 {
            let (l, r) = short.process(0.0);
            short_energy += l.abs() + r.abs();
        }

        // Long decay
        let mut long = PlateReverb::new(44100.0);
        long.decay = 0.9;
        long.mix = 1.0;
        long.process(1.0);
        let mut long_energy = 0.0f32;
        for _ in 0..44100 {
            let (l, r) = long.process(0.0);
            long_energy += l.abs() + r.abs();
        }

        assert!(
            long_energy > short_energy * 1.5,
            "Longer decay should have more energy: short={short_energy:.2}, long={long_energy:.2}"
        );
    }

    #[test]
    fn test_reverb_mix_zero_is_dry() {
        let mut reverb = PlateReverb::new(44100.0);
        reverb.mix = 0.0;

        for i in 0..1000 {
            let input = (i as f32 * 0.1).sin();
            let (l, r) = reverb.process(input);
            assert!((l - input).abs() < 0.01, "Mix=0 left should be dry");
            assert!((r - input).abs() < 0.01, "Mix=0 right should be dry");
        }
    }
}
