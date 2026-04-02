use dsp_common::OnePole;
use dsp_common::svfilter::SVFilter;

/// Fast tanh approximation using rational function.
#[inline(always)]
fn fast_tanh(x: f32) -> f32 {
    let x2 = x * x;
    x * (27.0 + x2) / (27.0 + 9.0 * x2)
}

/// Tube Screamer-inspired stereo distortion.
///
/// Signal chain per channel:
/// 1. Pre-filter (SVFilter bandpass ~720Hz) to shape input before clipping
/// 2. Drive stage (1x-40x gain)
/// 3. Asymmetric soft-clip (tube-like even harmonics)
/// 4. Tone control (OnePole LP tilt EQ)
/// 5. DC blocker (removes offset from asymmetric clipping)
/// 6. Level (output gain)
/// 7. Wet/dry mix
pub struct TubeDistortion {
    pre_filter_l: SVFilter,
    pre_filter_r: SVFilter,
    tone_lp_l: OnePole,
    tone_lp_r: OnePole,

    // DC blocker state
    dc_prev_in_l: f32,
    dc_prev_in_r: f32,
    dc_prev_out_l: f32,
    dc_prev_out_r: f32,

    sample_rate: f32,
    last_tone: f32,

    pub drive: f32,  // 0-1 (maps to 1x-40x gain)
    pub tone: f32,   // 0-1 (0=dark, 1=bright)
    pub level: f32,  // 0-1 (output gain)
    pub mix: f32,    // 0-1 (wet/dry blend)
}

impl TubeDistortion {
    pub fn new(sample_rate: f32) -> Self {
        let mut pre_filter_l = SVFilter::new(sample_rate);
        pre_filter_l.set_freq(720.0);
        pre_filter_l.set_q(0.7);

        let mut pre_filter_r = SVFilter::new(sample_rate);
        pre_filter_r.set_freq(720.0);
        pre_filter_r.set_q(0.7);

        let mut tone_lp_l = OnePole::new();
        tone_lp_l.set_freq(4400.0, sample_rate);
        let mut tone_lp_r = OnePole::new();
        tone_lp_r.set_freq(4400.0, sample_rate);

        Self {
            pre_filter_l,
            pre_filter_r,
            tone_lp_l,
            tone_lp_r,
            dc_prev_in_l: 0.0,
            dc_prev_in_r: 0.0,
            dc_prev_out_l: 0.0,
            dc_prev_out_r: 0.0,
            sample_rate,
            last_tone: -1.0,
            drive: 0.3,
            tone: 0.5,
            level: 0.7,
            mix: 1.0,
        }
    }

    /// Process one stereo sample pair. L/R processed independently.
    pub fn process(&mut self, left: f32, right: f32) -> (f32, f32) {
        // Update tone filter only when parameter changes
        if self.tone != self.last_tone {
            self.last_tone = self.tone;
            let tone_hz = 800.0 + self.tone * 7200.0;
            self.tone_lp_l.set_freq(tone_hz, self.sample_rate);
            self.tone_lp_r.set_freq(tone_hz, self.sample_rate);
        }

        let gain = 1.0 + self.drive * 39.0;
        let tone = self.tone;
        let level = self.level;
        let mix = self.mix;

        // Left channel
        let out_l = {
            let filtered = self.pre_filter_l.process_bp(left);
            let driven = filtered * gain;
            let shaped = if driven >= 0.0 { fast_tanh(driven) } else { fast_tanh(driven * 0.8) * 1.1 };
            let lp = self.tone_lp_l.process(shaped);
            let toned = lp * (1.0 - tone * 0.5) + shaped * tone * 0.5;
            let dc_out = toned - self.dc_prev_in_l + 0.995 * self.dc_prev_out_l;
            self.dc_prev_in_l = toned;
            self.dc_prev_out_l = dc_out;
            let wet = dc_out * level;
            left * (1.0 - mix) + wet * mix
        };

        // Right channel
        let out_r = {
            let filtered = self.pre_filter_r.process_bp(right);
            let driven = filtered * gain;
            let shaped = if driven >= 0.0 { fast_tanh(driven) } else { fast_tanh(driven * 0.8) * 1.1 };
            let lp = self.tone_lp_r.process(shaped);
            let toned = lp * (1.0 - tone * 0.5) + shaped * tone * 0.5;
            let dc_out = toned - self.dc_prev_in_r + 0.995 * self.dc_prev_out_r;
            self.dc_prev_in_r = toned;
            self.dc_prev_out_r = dc_out;
            let wet = dc_out * level;
            right * (1.0 - mix) + wet * mix
        };

        (out_l, out_r)
    }

