//! Volca Bass WASM wrapper.
//! Uses the shared NoteSequencer (same as Prophet/Braids) for the melodic sequencer pattern.

use volca_bass_dsp::engine::VolcaBass;
use dsp_common::note_sequencer::MAX_STEPS;

static mut ENGINE: Option<VolcaBass> = None;
static mut LEFT_BUF: [f32; 512] = [0.0; 512];
static mut RIGHT_BUF: [f32; 512] = [0.0; 512];

#[no_mangle]
pub extern "C" fn init(sample_rate: f32) {
    unsafe {
        let mut engine = VolcaBass::new(sample_rate);
        // Default: all 3 VCOs on saw, warm bass preset
        engine.voice.cutoff = 2000.0;
        engine.voice.resonance = 0.3;
        engine.voice.eg_intensity = 0.5;
        engine.voice.eg_to_vca = true;
        ENGINE = Some(engine);
    }
}

#[no_mangle]
pub extern "C" fn process(num_samples: u32) {
    unsafe {
        let engine = match ENGINE.as_mut() { Some(e) => e, None => return };
        let n = (num_samples as usize).min(512);
        for i in 0..n {
            let mono = engine.process();
            LEFT_BUF[i] = mono;
            RIGHT_BUF[i] = mono;
        }
    }
}

#[no_mangle] pub extern "C" fn get_left_ptr() -> *const f32 { unsafe { LEFT_BUF.as_ptr() } }
#[no_mangle] pub extern "C" fn get_right_ptr() -> *const f32 { unsafe { RIGHT_BUF.as_ptr() } }

#[no_mangle]
pub extern "C" fn set_param(id: u8, value: f32) {
    unsafe {
        if let Some(e) = ENGINE.as_mut() {
            use dsp_common::engine::SynthEngine;
            e.set_param(id as u32, value);
        }
    }
}

#[no_mangle]
pub extern "C" fn note_on(note: u8, velocity: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { e.note_on(note, velocity); } }
}

#[no_mangle]
pub extern "C" fn note_off(note: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { e.note_off(); } }
}

// --- NoteSequencer exports (same pattern as prophet-wasm / braids-wasm) ---

#[no_mangle] pub extern "C" fn seq_play() { unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.play(); } } }
#[no_mangle] pub extern "C" fn seq_stop() {
    unsafe {
        if let Some(e) = ENGINE.as_mut() {
            e.sequencer.stop();
            e.voice.note_off();
        }
    }
}
#[no_mangle] pub extern "C" fn seq_set_bpm(bpm: f32) { unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.set_bpm(bpm); } } }
#[no_mangle] pub extern "C" fn seq_get_current_step() -> u8 { unsafe { if let Some(e) = ENGINE.as_ref() { e.sequencer.current_step() as u8 } else { 0 } } }
#[no_mangle] pub extern "C" fn seq_clear() { unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.clear(); } } }
#[no_mangle] pub extern "C" fn seq_set_external(ext: u8) { unsafe { if let Some(e) = ENGINE.as_mut() { e.seq_external = ext != 0; } } }
#[no_mangle] pub extern "C" fn seq_set_length(len: u8) { unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.set_length(len as usize); } } }

#[no_mangle]
pub extern "C" fn seq_set_step_notes(step: u8, num: u8, n1: u8, n2: u8, n3: u8, n4: u8) {
    unsafe {
        if let Some(e) = ENGINE.as_mut() {
            let i = step as usize;
            if i < MAX_STEPS {
                e.sequencer.steps[i].notes = [n1, n2, n3, n4];
                e.sequencer.steps[i].num_notes = num.min(4);
                e.sequencer.steps[i].gate = true;
            }
        }
    }
}
#[no_mangle] pub extern "C" fn seq_set_step_gate(step: u8, gate: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { let i = step as usize; if i < MAX_STEPS { e.sequencer.steps[i].gate = gate != 0; } } }
}
#[no_mangle] pub extern "C" fn seq_set_step_velocity(step: u8, vel: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { let i = step as usize; if i < MAX_STEPS { e.sequencer.steps[i].velocity = vel; } } }
}
#[no_mangle] pub extern "C" fn seq_set_step_gate_pct(step: u8, pct: u16) {
    unsafe { if let Some(e) = ENGINE.as_mut() { let i = step as usize; if i < MAX_STEPS { e.sequencer.steps[i].gate_pct = pct; } } }
}
#[no_mangle] pub extern "C" fn seq_set_step_probability(step: u8, prob: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { let i = step as usize; if i < MAX_STEPS { e.sequencer.steps[i].probability = prob; } } }
}
#[no_mangle] pub extern "C" fn seq_set_step_ratchet(step: u8, count: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { let i = step as usize; if i < MAX_STEPS { e.sequencer.steps[i].ratchet = count.clamp(1, 4); } } }
}
#[no_mangle] pub extern "C" fn seq_set_step_skip(step: u8, skip: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { let i = step as usize; if i < MAX_STEPS { e.sequencer.steps[i].skip = skip != 0; } } }
}
#[no_mangle] pub extern "C" fn seq_set_direction(dir: u8) { unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.direction = dir; } } }
#[no_mangle] pub extern "C" fn seq_set_swing(swing: f32) { unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.swing = swing.clamp(0.0, 1.0); } } }
#[no_mangle] pub extern "C" fn seq_set_time_div(div: u8) { unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.set_time_div(div); } } }
#[no_mangle] pub extern "C" fn seq_rotate(dir: i32) { unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.rotate(dir); } } }
#[no_mangle] pub extern "C" fn seq_set_glitch(size: u8) { unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.set_glitch(size as usize); } } }
