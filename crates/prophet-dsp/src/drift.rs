use crate::noise::NoiseGenerator;

/// Per-voice analog drift generator.
/// Produces a slow random wandering signal (sub-Hz filtered noise)
/// to simulate analog component variation.
pub struct DriftGenerator {
    noise: NoiseGenerator,
    filter_state: f32,
    value: f32,
    amount: f32, // max drift in Hz
}

impl DriftGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            noise: NoiseGenerator::new(seed),
            filter_state: 0.0,
            value: 0.0,
            amount: 0.0,
        }
    }

    pub fn set_amount(&mut self, hz: f32) {
        self.amount = hz;
    }

    /// Process one sample. Returns drift offset in Hz.
    pub fn process(&mut self, sample_rate: f32) -> f32 {
        if self.amount == 0.0 {
            self.value = 0.0;
            return 0.0;
        }
        // Single-pole lowpass at ~0.5Hz for slow wandering
        let cutoff = 0.5;
        let coeff = (-2.0 * std::f32::consts::PI * cutoff / sample_rate).exp();
        let white = self.noise.white();
        self.filter_state = self.filter_state * coeff + white * (1.0 - coeff);
        self.value = self.filter_state * self.amount;
        self.value
    }

    pub fn value(&self) -> f32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drift_not_zero() {
        let mut drift = DriftGenerator::new(42);
        drift.set_amount(5.0);
        let mut has_nonzero = false;
        for _ in 0..44100 {
            let v = drift.process(44100.0);
            if v.abs() > 0.01 {
                has_nonzero = true;
                break;
            }
        }
        assert!(has_nonzero, "Drift should produce nonzero output");
    }

    #[test]
    fn test_drift_stays_in_range() {
        let mut drift = DriftGenerator::new(42);
        drift.set_amount(5.0);
        for _ in 0..441000 {
            let v = drift.process(44100.0);
            assert!(
                v.abs() < 20.0,
                "Drift {v} exceeds reasonable range for amount=5Hz"
            );
        }
    }

    #[test]
    fn test_drift_varies_between_instances() {
        let mut d1 = DriftGenerator::new(1);
        let mut d2 = DriftGenerator::new(2);
        d1.set_amount(5.0);
        d2.set_amount(5.0);

        // Run both for a while
        for _ in 0..44100 {
            d1.process(44100.0);
            d2.process(44100.0);
        }

        // They should have different values (different seeds)
        assert!(
            (d1.value() - d2.value()).abs() > 0.001,
            "Two drift generators with different seeds should diverge: {} vs {}",
            d1.value(), d2.value()
        );
    }

    #[test]
    fn test_drift_zero_amount_is_silent() {
        let mut drift = DriftGenerator::new(42);
        drift.set_amount(0.0);
        for _ in 0..44100 {
            let v = drift.process(44100.0);
            assert!(v.abs() < 0.0001, "Zero amount drift should be silent, got {v}");
        }
    }
}
