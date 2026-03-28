use dsp_common::{OnePole, ad_envelope::ADEnvelope, pitch_envelope::PitchEnvelope, svfilter::SVFilter};

/// TR-808 Bass Drum — bridged-T resonator with pitch sweep.
/// The most iconic 808 sound: a self-oscillating bandpass filter
/// excited by a 1ms impulse, with pitch sweeping from ~130Hz to ~49Hz.
pub struct BassDrum {
    resonator: SVFilter,
    pitch_env: PitchEnvelope,
    amp_env: ADEnvelope,
    tone_lpf: OnePole,
    dc_block: OnePole,
    impulse_counter: u32,
    impulse_length: u32, // ~1ms in samples
    sample_rate: f32,

    // Parameters
    pub level: f32,   // 0-1
    pub tone: f32,    // 0-1 (dark to bright, controls click)
    pub decay: f32,   // 0-1 (short to long, maps to 50-800ms)
}

impl BassDrum {
    pub fn new(sample_rate: f32) -> Self {
        let mut resonator = SVFilter::new(sample_rate);
        resonator.set_q(20.0); // high Q for self-oscillation character

        let mut pitch_env = PitchEnvelope::new(sample_rate);
        pitch_env.set_sweep(130.0, 49.0, 0.006); // 6ms sweep

        let mut amp_env = ADEnvelope::new(sample_rate);
        amp_env.set_attack(0.0005);
        amp_env.set_decay(0.3);

        let mut tone_lpf = OnePole::new();
        tone_lpf.set_freq(3000.0, sample_rate);

        let mut dc_block = OnePole::new();
        dc_block.set_freq(6.7, sample_rate); // 6.7Hz HPF for DC removal

        Self {
            resonator, pitch_env, amp_env, tone_lpf, dc_block,
            impulse_counter: 0,
            impulse_length: (sample_rate * 0.001) as u32, // 1ms
            sample_rate,
            level: 0.8, tone: 0.5, decay: 0.5,
        }
    }

    pub fn trigger(&mut self) {
        // Set decay from parameter
        let decay_secs = 0.05 + self.decay * 0.75; // 50-800ms
        self.amp_env.set_decay(decay_secs);

        // Set tone (click brightness)
        let tone_hz = 200.0 + self.tone * 4800.0; // 200Hz-5kHz
        self.tone_lpf.set_freq(tone_hz, self.sample_rate);

        // Trigger envelopes
        self.pitch_env.trigger(130.0);
        self.amp_env.trigger();
        self.impulse_counter = self.impulse_length;

        // Reset resonator for clean attack
        self.resonator.clear();
    }

    pub fn process(&mut self) -> f32 {
        // Pitch envelope modulates resonator frequency
        let freq = self.pitch_env.process();
        self.resonator.set_freq(freq);

        // Impulse excitation (1ms pulse)
        let impulse = if self.impulse_counter > 0 {
            self.impulse_counter -= 1;
            0.8 // impulse amplitude
        } else {
            0.0
        };

        // Resonator (bandpass mode = self-oscillating pitched drum)
        let resonated = self.resonator.process_bp(impulse);

        // Amplitude envelope
        let env = self.amp_env.process();

        // Tone control (lowpass — removes click when dark)
        let toned = self.tone_lpf.process(resonated * env);

        // DC blocking highpass at 6.7Hz
        let dc_in = toned;
        let dc_filtered = dc_in - self.dc_block.process(dc_in);

        dc_filtered * self.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(bd: &mut BassDrum, secs: f32) -> Vec<f32> {
        let n = (bd.sample_rate * secs) as usize;
        (0..n).map(|_| bd.process()).collect()
    }

    #[test]
    fn test_bd_produces_sound() {
        let mut bd = BassDrum::new(44100.0);
        bd.trigger();
        let buf = render(&mut bd, 0.3);
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
    }

    #[test]
    fn test_bd_pitch_is_low() {
        let mut bd = BassDrum::new(44100.0);
        bd.decay = 0.8; // long decay for pitch detection
        bd.trigger();
        let buf = render(&mut bd, 0.5);

        // Fundamental should be in the sub-bass range (~49Hz)
        if let Some(hz) = audio_test_harness::pitch::detect(&buf[4410..], 44100.0) {
            assert!(hz < 100.0, "BD fundamental should be sub-bass, got {hz}Hz");
        }
    }

    #[test]
    fn test_bd_decays_to_silence() {
        let mut bd = BassDrum::new(44100.0);
        bd.decay = 0.3; // moderate decay
        bd.trigger();
        let buf = render(&mut bd, 1.0);
        let tail = &buf[44100 - 4410..];
        audio_test_harness::level::assert_silent(tail, 0.01);
    }

    #[test]
    fn test_bd_tone_affects_brightness() {
        // Dark tone
        let mut bd_dark = BassDrum::new(44100.0);
        bd_dark.tone = 0.0;
        bd_dark.trigger();
        let buf_dark = render(&mut bd_dark, 0.1);

        // Bright tone
        let mut bd_bright = BassDrum::new(44100.0);
        bd_bright.tone = 1.0;
        bd_bright.trigger();
        let buf_bright = render(&mut bd_bright, 0.1);

        // Should sound different
        let corr = audio_test_harness::correlation::cross_correlation(&buf_dark, &buf_bright);
        assert!(corr < 0.95, "Different tone settings should sound different (corr={corr:.3})");
    }
}
