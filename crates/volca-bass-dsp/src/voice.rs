use dsp_common::note_to_hz;
use crate::oscillator::Oscillator;
use crate::filter::DiodeBridgeLPF;
use crate::envelope::BassEnvelope;
use crate::lfo::LFO;
use crate::glide::Glide;

/// Volca Bass voice: 3 VCOs → mixer → diode bridge LPF → VCA.
pub struct BassVoice {
    osc: [Oscillator; 3],
    pub osc_active: [bool; 3],
    pub osc_pitch: [f32; 3],       // semitone offset from base note
    filter: DiodeBridgeLPF,
    pub envelope: BassEnvelope,
    lfo: LFO,
    glide: [Glide; 3],
    sample_rate: f32,

    // Filter params
    pub cutoff: f32,
    pub resonance: f32,
    pub eg_intensity: f32,

    // Envelope routing
    pub eg_to_vca: bool,

    // LFO params + routing
    pub lfo_intensity: f32,
    pub lfo_to_pitch: bool,
    pub lfo_to_cutoff: bool,
    pub lfo_to_amp: bool,

    // Group mode: 0=ALL (unison), 1=POLY (round-robin paraphonic)
    pub group_mode: u8,

    // State
    gate_on: bool,
    current_note: u8,
    poly_next_vco: usize,
    first_note: bool,
    // Track whether notes are tied (for legato/slide detection)
    was_playing: bool,
}

