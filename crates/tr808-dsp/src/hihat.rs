use dsp_common::{ad_envelope::ADEnvelope, svfilter::SVFilter};

/// Shared metallic oscillator bank — 6 square waves at non-harmonic frequencies.
/// Used by Closed Hi-Hat, Open Hi-Hat, and Cymbal.
pub struct MetallicOscBank {
    phases: [f32; 6],
    increments: [f32; 6],
}

/// The 6 TR-808 metallic oscillator frequencies (from service manual + measurements)
const METALLIC_FREQS: [f32; 6] = [205.3, 304.4, 369.6, 522.7, 540.0, 800.0];

impl MetallicOscBank {
    pub fn new(sample_rate: f32) -> Self {
        let increments = METALLIC_FREQS.map(|f| f / sample_rate);
        Self { phases: [0.0; 6], increments }
    }

    /// Process one sample — returns the sum of 6 square waves.
    pub fn process(&mut self) -> f32 {
        let mut sum = 0.0;
        for i in 0..6 {
            self.phases[i] += self.increments[i];
            if self.phases[i] >= 1.0 { self.phases[i] -= 1.0; }
            // Square wave: +1 or -1
            sum += if self.phases[i] < 0.5 { 1.0 } else { -1.0 };
        }
        sum / 6.0 // normalize
    }
}

/// TR-808 Closed Hi-Hat
pub struct ClosedHiHat {
    osc: MetallicOscBank,
    bpf_high: SVFilter, // 7100 Hz
    bpf_low: SVFilter,  // 3440 Hz
    amp_env: ADEnvelope,
    pub level: f32,
}

impl ClosedHiHat {
    pub fn new(sample_rate: f32) -> Self {
        let mut bpf_high = SVFilter::new(sample_rate);
        bpf_high.set_freq(7100.0);
        bpf_high.set_q(3.0);

        let mut bpf_low = SVFilter::new(sample_rate);
        bpf_low.set_freq(3440.0);
        bpf_low.set_q(2.0);

        let mut amp_env = ADEnvelope::new(sample_rate);
        amp_env.set_attack(0.0005);
        amp_env.set_decay(0.05); // 50ms fixed decay

        Self { osc: MetallicOscBank::new(sample_rate), bpf_high, bpf_low, amp_env, level: 0.7 }
    }

    pub fn trigger(&mut self) {
        self.amp_env.trigger();
    }

    pub fn process(&mut self) -> f32 {
        let metal = self.osc.process();
        let high = self.bpf_high.process_bp(metal);
        let low = self.bpf_low.process_bp(metal);
        let mixed = high * 0.6 + low * 0.4; // favor the bright path
        let env = self.amp_env.process();
        mixed * env * self.level
    }

    pub fn is_active(&self) -> bool { self.amp_env.is_active() }
}

/// TR-808 Open Hi-Hat — same oscillators, longer decay, choked by CH
pub struct OpenHiHat {
    osc: MetallicOscBank,
    bpf_high: SVFilter,
    bpf_low: SVFilter,
    amp_env: ADEnvelope,
    pub level: f32,
    pub decay: f32, // 0-1, maps to 90-600ms
    sample_rate: f32,
}

impl OpenHiHat {
    pub fn new(sample_rate: f32) -> Self {
        let mut bpf_high = SVFilter::new(sample_rate);
        bpf_high.set_freq(7100.0);
        bpf_high.set_q(3.0);

        let mut bpf_low = SVFilter::new(sample_rate);
        bpf_low.set_freq(3440.0);
        bpf_low.set_q(2.0);

        let mut amp_env = ADEnvelope::new(sample_rate);
        amp_env.set_attack(0.0005);
        amp_env.set_decay(0.3);

        Self {
            osc: MetallicOscBank::new(sample_rate),
            bpf_high, bpf_low, amp_env,
            level: 0.7, decay: 0.5, sample_rate,
        }
    }

