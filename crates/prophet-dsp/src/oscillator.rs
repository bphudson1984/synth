pub struct Oscillator {
    phase: f32,
    freq_hz: f32,
    sample_rate: f32,
    pulse_width: f32,
    saw_on: bool,
    pulse_on: bool,
    tri_on: bool,
    lf_mode: bool,
    tri_integrator: f32,
    // Track phase wrap for sync
    last_phase: f32,
}

impl Oscillator {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            phase: 0.0,
            freq_hz: 440.0,
            sample_rate,
            pulse_width: 0.5,
            saw_on: false,
            pulse_on: false,
            tri_on: false,
            lf_mode: false,
            tri_integrator: 0.0,
            last_phase: 0.0,
        }
    }

    pub fn set_frequency(&mut self, freq_hz: f32) {
        self.freq_hz = freq_hz;
    }

    pub fn set_pulse_width(&mut self, pw: f32) {
        self.pulse_width = pw.clamp(0.01, 0.99);
    }

    pub fn set_saw(&mut self, on: bool) { self.saw_on = on; }
    pub fn set_pulse(&mut self, on: bool) { self.pulse_on = on; }
    pub fn set_tri(&mut self, on: bool) { self.tri_on = on; }

    /// Did this oscillator's phase wrap on the last process() call?
    /// Used by the voice to trigger hard sync on another oscillator.
    pub fn wrapped(&self) -> bool {
        self.phase < self.last_phase
    }

    /// Force a phase reset (hard sync from another oscillator).
    pub fn sync_reset(&mut self) {
        self.phase = 0.0;
        self.tri_integrator = 0.0;
    }

    /// Process with optional frequency modulation offset (for Poly Mod).
    pub fn process_with_fm(&mut self, fm_offset_hz: f32) -> f32 {
        let effective_freq = (self.freq_hz + fm_offset_hz).max(0.1);
        let dt = effective_freq / self.sample_rate;

        self.last_phase = self.phase;
        self.phase += dt;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        self.generate_waveforms(dt)
    }

    /// Process one sample. Returns the mixed output of enabled waveforms.
    pub fn process(&mut self) -> f32 {
        self.process_with_fm(0.0)
    }

    /// Get the current triangle output (used as Poly Mod source from Osc B).
    pub fn triangle_output(&self) -> f32 {
        if self.phase < 0.5 {
            self.phase * 4.0 - 1.0
        } else {
            3.0 - self.phase * 4.0
        }
    }

    fn generate_waveforms(&mut self, dt: f32) -> f32 {
        let mut out = 0.0;

        if self.saw_on {
            let mut saw = self.phase * 2.0 - 1.0;
            saw -= poly_blep(self.phase, dt);
            out += saw;
        }

        if self.pulse_on {
            let mut pulse = if self.phase < self.pulse_width { 1.0 } else { -1.0 };
            pulse += poly_blep(self.phase, dt);
            pulse -= poly_blep((self.phase - self.pulse_width + 1.0) % 1.0, dt);
            out += pulse;
        }

        if self.tri_on {
            let mut square = if self.phase < 0.5 { 1.0 } else { -1.0 };
            square += poly_blep(self.phase, dt);
            square -= poly_blep((self.phase + 0.5) % 1.0, dt);
            self.tri_integrator = self.tri_integrator * (1.0 - dt) + square * dt;
            let tri = self.tri_integrator * 4.0 / dt;
            out += tri.clamp(-1.0, 1.0);
        }

        out
    }
}

