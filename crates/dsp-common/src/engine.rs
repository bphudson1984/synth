/// Common interfaces for all sound engines.
///
/// These traits enforce a consistent contract across Prophet, Braids, TB303,
/// and TR808/909 engines. They can't cross the WASM boundary directly, but
/// they ensure each DSP engine exposes the same core API.

/// Base trait for all sound engines.
pub trait SynthEngine {
    /// Process one sample of audio output.
    fn process(&mut self) -> f32;

    /// Set a parameter by numeric ID. Each engine defines its own ID mapping.
    fn set_param(&mut self, id: u32, value: f32);

    /// Set master volume (0.0 - 1.0).
    fn set_master_volume(&mut self, vol: f32);

    /// Get current master volume.
    fn master_volume(&self) -> f32;
}

/// Engines that respond to MIDI-style note events (synths).
pub trait MelodicEngine: SynthEngine {
    fn note_on(&mut self, note: u8, velocity: u8);
    fn note_off(&mut self, note: u8);
}

/// Engines that respond to percussion trigger events (drum machines).
pub trait TriggerEngine: SynthEngine {
    fn trigger(&mut self, voice: u8);
}
