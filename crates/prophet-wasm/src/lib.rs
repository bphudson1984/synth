//! Thin C-ABI wrapper around prophet-dsp for WebAssembly.
//! Exposes functions callable from JavaScript AudioWorkletProcessor.

use prophet_dsp::synth::ProphetSynth;
use prophet_dsp::effects::chorus::StereoChorus;
use prophet_dsp::effects::delay::TapeDelay;
use prophet_dsp::effects::reverb::PlateReverb;
use prophet_dsp::arpeggiator::{Arpeggiator, ArpMode, ArpDivision};

// Static mutable globals — safe in single-threaded WASM.
static mut SYNTH: Option<ProphetSynth> = None;
static mut CHORUS: Option<StereoChorus> = None;
static mut DELAY: Option<TapeDelay> = None;
static mut REVERB: Option<PlateReverb> = None;
static mut ARP: Option<Arpeggiator> = None;
static mut ARP_LAST_NOTE: u8 = 0;
static mut LEFT_BUF: [f32; 256] = [0.0; 256];
static mut RIGHT_BUF: [f32; 256] = [0.0; 256];

#[no_mangle]
pub extern "C" fn init(sample_rate: f32) {
    unsafe {
        let mut synth = ProphetSynth::new(sample_rate);
        // Default: Osc A saw on, audible
        synth.for_each_voice(|v| {
            v.osc_a.set_saw(true);
            v.osc_a_level = 1.0;
            v.filter_cutoff = 10000.0;
            v.filter_env_amount = 5000.0;
            v.filter_env.set_attack(0.01);
            v.filter_env.set_decay(0.3);
            v.filter_env.set_sustain(0.2);
            v.filter_env.set_release(0.3);
            v.amp_env.set_attack(0.005);
            v.amp_env.set_decay(0.3);
            v.amp_env.set_sustain(0.8);
            v.amp_env.set_release(0.3);
        });
        synth.master_volume = 0.5;
        SYNTH = Some(synth);
        CHORUS = Some(StereoChorus::new(sample_rate));
        DELAY = Some(TapeDelay::new(sample_rate));
        REVERB = Some(PlateReverb::new(sample_rate));
        ARP = Some(Arpeggiator::new(sample_rate));
    }
}

#[no_mangle]
pub extern "C" fn process(num_samples: u32) {
    unsafe {
        let synth = SYNTH.as_mut().unwrap();
        let chorus = CHORUS.as_mut().unwrap();
        let delay = DELAY.as_mut().unwrap();
        let reverb = REVERB.as_mut().unwrap();

        let arp = ARP.as_mut().unwrap();

        let n = (num_samples as usize).min(256);
        for i in 0..n {
            // Arpeggiator generates note events
            if let Some((note, vel)) = arp.process() {
                if vel > 0 {
                    // Note off previous arp note, note on new one
                    if ARP_LAST_NOTE > 0 {
                        synth.note_off(ARP_LAST_NOTE);
                    }
                    synth.note_on(note, vel);
                    ARP_LAST_NOTE = note;
                } else {
                    // Gate off
                    if ARP_LAST_NOTE > 0 {
                        synth.note_off(ARP_LAST_NOTE);
                        ARP_LAST_NOTE = 0;
                    }
                }
            }

            let dry = synth.process();
            let (ch_l, ch_r) = chorus.process(dry);
            let (dl_l, dl_r) = delay.process(ch_l, ch_r);
            let (rv_l, rv_r) = reverb.process_stereo(dl_l, dl_r);
            LEFT_BUF[i] = rv_l;
            RIGHT_BUF[i] = rv_r;
        }
    }
}

#[no_mangle]
pub extern "C" fn get_left_ptr() -> *const f32 {
    unsafe { LEFT_BUF.as_ptr() }
}

#[no_mangle]
pub extern "C" fn get_right_ptr() -> *const f32 {
    unsafe { RIGHT_BUF.as_ptr() }
}

#[no_mangle]
pub extern "C" fn note_on(note: u8, velocity: u8) {
    unsafe {
        let arp = ARP.as_mut().unwrap();
        if arp.mode != ArpMode::Off {
            arp.note_on(note);
        } else if let Some(synth) = SYNTH.as_mut() {
            synth.note_on(note, velocity);
        }
    }
}

#[no_mangle]
pub extern "C" fn note_off(note: u8) {
    unsafe {
        let arp = ARP.as_mut().unwrap();
        if arp.mode != ArpMode::Off {
            arp.note_off(note);
        } else if let Some(synth) = SYNTH.as_mut() {
            synth.note_off(note);
        }
    }
}

