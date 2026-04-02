use dsp_common::{DelayLine, OnePole};

/// BBD-style stereo chorus (Boss CE-2 / Juno-106 inspired).
/// Mono input -> stereo output via phase-offset triangle LFO modulating a delay line.
pub struct StereoChorus {
    delay: DelayLine,
    lfo_phase: f32,
    lp_left: OnePole,
    lp_right: OnePole,
    sample_rate: f32,

    pub rate: f32,   // LFO rate in Hz (0.1-5.0)
    pub depth: f32,  // 0-1, maps to 0-8ms modulation depth
    pub mix: f32,    // 0-1 wet/dry
}

impl StereoChorus {
    pub fn new(sample_rate: f32) -> Self {
        let mut lp_left = OnePole::new();
        let mut lp_right = OnePole::new();
        // BBD anti-aliasing filter ~6kHz
        lp_left.set_freq(6000.0, sample_rate);
        lp_right.set_freq(6000.0, sample_rate);

        Self {
            delay: DelayLine::new(4096),
            lfo_phase: 0.0,
            lp_left,
            lp_right,
            sample_rate,
            rate: 0.8,
            depth: 0.5,
            mix: 0.5,
        }
    }

    /// Process one mono sample, returns (left, right) stereo pair.
    pub fn process(&mut self, input: f32) -> (f32, f32) {
        self.delay.write(input);

        let lfo_inc = self.rate / self.sample_rate;
        self.lfo_phase += lfo_inc;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        let tri_left = triangle(self.lfo_phase);
        let tri_right = triangle((self.lfo_phase + 0.25) % 1.0);

        let center_samples = 0.007 * self.sample_rate;
        let depth_samples = self.depth * 0.008 * self.sample_rate;

        let delay_left = center_samples + tri_left * depth_samples;
        let delay_right = center_samples + tri_right * depth_samples;

        let wet_left = self.delay.read(delay_left);
        let wet_right = self.delay.read(delay_right);

        let wet_left = self.lp_left.process(wet_left);
        let wet_right = self.lp_right.process(wet_right);

        let dry = input;
        let left = dry * (1.0 - self.mix) + wet_left * self.mix;
        let right = dry * (1.0 - self.mix) + wet_right * self.mix;

        (left, right)
    }

    /// Process stereo input by summing to mono, then producing stereo chorus output.
    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        let mono = (left + right) * 0.5;
        self.process(mono)
    }

    pub fn clear(&mut self) {
        self.delay.clear();
        self.lp_left.clear();
        self.lp_right.clear();
    }
}

/// Triangle wave from phase (0..1) -> output (-1..+1)
fn triangle(phase: f32) -> f32 {
    if phase < 0.5 {
        phase * 4.0 - 1.0
    } else {
        3.0 - phase * 4.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    fn sine_buf(freq: f32, sample_rate: f32, seconds: f32) -> Vec<f32> {
        let n = (sample_rate * seconds) as usize;
        (0..n).map(|i| (2.0 * PI * freq * i as f32 / sample_rate).sin()).collect()
    }

    #[test]
    fn test_chorus_creates_stereo_difference() {
        let mut chorus = StereoChorus::new(44100.0);
        chorus.rate = 1.0;
        chorus.depth = 0.5;
        chorus.mix = 1.0;

        let input = sine_buf(440.0, 44100.0, 0.5);
        let mut left = Vec::new();
        let mut right = Vec::new();
        for &s in &input {
            let (l, r) = chorus.process(s);
            left.push(l);
            right.push(r);
        }

        let corr = audio_test_harness::correlation::cross_correlation(&left, &right);
        assert!(corr < 0.99, "Stereo chorus should produce L/R difference (corr={corr:.3})");
    }

    #[test]
    fn test_chorus_mix_zero_is_dry() {
        let mut chorus = StereoChorus::new(44100.0);
        chorus.mix = 0.0;

        let input = sine_buf(440.0, 44100.0, 0.2);
        for &s in &input {
            let (l, r) = chorus.process(s);
            assert!((l - s).abs() < 0.001, "Mix=0 left should equal input");
            assert!((r - s).abs() < 0.001, "Mix=0 right should equal input");
        }
    }

    #[test]
    fn test_chorus_not_silent() {
        let mut chorus = StereoChorus::new(44100.0);
        chorus.mix = 0.5;
        let input = sine_buf(440.0, 44100.0, 0.3);
        let output: Vec<f32> = input.iter().map(|&s| {
            let (l, _) = chorus.process(s);
            l
        }).collect();
        audio_test_harness::level::assert_not_silent(&output, 0.1);
    }

    #[test]
    fn test_chorus_stereo_input() {
        let mut chorus = StereoChorus::new(44100.0);
        chorus.mix = 1.0;
        let input = sine_buf(440.0, 44100.0, 0.2);
        for &s in &input {
            let (l, r) = chorus.process_stereo(s, s);
            assert!(l.is_finite());
            assert!(r.is_finite());
        }
    }
}