    pub fn trigger(&mut self) {
        let decay_secs = 0.09 + self.decay * 0.51; // 90-600ms
        self.amp_env.set_decay(decay_secs);
        self.amp_env.trigger();
    }

    /// Choke — immediately kill the sound (triggered by closed hi-hat)
    pub fn choke(&mut self) {
        self.amp_env.set_decay(0.005); // very fast fade
    }

    pub fn process(&mut self) -> f32 {
        let metal = self.osc.process();
        let high = self.bpf_high.process_bp(metal);
        let low = self.bpf_low.process_bp(metal);
        let mixed = high * 0.6 + low * 0.4;
        let env = self.amp_env.process();
        mixed * env * self.level
    }

    pub fn is_active(&self) -> bool { self.amp_env.is_active() }
}

/// TR-808 Cymbal — same oscillator bank, lower BPF only, longer decay
pub struct Cymbal {
    osc: MetallicOscBank,
    bpf: SVFilter, // 3440 Hz only (darker than hi-hat)
    amp_env: ADEnvelope,
    pub level: f32,
    pub decay: f32, // 0-1, maps to 350-1200ms
    sample_rate: f32,
}

impl Cymbal {
    pub fn new(sample_rate: f32) -> Self {
        let mut bpf = SVFilter::new(sample_rate);
        bpf.set_freq(3440.0);
        bpf.set_q(2.0);

        let mut amp_env = ADEnvelope::new(sample_rate);
        amp_env.set_attack(0.0005);
        amp_env.set_decay(0.7);

        Self {
            osc: MetallicOscBank::new(sample_rate),
            bpf, amp_env,
            level: 0.6, decay: 0.5, sample_rate,
        }
    }

    pub fn trigger(&mut self) {
        let decay_secs = 0.35 + self.decay * 0.85; // 350-1200ms
        self.amp_env.set_decay(decay_secs);
        self.amp_env.trigger();
    }

    pub fn process(&mut self) -> f32 {
        let metal = self.osc.process();
        let filtered = self.bpf.process_bp(metal);
        let env = self.amp_env.process();
        filtered * env * self.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ch_produces_sound() {
        let mut ch = ClosedHiHat::new(44100.0);
        ch.trigger();
        let buf: Vec<f32> = (0..4410).map(|_| ch.process()).collect();
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
    }

    #[test]
    fn test_ch_short_decay() {
        let mut ch = ClosedHiHat::new(44100.0);
        ch.trigger();
        let buf: Vec<f32> = (0..44100).map(|_| ch.process()).collect();
        // After 200ms should be silent (50ms decay)
        let tail = &buf[8820..];
        audio_test_harness::level::assert_silent(tail, 0.01);
    }

    #[test]
    fn test_oh_longer_than_ch() {
        let mut ch = ClosedHiHat::new(44100.0);
        ch.trigger();
        let ch_buf: Vec<f32> = (0..22050).map(|_| ch.process()).collect();
        let ch_rms = audio_test_harness::level::rms(&ch_buf[4410..]);

        let mut oh = OpenHiHat::new(44100.0);
        oh.decay = 0.8;
        oh.trigger();
        let oh_buf: Vec<f32> = (0..22050).map(|_| oh.process()).collect();
        let oh_rms = audio_test_harness::level::rms(&oh_buf[4410..]);

        assert!(oh_rms > ch_rms * 2.0,
            "OH ({oh_rms:.4}) should be louder at 100-500ms than CH ({ch_rms:.4})");
    }

    #[test]
    fn test_metallic_bank_is_noisy() {
        // 6 non-harmonic square waves should produce complex, noise-like spectrum
        let mut bank = MetallicOscBank::new(44100.0);
        let buf: Vec<f32> = (0..44100).map(|_| bank.process()).collect();
        audio_test_harness::level::assert_not_silent(&buf, 0.1);
    }
}
