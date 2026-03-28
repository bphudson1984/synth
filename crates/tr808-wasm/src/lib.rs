use tr808_dsp::engine::{TR808, Voice};

static mut ENGINE: Option<TR808> = None;
static mut LEFT_BUF: [f32; 256] = [0.0; 256];
static mut RIGHT_BUF: [f32; 256] = [0.0; 256];

#[no_mangle]
pub extern "C" fn init(sample_rate: f32) {
    unsafe {
        ENGINE = Some(TR808::new(sample_rate));
    }
}

#[no_mangle]
pub extern "C" fn process(num_samples: u32) {
    unsafe {
        let engine = ENGINE.as_mut().unwrap();
        let n = (num_samples as usize).min(256);
        for i in 0..n {
            let mono = engine.process();
            LEFT_BUF[i] = mono;
            RIGHT_BUF[i] = mono;
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
        if let Some(engine) = ENGINE.as_mut() {
            if let Some(voice) = Voice::from_u8(voice_id) {
                engine.trigger(voice);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn set_param(voice_id: u8, param_id: u8, value: f32) {
    unsafe {
        let engine = match ENGINE.as_mut() { Some(e) => e, None => return };
        match (voice_id, param_id) {
            // BD: 0=level, 1=tone, 2=decay
            (0, 0) => engine.bd.level = value,
            (0, 1) => engine.bd.tone = value,
            (0, 2) => engine.bd.decay = value,
            // SD: 0=level, 1=tone, 2=snappy
            (1, 0) => engine.sd.level = value,
            (1, 1) => engine.sd.tone = value,
            (1, 2) => engine.sd.snappy = value,
            // LT/MT/HT: 0=level, 1=tuning
            (2, 0) => engine.lt.level = value,
            (2, 1) => engine.lt.tuning = value,
            (3, 0) => engine.mt.level = value,
            (3, 1) => engine.mt.tuning = value,
            (4, 0) => engine.ht.level = value,
            (4, 1) => engine.ht.tuning = value,
            // RS: 0=level
            (5, 0) => engine.rs.level = value,
            // CP: 0=level
            (6, 0) => engine.cp.level = value,
            // CH: 0=level
            (7, 0) => engine.ch.level = value,
            // OH: 0=level, 1=decay
            (8, 0) => engine.oh.level = value,
            (8, 1) => engine.oh.decay = value,
            // CY: 0=level, 1=decay
            (9, 0) => engine.cy.level = value,
            (9, 1) => engine.cy.decay = value,
            // CB: 0=level
            (10, 0) => engine.cb.level = value,
            // MA: 0=level
            (11, 0) => engine.ma.level = value,
            // CL: 0=level
            (12, 0) => engine.cl.level = value,
            // Master volume
            (255, 0) => engine.master_volume = value,
            _ => {}
        }
    }
}
