use braids_dsp::engine::BraidsSynth;

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
