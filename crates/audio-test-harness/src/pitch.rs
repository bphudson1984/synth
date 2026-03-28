/// Detect the fundamental frequency of a signal using autocorrelation.
/// Returns the detected frequency in Hz, or None if no clear pitch found.
///
/// Uses the approach of finding the first strong peak in the autocorrelation
/// function, which corresponds to the fundamental period.
pub fn detect(samples: &[f32], sample_rate: f32) -> Option<f32> {
    let n = samples.len();
    if n < 2 {
        return None;
    }

    // Search range: 20Hz to 5000Hz
    let min_lag = (sample_rate / 5000.0).ceil() as usize;
    let max_lag = ((sample_rate / 20.0) as usize).min(n / 2);

    if min_lag >= max_lag || max_lag >= n {
        return None;
    }

    // Compute autocorrelation for each lag, normalized by energy
    let mut corrs = Vec::with_capacity(max_lag + 1);
    for lag in 0..=max_lag {
        let mut sum = 0.0f32;
        let mut ea = 0.0f32;
        let mut eb = 0.0f32;
        for i in 0..n - lag {
            sum += samples[i] * samples[i + lag];
            ea += samples[i] * samples[i];
            eb += samples[i + lag] * samples[i + lag];
        }
        let energy = (ea * eb).sqrt();
        let c = if energy > 0.0 { sum / energy } else { 0.0 };
        corrs.push(c);
    }

    // Find the first local maximum in [min_lag, max_lag] where correlation > 0.5
    // A local max is where corrs[lag] > corrs[lag-1] and corrs[lag] >= corrs[lag+1]
    for lag in (min_lag + 1)..max_lag {
        if corrs[lag] > corrs[lag - 1] && corrs[lag] >= corrs[lag + 1] && corrs[lag] > 0.5 {
            // Parabolic interpolation for sub-sample accuracy
            let y_m = corrs[lag - 1];
            let y_0 = corrs[lag];
            let y_p = corrs[lag + 1];
            let denom = y_m - 2.0 * y_0 + y_p;
            if denom.abs() > 1e-10 {
                let delta = 0.5 * (y_m - y_p) / denom;
                let refined = lag as f32 + delta;
                if refined > 0.0 {
                    return Some(sample_rate / refined);
                }
            }
            return Some(sample_rate / lag as f32);
        }
    }

    None
}

/// Assert that the detected pitch matches expected frequency within a tolerance in cents.
pub fn assert_pitch(samples: &[f32], sample_rate: f32, expected_hz: f32, tolerance_cents: f32) {
    let detected = detect(samples, sample_rate)
        .unwrap_or_else(|| panic!("No pitch detected, expected {expected_hz}Hz"));
    let cents_off = 1200.0 * (detected / expected_hz).log2();
    assert!(
        cents_off.abs() < tolerance_cents,
        "Pitch {detected:.2}Hz is {cents_off:.1} cents from expected {expected_hz}Hz (tolerance: {tolerance_cents} cents)"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    fn generate_sine(freq_hz: f32, sample_rate: f32, duration_secs: f32) -> Vec<f32> {
        let num_samples = (sample_rate * duration_secs) as usize;
        (0..num_samples)
            .map(|i| (2.0 * PI * freq_hz * i as f32 / sample_rate).sin())
            .collect()
    }

    #[test]
    fn test_detect_a440() {
        let buf = generate_sine(440.0, 44100.0, 0.5);
        assert_pitch(&buf, 44100.0, 440.0, 1.0); // within 1 cent
    }

    #[test]
    fn test_detect_c4() {
        // Middle C = 261.63 Hz
        let buf = generate_sine(261.63, 44100.0, 0.5);
        assert_pitch(&buf, 44100.0, 261.63, 1.0);
    }

    #[test]
    fn test_detect_high_frequency() {
        // 2kHz
        let buf = generate_sine(2000.0, 44100.0, 0.5);
        assert_pitch(&buf, 44100.0, 2000.0, 2.0);
    }

    #[test]
    fn test_detect_low_frequency() {
        // 55Hz (A1)
        let buf = generate_sine(55.0, 44100.0, 1.0);
        assert_pitch(&buf, 44100.0, 55.0, 2.0);
    }

    #[test]
    fn test_detect_sawtooth_fundamental() {
        // A sawtooth wave should detect at the fundamental, not a harmonic
        let sample_rate = 44100.0;
        let freq = 440.0;
        let n = (sample_rate * 0.5) as usize;
        let buf: Vec<f32> = (0..n)
            .map(|i| {
                let phase = (i as f32 * freq / sample_rate).fract();
                phase * 2.0 - 1.0 // naive saw
            })
            .collect();
        assert_pitch(&buf, sample_rate, freq, 2.0);
    }
}
