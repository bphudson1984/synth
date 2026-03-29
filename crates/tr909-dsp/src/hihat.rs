use dsp_common::{ad_envelope::ADEnvelope, svfilter::SVFilter, bitcrusher::BitCrusher};

/// 909 metallic oscillator bank — different frequencies from 808.
pub struct MetallicOscBank909 {
    phases: [f32; 6],
    increments: [f32; 6],
}

const METALLIC_FREQS_909: [f32; 6] = [245.0, 307.0, 365.0, 415.0, 437.0, 547.0];

impl MetallicOscBank909 {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            phases: [0.0; 6],
            increments: METALLIC_FREQS_909.map(|f| f / sample_rate),
        }
    }

    pub fn process(&mut self) -> f32 {
        let mut sum = 0.0;
        for i in 0..6 {
            self.phases[i] += self.increments[i];
            if self.phases[i] >= 1.0 { self.phases[i] -= 1.0; }
            sum += if self.phases[i] < 0.5 { 1.0 } else { -1.0 };
        }
        sum / 6.0
    }
}

/// TR-909 Closed Hi-Hat — metallic oscs + HPF + 6-bit crush.
pub struct ClosedHiHat909 {
    osc: MetallicOscBank909,
    hpf: SVFilter,
    crush: BitCrusher,
    amp_env: ADEnvelope,
    pub level: f32,
}

impl ClosedHiHat909 {
    pub fn new(sample_rate: f32) -> Self {
        let mut hpf = SVFilter::new(sample_rate);
        hpf.set_freq(8000.0); // brighter than 808
        hpf.set_q(1.5);

        let mut amp_env = ADEnvelope::new(sample_rate);
        amp_env.set_attack(0.0003);
        amp_env.set_decay(0.03); // 30ms — shorter than 808's 50ms

        Self {
            osc: MetallicOscBank909::new(sample_rate),
            hpf, crush: BitCrusher::new(6, 0.75), // 6-bit, slight rate reduction
            amp_env, level: 0.7,
        }
    }

    pub fn trigger(&mut self) { self.amp_env.trigger(); }

    pub fn process(&mut self) -> f32 {
        let metal = self.osc.process();
        let hp = self.hpf.process_hp(metal);
        let crushed = self.crush.process(hp);
        let env = self.amp_env.process();
        crushed * env * self.level
    }

    pub fn is_active(&self) -> bool { self.amp_env.is_active() }
}

/// TR-909 Open Hi-Hat — same bank, longer decay, 6-bit crush.
pub struct OpenHiHat909 {
    osc: MetallicOscBank909,
    hpf: SVFilter,
    crush: BitCrusher,
    amp_env: ADEnvelope,
    pub level: f32,
    pub decay: f32,
    sample_rate: f32,
}

impl OpenHiHat909 {
    pub fn new(sample_rate: f32) -> Self {
        let mut hpf = SVFilter::new(sample_rate);
        hpf.set_freq(7000.0);
        hpf.set_q(1.5);

        let mut amp_env = ADEnvelope::new(sample_rate);
        amp_env.set_attack(0.0003);
        amp_env.set_decay(0.25);

        Self {
            osc: MetallicOscBank909::new(sample_rate),
            hpf, crush: BitCrusher::new(6, 0.75),
            amp_env, level: 0.7, decay: 0.5, sample_rate,
        }
    }

    pub fn trigger(&mut self) {
        let d = 0.1 + self.decay * 0.4; // 100-500ms
        self.amp_env.set_decay(d);
        self.amp_env.trigger();
    }

    pub fn choke(&mut self) { self.amp_env.set_decay(0.005); }

    pub fn process(&mut self) -> f32 {
        let metal = self.osc.process();
        let hp = self.hpf.process_hp(metal);
        let crushed = self.crush.process(hp);
        let env = self.amp_env.process();
        crushed * env * self.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ch909_produces_sound() {
        let mut ch = ClosedHiHat909::new(44100.0);
        ch.trigger();
        let buf: Vec<f32> = (0..4410).map(|_| ch.process()).collect();
        audio_test_harness::level::assert_not_silent(&buf, 0.005);
    }

    #[test]
    fn test_ch909_short_decay() {
        let mut ch = ClosedHiHat909::new(44100.0);
        ch.trigger();
        let buf: Vec<f32> = (0..44100).map(|_| ch.process()).collect();
        audio_test_harness::level::assert_silent(&buf[4410..], 0.01);
    }
}
