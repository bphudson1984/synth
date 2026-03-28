use hound::{WavReader, WavSpec, WavWriter, SampleFormat};
use std::path::Path;

/// Write f32 samples to a WAV file (mono, 32-bit float).
pub fn write(path: &Path, samples: &[f32], sample_rate: u32) -> Result<(), hound::Error> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };
    let mut writer = WavWriter::create(path, spec)?;
    for &s in samples {
        writer.write_sample(s)?;
    }
    writer.finalize()?;
    Ok(())
}

/// Read a WAV file into f32 samples.
pub fn read(path: &Path) -> Result<(Vec<f32>, u32), hound::Error> {
    let mut reader = WavReader::open(path)?;
    let spec = reader.spec();
    let sample_rate = spec.sample_rate;
    let samples: Vec<f32> = match spec.sample_format {
        SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap()).collect(),
        SampleFormat::Int => {
            let max_val = (1u32 << (spec.bits_per_sample - 1)) as f32;
            reader.samples::<i32>().map(|s| s.unwrap() as f32 / max_val).collect()
        }
    };
    Ok((samples, sample_rate))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_write_read_roundtrip() {
        let sample_rate = 44100u32;
        let samples: Vec<f32> = (0..4410)
            .map(|i| (2.0 * PI * 440.0 * i as f32 / sample_rate as f32).sin())
            .collect();

        let path = Path::new("test-artifacts/roundtrip_test.wav");
        write(path, &samples, sample_rate).unwrap();

        let (read_samples, read_sr) = read(path).unwrap();
        assert_eq!(read_sr, sample_rate);
        assert_eq!(read_samples.len(), samples.len());

        // Samples should be identical (f32 WAV is lossless for f32)
        for (a, b) in samples.iter().zip(read_samples.iter()) {
            assert!((a - b).abs() < 1e-6, "Sample mismatch: {a} vs {b}");
        }

        // Cleanup
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_write_creates_file() {
        let path = Path::new("test-artifacts/creation_test.wav");
        let samples = vec![0.0f32; 100];
        write(path, &samples, 44100).unwrap();
        assert!(path.exists());
        std::fs::remove_file(path).ok();
    }
}
