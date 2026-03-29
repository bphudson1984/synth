use tr808_dsp::engine::{TR808, Voice as Voice808};
use tr909_dsp::engine::{TR909, Voice909};
use tr808_dsp::sequencer::{Sequencer, SeqEvent};

static mut ENGINE_808: Option<TR808> = None;
static mut ENGINE_909: Option<TR909> = None;
static mut SEQ: Option<Sequencer> = None;
static mut SEQ_EVENTS: Vec<SeqEvent> = Vec::new();
// Per-track engine assignment: false=808, true=909
static mut TRACK_ENGINE: [bool; 13] = [false; 13];
static mut LEFT_BUF: [f32; 256] = [0.0; 256];
static mut RIGHT_BUF: [f32; 256] = [0.0; 256];

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
        let n = (num_samples as usize).min(256);
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
        if voice_id >= 100 && voice_id < 200 {
            // 909 params
            let e = match ENGINE_909.as_mut() { Some(e) => e, None => return };
            let v = voice_id - 100;
            match (v, param_id) {
                (0, 0) => e.bd.level = value,
                (0, 1) => e.bd.tone = value,
                (0, 2) => e.bd.decay = value,
                (1, 0) => e.sd.level = value,
                (1, 1) => e.sd.tone = value,
                (1, 2) => e.sd.snappy = value,
                (2, 0) => e.lt.level = value,
                (2, 1) => e.lt.tuning = value,
                (3, 0) => e.mt.level = value,
                (3, 1) => e.mt.tuning = value,
                (4, 0) => e.ht.level = value,
                (4, 1) => e.ht.tuning = value,
                (5, 0) => e.rs.level = value,
                (6, 0) => e.cp.level = value,
                (7, 0) => e.ch.level = value,
                (8, 0) => e.oh.level = value,
                (8, 1) => e.oh.decay = value,
                (9, 0) => e.cc.level = value,
                (9, 1) => e.cc.decay = value,
                (10, 0) => e.rc.level = value,
                (10, 1) => e.rc.decay = value,
                _ => {}
            }
        } else {
            // 808 params
            let e = match ENGINE_808.as_mut() { Some(e) => e, None => return };
            match (voice_id, param_id) {
                (0, 0) => e.bd.level = value,
                (0, 1) => e.bd.tone = value,
                (0, 2) => e.bd.decay = value,
                (1, 0) => e.sd.level = value,
                (1, 1) => e.sd.tone = value,
                (1, 2) => e.sd.snappy = value,
                (2, 0) => e.lt.level = value,
                (2, 1) => e.lt.tuning = value,
                (3, 0) => e.mt.level = value,
                (3, 1) => e.mt.tuning = value,
                (4, 0) => e.ht.level = value,
                (4, 1) => e.ht.tuning = value,
                (5, 0) => e.rs.level = value,
                (6, 0) => e.cp.level = value,
                (7, 0) => e.ch.level = value,
                (8, 0) => e.oh.level = value,
                (8, 1) => e.oh.decay = value,
                (9, 0) => e.cy.level = value,
                (9, 1) => e.cy.decay = value,
                (10, 0) => e.cb.level = value,
                (11, 0) => e.ma.level = value,
                (12, 0) => e.cl.level = value,
                (255, 0) => { e.master_volume = value; ENGINE_909.as_mut().unwrap().master_volume = value; },
                _ => {}
            }
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
