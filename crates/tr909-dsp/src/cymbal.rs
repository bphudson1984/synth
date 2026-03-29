use std::f32::consts::PI;
use dsp_common::{NoiseGenerator, ad_envelope::ADEnvelope, svfilter::SVFilter, bitcrusher::BitCrusher};

/// TR-909 Crash Cymbal — metallic oscs + noise, HPF, 6-bit crush, long decay.
pub struct CrashCymbal909 {
    phases: [f32; 6],
    increments: [f32; 6],
    noise: NoiseGenerator,
    hpf: SVFilter,
    crush: BitCrusher,
    amp_env: ADEnvelope,
    pub level: f32,
    pub decay: f32,
    sample_rate: f32,
}

const CRASH_FREQS: [f32; 6] = [205.0, 295.0, 365.0, 430.0, 540.0, 690.0];

impl CrashCymbal909 {
    pub fn new(sample_rate: f32) -> Self {
        let mut hpf = SVFilter::new(sample_rate);
        hpf.set_freq(4000.0); hpf.set_q(1.2);
        let mut amp_env = ADEnvelope::new(sample_rate);
        amp_env.set_attack(0.001); amp_env.set_decay(1.5);

        Self {
            phases: [0.0; 6],
            increments: CRASH_FREQS.map(|f| f / sample_rate),
            noise: NoiseGenerator::new(2909), hpf,
            crush: BitCrusher::new(6, 0.7),
            amp_env, level: 0.5, decay: 0.5, sample_rate,
        }
    }

    pub fn trigger(&mut self) {
        let d = 0.8 + self.decay * 2.2; // 0.8-3s
        self.amp_env.set_decay(d);
        self.amp_env.trigger();
    }

    pub fn process(&mut self) -> f32 {
        let mut sum = 0.0;
        for i in 0..6 {
            self.phases[i] += self.increments[i];
            if self.phases[i] >= 1.0 { self.phases[i] -= 1.0; }
            sum += if self.phases[i] < 0.5 { 1.0 } else { -1.0 };
        }
        let metal = sum / 6.0;
        let noise = self.noise.white() * 0.3;
        let hp = self.hpf.process_hp(metal + noise);
        let crushed = self.crush.process(hp);
        let env = self.amp_env.process();
        crushed * env * self.level
    }
}

/// TR-909 Ride Cymbal — metallic oscs + sine bell + HPF, 6-bit crush.
pub struct RideCymbal909 {
    phases: [f32; 6],
    increments: [f32; 6],
    bell_phase: f32,
    bell_inc: f32,
    hpf: SVFilter,
    crush: BitCrusher,
    amp_env: ADEnvelope,
    bell_env: ADEnvelope,
    pub level: f32,
    pub decay: f32,
    sample_rate: f32,
}

const RIDE_FREQS: [f32; 6] = [270.0, 340.0, 390.0, 470.0, 555.0, 680.0];

impl RideCymbal909 {
    pub fn new(sample_rate: f32) -> Self {
        let mut hpf = SVFilter::new(sample_rate);
        hpf.set_freq(5000.0); hpf.set_q(1.2);
        let mut amp_env = ADEnvelope::new(sample_rate);
        amp_env.set_attack(0.001); amp_env.set_decay(1.0);
        let mut bell_env = ADEnvelope::new(sample_rate);
        bell_env.set_attack(0.0003); bell_env.set_decay(0.5);

        Self {
            phases: [0.0; 6],
            increments: RIDE_FREQS.map(|f| f / sample_rate),
            bell_phase: 0.0, bell_inc: 3200.0 / sample_rate,
            hpf, crush: BitCrusher::new(6, 0.75),
            amp_env, bell_env,
            level: 0.5, decay: 0.5, sample_rate,
        }
    }

    pub fn trigger(&mut self) {
        let d = 0.5 + self.decay * 1.5;
        self.amp_env.set_decay(d);
        self.amp_env.trigger();
        self.bell_env.trigger();
        self.bell_phase = 0.0;
    }

    pub fn process(&mut self) -> f32 {
        let mut sum = 0.0;
        for i in 0..6 {
            self.phases[i] += self.increments[i];
            if self.phases[i] >= 1.0 { self.phases[i] -= 1.0; }
            sum += if self.phases[i] < 0.5 { 1.0 } else { -1.0 };
        }
        let metal = sum / 6.0;

        // Bell ping
        self.bell_phase += self.bell_inc;
        if self.bell_phase >= 1.0 { self.bell_phase -= 1.0; }
        let bell = (self.bell_phase * 2.0 * PI).sin() * self.bell_env.process();

        let hp = self.hpf.process_hp(metal);
        let crushed = self.crush.process(hp);
        let env = self.amp_env.process();
        (crushed * env + bell * 0.4) * self.level
    }
}
