use dsp_common::OnePole;
use dsp_common::svfilter::SVFilter;

/// Micro POG-inspired polyphonic octave pedal.
///
/// Generates one octave above and one octave below the input signal,
/// with independent level controls for dry, sub-octave, and upper-octave.
///
/// Signal chain per channel:
/// 1. Input → envelope follower (for amplitude tracking)
/// 2. Octave up: full-wave rectification + bandpass filter
/// 3. Octave down: half-wave flip-flop frequency divider + lowpass filter
/// 4. Mix dry + sub + up at configurable levels
pub struct OctavePedal {
    #[allow(dead_code)]
    sample_rate: f32,

    // Octave-up: full-wave rectification + filtering
    up_filter_l: SVFilter,
    up_filter_r: SVFilter,
    up_dc_prev_in_l: f32,
    up_dc_prev_in_r: f32,
    up_dc_prev_out_l: f32,
    up_dc_prev_out_r: f32,

    // Octave-down: zero-crossing flip-flop divider
    prev_sample_l: f32,
    prev_sample_r: f32,
    flip_l: f32,
    flip_r: f32,
    sub_filter_l: SVFilter,
    sub_filter_r: SVFilter,
    // Envelope follower for sub-octave amplitude
    env_l: OnePole,
    env_r: OnePole,

    /// Dry signal level (0-1)
    pub dry: f32,
    /// Sub-octave (one octave below) level (0-1)
    pub sub: f32,
    /// Upper octave (one octave above) level (0-1)
    pub up: f32,
}

impl OctavePedal {
    pub fn new(sample_rate: f32) -> Self {
        // Octave-up bandpass: centre around midrange to clean up rectified signal
        let mut up_filter_l = SVFilter::new(sample_rate);
        up_filter_l.set_freq(1200.0);
        up_filter_l.set_q(0.5);
        let mut up_filter_r = SVFilter::new(sample_rate);
        up_filter_r.set_freq(1200.0);
        up_filter_r.set_q(0.5);

        // Sub-octave lowpass: smooth the square wave from the divider
        let mut sub_filter_l = SVFilter::new(sample_rate);
        sub_filter_l.set_freq(800.0);
        sub_filter_l.set_q(0.5);
        let mut sub_filter_r = SVFilter::new(sample_rate);
        sub_filter_r.set_freq(800.0);
        sub_filter_r.set_q(0.5);

        // Envelope follower for amplitude tracking (fast attack, moderate release)
        let mut env_l = OnePole::new();
        env_l.set_freq(30.0, sample_rate);
        let mut env_r = OnePole::new();
        env_r.set_freq(30.0, sample_rate);

        Self {
            sample_rate,
            up_filter_l,
            up_filter_r,
            up_dc_prev_in_l: 0.0,
            up_dc_prev_in_r: 0.0,
            up_dc_prev_out_l: 0.0,
            up_dc_prev_out_r: 0.0,
            prev_sample_l: 0.0,
            prev_sample_r: 0.0,
            flip_l: 1.0,
            flip_r: 1.0,
            sub_filter_l,
            sub_filter_r,
            env_l,
            env_r,
            dry: 1.0,
            sub: 0.5,
            up: 0.0,
        }
    }

    /// Process one stereo sample pair. L/R processed independently.
    pub fn process(&mut self, left: f32, right: f32) -> (f32, f32) {
        let out_l = self.process_channel_left(left);
        let out_r = self.process_channel_right(right);
        (out_l, out_r)
    }

    fn process_channel_left(&mut self, input: f32) -> f32 {
        // --- Octave Up ---
        // Full-wave rectification doubles the frequency
        let rectified = input.abs();
        // Bandpass filter to clean up harmonics
        let up_filtered = self.up_filter_l.process_bp(rectified);
        // DC blocker for rectified signal
        let dc_out = up_filtered - self.up_dc_prev_in_l + 0.995 * self.up_dc_prev_out_l;
        self.up_dc_prev_in_l = up_filtered;
        self.up_dc_prev_out_l = dc_out;
        let octave_up = dc_out * 2.0;

        // --- Octave Down ---
        // Envelope follower: track input amplitude
        let env = self.env_l.process(input.abs());
        // Zero-crossing detector with flip-flop for frequency division
        if self.prev_sample_l <= 0.0 && input > 0.0 {
            self.flip_l = -self.flip_l;
        }
        self.prev_sample_l = input;
        // Generate sub signal: square wave at half frequency, shaped by envelope
        let sub_raw = self.flip_l * env;
        // Lowpass filter to smooth the square wave into something more sinusoidal
        let octave_down = self.sub_filter_l.process_lp(sub_raw);

        // --- Mix ---
        self.dry * input + self.sub * octave_down + self.up * octave_up
    }

    fn process_channel_right(&mut self, input: f32) -> f32 {
        // --- Octave Up ---
        let rectified = input.abs();
        let up_filtered = self.up_filter_r.process_bp(rectified);
        let dc_out = up_filtered - self.up_dc_prev_in_r + 0.995 * self.up_dc_prev_out_r;
        self.up_dc_prev_in_r = up_filtered;
        self.up_dc_prev_out_r = dc_out;
        let octave_up = dc_out * 2.0;

        // --- Octave Down ---
        let env = self.env_r.process(input.abs());
        if self.prev_sample_r <= 0.0 && input > 0.0 {
            self.flip_r = -self.flip_r;
        }
        self.prev_sample_r = input;
        let sub_raw = self.flip_r * env;
        let octave_down = self.sub_filter_r.process_lp(sub_raw);

        // --- Mix ---
        self.dry * input + self.sub * octave_down + self.up * octave_up
    }

