//! SP-404MK2 inspired sampler WASM wrapper.
//! Handles sample data loading from JS and stereo playback via voice pool.

use sampler_dsp::engine::{Sampler, MAX_PADS};
use dsp_common::note_sequencer::MAX_STEPS;

static mut ENGINE: Option<Sampler> = None;
static mut LEFT_BUF: [f32; 512] = [0.0; 512];
static mut RIGHT_BUF: [f32; 512] = [0.0; 512];

#[no_mangle]
pub extern "C" fn init(sample_rate: f32) {
    unsafe { ENGINE = Some(Sampler::new(sample_rate)); }
}

#[no_mangle]
pub extern "C" fn process(num_samples: u32) {
    unsafe {
        let engine = match ENGINE.as_mut() { Some(e) => e, None => return };
        let n = (num_samples as usize).min(512);
        for i in 0..n {
            let (l, r) = engine.process_stereo();
            LEFT_BUF[i] = l;
            RIGHT_BUF[i] = r;
        }
    }
}

#[no_mangle] pub extern "C" fn get_left_ptr() -> *const f32 { unsafe { LEFT_BUF.as_ptr() } }
#[no_mangle] pub extern "C" fn get_right_ptr() -> *const f32 { unsafe { RIGHT_BUF.as_ptr() } }

// --- Sample loading ---
// JS allocates a buffer in WASM memory, writes sample data into it,
// then calls load_sample to register it with a pad.

/// Allocate a buffer of `length` f32s in WASM memory. Returns byte pointer.
/// JS writes Float32Array data at this pointer, then calls load_sample.
#[no_mangle]
pub extern "C" fn alloc_sample_buffer(length: u32) -> *mut f32 {
    let mut buf = vec![0.0f32; length as usize];
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf); // prevent deallocation — JS will write here
    ptr
}

/// Register sample data for a pad. left_ptr and right_ptr are from alloc_sample_buffer.
#[no_mangle]
pub extern "C" fn load_sample(pad: u8, left_ptr: *const f32, right_ptr: *const f32, length: u32, sample_rate: f32) {
    unsafe {
        let engine = match ENGINE.as_mut() { Some(e) => e, None => return };
        let len = length as usize;
        // Copy from allocated buffers into engine storage
        let left = std::slice::from_raw_parts(left_ptr, len).to_vec();
        let right = std::slice::from_raw_parts(right_ptr, len).to_vec();
        engine.load_sample(pad as usize, left, right, sample_rate);
        // Free the temporary buffers
        drop(Vec::from_raw_parts(left_ptr as *mut f32, len, len));
        drop(Vec::from_raw_parts(right_ptr as *mut f32, len, len));
    }
}

// --- Trigger / Release ---

#[no_mangle]
pub extern "C" fn trigger(pad: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { e.trigger(pad); } }
}

#[no_mangle]
pub extern "C" fn release(pad: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { e.release_pad(pad); } }
}

#[no_mangle]
pub extern "C" fn stop(pad: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { e.stop_pad(pad); } }
}

// --- Per-pad params ---

#[no_mangle]
pub extern "C" fn set_pad_param(pad: u8, param: u8, value: f32) {
    unsafe { if let Some(e) = ENGINE.as_mut() { e.set_pad_param(pad, param, value); } }
}

// --- Global params ---

#[no_mangle]
pub extern "C" fn set_param(id: u8, value: f32) {
    unsafe {
        if let Some(e) = ENGINE.as_mut() {
            use dsp_common::engine::SynthEngine;
            e.set_param(id as u32, value);
        }
    }
}

// --- Note on/off (for sequencer / MIDI) ---

#[no_mangle]
pub extern "C" fn note_on(note: u8, velocity: u8) {
    unsafe {
        if let Some(e) = ENGINE.as_mut() {
            use dsp_common::engine::MelodicEngine;
            e.note_on(note, velocity);
        }
    }
}

#[no_mangle]
pub extern "C" fn note_off(note: u8) {
    unsafe {
        if let Some(e) = ENGINE.as_mut() {
            use dsp_common::engine::MelodicEngine;
            e.note_off(note);
        }
    }
}

// --- NoteSequencer exports (same pattern as prophet/braids/bass) ---

#[no_mangle] pub extern "C" fn seq_play() { unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.play(); } } }
#[no_mangle] pub extern "C" fn seq_stop() {
    unsafe {
        if let Some(e) = ENGINE.as_mut() {
            e.sequencer.stop();
            // Stop all playing voices
            for pad in 0..MAX_PADS as u8 { e.stop_pad(pad); }
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
