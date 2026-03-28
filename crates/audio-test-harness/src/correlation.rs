/// Compute normalized cross-correlation between two equal-length buffers.
/// Returns a value from -1.0 (perfectly anti-correlated) to 1.0 (identical).
pub fn cross_correlation(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "Buffers must be equal length");
    let n = a.len();
    if n == 0 {
        return 0.0;
    }

    let mut sum_ab = 0.0f32;
    let mut sum_aa = 0.0f32;
    let mut sum_bb = 0.0f32;

    for i in 0..n {
        sum_ab += a[i] * b[i];
        sum_aa += a[i] * a[i];
        sum_bb += b[i] * b[i];
    }

    let denom = (sum_aa * sum_bb).sqrt();
    if denom > 0.0 {
        sum_ab / denom
    } else {
        0.0
    }
}

/// Assert two buffers are similar (correlation above threshold).
pub fn assert_similar(a: &[f32], b: &[f32], min_correlation: f32) {
    let corr = cross_correlation(a, b);
    assert!(
        corr >= min_correlation,
        "Correlation {corr:.4} below threshold {min_correlation:.4}"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_identical_signals() {
        let buf: Vec<f32> = (0..4410)
            .map(|i| (2.0 * PI * 440.0 * i as f32 / 44100.0).sin())
            .collect();
        let corr = cross_correlation(&buf, &buf);
        assert!((corr - 1.0).abs() < 0.001, "Self-correlation: {corr}");
    }

    #[test]
    fn test_uncorrelated_signals() {
        let a: Vec<f32> = (0..44100)
            .map(|i| (2.0 * PI * 440.0 * i as f32 / 44100.0).sin())
            .collect();
        let b: Vec<f32> = (0..44100)
            .map(|i| (2.0 * PI * 1000.0 * i as f32 / 44100.0).sin())
            .collect();
        let corr = cross_correlation(&a, &b);
        assert!(corr.abs() < 0.1, "Uncorrelated signals should be near 0, got {corr}");
    }

    #[test]
    fn test_inverted_signal() {
        let a: Vec<f32> = (0..4410)
            .map(|i| (2.0 * PI * 440.0 * i as f32 / 44100.0).sin())
            .collect();
        let b: Vec<f32> = a.iter().map(|s| -s).collect();
        let corr = cross_correlation(&a, &b);
        assert!((corr + 1.0).abs() < 0.001, "Inverted correlation: {corr}");
    }
}
