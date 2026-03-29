use dsp_common::{NoiseGenerator, ad_envelope::ADEnvelope, svfilter::SVFilter};

/// TR-909 Hand Clap — noise bursts through BPF(1100Hz). More "crack" than 808.
pub struct HandClap909 {
    noise: NoiseGenerator,
    bpf: SVFilter,
    reverb_env: ADEnvelope,
    burst_counter: u32,
    burst_index: u8,
    burst_on: bool,
    samples_per_burst: u32,
    samples_per_gap: u32,
    pub level: f32,
}

impl HandClap909 {
    pub fn new(sample_rate: f32) -> Self {
        let mut bpf = SVFilter::new(sample_rate);
        bpf.set_freq(1100.0); // slightly higher than 808's 1000Hz
        bpf.set_q(3.5);

        let mut reverb_env = ADEnvelope::new(sample_rate);
        reverb_env.set_attack(0.001);
        reverb_env.set_decay(0.12);

        Self {
            noise: NoiseGenerator::new(1909), bpf, reverb_env,
            burst_counter: 0, burst_index: 4, burst_on: false,
            samples_per_burst: (sample_rate * 0.008) as u32,  // 8ms bursts (shorter than 808)
            samples_per_gap: (sample_rate * 0.018) as u32,    // 18ms spacing (wider than 808)
            level: 0.7,
        }
    }

    pub fn trigger(&mut self) {
        self.burst_index = 0;
        self.burst_counter = self.samples_per_burst;
        self.burst_on = true;
        self.reverb_env.trigger();
    }

    pub fn process(&mut self) -> f32 {
        let burst_amp = if self.burst_index < 4 {
            if self.burst_on {
                self.burst_counter = self.burst_counter.saturating_sub(1);
                if self.burst_counter == 0 { self.burst_on = false; self.burst_counter = self.samples_per_gap; }
                1.0 - self.burst_index as f32 * 0.2
            } else {
                self.burst_counter = self.burst_counter.saturating_sub(1);
                if self.burst_counter == 0 { self.burst_index += 1; self.burst_on = true; self.burst_counter = self.samples_per_burst; }
                0.0
            }
        } else { 0.0 };

        let reverb_amp = self.reverb_env.process();
        let env = burst_amp.max(reverb_amp * 0.5);
        let raw = self.noise.white();
        let filtered = self.bpf.process_bp(raw);
        filtered * env * self.level
    }
}