    pub fn clear(&mut self) {
        self.up_filter_l.clear();
        self.up_filter_r.clear();
        self.up_dc_prev_in_l = 0.0;
        self.up_dc_prev_in_r = 0.0;
        self.up_dc_prev_out_l = 0.0;
        self.up_dc_prev_out_r = 0.0;
        self.prev_sample_l = 0.0;
        self.prev_sample_r = 0.0;
        self.flip_l = 1.0;
        self.flip_r = 1.0;
        self.sub_filter_l.clear();
        self.sub_filter_r.clear();
        self.env_l.clear();
        self.env_r.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    fn sine_buf(freq: f32, sample_rate: f32, seconds: f32) -> Vec<f32> {
        let n = (sample_rate * seconds) as usize;
        (0..n)
            .map(|i| (2.0 * PI * freq * i as f32 / sample_rate).sin())
            .collect()
    }

    #[test]
    fn test_octave_not_silent() {
        let mut oct = OctavePedal::new(44100.0);
        oct.dry = 0.0;
        oct.sub = 1.0;
        oct.up = 1.0;

        let input = sine_buf(220.0, 44100.0, 0.5);
        let output: Vec<f32> = input
            .iter()
            .map(|&s| {
                let (l, _) = oct.process(s, s);
                l
            })
            .collect();

        audio_test_harness::level::assert_not_silent(&output, 0.01);
    }

    #[test]
    fn test_octave_dry_only_passes_signal() {
        let mut oct = OctavePedal::new(44100.0);
        oct.dry = 1.0;
        oct.sub = 0.0;
        oct.up = 0.0;

        for i in 0..1000 {
            let input = (i as f32 * 0.1).sin() * 0.5;
            let (l, r) = oct.process(input, input);
            assert!(
                (l - input).abs() < 0.001,
                "Dry-only left should match input, got {l} vs {input}"
            );
            assert!(
                (r - input).abs() < 0.001,
                "Dry-only right should match input"
            );
        }
    }

    #[test]
    fn test_octave_all_zero_is_silent() {
        let mut oct = OctavePedal::new(44100.0);
        oct.dry = 0.0;
        oct.sub = 0.0;
        oct.up = 0.0;

        let input = sine_buf(440.0, 44100.0, 0.3);
        let output: Vec<f32> = input
            .iter()
            .map(|&s| {
                let (l, _) = oct.process(s, s);
                l
            })
            .collect();

        let rms: f32 =
            (output.iter().map(|s| s * s).sum::<f32>() / output.len() as f32).sqrt();
        assert!(rms < 0.001, "All levels at 0 should be silent, got rms={rms}");
    }

    #[test]
    fn test_octave_sub_produces_output() {
        let mut oct = OctavePedal::new(44100.0);
        oct.dry = 0.0;
        oct.sub = 1.0;
        oct.up = 0.0;

        let input = sine_buf(440.0, 44100.0, 0.5);
        let output: Vec<f32> = input
            .iter()
            .map(|&s| {
                let (l, _) = oct.process(s, s);
                l
            })
            .collect();

        audio_test_harness::level::assert_not_silent(&output, 0.001);
    }

    #[test]
    fn test_octave_up_produces_output() {
        let mut oct = OctavePedal::new(44100.0);
        oct.dry = 0.0;
        oct.sub = 0.0;
        oct.up = 1.0;

        let input = sine_buf(440.0, 44100.0, 0.5);
        let output: Vec<f32> = input
            .iter()
            .map(|&s| {
                let (l, _) = oct.process(s, s);
                l
            })
            .collect();

        audio_test_harness::level::assert_not_silent(&output, 0.001);
    }

    #[test]
    fn test_octave_output_is_finite() {
        let mut oct = OctavePedal::new(44100.0);
        oct.dry = 1.0;
        oct.sub = 1.0;
        oct.up = 1.0;

        let input = sine_buf(440.0, 44100.0, 1.0);
        for &s in &input {
            let (l, r) = oct.process(s, s);
            assert!(l.is_finite(), "Left output must be finite");
            assert!(r.is_finite(), "Right output must be finite");
        }
    }

    #[test]
    fn test_octave_clear_resets_state() {
        let mut oct = OctavePedal::new(44100.0);
        oct.dry = 0.0;
        oct.sub = 1.0;
        oct.up = 1.0;

        // Process some signal
        let input = sine_buf(440.0, 44100.0, 0.1);
        for &s in &input {
            oct.process(s, s);
        }

        // Clear and check that silence input gives near-silence output
        oct.clear();
        let mut max_val: f32 = 0.0;
        for _ in 0..100 {
            let (l, _) = oct.process(0.0, 0.0);
            max_val = max_val.max(l.abs());
        }
        assert!(
            max_val < 0.01,
            "After clear, silent input should give near-silent output, got {max_val}"
        );
    }
}
