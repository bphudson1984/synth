use dsp_common::{DelayLine, OnePole};

/// Fast tanh approximation using rational function.
#[inline(always)]
fn fast_tanh(x: f32) -> f32 {
    let x2 = x * x;
    x * (27.0 + x2) / (27.0 + 9.0 * x2)
}

/// Fast sine approximation for phase in [0, 1).
fn fast_sin(phase: f32) -> f32 {
    let x = phase * 2.0 - 1.0;
    4.0 * x * (1.0 - x.abs())
}

/// Tape-style stereo delay with filtered feedback, soft saturation, and subtle wow/flutter.
pub struct TapeDelay {
    delay_l: DelayLine,
    delay_r: DelayLine,
    feedback_l: f32,
    feedback_r: f32,
    tone_filter_l: OnePole,
    tone_filter_r: OnePole,
    time_smoother: OnePole,
    wow_phase: f32,
    flutter_phase: f32,
    sample_rate: f32,

    pub time_ms: f32,      // 1-2000 ms
    pub feedback: f32,     // 0-0.95
    pub tone: f32,         // 0-1 (maps to 800Hz-12kHz feedback filter)
    pub mix: f32,          // 0-1
    wow_inc: f32,
    flutter_inc: f32,
    last_tone: f32,
}

impl TapeDelay {
    pub fn new(sample_rate: f32) -> Self {
        let buf_size = 131072; // ~2.7s at 48kHz

        let mut time_smoother = OnePole::new();
        time_smoother.set_coeff(0.999);

        Self {
            delay_l: DelayLine::new(buf_size),
            delay_r: DelayLine::new(buf_size),
            feedback_l: 0.0,
            feedback_r: 0.0,
            tone_filter_l: OnePole::new(),
            tone_filter_r: OnePole::new(),
            time_smoother,
            wow_phase: 0.0,
            flutter_phase: 0.0,
            sample_rate,
            time_ms: 375.0,
            feedback: 0.4,
            tone: 0.6,
            mix: 0.3,
            wow_inc: 0.8 / sample_rate,
            flutter_inc: 6.0 / sample_rate,
            last_tone: -1.0,
        }
    }

    /// Process stereo input, returns stereo output.
    pub fn process(&mut self, input_l: f32, input_r: f32) -> (f32, f32) {
        let target_samples = self.time_ms * 0.001 * self.sample_rate;
        let smooth_samples = self.time_smoother.process(target_samples);

        self.wow_phase += self.wow_inc;
        if self.wow_phase >= 1.0 { self.wow_phase -= 1.0; }
        self.flutter_phase += self.flutter_inc;
        if self.flutter_phase >= 1.0 { self.flutter_phase -= 1.0; }

        let wow = fast_sin(self.wow_phase) * 0.3;
        let flutter = fast_sin(self.flutter_phase) * 0.15;
        let mod_offset = wow + flutter;

        let read_pos = (smooth_samples + mod_offset).max(1.0);

        if self.tone != self.last_tone {
            self.last_tone = self.tone;
            let tone_hz = 800.0 + self.tone * 11200.0;
            self.tone_filter_l.set_freq(tone_hz, self.sample_rate);
            self.tone_filter_r.set_freq(tone_hz, self.sample_rate);
        }

        self.delay_l.write(input_l + self.feedback_l * self.feedback);
        self.delay_r.write(input_r + self.feedback_r * self.feedback);

        let wet_l = self.delay_l.read_cubic(read_pos);
        let wet_r = self.delay_r.read_cubic(read_pos);

        let filtered_l = self.tone_filter_l.process(wet_l);
        let filtered_r = self.tone_filter_r.process(wet_r);

        self.feedback_l = fast_tanh(filtered_l * 0.5) * 2.0;
        self.feedback_r = fast_tanh(filtered_r * 0.5) * 2.0;

        let out_l = input_l * (1.0 - self.mix) + wet_l * self.mix;
        let out_r = input_r * (1.0 - self.mix) + wet_r * self.mix;

        (out_l, out_r)
    }

    pub fn clear(&mut self) {
        self.delay_l.clear();
        self.delay_r.clear();
        self.feedback_l = 0.0;
        self.feedback_r = 0.0;
        self.tone_filter_l.clear();
        self.tone_filter_r.clear();
        self.time_smoother.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay_produces_echo() {
        let mut delay = TapeDelay::new(44100.0);
        delay.time_ms = 100.0;
        delay.feedback = 0.5;
        delay.tone = 1.0;
        delay.mix = 1.0;

        let (_, _) = delay.process(1.0, 1.0);

        let mut has_energy = false;
        for i in 0..8820 {
            let (l, _) = delay.process(0.0, 0.0);
            if i > 4000 && i < 5000 && l.abs() > 0.01 {
                has_energy = true;
            }
        }
        assert!(has_energy, "Delay should produce an echo after the delay time");
    }

    #[test]
    fn test_delay_feedback_decays() {
        let mut delay = TapeDelay::new(44100.0);
        delay.time_ms = 50.0;
        delay.feedback = 0.5;
        delay.tone = 1.0;
        delay.mix = 1.0;

        delay.process(1.0, 1.0);

        let mut peaks = Vec::new();
        let mut max_in_window = 0.0f32;
        for i in 0..44100 {
            let (l, _) = delay.process(0.0, 0.0);
            max_in_window = max_in_window.max(l.abs());
            if (i + 1) % 2205 == 0 {
                peaks.push(max_in_window);
                max_in_window = 0.0;
            }
        }

        if peaks.len() >= 4 {
            assert!(
                peaks[3] < peaks[1],
                "Feedback should decay: peak[1]={}, peak[3]={}",
                peaks[1], peaks[3]
            );
        }
    }

    #[test]
    fn test_delay_mix_zero_is_dry() {
        let mut delay = TapeDelay::new(44100.0);
        delay.mix = 0.0;

        for i in 0..1000 {
            let input = (i as f32 * 0.1).sin();
            let (l, r) = delay.process(input, input);
            assert!((l - input).abs() < 0.001, "Mix=0 should pass dry signal");
            assert!((r - input).abs() < 0.001);
        }
    }
}
