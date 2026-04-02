//! Thin C-ABI wrapper around prophet-dsp for WebAssembly.
//! Exposes functions callable from JavaScript AudioWorkletProcessor.

use prophet_dsp::synth::ProphetSynth;
use prophet_dsp::effects::chorus::StereoChorus;
use prophet_dsp::effects::delay::TapeDelay;
use prophet_dsp::effects::reverb::PlateReverb;
use prophet_dsp::arpeggiator::{Arpeggiator, ArpMode, ArpDivision};
use dsp_common::note_sequencer::MAX_STEPS;

static mut SYNTH: Option<ProphetSynth> = None;
static mut CHORUS: Option<StereoChorus> = None;
static mut DELAY: Option<TapeDelay> = None;
static mut REVERB: Option<PlateReverb> = None;
static mut ARP: Option<Arpeggiator> = None;
static mut ARP_LAST_NOTE: u8 = 0;
static mut LEFT_BUF: [f32; 512] = [0.0; 512];
static mut RIGHT_BUF: [f32; 512] = [0.0; 512];

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

        let n = (num_samples as usize).min(512);
        for i in 0..n {
            // Arpeggiator generates note events
            if let Some((note, vel)) = arp.process() {
                if vel > 0 {
                    if ARP_LAST_NOTE > 0 { synth.note_off(ARP_LAST_NOTE); }
                    synth.note_on(note, vel);
                    ARP_LAST_NOTE = note;
                } else {
                    if ARP_LAST_NOTE > 0 { synth.note_off(ARP_LAST_NOTE); ARP_LAST_NOTE = 0; }
                }
            }

            // Synth process (includes embedded sequencer)
            let dry = synth.process();
            let (ch_l, ch_r) = chorus.process(dry);
            let (dl_l, dl_r) = delay.process(ch_l, ch_r);
            let (rv_l, rv_r) = reverb.process_stereo(dl_l, dl_r);
            LEFT_BUF[i] = rv_l.clamp(-1.0, 1.0);
            RIGHT_BUF[i] = rv_r.clamp(-1.0, 1.0);
        }
    }
}

#[no_mangle] pub extern "C" fn get_left_ptr() -> *const f32 { unsafe { LEFT_BUF.as_ptr() } }
#[no_mangle] pub extern "C" fn get_right_ptr() -> *const f32 { unsafe { RIGHT_BUF.as_ptr() } }

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

/// Generic parameter setter. IDs 0-47 are synth params (delegated to DSP),
/// 50-59 are effects, 60-67 are arpeggiator.
#[no_mangle]
pub extern "C" fn set_param(id: u32, value: f32) {
    unsafe {
        match id {
            0..=47 => {
                use dsp_common::engine::SynthEngine;
                if let Some(synth) = SYNTH.as_mut() { synth.set_param(id, value); }
            }
            // Effects — Chorus
            50 => { CHORUS.as_mut().unwrap().rate = value; }
            51 => { CHORUS.as_mut().unwrap().depth = value; }
            52 => { CHORUS.as_mut().unwrap().mix = value; }
            // Effects — Delay
            53 => { DELAY.as_mut().unwrap().time_ms = value; }
            54 => { DELAY.as_mut().unwrap().feedback = value; }
            55 => { DELAY.as_mut().unwrap().tone = value; }
            56 => { DELAY.as_mut().unwrap().mix = value; }
            // Effects — Reverb
            57 => { REVERB.as_mut().unwrap().decay = value; }
            58 => { REVERB.as_mut().unwrap().damping = value; }
            59 => { REVERB.as_mut().unwrap().mix = value; }
            // Arpeggiator (60-67)
            60 => { ARP.as_mut().unwrap().mode = ArpMode::from_u8(value as u8); }
            61 => { ARP.as_mut().unwrap().division = ArpDivision::from_u8(value as u8); }
            62 => { ARP.as_mut().unwrap().bpm = value; }
            63 => { ARP.as_mut().unwrap().octaves = (value as u8).clamp(1, 4); }
            64 => { ARP.as_mut().unwrap().gate = value; }
            65 => { ARP.as_mut().unwrap().swing = value; }
            66 => { ARP.as_mut().unwrap().hold = value > 0.5; }
            67 => { ARP.as_mut().unwrap().all_notes_off(); }
            _ => {}
        }
    }
}

