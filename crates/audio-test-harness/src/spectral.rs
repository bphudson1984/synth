use rustfft::{FftPlanner, num_complex::Complex};

/// Compute the magnitude spectrum of a signal buffer.
/// Returns magnitudes for bins 0..N/2 (DC to Nyquist).
pub fn magnitude_spectrum(samples: &[f32]) -> Vec<f32> {
    let n = samples.len();
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(n);

    let mut buffer: Vec<Complex<f32>> = samples
        .iter()
        .map(|&s| Complex { re: s, im: 0.0 })
        .collect();

    fft.process(&mut buffer);

    // Return magnitudes for bins 0..N/2
    buffer[..n / 2]
        .iter()
        .map(|c| c.norm())
        .collect()
}

/// Returns the frequency (Hz) and magnitude (linear) of the top `count` peaks
/// in the magnitude spectrum, sorted by magnitude descending.
pub fn find_peaks(spectrum: &[f32], sample_rate: f32, count: usize) -> Vec<(f32, f32)> {
    let bin_resolution = sample_rate / (spectrum.len() * 2) as f32;

    // Find local maxima (bins higher than both neighbors)
    let mut peaks: Vec<(f32, f32)> = Vec::new();
    for i in 1..spectrum.len() - 1 {
        if spectrum[i] > spectrum[i - 1] && spectrum[i] > spectrum[i + 1] {
            let freq = i as f32 * bin_resolution;
            peaks.push((freq, spectrum[i]));
        }
    }

    // Sort by magnitude descending
    peaks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    peaks.truncate(count);
    peaks
}

/// Assert that a signal's harmonics follow an expected rolloff pattern.
/// `expected_rolloff` is a function from harmonic number (1-based) to expected
/// relative amplitude. For sawtooth: |1/n|. For square: |1/n| odd only, 0 even.
pub fn assert_harmonic_series<F>(
    samples: &[f32],
    sample_rate: f32,
    fundamental_hz: f32,
    num_harmonics: usize,
    expected_rolloff: F,
    tolerance_db: f32,
) where
    F: Fn(usize) -> f32,
{
    let spectrum = magnitude_spectrum(samples);
    let bin_resolution = sample_rate / samples.len() as f32;
    let nyquist = sample_rate / 2.0;

    // Get magnitude of fundamental (harmonic 1)
    let fund_bin = (fundamental_hz / bin_resolution).round() as usize;
    let fund_mag = spectrum[fund_bin];
    assert!(fund_mag > 0.0, "Fundamental has zero magnitude");

    for n in 2..=num_harmonics {
        let harmonic_hz = fundamental_hz * n as f32;
        if harmonic_hz >= nyquist {
            break;
        }

        let expected_rel = expected_rolloff(n);
        let bin = (harmonic_hz / bin_resolution).round() as usize;
        if bin >= spectrum.len() {
            break;
        }
        let actual_rel = spectrum[bin] / fund_mag;

        if expected_rel < 0.001 {
            // Expect this harmonic to be absent — check it's at least 30dB down
            let actual_db = if actual_rel > 0.0 { 20.0 * actual_rel.log10() } else { -100.0 };
            assert!(
                actual_db < -20.0,
                "Harmonic {n} at {harmonic_hz}Hz should be absent, but is {actual_db:.1}dB relative to fundamental"
            );
        } else {
            let expected_db = 20.0 * expected_rel.log10();
            let actual_db = if actual_rel > 0.0 { 20.0 * actual_rel.log10() } else { -100.0 };
            assert!(
                (actual_db - expected_db).abs() < tolerance_db,
                "Harmonic {n} at {harmonic_hz}Hz: expected {expected_db:.1}dB, got {actual_db:.1}dB (tolerance: {tolerance_db}dB)"
            );
        }
    }
}

/// Assert that the spectral slope between two frequencies matches expected dB/octave.
pub fn assert_spectral_slope(
    samples: &[f32],
    sample_rate: f32,
    freq_low: f32,
    freq_high: f32,
    expected_db_per_oct: f32,
    tolerance_db: f32,
) {
    let spectrum = magnitude_spectrum(samples);
    let bin_resolution = sample_rate / samples.len() as f32;

    let bin_low = (freq_low / bin_resolution).round() as usize;
    let bin_high = (freq_high / bin_resolution).round() as usize;

    // Average magnitude in small bands around low and high frequencies
    let avg_low = average_magnitude(&spectrum, bin_low, 5);
    let avg_high = average_magnitude(&spectrum, bin_high, 5);

    if avg_low <= 0.0 || avg_high <= 0.0 {
        panic!("Zero magnitude in measurement band");
    }

    let db_diff = 20.0 * (avg_high / avg_low).log10();
    let octaves = (freq_high / freq_low).log2();
    let measured_slope = db_diff / octaves;

    assert!(
        (measured_slope - expected_db_per_oct).abs() < tolerance_db,
        "Spectral slope: expected {expected_db_per_oct}dB/oct, got {measured_slope:.1}dB/oct"
    );
}

