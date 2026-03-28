use dsp_common::{NoiseGenerator, ad_envelope::ADEnvelope, svfilter::SVFilter};

/// TR-808 Hand Clap — noise through BPF(1000Hz), 4-burst envelope + reverb tail.
pub struct HandClap {
    noise: NoiseGenerator,
    bpf: SVFilter,
    reverb_env: ADEnvelope,
    sample_rate: f32,
    // Burst state
    burst_counter: u32,
    burst_index: u8,       // which burst (0-3)
    burst_on: bool,
    samples_per_burst: u32,
    samples_per_gap: u32,
    pub level: f32,
}

impl HandClap {
    pub fn new(sample_rate: f32) -> Self {
        let mut bpf = SVFilter::new(sample_rate);
        bpf.set_freq(1000.0);
        bpf.set_q(3.0);

        let mut reverb_env = ADEnvelope::new(sample_rate);
        reverb_env.set_attack(0.001);
        reverb_env.set_decay(0.15); // 150ms reverb tail

        Self {
            noise: NoiseGenerator::new(909),
            bpf, reverb_env, sample_rate,
            burst_counter: 0, burst_index: 4, burst_on: false,
            samples_per_burst: (sample_rate * 0.01) as u32,  // 10ms bursts
            samples_per_gap: (sample_rate * 0.011) as u32,   // 11ms spacing
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
        // Burst envelope — 4 rapid noise bursts
        let burst_amp = if self.burst_index < 4 {
            if self.burst_on {
                self.burst_counter = self.burst_counter.saturating_sub(1);
                if self.burst_counter == 0 {
                    self.burst_on = false;
                    self.burst_counter = self.samples_per_gap;
                }
                // Diminishing amplitude per burst
                1.0 - self.burst_index as f32 * 0.15
            } else {
                self.burst_counter = self.burst_counter.saturating_sub(1);
                if self.burst_counter == 0 {
                    self.burst_index += 1;
                    self.burst_on = true;
                    self.burst_counter = self.samples_per_burst;
                }
                0.0
            }
        } else {
            0.0
        };

        // Reverb tail envelope (smooth exponential decay after bursts)
        let reverb_amp = self.reverb_env.process();

        // Combine: bursts + reverb tail
        let env = burst_amp.max(reverb_amp * 0.6);

        // Noise through bandpass
        let raw = self.noise.white();
        let filtered = self.bpf.process_bp(raw);

        filtered * env * self.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cp_produces_sound() {
        let mut cp = HandClap::new(44100.0);
        cp.trigger();
        let buf: Vec<f32> = (0..4410).map(|_| cp.process()).collect();
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
    }

    #[test]
    fn test_cp_has_bursts() {
        let mut cp = HandClap::new(44100.0);
        cp.trigger();
        // Render 100ms in 10ms chunks, check for amplitude variation (bursts)
        let mut chunk_rms = Vec::new();
        for _ in 0..10 {
            let chunk: Vec<f32> = (0..441).map(|_| cp.process()).collect();
            chunk_rms.push(audio_test_harness::level::rms(&chunk));
        }
        // Should have variation (bursts create peaks and gaps)
        let max = chunk_rms.iter().cloned().fold(0.0f32, f32::max);
        let min = chunk_rms.iter().cloned().fold(f32::MAX, f32::min);
        assert!(max > min * 2.0, "Clap should have burst pattern, max={max:.4} min={min:.4}");
    }
}
