use std::f32::consts::PI;
use dsp_common::{OnePole, ad_envelope::ADEnvelope, pitch_envelope::PitchEnvelope};

/// TR-909 Bass Drum — sine VCO with fast pitch sweep + click transient.
/// THE kick of house and techno. Punchier and harder than the 808.
pub struct BassDrum909 {
    phase: f32,
    pitch_env: PitchEnvelope,
    amp_env: ADEnvelope,
    click_counter: u32,
    click_length: u32,
    tone_lpf: OnePole,
    dc_block: OnePole,
    sample_rate: f32,

    pub level: f32,
    pub tone: f32,  // 0-1: click amount
    pub decay: f32, // 0-1: maps to 100-500ms
}

impl BassDrum909 {
    pub fn new(sample_rate: f32) -> Self {
        let mut pitch_env = PitchEnvelope::new(sample_rate);
        pitch_env.set_sweep(180.0, 50.0, 0.005); // 5ms sweep — faster than 808

        let mut amp_env = ADEnvelope::new(sample_rate);
        amp_env.set_attack(0.0003);
        amp_env.set_decay(0.25);

        let mut tone_lpf = OnePole::new();
        tone_lpf.set_freq(3000.0, sample_rate);

        let mut dc_block = OnePole::new();
        dc_block.set_freq(8.0, sample_rate);

        Self {
            phase: 0.0,
            pitch_env, amp_env,
            click_counter: 0,
            click_length: (sample_rate * 0.0005) as u32, // 0.5ms click
            tone_lpf, dc_block, sample_rate,
            level: 0.8, tone: 0.5, decay: 0.5,
        }
    }

    pub fn trigger(&mut self) {
        let decay_secs = 0.1 + self.decay * 0.4; // 100-500ms
        self.amp_env.set_decay(decay_secs);

        let tone_hz = 200.0 + self.tone * 4800.0;
        self.tone_lpf.set_freq(tone_hz, self.sample_rate);

        self.pitch_env.trigger(180.0);
        self.amp_env.trigger();
        self.click_counter = self.click_length;
        self.phase = 0.0;
    }

    pub fn process(&mut self) -> f32 {
        let freq = self.pitch_env.process();
        self.phase += freq / self.sample_rate;
        if self.phase >= 1.0 { self.phase -= 1.0; }

        // Sine oscillator
        let sine = (self.phase * 2.0 * PI).sin();

        // Click transient — sharp pulse
        let click = if self.click_counter > 0 {
            self.click_counter -= 1;
            0.8
        } else { 0.0 };

        let env = self.amp_env.process();
        let raw = (sine + click) * env;

        // Tone LPF (controls click amount)
        let toned = self.tone_lpf.process(raw);

        // DC block
        let dc_in = toned;
        let out = dc_in - self.dc_block.process(dc_in);

        out * self.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(bd: &mut BassDrum909, secs: f32) -> Vec<f32> {
        let n = (bd.sample_rate * secs) as usize;
        (0..n).map(|_| bd.process()).collect()
    }

    #[test]
    fn test_bd909_produces_sound() {
        let mut bd = BassDrum909::new(44100.0);
        bd.trigger();
        let buf = render(&mut bd, 0.2);
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
    }

    #[test]
    fn test_bd909_pitch_is_low() {
        let mut bd = BassDrum909::new(44100.0);
        bd.decay = 0.8;
        bd.trigger();
        let buf = render(&mut bd, 0.5);
        if let Some(hz) = audio_test_harness::pitch::detect(&buf[4410..], 44100.0) {
            assert!(hz < 100.0, "909 BD should be sub-bass, got {hz}Hz");
        }
    }

    #[test]
    fn test_bd909_decays_to_silence() {
        let mut bd = BassDrum909::new(44100.0);
        bd.decay = 0.3;
        bd.trigger();
        let buf = render(&mut bd, 1.0);
        audio_test_harness::level::assert_silent(&buf[44100 - 4410..], 0.01);
    }

    #[test]
    fn test_bd909_has_click_transient() {
        let mut bd = BassDrum909::new(44100.0);
        bd.tone = 1.0; // maximum click
        bd.trigger();
        let buf = render(&mut bd, 0.01); // first 10ms
        let early_rms = audio_test_harness::level::rms(&buf[..22]);
        let mid_rms = audio_test_harness::level::rms(&buf[220..]);
        // The very first samples should have strong transient energy
        assert!(early_rms > 0.01, "Should have click transient, early_rms={early_rms}");
    }
}