fn average_magnitude(spectrum: &[f32], center_bin: usize, half_width: usize) -> f32 {
    let start = center_bin.saturating_sub(half_width);
    let end = (center_bin + half_width + 1).min(spectrum.len());
    let sum: f32 = spectrum[start..end].iter().sum();
    sum / (end - start) as f32
}

/// Assert no significant energy above Nyquist/2 (aliasing check).
/// Compares energy in alias-prone bins against the fundamental.
pub fn assert_no_aliasing(
    samples: &[f32],
    sample_rate: f32,
    fundamental_hz: f32,
    max_aliasing_db: f32,
) {
    let spectrum = magnitude_spectrum(samples);
    let bin_resolution = sample_rate / samples.len() as f32;
    let nyquist = sample_rate / 2.0;

    // Find the fundamental magnitude
    let fund_bin = (fundamental_hz / bin_resolution).round() as usize;
    let fund_mag = spectrum[fund_bin];
    assert!(fund_mag > 0.0, "Fundamental has zero magnitude");

    // Compute expected harmonic positions below Nyquist
    let mut harmonic_bins = std::collections::HashSet::new();
    let mut n = 1;
    loop {
        let freq = fundamental_hz * n as f32;
        if freq >= nyquist {
            break;
        }
        let bin = (freq / bin_resolution).round() as usize;
        // Mark this bin and neighbors as legitimate harmonics
        for b in bin.saturating_sub(2)..=(bin + 2).min(spectrum.len() - 1) {
            harmonic_bins.insert(b);
        }
        n += 1;
    }

    // Check all non-harmonic bins for aliased energy
    let mut max_alias_mag = 0.0f32;
    for (bin, &mag) in spectrum.iter().enumerate() {
        if bin < 5 { continue; } // skip DC region
        if !harmonic_bins.contains(&bin) {
            max_alias_mag = max_alias_mag.max(mag);
        }
    }

    let alias_db = if max_alias_mag > 0.0 {
        20.0 * (max_alias_mag / fund_mag).log10()
    } else {
        -120.0
    };

    assert!(
        alias_db < max_aliasing_db,
        "Aliasing detected: {alias_db:.1}dB relative to fundamental (threshold: {max_aliasing_db}dB)"
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
    fn test_magnitude_spectrum_sine_peak() {
        // A 440Hz sine wave should have a single dominant peak at 440Hz
        let sample_rate = 44100.0;
        let buf = generate_sine(440.0, sample_rate, 1.0);
        let spectrum = magnitude_spectrum(&buf);

        // Find the bin closest to 440Hz
        let bin_resolution = sample_rate / buf.len() as f32;
        let expected_bin = (440.0 / bin_resolution).round() as usize;

        // The peak bin should have the highest magnitude
        let peak_bin = spectrum
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;

        assert_eq!(peak_bin, expected_bin, "Peak should be at 440Hz bin");

        // Peak magnitude should be approximately N/2 for a unit-amplitude sine
        let expected_magnitude = buf.len() as f32 / 2.0;
        let ratio = spectrum[peak_bin] / expected_magnitude;
        assert!(
            (ratio - 1.0).abs() < 0.01,
            "Peak magnitude ratio: {ratio}, expected ~1.0"
        );
    }

    #[test]
    fn test_magnitude_spectrum_two_sines() {
        // Two sines at 440Hz and 880Hz — should have two peaks
        let sample_rate = 44100.0;
        let n = (sample_rate * 1.0) as usize;
        let buf: Vec<f32> = (0..n)
            .map(|i| {
                let t = i as f32 / sample_rate;
                (2.0 * PI * 440.0 * t).sin() + 0.5 * (2.0 * PI * 880.0 * t).sin()
            })
            .collect();

        let spectrum = magnitude_spectrum(&buf);
        let peaks = find_peaks(&spectrum, sample_rate, 2);

        // First peak should be near 440Hz (louder)
        assert!((peaks[0].0 - 440.0).abs() < 2.0, "First peak at {}", peaks[0].0);
        // Second peak should be near 880Hz
        assert!((peaks[1].0 - 880.0).abs() < 2.0, "Second peak at {}", peaks[1].0);
        // Second peak should be roughly half the magnitude of the first
        let ratio = peaks[1].1 / peaks[0].1;
        assert!(
            (ratio - 0.5).abs() < 0.05,
            "Amplitude ratio: {ratio}, expected ~0.5"
        );
    }
}
