/// Convert MIDI note number to frequency in Hz.
/// A4 (note 69) = 440 Hz.
pub fn note_to_hz(note: u8) -> f32 {
    440.0 * 2.0f32.powf((note as f32 - 69.0) / 12.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a440() {
        let hz = note_to_hz(69);
        assert!((hz - 440.0).abs() < 0.01, "A4 should be 440Hz, got {hz}");
    }

    #[test]
    fn test_middle_c() {
        let hz = note_to_hz(60);
        assert!((hz - 261.63).abs() < 0.1, "C4 should be ~261.63Hz, got {hz}");
    }

    #[test]
    fn test_octave_relationship() {
        let a3 = note_to_hz(57);
        let a4 = note_to_hz(69);
        assert!((a4 / a3 - 2.0).abs() < 0.01, "Octave should be 2:1 ratio");
    }
}