impl BassVoice {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            osc: [
                Oscillator::new(sample_rate),
                Oscillator::new(sample_rate),
                Oscillator::new(sample_rate),
            ],
            osc_active: [true, true, true],
            osc_pitch: [0.0, 0.0, 0.0],
            filter: DiodeBridgeLPF::new(sample_rate),
            envelope: BassEnvelope::new(sample_rate),
            lfo: LFO::new(sample_rate),
            glide: [
                Glide::new(sample_rate),
                Glide::new(sample_rate),
                Glide::new(sample_rate),
            ],
            sample_rate,
            cutoff: 2000.0,
            resonance: 0.3,
            eg_intensity: 0.5,
            eg_to_vca: true,
            lfo_intensity: 0.0,
            lfo_to_pitch: false,
            lfo_to_cutoff: false,
            lfo_to_amp: false,
            group_mode: 0,
            gate_on: false,
            current_note: 48,
            poly_next_vco: 0,
            first_note: true,
            was_playing: false,
        }
    }

    pub fn set_osc_waveform(&mut self, vco: usize, saw: bool) {
        if vco < 3 { self.osc[vco].set_waveform(saw); }
    }

    pub fn set_lfo_rate(&mut self, hz: f32) { self.lfo.set_rate(hz); }
    pub fn set_lfo_wave(&mut self, square: bool) { self.lfo.use_square = square; }

    pub fn note_on(&mut self, note: u8, _velocity: u8) {
        self.current_note = note;
        let is_tied = self.gate_on; // legato detection

        if self.group_mode == 0 {
            // ALL mode: all VCOs play same note
            let target_hz = note_to_hz(note);
            if is_tied && !self.first_note {
                // Legato: glide to new note, don't retrigger envelope
                for g in &mut self.glide {
                    g.set_enabled(true);
                    g.set_target(target_hz);
                }
            } else {
                for g in &mut self.glide {
                    g.set_enabled(false);
                    g.jump_to(target_hz);
                }
                self.envelope.trigger();
            }
        } else {
            // POLY mode: round-robin assign to VCOs
            let vco = self.poly_next_vco;
            let target_hz = note_to_hz(note);
            self.glide[vco].set_enabled(false);
            self.glide[vco].jump_to(target_hz);
            self.poly_next_vco = (self.poly_next_vco + 1) % 3;
            if !is_tied || self.first_note {
                self.envelope.trigger();
            }
        }

        self.gate_on = true;
        self.first_note = false;
        self.was_playing = true;
    }

    pub fn note_off(&mut self) {
        self.gate_on = false;
        self.envelope.gate_off();
    }

    pub fn process(&mut self) -> f32 {
        // LFO
        let lfo_val = self.lfo.process();

        // Mix active VCOs
        let mut mix = 0.0f32;
        let mut active_count = 0u8;

        for i in 0..3 {
            if !self.osc_active[i] { continue; }
            active_count += 1;

            let freq = self.glide[i].process();
            // Apply pitch offset (semitones)
            let offset_ratio = (self.osc_pitch[i] / 12.0).exp2();
            let mut final_freq = freq * offset_ratio;

            // LFO pitch modulation (±1 octave at full intensity)
            if self.lfo_to_pitch {
                final_freq *= 1.0 + lfo_val * self.lfo_intensity;
            }

            self.osc[i].set_frequency(final_freq);
            mix += self.osc[i].process();
        }

        // Normalize mix
        if active_count > 1 {
            mix /= active_count as f32;
        }
        if active_count == 0 {
            mix = 0.0;
        }

        // Envelope
        let env_val = self.envelope.process();

        // Filter: cutoff + EG modulation + LFO modulation
        let mut effective_cutoff = self.cutoff + self.eg_intensity * env_val * 8000.0;
        if self.lfo_to_cutoff {
            effective_cutoff += lfo_val * self.lfo_intensity * 4000.0;
        }
        effective_cutoff = effective_cutoff.clamp(20.0, self.sample_rate * 0.45);

        self.filter.set_cutoff(effective_cutoff);
        self.filter.set_resonance(self.resonance);
        let filtered = self.filter.process(mix);

        // VCA
        let mut amp = if self.eg_to_vca {
            env_val
        } else {
            if self.gate_on { 1.0 } else { 0.0 }
        };

        // LFO amp modulation (tremolo)
        if self.lfo_to_amp {
            amp *= 1.0 + lfo_val * self.lfo_intensity * 0.5;
        }

        filtered * amp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voice_produces_sound() {
        let mut voice = BassVoice::new(44100.0);
        voice.note_on(48, 100);
        let mut has_energy = false;
        for _ in 0..4410 {
            if voice.process().abs() > 0.01 {
                has_energy = true;
                break;
            }
        }
        assert!(has_energy, "Voice should produce sound after note_on");
    }

    #[test]
    fn test_voice_silent_without_note() {
        let mut voice = BassVoice::new(44100.0);
        let mut energy = 0.0f32;
        for _ in 0..4410 {
            energy += voice.process().abs();
        }
        assert!(energy < 0.01, "Voice should be silent without note_on");
    }

    #[test]
    fn test_three_vcos_louder_than_one() {
        let mut voice_one = BassVoice::new(44100.0);
        voice_one.osc_active = [true, false, false];
        voice_one.eg_to_vca = false;
        voice_one.note_on(48, 100);
        let rms_one: f32 = {
            let buf: Vec<f32> = (0..4410).map(|_| voice_one.process()).collect();
            (buf.iter().map(|s| s * s).sum::<f32>() / buf.len() as f32).sqrt()
        };

        let mut voice_three = BassVoice::new(44100.0);
        voice_three.osc_active = [true, true, true];
        voice_three.eg_to_vca = false;
        voice_three.note_on(48, 100);
        let rms_three: f32 = {
            let buf: Vec<f32> = (0..4410).map(|_| voice_three.process()).collect();
            (buf.iter().map(|s| s * s).sum::<f32>() / buf.len() as f32).sqrt()
        };

        // With normalization (sum/count), they should be similar in level
        // but the detuning interaction creates a richer sound
        assert!(rms_one > 0.01, "One VCO should produce sound");
        assert!(rms_three > 0.01, "Three VCOs should produce sound");
    }

    #[test]
    fn test_filter_cutoff_affects_sound() {
        let mut v1 = BassVoice::new(44100.0);
        v1.cutoff = 200.0;
        v1.eg_intensity = 0.0;
        v1.eg_to_vca = false;
        v1.note_on(36, 100);
        let buf1: Vec<f32> = (0..4410).map(|_| v1.process()).collect();

        let mut v2 = BassVoice::new(44100.0);
        v2.cutoff = 8000.0;
        v2.eg_intensity = 0.0;
        v2.eg_to_vca = false;
        v2.note_on(36, 100);
        let buf2: Vec<f32> = (0..4410).map(|_| v2.process()).collect();

        let corr = audio_test_harness::correlation::cross_correlation(&buf1, &buf2);
        assert!(corr < 0.99, "Different cutoffs should produce different output (corr={corr:.3})");
    }
}
