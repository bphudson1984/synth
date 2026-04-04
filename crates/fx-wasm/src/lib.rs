//! FX rack WASM wrapper.
//! Provides 5 stereo effect processors (chorus, delay, reverb, distortion, octave)
//! with 5 stereo input buffers and 1 summed stereo output.

use fx_dsp::chorus::StereoChorus;
use fx_dsp::delay::TapeDelay;
use fx_dsp::reverb::PlateReverb;
use fx_dsp::distortion::TubeDistortion;
use fx_dsp::octave::OctavePedal;

const BUF: usize = 512;

// 5 stereo input buffer pairs (one per effect bus)
static mut CHORUS_IN_L: [f32; BUF] = [0.0; BUF];
static mut CHORUS_IN_R: [f32; BUF] = [0.0; BUF];
static mut DELAY_IN_L:  [f32; BUF] = [0.0; BUF];
static mut DELAY_IN_R:  [f32; BUF] = [0.0; BUF];
static mut REVERB_IN_L: [f32; BUF] = [0.0; BUF];
static mut REVERB_IN_R: [f32; BUF] = [0.0; BUF];
static mut DIST_IN_L:   [f32; BUF] = [0.0; BUF];
static mut DIST_IN_R:   [f32; BUF] = [0.0; BUF];
static mut OCTAVE_IN_L: [f32; BUF] = [0.0; BUF];
static mut OCTAVE_IN_R: [f32; BUF] = [0.0; BUF];

// 1 stereo output (sum of all 5 wet signals)
static mut OUT_L: [f32; BUF] = [0.0; BUF];
static mut OUT_R: [f32; BUF] = [0.0; BUF];

static mut CHORUS: Option<StereoChorus> = None;
static mut DELAY:  Option<TapeDelay> = None;
static mut REVERB: Option<PlateReverb> = None;
static mut DIST:   Option<TubeDistortion> = None;
static mut OCTAVE: Option<OctavePedal> = None;

#[no_mangle]
pub extern "C" fn init(sample_rate: f32) {
    unsafe {
        let mut chorus = StereoChorus::new(sample_rate);
        chorus.mix = 1.0; // 100% wet in send/return topology
        CHORUS = Some(chorus);

        let mut delay = TapeDelay::new(sample_rate);
        delay.mix = 1.0;
        DELAY = Some(delay);

        let mut reverb = PlateReverb::new(sample_rate);
        reverb.mix = 1.0;
        REVERB = Some(reverb);

        let mut dist = TubeDistortion::new(sample_rate);
        dist.mix = 1.0;
        DIST = Some(dist);

        let octave = OctavePedal::new(sample_rate);
        OCTAVE = Some(octave);
    }
}

#[no_mangle]
pub extern "C" fn process(num_samples: u32) {
    unsafe {
        let chorus = CHORUS.as_mut().unwrap();
        let delay = DELAY.as_mut().unwrap();
        let reverb = REVERB.as_mut().unwrap();
        let dist = DIST.as_mut().unwrap();
        let octave = OCTAVE.as_mut().unwrap();

        let n = (num_samples as usize).min(BUF);
        for i in 0..n {
            // Chorus: stereo in -> stereo out
            let (ch_l, ch_r) = chorus.process_stereo(CHORUS_IN_L[i], CHORUS_IN_R[i]);

            // Delay: stereo in -> stereo out
            let (dl_l, dl_r) = delay.process(DELAY_IN_L[i], DELAY_IN_R[i]);

            // Reverb: stereo in -> stereo out
            let (rv_l, rv_r) = reverb.process_stereo(REVERB_IN_L[i], REVERB_IN_R[i]);

            // Distortion: stereo in -> stereo out
            let (dt_l, dt_r) = dist.process(DIST_IN_L[i], DIST_IN_R[i]);

            // Octave: stereo in -> stereo out
            let (oc_l, oc_r) = octave.process(OCTAVE_IN_L[i], OCTAVE_IN_R[i]);

            // Sum all wet outputs
            OUT_L[i] = ch_l + dl_l + rv_l + dt_l + oc_l;
            OUT_R[i] = ch_r + dl_r + rv_r + dt_r + oc_r;
        }
    }
}

/// Set an effect parameter.
/// effect_id: 0=chorus, 1=delay, 2=reverb, 3=distortion, 4=octave
#[no_mangle]
pub extern "C" fn set_param(effect_id: u32, param_id: u32, value: f32) {
    unsafe {
        match effect_id {
            // Chorus: 0=rate, 1=depth
            0 => if let Some(c) = CHORUS.as_mut() {
                match param_id {
                    0 => c.rate = value,
                    1 => c.depth = value,
                    _ => {}
                }
            },
            // Delay: 0=time_ms, 1=feedback, 2=tone
            1 => if let Some(d) = DELAY.as_mut() {
                match param_id {
                    0 => d.time_ms = value,
                    1 => d.feedback = value.min(0.95),
                    2 => d.tone = value,
                    _ => {}
                }
            },
            // Reverb: 0=decay, 1=damping
            2 => if let Some(r) = REVERB.as_mut() {
                match param_id {
                    0 => r.decay = value.min(0.99),
                    1 => r.damping = value,
                    _ => {}
                }
            },
            // Distortion: 0=drive, 1=tone, 2=level
            3 => if let Some(d) = DIST.as_mut() {
                match param_id {
                    0 => d.drive = value,
                    1 => d.tone = value,
                    2 => d.level = value,
                    _ => {}
                }
            },
            // Octave: 0=dry, 1=sub, 2=up
            4 => if let Some(o) = OCTAVE.as_mut() {
                match param_id {
                    0 => o.dry = value,
                    1 => o.sub = value,
                    2 => o.up = value,
                    _ => {}
                }
            },
            _ => {}
        }
    }
}

// --- Buffer pointer getters ---
#[no_mangle] pub extern "C" fn get_chorus_in_l_ptr() -> *const f32 { unsafe { CHORUS_IN_L.as_ptr() } }
#[no_mangle] pub extern "C" fn get_chorus_in_r_ptr() -> *const f32 { unsafe { CHORUS_IN_R.as_ptr() } }
#[no_mangle] pub extern "C" fn get_delay_in_l_ptr()  -> *const f32 { unsafe { DELAY_IN_L.as_ptr() } }
#[no_mangle] pub extern "C" fn get_delay_in_r_ptr()  -> *const f32 { unsafe { DELAY_IN_R.as_ptr() } }
#[no_mangle] pub extern "C" fn get_reverb_in_l_ptr() -> *const f32 { unsafe { REVERB_IN_L.as_ptr() } }
#[no_mangle] pub extern "C" fn get_reverb_in_r_ptr() -> *const f32 { unsafe { REVERB_IN_R.as_ptr() } }
#[no_mangle] pub extern "C" fn get_dist_in_l_ptr()   -> *const f32 { unsafe { DIST_IN_L.as_ptr() } }
#[no_mangle] pub extern "C" fn get_dist_in_r_ptr()   -> *const f32 { unsafe { DIST_IN_R.as_ptr() } }
#[no_mangle] pub extern "C" fn get_octave_in_l_ptr() -> *const f32 { unsafe { OCTAVE_IN_L.as_ptr() } }
#[no_mangle] pub extern "C" fn get_octave_in_r_ptr() -> *const f32 { unsafe { OCTAVE_IN_R.as_ptr() } }
#[no_mangle] pub extern "C" fn get_out_l_ptr()       -> *const f32 { unsafe { OUT_L.as_ptr() } }
#[no_mangle] pub extern "C" fn get_out_r_ptr()       -> *const f32 { unsafe { OUT_R.as_ptr() } }