// --- Note Sequencer (delegated to embedded synth.sequencer) ---
#[no_mangle] pub extern "C" fn seq_play() { unsafe { if let Some(s) = SYNTH.as_mut() { s.sequencer.play(); } } }
#[no_mangle] pub extern "C" fn seq_stop() {
    unsafe {
        if let Some(synth) = SYNTH.as_mut() {
            synth.sequencer.stop();
            // Release all voices
            for n in 0..128u8 { synth.note_off(n); }
        }
    }
}
#[no_mangle] pub extern "C" fn seq_set_bpm(bpm: f32) { unsafe { if let Some(s) = SYNTH.as_mut() { s.sequencer.set_bpm(bpm); } } }
#[no_mangle] pub extern "C" fn seq_get_current_step() -> u8 { unsafe { if let Some(s) = SYNTH.as_ref() { s.sequencer.current_step() as u8 } else { 0 } } }
#[no_mangle] pub extern "C" fn seq_clear() { unsafe { if let Some(s) = SYNTH.as_mut() { s.sequencer.clear(); } } }
#[no_mangle] pub extern "C" fn seq_set_external(ext: u8) { unsafe { if let Some(s) = SYNTH.as_mut() { s.seq_external = ext != 0; } } }
#[no_mangle] pub extern "C" fn seq_set_length(len: u8) { unsafe { if let Some(s) = SYNTH.as_mut() { s.sequencer.set_length(len as usize); } } }
#[no_mangle] pub extern "C" fn seq_set_step_notes(step: u8, num: u8, n1: u8, n2: u8, n3: u8, n4: u8) {
    unsafe { if let Some(s) = SYNTH.as_mut() { let i = step as usize; if i < MAX_STEPS { s.sequencer.steps[i].notes = [n1, n2, n3, n4]; s.sequencer.steps[i].num_notes = num.min(4); s.sequencer.steps[i].gate = true; } } }
}
#[no_mangle] pub extern "C" fn seq_set_step_gate(step: u8, gate: u8) {
    unsafe { if let Some(s) = SYNTH.as_mut() { let i = step as usize; if i < MAX_STEPS { s.sequencer.steps[i].gate = gate != 0; } } }
}
#[no_mangle] pub extern "C" fn seq_set_step_velocity(step: u8, vel: u8) {
    unsafe { if let Some(s) = SYNTH.as_mut() { let i = step as usize; if i < MAX_STEPS { s.sequencer.steps[i].velocity = vel; } } }
}
#[no_mangle] pub extern "C" fn seq_set_step_gate_pct(step: u8, pct: u8) {
    unsafe { if let Some(s) = SYNTH.as_mut() { let i = step as usize; if i < MAX_STEPS { s.sequencer.steps[i].gate_pct = pct; } } }
}
#[no_mangle] pub extern "C" fn seq_set_step_probability(step: u8, prob: u8) {
    unsafe { if let Some(s) = SYNTH.as_mut() { let i = step as usize; if i < MAX_STEPS { s.sequencer.steps[i].probability = prob; } } }
}
#[no_mangle] pub extern "C" fn seq_set_step_ratchet(step: u8, count: u8) {
    unsafe { if let Some(s) = SYNTH.as_mut() { let i = step as usize; if i < MAX_STEPS { s.sequencer.steps[i].ratchet = count.clamp(1, 4); } } }
}
#[no_mangle] pub extern "C" fn seq_set_step_skip(step: u8, skip: u8) {
    unsafe { if let Some(s) = SYNTH.as_mut() { let i = step as usize; if i < MAX_STEPS { s.sequencer.steps[i].skip = skip != 0; } } }
}
#[no_mangle] pub extern "C" fn seq_set_direction(dir: u8) { unsafe { if let Some(s) = SYNTH.as_mut() { s.sequencer.direction = dir; } } }
#[no_mangle] pub extern "C" fn seq_set_swing(swing: f32) { unsafe { if let Some(s) = SYNTH.as_mut() { s.sequencer.swing = swing.clamp(0.0, 1.0); } } }
#[no_mangle] pub extern "C" fn seq_set_time_div(div: u8) { unsafe { if let Some(s) = SYNTH.as_mut() { s.sequencer.set_time_div(div); } } }
#[no_mangle] pub extern "C" fn seq_rotate(dir: i32) { unsafe { if let Some(s) = SYNTH.as_mut() { s.sequencer.rotate(dir); } } }