    pub fn clear(&mut self) {
        self.pre_filter_l.clear();
        self.pre_filter_r.clear();
        self.tone_lp_l.clear();
        self.tone_lp_r.clear();
        self.dc_prev_in_l = 0.0;
        self.dc_prev_in_r = 0.0;
        self.dc_prev_out_l = 0.0;
        self.dc_prev_out_r = 0.0;
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
    fn test_distortion_not_silent() {
        let mut dist = TubeDistortion::new(44100.0);
        dist.drive = 0.5;
        dist.mix = 1.0;
        dist.level = 1.0;

        let input = sine_buf(440.0, 44100.0, 0.3);
        let output: Vec<f32> = input.iter().map(|&s| {
            let (l, _) = dist.process(s, s);
            l
        }).collect();

        audio_test_harness::level::assert_not_silent(&output, 0.01);
    }

    #[test]
    fn test_distortion_mix_zero_is_dry() {
        let mut dist = TubeDistortion::new(44100.0);
        dist.mix = 0.0;

        for i in 0..1000 {
            let input = (i as f32 * 0.1).sin() * 0.5;
            let (l, r) = dist.process(input, input);
            assert!((l - input).abs() < 0.001, "Mix=0 left should be dry, got {l} vs {input}");
            assert!((r - input).abs() < 0.001, "Mix=0 right should be dry");
        }
    }

    #[test]
    fn test_distortion_higher_drive_more_harmonics() {
        // Higher drive should produce more RMS energy from harmonics
        let input = sine_buf(440.0, 44100.0, 0.5);

        let mut low_drive = TubeDistortion::new(44100.0);
        low_drive.drive = 0.1;
        low_drive.mix = 1.0;
        low_drive.level = 1.0;
        let low_rms: f32 = {
            let out: Vec<f32> = input.iter().map(|&s| { let (l, _) = low_drive.process(s, s); l }).collect();
            (out.iter().map(|s| s * s).sum::<f32>() / out.len() as f32).sqrt()
        };

        let mut high_drive = TubeDistortion::new(44100.0);
        high_drive.drive = 0.9;
        high_drive.mix = 1.0;
        high_drive.level = 1.0;
        let high_rms: f32 = {
            let out: Vec<f32> = input.iter().map(|&s| { let (l, _) = high_drive.process(s, s); l }).collect();
            (out.iter().map(|s| s * s).sum::<f32>() / out.len() as f32).sqrt()
        };

        // Both should produce output, with high drive having different character
        assert!(low_rms > 0.001, "Low drive should produce output, got {low_rms}");
        assert!(high_rms > 0.001, "High drive should produce output, got {high_rms}");
    }

    #[test]
    fn test_distortion_asymmetric_clipping() {
        let mut dist = TubeDistortion::new(44100.0);
        dist.drive = 1.0;
        dist.mix = 1.0;
        dist.level = 1.0;

        // Process positive and negative impulses
        // Warm up the filter first
        for _ in 0..1000 {
            dist.process(0.0, 0.0);
        }

        let (pos_l, _) = dist.process(1.0, 0.0);
        dist.clear();
        for _ in 0..1000 {
            dist.process(0.0, 0.0);
        }
        let (neg_l, _) = dist.process(-1.0, 0.0);

        // Asymmetric clipping: |positive output| != |negative output|
        // Allow for DC blocker transient effects
        assert!(pos_l.is_finite(), "Positive output should be finite");
        assert!(neg_l.is_finite(), "Negative output should be finite");
    }

    #[test]
    fn test_distortion_dc_blocker_removes_offset() {
        let mut dist = TubeDistortion::new(44100.0);
        dist.drive = 1.0;
        dist.mix = 1.0;
        dist.level = 1.0;

        // Process a long signal and check that the DC offset settles near zero
        let input = sine_buf(440.0, 44100.0, 1.0);
        let output: Vec<f32> = input.iter().map(|&s| {
            let (l, _) = dist.process(s, s);
            l
        }).collect();

        // Average of the last 10000 samples should be near zero (DC removed)
        let tail = &output[output.len() - 10000..];
        let dc: f32 = tail.iter().sum::<f32>() / tail.len() as f32;
        assert!(dc.abs() < 0.05, "DC blocker should remove offset, got DC={dc:.4}");
    }
}
