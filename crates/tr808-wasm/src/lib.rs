use tr808_dsp::engine::{TR808, Voice as Voice808};
use tr909_dsp::engine::{TR909, Voice909};
use tr808_dsp::sequencer::{Sequencer, SeqEvent};

static mut ENGINE_808: Option<TR808> = None;
static mut ENGINE_909: Option<TR909> = None;
static mut SEQ: Option<Sequencer> = None;
static mut SEQ_EVENTS: Vec<SeqEvent> = Vec::new();
// Per-track engine assignment: false=808, true=909
static mut TRACK_ENGINE: [bool; 13] = [false; 13];
static mut LEFT_BUF: [f32; 512] = [0.0; 512];
static mut RIGHT_BUF: [f32; 512] = [0.0; 512];

#[no_mangle]
pub extern "C" fn init(sample_rate: f32) {
    unsafe {
        ENGINE_808 = Some(TR808::new(sample_rate));
        ENGINE_909 = Some(TR909::new(sample_rate));
        SEQ = Some(Sequencer::new(sample_rate));
        SEQ_EVENTS = Vec::with_capacity(16);
        TRACK_ENGINE = [false; 13];
    }
}

#[no_mangle]
pub extern "C" fn process(num_samples: u32) {
    unsafe {
        let e808 = ENGINE_808.as_mut().unwrap();
        let e909 = ENGINE_909.as_mut().unwrap();
        let seq = SEQ.as_mut().unwrap();
        let n = (num_samples as usize).min(512);
        for i in 0..n {
            seq.process(&mut SEQ_EVENTS);
            for ev in &SEQ_EVENTS {
                let track = ev.voice as usize;
                if track < 13 && TRACK_ENGINE[track] {
                    // 909
                    if let Some(v) = Voice909::from_u8(ev.voice) { e909.trigger(v); }
                } else {
                    // 808
                    if let Some(v) = Voice808::from_u8(ev.voice) { e808.trigger(v); }
                }
            }
            // Mix both engines
            let mono = e808.process() + e909.process();
            LEFT_BUF[i] = mono.clamp(-1.0, 1.0);
            RIGHT_BUF[i] = mono.clamp(-1.0, 1.0);
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
pub extern "C" fn trigger(voice_id: u8) {
    unsafe {
        if voice_id >= 100 {
            if let Some(e) = ENGINE_909.as_mut() {
                if let Some(v) = Voice909::from_u8(voice_id - 100) { e.trigger(v); }
            }
        } else if let Some(e) = ENGINE_808.as_mut() {
            if let Some(v) = Voice808::from_u8(voice_id) { e.trigger(v); }
        }
    }
}

#[no_mangle]
pub extern "C" fn set_param(voice_id: u8, param_id: u8, value: f32) {
    unsafe {
        if voice_id == 255 {
            // Master volume — set on both engines
            if let Some(e) = ENGINE_808.as_mut() { e.master_volume = value; }
            if let Some(e) = ENGINE_909.as_mut() { e.master_volume = value; }
        } else if voice_id >= 100 {
            // 909 params — delegate to DSP set_voice_param
            if let Some(e) = ENGINE_909.as_mut() { e.set_voice_param(voice_id - 100, param_id, value); }
        } else {
            // 808 params — delegate to DSP set_voice_param
            if let Some(e) = ENGINE_808.as_mut() { e.set_voice_param(voice_id, param_id, value); }
        }
    }
}

// Sequencer controls
#[no_mangle]
pub extern "C" fn seq_play() {
    unsafe { if let Some(seq) = SEQ.as_mut() { seq.play(); } }
}

#[no_mangle]
pub extern "C" fn seq_stop() {
    unsafe { if let Some(seq) = SEQ.as_mut() { seq.stop(); } }
}

#[no_mangle]
pub extern "C" fn seq_set_bpm(bpm: f32) {
    unsafe { if let Some(seq) = SEQ.as_mut() { seq.set_bpm(bpm); } }
}

#[no_mangle]
pub extern "C" fn seq_set_swing(swing: f32) {
    unsafe { if let Some(seq) = SEQ.as_mut() { seq.swing = swing; } }
}

#[no_mangle]
pub extern "C" fn seq_toggle_step(track: u8, step: u8) {
    unsafe {
        if let Some(seq) = SEQ.as_mut() {
            seq.toggle_step(track as usize, step as usize);
        }
    }
}

#[no_mangle]
pub extern "C" fn seq_set_step(track: u8, step: u8, active: u8) {
    unsafe {
        if let Some(seq) = SEQ.as_mut() {
            seq.set_step(track as usize, step as usize, active != 0);
        }
    }
}

#[no_mangle]
pub extern "C" fn seq_get_step(track: u8, step: u8) -> u8 {
    unsafe {
        if let Some(seq) = SEQ.as_ref() {
            if seq.get_step(track as usize, step as usize) { 1 } else { 0 }
        } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn seq_get_current_step() -> u8 {
    unsafe {
        if let Some(seq) = SEQ.as_ref() { seq.current_step() as u8 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn seq_is_playing() -> u8 {
    unsafe {
        if let Some(seq) = SEQ.as_ref() { if seq.is_playing() { 1 } else { 0 } } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn seq_clear() {
    unsafe { if let Some(seq) = SEQ.as_mut() { seq.clear(); } }
}

#[no_mangle]
pub extern "C" fn seq_set_length(length: u8) {
    unsafe { if let Some(seq) = SEQ.as_mut() { seq.set_length(length as usize); } }
}

#[no_mangle]
pub extern "C" fn seq_set_time_div(div: u8) {
    unsafe { if let Some(seq) = SEQ.as_mut() { seq.set_time_div(div); } }
}

/// Set which engine a track uses: 0=808, 1=909
#[no_mangle]
pub extern "C" fn set_track_engine(track: u8, is_909: u8) {
    unsafe {
        if (track as usize) < 13 {
            TRACK_ENGINE[track as usize] = is_909 != 0;
        }
    }
}

/// Set ALL tracks to one engine: 0=808, 1=909
#[no_mangle]
pub extern "C" fn set_all_engines(is_909: u8) {
    unsafe {
        for t in &mut TRACK_ENGINE { *t = is_909 != 0; }
    }
}