/// Generic parameter setter. ID maps to specific synth/effect parameters.
#[no_mangle]
pub extern "C" fn set_param(id: u32, value: f32) {
    unsafe {
        let synth = match SYNTH.as_mut() { Some(s) => s, None => return };
        let chorus = match CHORUS.as_mut() { Some(c) => c, None => return };
        let delay = match DELAY.as_mut() { Some(d) => d, None => return };
        let reverb = match REVERB.as_mut() { Some(r) => r, None => return };

        match id {
            // Oscillator A
            0 => synth.for_each_voice(|v| v.osc_a.set_saw(value > 0.5)),
            1 => synth.for_each_voice(|v| v.osc_a.set_pulse(value > 0.5)),
            2 => synth.for_each_voice(|v| v.osc_a_pw = value),

            // Oscillator B
            3 => synth.for_each_voice(|v| v.osc_b.set_saw(value > 0.5)),
            4 => synth.for_each_voice(|v| v.osc_b.set_tri(value > 0.5)),
            5 => synth.for_each_voice(|v| v.osc_b.set_pulse(value > 0.5)),
            6 => synth.for_each_voice(|v| v.osc_b_pw = value),
            7 => synth.for_each_voice(|v| v.osc_b_semitones = value),
            8 => synth.for_each_voice(|v| v.osc_b_fine = value),

            // Mixer
            9 => synth.for_each_voice(|v| v.osc_a_level = value),
            10 => synth.for_each_voice(|v| v.osc_b_level = value),
            11 => synth.for_each_voice(|v| v.noise_level = value),

            // Filter
            12 => synth.for_each_voice(|v| v.filter_cutoff = value),
            13 => synth.for_each_voice(|v| v.filter.set_resonance(value * 4.0)),
            14 => synth.for_each_voice(|v| v.filter_env_amount = value),
            15 => synth.for_each_voice(|v| v.filter_drive = value),

            // Filter Envelope
            16 => synth.for_each_voice(|v| v.filter_env.set_attack(value)),
            17 => synth.for_each_voice(|v| v.filter_env.set_decay(value)),
            18 => synth.for_each_voice(|v| v.filter_env.set_sustain(value)),
            19 => synth.for_each_voice(|v| v.filter_env.set_release(value)),

            // Amp Envelope
            20 => synth.for_each_voice(|v| v.amp_env.set_attack(value)),
            21 => synth.for_each_voice(|v| v.amp_env.set_decay(value)),
            22 => synth.for_each_voice(|v| v.amp_env.set_sustain(value)),
            23 => synth.for_each_voice(|v| v.amp_env.set_release(value)),

            // Sync
            24 => synth.for_each_voice(|v| v.sync_enabled = value > 0.5),

            // Poly Mod
            25 => synth.for_each_voice(|v| v.poly_mod_filt_env_amt = value),
            26 => synth.for_each_voice(|v| v.poly_mod_osc_b_amt = value),
            27 => synth.for_each_voice(|v| v.poly_mod_dest_freq_a = value > 0.5),
            28 => synth.for_each_voice(|v| v.poly_mod_dest_pw_a = value > 0.5),
            29 => synth.for_each_voice(|v| v.poly_mod_dest_filter = value > 0.5),

            // LFO
            30 => synth.lfo.set_frequency(value),
            31 => synth.lfo.set_triangle(value > 0.5),
            32 => synth.lfo.set_sawtooth(value > 0.5),
            33 => synth.lfo.set_square(value > 0.5),
            34 => synth.lfo_initial_amount = value,

            // Wheel Mod
            35 => synth.wheel_mod_source_mix = value,
            36 => synth.wheel_mod_dest_freq_a = value > 0.5,
            37 => synth.wheel_mod_dest_freq_b = value > 0.5,
            38 => synth.wheel_mod_dest_pw_a = value > 0.5,
            39 => synth.wheel_mod_dest_pw_b = value > 0.5,
            40 => synth.wheel_mod_dest_filter = value > 0.5,

            // Master
            41 => synth.master_volume = value,
            42 => synth.for_each_voice(|v| v.set_glide_rate(value)),
            43 => synth.for_each_voice(|v| v.set_glide_enabled(value > 0.5)),
            44 => synth.unison = value > 0.5,
            45 => synth.for_each_voice(|v| v.set_drift_amount(value)),

            // Mod wheel + pitch bend (from MIDI)
            46 => synth.mod_wheel = value,
            47 => synth.pitch_bend = value,

            // Effects — Chorus
            50 => chorus.rate = value,
            51 => chorus.depth = value,
            52 => chorus.mix = value,

            // Effects — Delay
            53 => delay.time_ms = value,
            54 => delay.feedback = value,
            55 => delay.tone = value,
            56 => delay.mix = value,

            // Effects — Reverb
            57 => reverb.decay = value,
            58 => reverb.damping = value,
            59 => reverb.mix = value,

            // Arpeggiator (60-67)
            60 => { let a = ARP.as_mut().unwrap(); a.mode = ArpMode::from_u8(value as u8); },
            61 => { let a = ARP.as_mut().unwrap(); a.division = ArpDivision::from_u8(value as u8); },
            62 => { let a = ARP.as_mut().unwrap(); a.bpm = value; },
            63 => { let a = ARP.as_mut().unwrap(); a.octaves = (value as u8).clamp(1, 4); },
            64 => { let a = ARP.as_mut().unwrap(); a.gate = value; },
            65 => { let a = ARP.as_mut().unwrap(); a.swing = value; },
            66 => { let a = ARP.as_mut().unwrap(); a.hold = value > 0.5; },
            67 => { let a = ARP.as_mut().unwrap(); a.all_notes_off(); }, // panic button

            _ => {} // unknown param
        }
    }
}
