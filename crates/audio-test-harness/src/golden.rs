use std::path::Path;
use crate::{wav, level};

/// Compare a rendered buffer against a golden WAV file.
/// If the golden file doesn't exist, create it (first run).
/// Returns Ok if similar within tolerance, Err with details otherwise.
pub fn compare_or_create(
    golden_path: &Path,
    samples: &[f32],
    sample_rate: u32,
    max_rms_diff: f32,
) -> Result<(), String> {
    if !golden_path.exists() {
        // First run: create the golden file
        wav::write(golden_path, samples, sample_rate)
            .map_err(|e| format!("Failed to write golden file: {e}"))?;
        return Ok(());
    }

    // Read existing golden file
    let (golden_samples, golden_sr) = wav::read(golden_path)
        .map_err(|e| format!("Failed to read golden file: {e}"))?;

    if golden_sr != sample_rate {
        return Err(format!(
            "Sample rate mismatch: golden={golden_sr}, rendered={sample_rate}"
        ));
    }

    if golden_samples.len() != samples.len() {
        return Err(format!(
            "Length mismatch: golden={}, rendered={}",
            golden_samples.len(),
            samples.len()
        ));
    }

    // Compute RMS of the difference signal
    let diff: Vec<f32> = golden_samples
        .iter()
        .zip(samples.iter())
        .map(|(g, r)| g - r)
        .collect();
    let rms_diff = level::rms(&diff);

    if rms_diff > max_rms_diff {
        Err(format!(
            "Golden file mismatch: RMS difference {rms_diff:.6} exceeds threshold {max_rms_diff:.6}"
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    fn sine_buf(freq: f32) -> Vec<f32> {
        (0..4410)
            .map(|i| (2.0 * PI * freq * i as f32 / 44100.0).sin())
            .collect()
    }

    #[test]
    fn test_golden_creates_on_first_run() {
        let path = Path::new("test-artifacts/golden_create_test.wav");
        // Ensure clean state
        std::fs::remove_file(path).ok();

        let buf = sine_buf(440.0);
        let result = compare_or_create(path, &buf, 44100, 0.01);
        assert!(result.is_ok(), "First run should create golden file: {result:?}");
        assert!(path.exists(), "Golden file should exist after creation");

        // Cleanup
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_golden_matches_identical() {
        let path = Path::new("test-artifacts/golden_match_test.wav");
        std::fs::remove_file(path).ok();

        let buf = sine_buf(440.0);
        // First run: create
        compare_or_create(path, &buf, 44100, 0.01).unwrap();
        // Second run: compare — should match
        let result = compare_or_create(path, &buf, 44100, 0.01);
        assert!(result.is_ok(), "Identical buffer should match: {result:?}");

        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_golden_rejects_different() {
        let path = Path::new("test-artifacts/golden_reject_test.wav");
        std::fs::remove_file(path).ok();

        let buf_a = sine_buf(440.0);
        let buf_b = sine_buf(880.0);
        // Create with buf_a
        compare_or_create(path, &buf_a, 44100, 0.01).unwrap();
        // Compare with buf_b — should fail
        let result = compare_or_create(path, &buf_b, 44100, 0.01);
        assert!(result.is_err(), "Different buffer should be rejected");

        std::fs::remove_file(path).ok();
    }
}