fn poly_blep(t: f32, dt: f32) -> f32 {
    if t < dt {
        let t = t / dt;
        2.0 * t - t * t - 1.0
    } else if t > 1.0 - dt {
        let t = (t - 1.0) / dt;
        t * t + 2.0 * t + 1.0
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(osc: &mut Oscillator, duration_secs: f32) -> Vec<f32> {
        let n = (osc.sample_rate * duration_secs) as usize;
        (0..n).map(|_| osc.process()).collect()
    }

    #[test]
    fn test_saw_pitch_440() {
        let mut osc = Oscillator::new(44100.0);
        osc.set_frequency(440.0);
        osc.set_saw(true);
        let buf = render(&mut osc, 0.5);
        audio_test_harness::pitch::assert_pitch(&buf, 44100.0, 440.0, 2.0);
    }

    #[test]
    fn test_saw_not_silent() {
        let mut osc = Oscillator::new(44100.0);
        osc.set_frequency(440.0);
        osc.set_saw(true);
        let buf = render(&mut osc, 0.1);
        audio_test_harness::level::assert_not_silent(&buf, 0.1);
    }

    #[test]
    fn test_saw_harmonics() {
        let mut osc = Oscillator::new(44100.0);
        osc.set_frequency(440.0);
        osc.set_saw(true);
        let buf = render(&mut osc, 1.0);
        audio_test_harness::spectral::assert_harmonic_series(
            &buf, 44100.0, 440.0, 10, |n| 1.0 / n as f32, 3.0,
        );
    }

    #[test]
    fn test_saw_no_aliasing() {
        let mut osc = Oscillator::new(44100.0);
        osc.set_frequency(440.0);
        osc.set_saw(true);
        let buf = render(&mut osc, 1.0);
        audio_test_harness::spectral::assert_no_aliasing(&buf, 44100.0, 440.0, -40.0);
    }

    #[test]
    fn test_pulse_pitch_440() {
        let mut osc = Oscillator::new(44100.0);
        osc.set_frequency(440.0);
        osc.set_pulse(true);
        osc.set_pulse_width(0.5);
        let buf = render(&mut osc, 0.5);
        audio_test_harness::pitch::assert_pitch(&buf, 44100.0, 440.0, 2.0);
    }

    #[test]
    fn test_pulse_square_odd_harmonics() {
        let mut osc = Oscillator::new(44100.0);
        osc.set_frequency(440.0);
        osc.set_pulse(true);
        osc.set_pulse_width(0.5);
        let buf = render(&mut osc, 1.0);
        audio_test_harness::spectral::assert_harmonic_series(
            &buf, 44100.0, 440.0, 10,
            |n| if n % 2 == 1 { 1.0 / n as f32 } else { 0.0 }, 3.0,
        );
    }

    #[test]
    fn test_triangle_pitch_440() {
        let mut osc = Oscillator::new(44100.0);
        osc.set_frequency(440.0);
        osc.set_tri(true);
        let buf = render(&mut osc, 0.5);
        audio_test_harness::pitch::assert_pitch(&buf, 44100.0, 440.0, 2.0);
    }

    #[test]
    fn test_no_waveform_is_silent() {
        let mut osc = Oscillator::new(44100.0);
        osc.set_frequency(440.0);
        let buf = render(&mut osc, 0.1);
        audio_test_harness::level::assert_silent(&buf, 0.0001);
    }

    #[test]
    fn test_simultaneous_saw_pulse() {
        let mut osc_both = Oscillator::new(44100.0);
        osc_both.set_frequency(440.0);
        osc_both.set_saw(true);
        osc_both.set_pulse(true);
        let buf_both = render(&mut osc_both, 0.1);

        let mut osc_saw = Oscillator::new(44100.0);
        osc_saw.set_frequency(440.0);
        osc_saw.set_saw(true);
        let buf_saw = render(&mut osc_saw, 0.1);

        let mut osc_pulse = Oscillator::new(44100.0);
        osc_pulse.set_frequency(440.0);
        osc_pulse.set_pulse(true);
        let buf_pulse = render(&mut osc_pulse, 0.1);

        let corr_vs_saw = audio_test_harness::correlation::cross_correlation(&buf_both, &buf_saw);
        let corr_vs_pulse = audio_test_harness::correlation::cross_correlation(&buf_both, &buf_pulse);
        assert!(corr_vs_saw < 0.99, "Saw+pulse should differ from saw alone (corr={corr_vs_saw})");
        assert!(corr_vs_pulse < 0.99, "Saw+pulse should differ from pulse alone (corr={corr_vs_pulse})");
        audio_test_harness::level::assert_not_silent(&buf_both, 0.1);
    }

    #[test]
    fn test_hard_sync() {
        // Osc B (master) at 200Hz, Osc A (slave) at 500Hz
        // With sync, the output fundamental should be at 200Hz (master freq)
        let mut osc_b = Oscillator::new(44100.0);
        osc_b.set_frequency(200.0);
        osc_b.set_saw(true);

        let mut osc_a = Oscillator::new(44100.0);
        osc_a.set_frequency(500.0);
        osc_a.set_saw(true);

        let n = (44100.0 * 0.5) as usize;
        let mut buf = Vec::with_capacity(n);
        for _ in 0..n {
            osc_b.process();
            if osc_b.wrapped() {
                osc_a.sync_reset();
            }
            buf.push(osc_a.process());
        }

        // Fundamental should be at master frequency (200Hz)
        audio_test_harness::pitch::assert_pitch(&buf, 44100.0, 200.0, 5.0);
        // Should have rich harmonics (more than a plain 200Hz saw)
        let spectrum = audio_test_harness::spectral::magnitude_spectrum(&buf);
        let peaks = audio_test_harness::spectral::find_peaks(&spectrum, 44100.0, 5);
        assert!(peaks.len() >= 3, "Sync should produce rich harmonics");
    }

    #[test]
    fn test_fm_modulation() {
        // FM should produce sidebands — output differs from plain oscillator
        let mut osc = Oscillator::new(44100.0);
        osc.set_frequency(440.0);
        osc.set_saw(true);

        let n = (44100.0 * 0.5) as usize;
        let modulator_freq = 110.0;
        let mod_depth = 200.0; // Hz
        let buf_fm: Vec<f32> = (0..n)
            .map(|i| {
                let t = i as f32 / 44100.0;
                let fm = (2.0 * std::f32::consts::PI * modulator_freq * t).sin() * mod_depth;
                osc.process_with_fm(fm)
            })
            .collect();

        let mut osc_plain = Oscillator::new(44100.0);
        osc_plain.set_frequency(440.0);
        osc_plain.set_saw(true);
        let buf_plain: Vec<f32> = (0..n).map(|_| osc_plain.process()).collect();

        // FM output should differ from plain
        let corr = audio_test_harness::correlation::cross_correlation(&buf_fm, &buf_plain);
        assert!(corr < 0.9, "FM should change the sound (corr={corr})");
    }
}
