use braids_dsp::engine::BraidsSynth;
use braids_dsp::sequencer::MAX_STEPS;

static mut ENGINE: Option<BraidsSynth> = None;
static mut LEFT_BUF: [f32; 256] = [0.0; 256];
static mut RIGHT_BUF: [f32; 256] = [0.0; 256];

#[no_mangle]
pub extern "C" fn init(sample_rate: f32) {
    unsafe { ENGINE = Some(BraidsSynth::new(sample_rate)); }
}

#[no_mangle]
pub extern "C" fn process(num_samples: u32) {
    unsafe {
        let e = match ENGINE.as_mut() { Some(e) => e, None => return };
        let n = (num_samples as usize).min(256);
        for i in 0..n {
            let mono = e.process();
            LEFT_BUF[i] = mono;
            RIGHT_BUF[i] = mono;
        }
    }
}

#[no_mangle] pub extern "C" fn get_left_ptr() -> *const f32 { unsafe { LEFT_BUF.as_ptr() } }
#[no_mangle] pub extern "C" fn get_right_ptr() -> *const f32 { unsafe { RIGHT_BUF.as_ptr() } }

#[no_mangle]
pub extern "C" fn note_on(note: u8, velocity: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { e.note_on(note, velocity); } }
}

#[no_mangle]
pub extern "C" fn note_off(note: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { e.note_off(note); } }
}

#[no_mangle]
pub extern "C" fn set_param(id: u8, value: f32) {
    unsafe {
        let e = match ENGINE.as_mut() { Some(e) => e, None => return };
        match id {
            0 => e.set_mode(value as u8),
            1 => e.timbre = value,
            2 => e.color = value,
            3 => e.filter_cutoff = value,
            4 => e.filter_resonance = value,
            5 => e.filter_env_amt = value,
            6 => e.amp_env.set_attack(value),
            7 => e.amp_env.set_decay(value),
            8 => e.amp_env.set_sustain(value),
            9 => e.amp_env.set_release(value),
            10 => e.set_lfo_rate(value),
            11 => e.lfo_amount = value,
            12 => e.lfo_dest = value as u8,
            13 => e.master_volume = value,
            14 => e.set_glide_time(value),
            15 => e.filter_env.set_attack(value),
            16 => e.filter_env.set_decay(value),
            17 => e.filter_env.set_sustain(value),
            18 => e.filter_env.set_release(value),
            _ => {}
        }
    }
}

// --- Sequencer ---
#[no_mangle] pub extern "C" fn seq_play() { unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.play(); } } }
#[no_mangle] pub extern "C" fn seq_stop() { unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.stop(); } } }
#[no_mangle] pub extern "C" fn seq_set_bpm(bpm: f32) { unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.set_bpm(bpm); } } }
#[no_mangle] pub extern "C" fn seq_get_current_step() -> u8 { unsafe { if let Some(e) = ENGINE.as_ref() { e.sequencer.current_step() as u8 } else { 0 } } }
#[no_mangle] pub extern "C" fn seq_clear() { unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.clear(); } } }
#[no_mangle] pub extern "C" fn seq_set_external(ext: u8) { unsafe { if let Some(e) = ENGINE.as_mut() { e.seq_external = ext != 0; } } }
#[no_mangle] pub extern "C" fn seq_set_length(len: u8) { unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.set_length(len as usize); } } }

/// Set notes for a step. note1 is always set; note2-4 are 0 if unused.
#[no_mangle]
pub extern "C" fn seq_set_step_notes(step: u8, num: u8, n1: u8, n2: u8, n3: u8, n4: u8) {
    unsafe {
        if let Some(e) = ENGINE.as_mut() {
            let s = step as usize;
            if s < MAX_STEPS {
                e.sequencer.steps[s].notes = [n1, n2, n3, n4];
                e.sequencer.steps[s].num_notes = num.min(4);
                e.sequencer.steps[s].gate = true;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn seq_set_step_gate(step: u8, gate: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { let s = step as usize; if s < MAX_STEPS { e.sequencer.steps[s].gate = gate != 0; } } }
}

// Per-step parameter setters
#[no_mangle] pub extern "C" fn seq_set_step_velocity(step: u8, vel: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { let s = step as usize; if s < MAX_STEPS { e.sequencer.steps[s].velocity = vel; } } }
}
#[no_mangle] pub extern "C" fn seq_set_step_gate_pct(step: u8, pct: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { let s = step as usize; if s < MAX_STEPS { e.sequencer.steps[s].gate_pct = pct; } } }
}
#[no_mangle] pub extern "C" fn seq_set_step_probability(step: u8, prob: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { let s = step as usize; if s < MAX_STEPS { e.sequencer.steps[s].probability = prob; } } }
}
#[no_mangle] pub extern "C" fn seq_set_step_ratchet(step: u8, count: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { let s = step as usize; if s < MAX_STEPS { e.sequencer.steps[s].ratchet = count.clamp(1, 4); } } }
}
#[no_mangle] pub extern "C" fn seq_set_step_skip(step: u8, skip: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { let s = step as usize; if s < MAX_STEPS { e.sequencer.steps[s].skip = skip != 0; } } }
}

// Pattern-level setters
#[no_mangle] pub extern "C" fn seq_set_direction(dir: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.direction = dir; } }
}
#[no_mangle] pub extern "C" fn seq_set_swing(swing: f32) {
    unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.swing = swing.clamp(0.0, 1.0); } }
}
#[no_mangle] pub extern "C" fn seq_set_time_div(div: u8) {
    unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.set_time_div(div); } }
}
#[no_mangle] pub extern "C" fn seq_rotate(dir: i32) {
    unsafe { if let Some(e) = ENGINE.as_mut() { e.sequencer.rotate(dir); } }
}
