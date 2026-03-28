/// Compute RMS (root mean square) of a signal buffer.
pub fn rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let sum_sq: f32 = samples.iter().map(|s| s * s).sum();
    (sum_sq / samples.len() as f32).sqrt()
}

/// Compute peak absolute value of a signal buffer.
pub fn peak(samples: &[f32]) -> f32 {
    samples
        .iter()
        .map(|s| s.abs())
        .fold(0.0f32, f32::max)
}

/// Assert RMS is within a range.
pub fn assert_rms_in_range(samples: &[f32], min: f32, max: f32) {
    let r = rms(samples);
    assert!(
        r >= min && r <= max,
        "RMS {r:.6} outside range [{min:.6}, {max:.6}]"
    );
}

/// Assert the signal is effectively silent (RMS below threshold).
pub fn assert_silent(samples: &[f32], threshold: f32) {
    let r = rms(samples);
    assert!(r < threshold, "Expected silence, got RMS {r:.6} (threshold: {threshold:.6})");
}

/// Assert the signal is not silent (RMS above threshold).
pub fn assert_not_silent(samples: &[f32], threshold: f32) {
    let r = rms(samples);
    assert!(r >= threshold, "Expected sound, got RMS {r:.6} (threshold: {threshold:.6})");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_rms_sine() {
        // RMS of a unit-amplitude sine wave = 1/sqrt(2) ≈ 0.7071
        let sample_rate = 44100.0;
        let n = (sample_rate * 1.0) as usize;
        let buf: Vec<f32> = (0..n)
            .map(|i| (2.0 * PI * 440.0 * i as f32 / sample_rate).sin())
            .collect();
        let r = rms(&buf);
        let expected = 1.0 / 2.0_f32.sqrt();
        assert!((r - expected).abs() < 0.001, "RMS {r}, expected {expected}");
    }

    #[test]
    fn test_rms_silence() {
        let buf = vec![0.0f32; 44100];
        assert_eq!(rms(&buf), 0.0);
    }

    #[test]
    fn test_rms_dc() {
        // RMS of constant 0.5 = 0.5
        let buf = vec![0.5f32; 44100];
        let r = rms(&buf);
        assert!((r - 0.5).abs() < 0.001, "RMS {r}, expected 0.5");
    }

    #[test]
    fn test_peak_sine() {
        let sample_rate = 44100.0;
        let n = (sample_rate * 1.0) as usize;
        let buf: Vec<f32> = (0..n)
            .map(|i| (2.0 * PI * 440.0 * i as f32 / sample_rate).sin())
            .collect();
        let p = peak(&buf);
        assert!((p - 1.0).abs() < 0.001, "Peak {p}, expected ~1.0");
    }

    #[test]
    fn test_assert_silent_passes() {
        let buf = vec![0.0f32; 1000];
        assert_silent(&buf, 0.001);
    }

    #[test]
    fn test_assert_not_silent_passes() {
        let buf: Vec<f32> = (0..44100)
            .map(|i| (2.0 * PI * 440.0 * i as f32 / 44100.0).sin())
            .collect();
        assert_not_silent(&buf, 0.1);
    }
}
