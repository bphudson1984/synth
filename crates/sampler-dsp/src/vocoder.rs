use dsp_common::svfilter::SVFilter;

const MAX_BANDS: usize = 16;

/// Channel vocoder: analyzes the spectral envelope of a modulator signal
/// (sample) and applies it to a carrier signal (internal oscillator).
///
/// The carrier pitch is set by the played note, while the spectral shape
/// comes from the sample — making the synth "speak" with the sample's character.
pub struct Vocoder {
    // Analysis filterbank (bandpass on modulator/sample)
    analysis: [SVFilter; MAX_BANDS],
    // Synthesis filterbank (bandpass on carrier)
    synthesis: [SVFilter; MAX_BANDS],
    // Envelope followers per band
    envelopes: [f32; MAX_BANDS],
    // Carrier oscillator
    carrier_phase: f32,
    carrier_freq: f32,
    noise_state: u32,
    // Settings
    pub enabled: bool,
    pub num_bands: usize,       // 4-16
    pub root_note: u8,          // MIDI note the sample was recorded at
    pub carrier_wave: u8,       // 0=saw, 1=square, 2=noise
    pub formant_shift: f32,     // -12 to +12 semitones (shifts analysis bands)
    pub attack: f32,            // envelope follower attack (0.001-0.1)
    pub release: f32,           // envelope follower release (0.01-0.5)
    pub mix: f32,               // 0=dry sample, 1=full vocoder
    sample_rate: f32,
    attack_coeff: f32,
    release_coeff: f32,
    bands_dirty: bool,
}

impl Vocoder {
    pub fn new(sample_rate: f32) -> Self {
        let mut v = Self {
            analysis: core::array::from_fn(|_| SVFilter::new(sample_rate)),
            synthesis: core::array::from_fn(|_| SVFilter::new(sample_rate)),
            envelopes: [0.0; MAX_BANDS],
            carrier_phase: 0.0,
            carrier_freq: 440.0,
            noise_state: 12345,
            enabled: false,
            num_bands: 12,
            root_note: 60, // C4
            carrier_wave: 0,
            formant_shift: 0.0,
            attack: 0.01,
            release: 0.08,
            mix: 1.0,
            sample_rate,
            attack_coeff: 0.0,
            release_coeff: 0.0,
            bands_dirty: true,
        };
        v.update_coeffs();
        v.setup_bands();
        v
    }

    fn update_coeffs(&mut self) {
        // Envelope follower coefficients
        self.attack_coeff = (-1.0 / (self.attack * self.sample_rate)).exp();
        self.release_coeff = (-1.0 / (self.release * self.sample_rate)).exp();
    }

    fn setup_bands(&mut self) {
        let n = self.num_bands.clamp(4, MAX_BANDS);
        // Logarithmic spacing from 60Hz to 12kHz (wide range for full spectrum)
        let lo = 60.0f32.ln();
        let hi = 12000.0f32.ln();
        let formant_ratio = (self.formant_shift / 12.0).exp2();

        for i in 0..n {
            let t = i as f32 / (n - 1).max(1) as f32;
            let center = (lo + t * (hi - lo)).exp();
            // Wide, overlapping bands — lower Q retains more source character.
            // Fewer bands → wider each band needs to be.
            let q = 0.5 + (n as f32 / MAX_BANDS as f32) * 1.5;

            // Analysis bands (shifted by formant)
            self.analysis[i].set_freq(center * formant_ratio);
            self.analysis[i].set_q(q);

            // Synthesis bands (at original center frequencies)
            self.synthesis[i].set_freq(center);
            self.synthesis[i].set_q(q);
        }
        self.bands_dirty = false;
    }

    pub fn set_num_bands(&mut self, n: usize) {
        self.num_bands = n.clamp(4, MAX_BANDS);
        self.bands_dirty = true;
    }

    pub fn set_formant_shift(&mut self, semitones: f32) {
        self.formant_shift = semitones.clamp(-12.0, 12.0);
        self.bands_dirty = true;
    }

    pub fn set_attack(&mut self, secs: f32) {
        self.attack = secs.clamp(0.001, 0.1);
        self.update_coeffs();
    }

    pub fn set_release(&mut self, secs: f32) {
        self.release = secs.clamp(0.01, 0.5);
        self.update_coeffs();
    }

    /// Set carrier frequency from MIDI note.
    pub fn set_note(&mut self, note: u8) {
        self.carrier_freq = 440.0 * 2.0f32.powf((note as f32 - 69.0) / 12.0);
    }

    /// Process one mono sample through the vocoder.
    /// `modulator` is the sample audio, returns vocoded output.
    pub fn process(&mut self, modulator: f32) -> f32 {
        if !self.enabled { return modulator; }
        if self.bands_dirty { self.setup_bands(); }

        let n = self.num_bands.clamp(4, MAX_BANDS);

        // Generate carrier
        let dt = self.carrier_freq / self.sample_rate;
        self.carrier_phase += dt;
        if self.carrier_phase >= 1.0 { self.carrier_phase -= 1.0; }

        // Carrier at full amplitude — the vocoder bands control the final level
        let carrier = match self.carrier_wave {
            1 => { // square
                if self.carrier_phase < 0.5 { 1.0 } else { -1.0 }
            }
            2 => { // noise
                self.noise_state ^= self.noise_state << 13;
                self.noise_state ^= self.noise_state >> 17;
                self.noise_state ^= self.noise_state << 5;
                (self.noise_state as i32 as f32) / i32::MAX as f32
            }
            _ => { // saw (harmonically rich — best for classic vocoder)
                self.carrier_phase * 2.0 - 1.0
            }
        };

        // Vocoder processing: analyze modulator, apply envelope to carrier bands
        let mut vocoded = 0.0f32;

        for i in 0..n {
            // Analysis: bandpass filter the modulator
            let band_signal = self.analysis[i].process_bp(modulator);

            // Envelope follower — smooth RMS-like tracking
            // Square the signal for power detection, then smooth
            let power = band_signal * band_signal;
            let coeff = if power > self.envelopes[i] { self.attack_coeff } else { self.release_coeff };
            self.envelopes[i] = power + (self.envelopes[i] - power) * coeff;

            // Amplitude = sqrt of smoothed power
            let amplitude = self.envelopes[i].sqrt();

            // Synthesis: bandpass filter the carrier, scale by tracked amplitude
            let carrier_band = self.synthesis[i].process_bp(carrier);
            vocoded += carrier_band * amplitude;
        }

        // Gentle normalization — scale up to compensate for band splitting losses
        vocoded *= 4.0;

        // Mix dry/wet
        modulator * (1.0 - self.mix) + vocoded * self.mix
    }

    pub fn clear(&mut self) {
        for i in 0..MAX_BANDS {
            self.analysis[i].clear();
            self.synthesis[i].clear();
            self.envelopes[i] = 0.0;
        }
    }
}
