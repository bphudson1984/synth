use tb303_dsp::engine::TB303;
use tb303_dsp::sequencer::NUM_STEPS;

static mut ENGINE: Option<TB303> = None;
static mut LEFT_BUF: [f32; 512] = [0.0; 512];
static mut RIGHT_BUF: [f32; 512] = [0.0; 512];

#[no_mangle]
pub extern "C" fn init(sample_rate: f32) {
    unsafe { ENGINE = Some(TB303::new(sample_rate)); }
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

#[no_mangle]
pub extern "C" fn get_left_ptr() -> *const f32 { unsafe { LEFT_BUF.as_ptr() } }
#[no_mangle]
pub extern "C" fn get_right_ptr() -> *const f32 { unsafe { RIGHT_BUF.as_ptr() } }

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
pub extern "C" fn note_off(_note: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { e.note_off(); } }
}

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
#[no_mangle] pub extern "C" fn seq_is_playing() -> u8 { unsafe { if let Some(e) = ENGINE.as_ref() { if e.sequencer.is_playing() { 1 } else { 0 } } else { 0 } } }
#[no_mangle] pub extern "C" fn seq_clear() { unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.clear(); } } }

#[no_mangle]
pub extern "C" fn seq_set_step_note(step: u8, note: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { let s = step as usize; if s < NUM_STEPS { e.sequencer.steps[s].note = note; } } }
}
#[no_mangle]
pub extern "C" fn seq_set_step_gate(step: u8, gate: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { let s = step as usize; if s < NUM_STEPS { e.sequencer.steps[s].gate = gate != 0; } } }
}
#[no_mangle]
pub extern "C" fn seq_set_step_accent(step: u8, accent: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { let s = step as usize; if s < NUM_STEPS { e.sequencer.steps[s].accent = accent != 0; } } }
}
#[no_mangle]
pub extern "C" fn seq_set_step_slide(step: u8, slide: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { let s = step as usize; if s < NUM_STEPS { e.sequencer.steps[s].slide = slide != 0; } } }
}
#[no_mangle] pub extern "C" fn seq_set_direction(dir: u8) { unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.direction = dir; } } }
#[no_mangle] pub extern "C" fn seq_set_swing(swing: f32) { unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.swing = swing.clamp(0.0, 1.0); } } }
#[no_mangle] pub extern "C" fn seq_set_time_div(div: u8) { unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.set_time_div(div); } } }
