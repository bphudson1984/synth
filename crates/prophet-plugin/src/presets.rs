use nih_plug::prelude::*;
use crate::ProphetParams;
use std::sync::Arc;

pub fn apply_preset(preset: &Preset, params: &Arc<ProphetParams>, setter: &ParamSetter) {
    set_bool(setter, &params.osc_a_saw, preset.osc_a_saw);
    set_bool(setter, &params.osc_a_pulse, preset.osc_a_pulse);
    set_bool(setter, &params.osc_b_saw, preset.osc_b_saw);
    set_bool(setter, &params.osc_b_tri, preset.osc_b_tri);
    set_bool(setter, &params.osc_b_pulse, preset.osc_b_pulse);
    set_float(setter, &params.osc_a_pw, preset.osc_a_pw);
    set_float(setter, &params.osc_b_pw, preset.osc_b_pw);
    set_float(setter, &params.osc_b_semitones, preset.osc_b_semi);
    set_float(setter, &params.osc_b_fine, preset.osc_b_fine);
    set_float(setter, &params.osc_a_level, preset.osc_a_level);
    set_float(setter, &params.osc_b_level, preset.osc_b_level);
    set_float(setter, &params.noise_level, preset.noise_level);
    set_float(setter, &params.filter_cutoff, preset.filter_cutoff);
    set_float(setter, &params.filter_resonance, preset.filter_resonance);
    set_float(setter, &params.filter_env_amount, preset.filter_env_amount);
    set_float(setter, &params.filter_drive, preset.filter_drive);
    set_float(setter, &params.filter_attack, preset.f_attack);
    set_float(setter, &params.filter_decay, preset.f_decay);
    set_float(setter, &params.filter_sustain, preset.f_sustain);
    set_float(setter, &params.filter_release, preset.f_release);
    set_float(setter, &params.amp_attack, preset.a_attack);
    set_float(setter, &params.amp_decay, preset.a_decay);
    set_float(setter, &params.amp_sustain, preset.a_sustain);
    set_float(setter, &params.amp_release, preset.a_release);
    set_bool(setter, &params.sync, preset.sync);
    set_float(setter, &params.poly_mod_filt_env, preset.pm_filt_env);
    set_float(setter, &params.poly_mod_osc_b, preset.pm_osc_b);
    set_bool(setter, &params.poly_mod_freq_a, preset.pm_freq_a);
    set_bool(setter, &params.poly_mod_pw_a, preset.pm_pw_a);
    set_bool(setter, &params.poly_mod_filter, preset.pm_filter);
    set_float(setter, &params.lfo_freq, preset.lfo_freq);
    set_float(setter, &params.lfo_initial_amount, preset.lfo_amount);
    set_bool(setter, &params.lfo_tri, preset.lfo_tri);
    set_bool(setter, &params.lfo_saw, preset.lfo_saw);
    set_bool(setter, &params.lfo_square, preset.lfo_square);
    set_float(setter, &params.wheel_mod_mix, preset.wm_mix);
    set_bool(setter, &params.wheel_mod_freq_a, preset.wm_freq_a);
    set_bool(setter, &params.wheel_mod_freq_b, preset.wm_freq_b);
    set_bool(setter, &params.wheel_mod_pw_a, preset.wm_pw_a);
    set_bool(setter, &params.wheel_mod_pw_b, preset.wm_pw_b);
    set_bool(setter, &params.wheel_mod_filter, preset.wm_filter);
    set_float(setter, &params.glide_rate, preset.glide_rate);
    set_bool(setter, &params.glide_on, preset.glide_on);
    set_bool(setter, &params.unison, preset.unison);
    set_float(setter, &params.drift, preset.drift);
    set_float(setter, &params.volume, preset.volume);
    // Effects
    set_float(setter, &params.chorus_rate, preset.chorus_rate);
    set_float(setter, &params.chorus_depth, preset.chorus_depth);
    set_float(setter, &params.chorus_mix, preset.chorus_mix);
    set_float(setter, &params.delay_time, preset.delay_time);
    set_float(setter, &params.delay_feedback, preset.delay_feedback);
    set_float(setter, &params.delay_tone, preset.delay_tone);
    set_float(setter, &params.delay_mix, preset.delay_mix);
    set_float(setter, &params.reverb_decay, preset.reverb_decay);
    set_float(setter, &params.reverb_damping, preset.reverb_damping);
    set_float(setter, &params.reverb_mix, preset.reverb_mix);
}

fn set_float(setter: &ParamSetter, param: &FloatParam, value: f32) {
    setter.begin_set_parameter(param);
    setter.set_parameter(param, value);
    setter.end_set_parameter(param);
}

fn set_bool(setter: &ParamSetter, param: &BoolParam, value: bool) {
    setter.begin_set_parameter(param);
    setter.set_parameter(param, value);
    setter.end_set_parameter(param);
}

// =====================================================================
// PRESET DATA
// =====================================================================

pub struct Preset {
    pub name: &'static str,
    pub category: &'static str,
    pub osc_a_saw: bool, pub osc_a_pulse: bool,
    pub osc_b_saw: bool, pub osc_b_tri: bool, pub osc_b_pulse: bool,
    pub osc_a_pw: f32, pub osc_b_pw: f32,
    pub osc_b_semi: f32,  // semitone offset (-24..+24)
    pub osc_b_fine: f32,  // fine tune in cents (-100..+100)
    pub osc_a_level: f32, pub osc_b_level: f32, pub noise_level: f32,
    pub filter_cutoff: f32, pub filter_resonance: f32, pub filter_env_amount: f32,
    pub filter_drive: f32,
    pub f_attack: f32, pub f_decay: f32, pub f_sustain: f32, pub f_release: f32,
    pub a_attack: f32, pub a_decay: f32, pub a_sustain: f32, pub a_release: f32,
    pub sync: bool,
    pub pm_filt_env: f32, pub pm_osc_b: f32,
    pub pm_freq_a: bool, pub pm_pw_a: bool, pub pm_filter: bool,
    pub lfo_freq: f32, pub lfo_tri: bool, pub lfo_saw: bool, pub lfo_square: bool,
    pub lfo_amount: f32, // initial LFO modulation depth (always on)
    pub wm_mix: f32,
    pub wm_freq_a: bool, pub wm_freq_b: bool,
    pub wm_pw_a: bool, pub wm_pw_b: bool, pub wm_filter: bool,
    pub glide_rate: f32, pub glide_on: bool, pub unison: bool,
    pub drift: f32, pub volume: f32,
    // Effects
    pub chorus_rate: f32, pub chorus_depth: f32, pub chorus_mix: f32,
    pub delay_time: f32, pub delay_feedback: f32, pub delay_tone: f32, pub delay_mix: f32,
    pub reverb_decay: f32, pub reverb_damping: f32, pub reverb_mix: f32,
}

impl Default for Preset {
    fn default() -> Self {
        Self {
            name: "Init", category: "Utility",
            osc_a_saw: true, osc_a_pulse: false,
            osc_b_saw: false, osc_b_tri: false, osc_b_pulse: false,
            osc_a_pw: 0.5, osc_b_pw: 0.5,
            osc_b_semi: 0.0, osc_b_fine: 0.0,
            osc_a_level: 1.0, osc_b_level: 0.0, noise_level: 0.0,
            filter_cutoff: 10000.0, filter_resonance: 0.0, filter_env_amount: 5000.0,
            filter_drive: 1.0,
            f_attack: 0.01, f_decay: 0.3, f_sustain: 0.2, f_release: 0.3,
            a_attack: 0.005, a_decay: 0.3, a_sustain: 0.8, a_release: 0.3,
            sync: false,
            pm_filt_env: 0.0, pm_osc_b: 0.0,
            pm_freq_a: false, pm_pw_a: false, pm_filter: false,
            lfo_freq: 5.0, lfo_tri: true, lfo_saw: false, lfo_square: false,
            lfo_amount: 0.0,
            wm_mix: 0.0, wm_freq_a: false, wm_freq_b: false,
            wm_pw_a: false, wm_pw_b: false, wm_filter: false,
            glide_rate: 0.1, glide_on: false, unison: false,
            drift: 1.0, volume: -6.0,
            chorus_rate: 0.8, chorus_depth: 0.5, chorus_mix: 0.0,
            delay_time: 375.0, delay_feedback: 0.4, delay_tone: 0.6, delay_mix: 0.0,
            reverb_decay: 0.7, reverb_damping: 0.7, reverb_mix: 0.0,
        }
    }
}

/// All categories in display order.
pub fn categories() -> &'static [&'static str] {
    &["Brass", "Pads", "Leads", "Bass", "Keys", "Strings", "Film/TV", "SFX", "Utility"]
}

pub fn factory_presets() -> Vec<Preset> {
    vec![
        // =========================== BRASS ===========================

        // The definitive polysynth brass — Talking Heads, Duran Duran, ABBA
        Preset {
            name: "Prophet Brass", category: "Brass",
            osc_a_saw: true, osc_b_saw: true,
            osc_b_fine: 5.0,
            osc_a_level: 0.8, osc_b_level: 0.8,
            filter_cutoff: 400.0, filter_resonance: 0.15, filter_env_amount: 6000.0,
            f_attack: 0.003, f_decay: 0.35, f_sustain: 0.5, f_release: 0.15,
            a_attack: 0.003, a_decay: 0.3, a_sustain: 0.8, a_release: 0.15,
            // FIX: vibrato via mod wheel
            lfo_freq: 5.5, wm_freq_a: true, wm_freq_b: true,
            drift: 1.5,
            ..Preset::default()
        },
        // Softer, more expressive — French horn quality
        Preset {
            name: "Soft Brass", category: "Brass",
            osc_a_saw: true, osc_b_saw: true,
            osc_b_fine: 7.0,
            osc_a_level: 0.7, osc_b_level: 0.7,
            // FIX: more resonance for horn "honk"
            filter_cutoff: 350.0, filter_resonance: 0.2, filter_env_amount: 4000.0,
            f_attack: 0.05, f_decay: 0.5, f_sustain: 0.6, f_release: 0.3,
            a_attack: 0.05, a_decay: 0.4, a_sustain: 0.85, a_release: 0.25,
            lfo_freq: 5.5, wm_freq_a: true, wm_freq_b: true,
            drift: 2.0,
            ..Preset::default()
        },
        // Punchy stab — Tears for Fears "Shout" bass layer
        Preset {
            name: "Shout Stab", category: "Brass",
            osc_a_saw: true, osc_b_saw: true,
            // FIX: add detune, lower sustain for stab character
            osc_b_fine: 5.0,
            osc_a_level: 0.8, osc_b_level: 0.8,
            filter_cutoff: 400.0, filter_resonance: 0.15, filter_env_amount: 3000.0,
            f_attack: 0.001, f_decay: 0.3, f_sustain: 0.4, f_release: 0.1,
            a_attack: 0.001, a_decay: 0.3, a_sustain: 0.4, a_release: 0.13,
            unison: true, drift: 2.0,
            ..Preset::default()
        },

        // ============================ PADS ============================

        // Classic PWM pad — Phil Collins "In The Air Tonight"
        Preset {
            name: "In The Air", category: "Pads",
            osc_a_pulse: true, osc_b_saw: true,
            osc_a_pw: 0.35, osc_b_fine: 8.0,
            osc_a_level: 0.7, osc_b_level: 0.5,
            filter_cutoff: 2000.0, filter_resonance: 0.08, filter_env_amount: 1500.0,
            f_attack: 0.5, f_decay: 1.5, f_sustain: 0.7, f_release: 1.5,
            a_attack: 0.6, a_decay: 0.5, a_sustain: 1.0, a_release: 1.8,
            // FIX: lfo_amount provides always-on PWM
            lfo_freq: 1.2, lfo_amount: 0.6, wm_pw_a: true, drift: 2.0,
            chorus_mix: 0.4, reverb_decay: 0.8, reverb_mix: 0.3,
            ..Preset::default()
        },
        // Tears for Fears "EWTRTW" — verified exact values from Reverb Machine
        Preset {
            name: "Rule The World", category: "Pads",
            osc_a_pulse: true, osc_b_pulse: true,
            osc_a_pw: 0.31, osc_b_pw: 0.83,
            // FIX: add detune between oscillators
            osc_b_fine: 7.0,
            osc_a_level: 0.7, osc_b_level: 0.7,
            filter_cutoff: 482.0, filter_resonance: 0.1, filter_env_amount: 3750.0,
            f_attack: 0.01, f_decay: 3.27, f_sustain: 0.15, f_release: 0.3,
            a_attack: 0.01, a_decay: 0.5, a_sustain: 0.9, a_release: 0.4,
            // FIX: always-on PWM shimmer
            lfo_freq: 6.82, lfo_amount: 0.4, wm_pw_a: true, drift: 1.5,
            chorus_mix: 0.35, reverb_decay: 0.75, reverb_mix: 0.2,
            ..Preset::default()
        },
        // Dreamy dual-PWM — Michael Jackson "Human Nature"
        Preset {
            name: "Human Nature", category: "Pads",
            osc_a_pulse: true, osc_b_pulse: true,
            osc_a_pw: 0.35, osc_b_pw: 0.6,
            // FIX: detune + always-on PWM
            osc_b_fine: 6.0,
            osc_a_level: 0.6, osc_b_level: 0.5,
            filter_cutoff: 1500.0, filter_resonance: 0.1, filter_env_amount: 800.0,
            f_attack: 0.8, f_decay: 2.0, f_sustain: 0.6, f_release: 2.0,
            a_attack: 0.7, a_decay: 0.5, a_sustain: 1.0, a_release: 2.5,
            lfo_freq: 0.8, lfo_amount: 0.5, wm_pw_a: true, wm_pw_b: true, drift: 2.5,
            chorus_mix: 0.3, reverb_decay: 0.85, reverb_mix: 0.35,
            ..Preset::default()
        },
        // Dark cinematic — Blade Runner style
        Preset {
            name: "Blade Runner", category: "Pads",
            osc_a_saw: true, osc_b_saw: true,
            osc_b_fine: 5.0, // gentle detune for warmth
            osc_a_level: 0.7, osc_b_level: 0.7, noise_level: 0.05,
            filter_cutoff: 800.0, filter_resonance: 0.18, filter_env_amount: 2000.0,
            f_attack: 1.0, f_decay: 3.0, f_sustain: 0.5, f_release: 2.5,
            a_attack: 0.8, a_decay: 1.0, a_sustain: 1.0, a_release: 3.0,
            // FIX: always-on slow filter sweep
            lfo_freq: 0.2, lfo_amount: 0.3, wm_filter: true, drift: 3.0,
            reverb_decay: 0.9, reverb_damping: 0.5, reverb_mix: 0.4,
            ..Preset::default()
        },
        // Japan "Ghosts" — Richard Barbieri's drone
        Preset {
            name: "Ghosts Drone", category: "Pads",
            // FIX: add pulse on Osc A too for PW mod, plus detune
            osc_a_saw: true, osc_a_pulse: true, osc_b_pulse: true,
            osc_a_pw: 0.4, osc_b_pw: 0.35, osc_b_fine: 5.0,
            osc_a_level: 0.5, osc_b_level: 0.6, noise_level: 0.03,
            filter_cutoff: 1200.0, filter_resonance: 0.15, filter_env_amount: 800.0,
            f_attack: 1.0, f_decay: 3.0, f_sustain: 0.5, f_release: 2.5,
            a_attack: 1.5, a_decay: 1.0, a_sustain: 1.0, a_release: 3.0,
            pm_osc_b: 0.15, pm_filter: true,
            // FIX: always-on LFO for filter + PW movement
            lfo_freq: 0.3, lfo_amount: 0.4, wm_filter: true, wm_pw_a: true, drift: 3.5,
            ..Preset::default()
        },
        // Depeche Mode "Enjoy The Silence" — warm breathy pad
        Preset {
            name: "Enjoy Silence", category: "Pads",
            osc_a_pulse: true, osc_b_saw: true,
            // FIX: add detune + always-on PWM
            osc_a_pw: 0.35, osc_b_fine: 6.0,
            osc_a_level: 0.7, osc_b_level: 0.5, noise_level: 0.03,
            filter_cutoff: 1800.0, filter_resonance: 0.08, filter_env_amount: 1200.0,
            f_attack: 0.6, f_decay: 1.8, f_sustain: 0.65, f_release: 2.0,
            a_attack: 0.7, a_decay: 0.5, a_sustain: 1.0, a_release: 2.5,
            lfo_freq: 1.0, lfo_amount: 0.5, wm_pw_a: true, drift: 2.5,
            ..Preset::default()
        },
        // Twin Peaks — Laura Palmer's Theme
        Preset {
            name: "Twin Peaks", category: "Pads",
            // FIX: use pulse for PW mod (was saw which can't be PW modulated)
            osc_a_pulse: true, osc_b_saw: true,
            osc_a_pw: 0.4, osc_b_fine: 6.0,
            osc_a_level: 0.65, osc_b_level: 0.65, noise_level: 0.02,
            filter_cutoff: 1200.0, filter_resonance: 0.08, filter_env_amount: 800.0,
            f_attack: 0.8, f_decay: 2.0, f_sustain: 0.5, f_release: 2.0,
            a_attack: 1.0, a_decay: 0.5, a_sustain: 1.0, a_release: 3.0,
            // FIX: always-on slow PWM for dreamy movement
            lfo_freq: 0.5, lfo_amount: 0.4, wm_pw_a: true, drift: 3.0,
            ..Preset::default()
        },
        // Terminator — haunting pad
        Preset {
            name: "Terminator", category: "Pads",
            osc_a_saw: true, osc_b_pulse: true,
            // FIX: add detune
            osc_b_pw: 0.4, osc_b_fine: 5.0,
            osc_a_level: 0.7, osc_b_level: 0.6, noise_level: 0.03,
            filter_cutoff: 600.0, filter_resonance: 0.2, filter_env_amount: 1500.0,
            f_attack: 1.5, f_decay: 3.0, f_sustain: 0.4, f_release: 2.0,
            a_attack: 1.0, a_decay: 1.0, a_sustain: 0.9, a_release: 2.5,
            // FIX: always-on slow filter movement
            lfo_freq: 0.2, lfo_amount: 0.3, wm_filter: true, drift: 3.0,
            ..Preset::default()
        },

        // =========================== LEADS ===========================

        // Classic sync lead — The Cars "Let's Go", Gary Numan
        Preset {
            name: "Sync Lead", category: "Leads",
            osc_a_saw: true, osc_b_saw: true,
            // FIX: don't mix Osc B directly — only hear sync'd Osc A
            osc_a_level: 1.0, osc_b_level: 0.0,
            filter_cutoff: 8000.0, filter_resonance: 0.1, filter_env_amount: 3000.0,
            f_attack: 0.001, f_decay: 0.5, f_sustain: 0.3, f_release: 0.2,
            a_attack: 0.001, a_decay: 0.3, a_sustain: 0.85, a_release: 0.2,
            sync: true, pm_filt_env: 0.6, pm_freq_a: true,
            drift: 1.0,
            delay_time: 350.0, delay_feedback: 0.3, delay_mix: 0.2,
            reverb_decay: 0.6, reverb_mix: 0.15,
            ..Preset::default()
        },
        // Massive unison lead
        Preset {
            name: "Unison Lead", category: "Leads",
            osc_a_saw: true, osc_b_saw: true,
            // FIX: detune Osc B within each voice for extra fatness
            osc_b_fine: 5.0,
            osc_a_level: 0.8, osc_b_level: 0.8,
            filter_cutoff: 800.0, filter_resonance: 0.2, filter_env_amount: 6000.0,
            f_attack: 0.001, f_decay: 0.5, f_sustain: 0.5, f_release: 0.3,
            a_attack: 0.001, a_decay: 0.3, a_sustain: 0.9, a_release: 0.2,
            unison: true, glide_on: true, glide_rate: 0.06,
            lfo_freq: 5.5, wm_freq_a: true, wm_freq_b: true, drift: 2.5,
            ..Preset::default()
        },
        // MGMT "Kids" — honky resonant square
        Preset {
            name: "Kids", category: "Leads",
            osc_a_pulse: true,
            osc_a_pw: 0.5, osc_a_level: 1.0,
            filter_cutoff: 1687.0, filter_resonance: 0.57, filter_env_amount: 0.0,
            a_attack: 0.01, a_decay: 0.3, a_sustain: 0.85, a_release: 0.2,
            // FIX: always-on vibrato (defining characteristic)
            lfo_freq: 5.65, lfo_amount: 0.3, wm_freq_a: true, drift: 1.0,
            ..Preset::default()
        },
        // Yazoo "Don't Go" — brassy squelch
        Preset {
            name: "Don't Go", category: "Leads",
            osc_a_saw: true, osc_b_saw: true,
            // FIX: Osc B TWO OCTAVES up — the bright cutting character
            osc_b_semi: 24.0,
            osc_a_level: 0.2, osc_b_level: 0.8,
            filter_cutoff: 3200.0, filter_resonance: 0.0, filter_env_amount: 2750.0,
            f_attack: 0.02, f_decay: 0.15, f_sustain: 0.0, f_release: 0.1,
            a_attack: 0.01, a_decay: 0.3, a_sustain: 1.0, a_release: 0.015,
            drift: 1.0,
            ..Preset::default()
        },
        // OMD "Enola Gay" — piercing bright lead
        Preset {
            name: "Enola Gay", category: "Leads",
            osc_a_pulse: true,
            osc_a_pw: 0.5, osc_a_level: 1.0, osc_b_level: 0.1,
            filter_cutoff: 6000.0, filter_resonance: 0.45, filter_env_amount: 2000.0,
            f_attack: 0.001, f_decay: 0.4, f_sustain: 0.2, f_release: 0.15,
            a_attack: 0.01, a_decay: 0.4, a_sustain: 0.2, a_release: 0.15,
            drift: 0.5,
            ..Preset::default()
        },
        // M83 "Midnight City" — sync sweep
        Preset {
            name: "Midnight City", category: "Leads",
            osc_a_saw: true, osc_b_saw: true,
            osc_a_level: 0.7, osc_b_level: 0.5,
            sync: true,
            filter_cutoff: 4000.0, filter_resonance: 0.2, filter_env_amount: 3000.0,
            f_attack: 0.1, f_decay: 1.0, f_sustain: 0.5, f_release: 0.5,
            a_attack: 0.05, a_decay: 0.5, a_sustain: 0.85, a_release: 0.5,
            pm_filt_env: 0.4, pm_freq_a: true,
            lfo_freq: 0.3, wm_filter: true, drift: 2.0,
            ..Preset::default()
        },
        // Gary Numan "Cars" — lead
        Preset {
            name: "Cars", category: "Leads",
            osc_a_saw: true,
            osc_a_level: 1.0,
            filter_cutoff: 1600.0, filter_resonance: 0.3, filter_env_amount: 0.0,
            a_attack: 0.14, a_decay: 0.3, a_sustain: 1.0, a_release: 0.65,
            // FIX: always-on vibrato for Polymoog character
            lfo_freq: 5.0, lfo_amount: 0.15, wm_freq_a: true, drift: 1.5,
            ..Preset::default()
        },

        // ============================ BASS ============================

        // Punchy dual-saw bass — Talking Heads, INXS
        Preset {
            name: "Fat Bass", category: "Bass",
            osc_a_saw: true, osc_b_saw: true,
            osc_b_semi: -12.0, // Osc B one octave lower for sub weight
            osc_a_level: 0.6, osc_b_level: 1.0,
            filter_cutoff: 300.0, filter_resonance: 0.2, filter_env_amount: 4000.0,
            f_attack: 0.001, f_decay: 0.2, f_sustain: 0.15, f_release: 0.05,
            a_attack: 0.001, a_decay: 0.4, a_sustain: 0.7, a_release: 0.08,
            drift: 0.5,
            ..Preset::default()
        },
        // New Order "Blue Monday" — from Syntorial recipe
        Preset {
            name: "Blue Monday", category: "Bass",
            osc_a_saw: true, osc_b_pulse: true,
            osc_b_pw: 0.99, osc_b_semi: -12.0,
            osc_a_level: 0.7, osc_b_level: 0.3,
            filter_cutoff: 2200.0, filter_resonance: 0.0, filter_env_amount: 1250.0,
            f_attack: 0.001, f_decay: 0.3, f_sustain: 0.0, f_release: 10.0,
            a_attack: 0.001, a_decay: 0.25, a_sustain: 0.2, a_release: 0.16,
            unison: true, drift: 1.0,
            ..Preset::default()
        },
        // Acid-style resonant squelch
        Preset {
            name: "Acid Bass", category: "Bass",
            osc_a_saw: true,
            osc_a_level: 1.0,
            filter_cutoff: 180.0, filter_resonance: 0.7, filter_env_amount: 7000.0,
            f_attack: 0.001, f_decay: 0.2, f_sustain: 0.0, f_release: 0.05,
            a_attack: 0.001, a_decay: 0.3, a_sustain: 0.6, a_release: 0.05,
            drift: 0.5,
            ..Preset::default()
        },
        // Depeche Mode "Personal Jesus" — aggressive filter hook
        Preset {
            name: "Personal Jesus", category: "Bass",
            osc_a_saw: true, osc_b_pulse: true,
            osc_b_pw: 0.5, osc_b_semi: -12.0,
            osc_a_level: 0.8, osc_b_level: 0.6,
            filter_cutoff: 350.0, filter_resonance: 0.65, filter_env_amount: 5000.0,
            f_attack: 0.001, f_decay: 0.25, f_sustain: 0.1, f_release: 0.05,
            a_attack: 0.001, a_decay: 0.3, a_sustain: 0.7, a_release: 0.08,
            drift: 1.0,
            ..Preset::default()
        },
        // Carpenter "Escape from NY" bass
        Preset {
            name: "Escape Bass", category: "Bass",
            osc_a_saw: true, osc_b_saw: true,
            // FIX: detune + sub octave
            osc_b_semi: -12.0, osc_b_fine: 3.0,
            osc_a_level: 0.8, osc_b_level: 0.8,
            filter_cutoff: 300.0, filter_resonance: 0.25, filter_env_amount: 3000.0,
            f_attack: 0.001, f_decay: 0.3, f_sustain: 0.2, f_release: 0.1,
            a_attack: 0.001, a_decay: 0.4, a_sustain: 0.7, a_release: 0.1,
            unison: true, drift: 2.0,
            ..Preset::default()
        },
        // Daft Punk "Giorgio" — filter sweep bass
        Preset {
            name: "Giorgio Bass", category: "Bass",
            osc_a_saw: true,
            osc_a_level: 1.0,
            filter_cutoff: 200.0, filter_resonance: 0.3, filter_env_amount: 8000.0,
            f_attack: 0.001, f_decay: 1.0, f_sustain: 0.0, f_release: 0.5,
            a_attack: 0.001, a_decay: 0.5, a_sustain: 0.6, a_release: 0.2,
            drift: 1.5,
            ..Preset::default()
        },

        // ============================ KEYS ============================

        // Classic tonewheel organ
        Preset {
            name: "Organ", category: "Keys",
            osc_a_pulse: true, osc_b_pulse: true,
            osc_a_pw: 0.5, osc_b_pw: 0.5,
            // FIX: Osc B up an octave for drawbar registration, zero drift for clean
            osc_b_semi: 12.0,
            osc_a_level: 0.7, osc_b_level: 0.5,
            filter_cutoff: 6000.0, filter_resonance: 0.0, filter_env_amount: 0.0,
            f_attack: 0.001, f_decay: 0.1, f_sustain: 1.0, f_release: 0.01,
            a_attack: 0.001, a_decay: 0.05, a_sustain: 1.0, a_release: 0.03,
            lfo_freq: 6.0, wm_freq_a: true, wm_freq_b: true, drift: 0.0,
            ..Preset::default()
        },
        // Peter Gabriel "Sledgehammer" — vibrato organ
        Preset {
            name: "Sledgehammer", category: "Keys",
            osc_a_saw: true, osc_b_pulse: true,
            osc_b_pw: 0.5,
            // FIX: Osc B up an octave + always-on vibrato
            osc_b_semi: 12.0,
            osc_a_level: 0.6, osc_b_level: 0.5,
            filter_cutoff: 5000.0, filter_resonance: 0.0, filter_env_amount: 0.0,
            a_attack: 0.001, a_decay: 0.05, a_sustain: 1.0, a_release: 0.03,
            lfo_freq: 6.0, lfo_amount: 0.3, wm_freq_a: true, wm_freq_b: true, drift: 0.5,
            ..Preset::default()
        },
        // Percussive clav/pluck
        Preset {
            name: "Clav Pluck", category: "Keys",
            osc_a_pulse: true, osc_b_saw: true,
            osc_a_pw: 0.4,
            osc_a_level: 0.8, osc_b_level: 0.6,
            filter_cutoff: 200.0, filter_resonance: 0.25, filter_env_amount: 8000.0,
            f_attack: 0.001, f_decay: 0.15, f_sustain: 0.0, f_release: 0.08,
            a_attack: 0.001, a_decay: 0.4, a_sustain: 0.0, a_release: 0.15,
            drift: 0.8,
            ..Preset::default()
        },
        // A-ha "Take On Me" — bouncy staccato
        Preset {
            name: "Take On Me", category: "Keys",
            osc_a_pulse: true, osc_b_saw: true,
            // FIX: Osc B one octave up for bright character
            osc_a_pw: 0.4, osc_b_semi: 12.0,
            osc_a_level: 0.8, osc_b_level: 0.5,
            filter_cutoff: 5000.0, filter_resonance: 0.15, filter_env_amount: 2000.0,
            f_attack: 0.001, f_decay: 0.3, f_sustain: 0.2, f_release: 0.1,
            a_attack: 0.001, a_decay: 0.25, a_sustain: 0.0, a_release: 0.1,
            drift: 1.0,
            ..Preset::default()
        },
        // Sweet Dreams sequence — Eurythmics
        Preset {
            name: "Sweet Dreams", category: "Keys",
            osc_a_pulse: true, osc_b_saw: true,
            // FIX: detune + always-on PWM
            osc_a_pw: 0.45, osc_b_fine: 8.0,
            osc_a_level: 0.6, osc_b_level: 0.6,
            filter_cutoff: 8000.0, filter_resonance: 0.0, filter_env_amount: 0.0,
            a_attack: 0.03, a_decay: 0.45, a_sustain: 0.0, a_release: 0.4,
            lfo_freq: 2.5, lfo_amount: 0.4, wm_pw_a: true, drift: 1.0,
            ..Preset::default()
        },

        // ========================== STRINGS ==========================

        // Lush poly strings — Genesis, Kate Bush
        Preset {
            name: "Prophet Strings", category: "Strings",
            osc_a_pulse: true, osc_b_saw: true,
            osc_a_pw: 0.4, osc_b_fine: 7.0,
            osc_a_level: 0.65, osc_b_level: 0.65,
            filter_cutoff: 2500.0, filter_resonance: 0.05, filter_env_amount: 1000.0,
            f_attack: 0.4, f_decay: 0.8, f_sustain: 0.7, f_release: 0.8,
            a_attack: 0.4, a_decay: 0.5, a_sustain: 0.9, a_release: 0.7,
            // FIX: always-on PWM for string movement
            lfo_freq: 1.5, lfo_amount: 0.5, wm_pw_a: true, drift: 2.0,
            ..Preset::default()
        },
        // Yazoo "Only You" — melancholic string pad
        Preset {
            name: "Only You", category: "Strings",
            osc_a_pulse: true, osc_b_saw: true,
            // FIX: add detune + always-on PWM
            osc_a_pw: 0.4, osc_b_fine: 5.0,
            osc_a_level: 0.65, osc_b_level: 0.65,
            filter_cutoff: 2500.0, filter_resonance: 0.05, filter_env_amount: 800.0,
            f_attack: 0.3, f_decay: 0.8, f_sustain: 0.65, f_release: 0.8,
            a_attack: 0.4, a_decay: 0.5, a_sustain: 0.9, a_release: 0.7,
            lfo_freq: 1.5, lfo_amount: 0.4, wm_pw_a: true, drift: 2.0,
            ..Preset::default()
        },
        // Kate Bush "Running Up That Hill" — driving strings with glide
        Preset {
            name: "Running Up", category: "Strings",
            osc_a_saw: true, osc_b_saw: true,
            // FIX: detune for ensemble width
            osc_b_fine: 4.0,
            osc_a_level: 0.8, osc_b_level: 0.6,
            filter_cutoff: 2500.0, filter_resonance: 0.1, filter_env_amount: 1500.0,
            f_attack: 0.05, f_decay: 0.4, f_sustain: 0.5, f_release: 0.3,
            a_attack: 0.03, a_decay: 0.3, a_sustain: 0.8, a_release: 0.3,
            glide_on: true, glide_rate: 0.08,
            lfo_freq: 1.5, wm_pw_a: true, drift: 1.5,
            ..Preset::default()
        },

        // ========================== FILM/TV ==========================

        // John Carpenter horror drone
        Preset {
            name: "Carpenter Horror", category: "Film/TV",
            osc_a_saw: true, osc_b_tri: true,
            filter_drive: 2.5, // extra grit
            osc_a_level: 0.8, osc_b_level: 0.3, noise_level: 0.08,
            filter_cutoff: 250.0, filter_resonance: 0.6, filter_env_amount: 1500.0,
            f_attack: 2.0, f_decay: 3.0, f_sustain: 0.3, f_release: 2.0,
            a_attack: 1.5, a_decay: 1.0, a_sustain: 0.8, a_release: 2.5,
            pm_osc_b: 0.2, pm_freq_a: true, pm_filter: true,
            // FIX: always-on slow filter sweep
            lfo_freq: 0.15, lfo_amount: 0.4, wm_filter: true, drift: 4.0,
            ..Preset::default()
        },
        // Carpenter unison stab — Halloween II style
        Preset {
            name: "Halloween Lead", category: "Film/TV",
            osc_a_saw: true, osc_b_saw: true,
            // FIX: detune for massive unison
            osc_b_fine: 5.0,
            osc_a_level: 0.8, osc_b_level: 0.8,
            filter_cutoff: 3000.0, filter_resonance: 0.2, filter_env_amount: 4000.0,
            f_attack: 0.001, f_decay: 0.5, f_sustain: 0.3, f_release: 0.2,
            a_attack: 0.001, a_decay: 0.3, a_sustain: 0.85, a_release: 0.2,
            unison: true, glide_on: true, glide_rate: 0.06, drift: 2.5,
            ..Preset::default()
        },
        // Escape from New York — lead
        Preset {
            name: "Escape Lead", category: "Film/TV",
            osc_a_saw: true, osc_b_pulse: true,
            // FIX: detune for analog warmth
            osc_b_pw: 0.45, osc_b_fine: 3.0,
            osc_a_level: 0.8, osc_b_level: 0.6,
            filter_cutoff: 2000.0, filter_resonance: 0.2, filter_env_amount: 4000.0,
            f_attack: 0.001, f_decay: 0.6, f_sustain: 0.4, f_release: 0.3,
            a_attack: 0.001, a_decay: 0.4, a_sustain: 0.85, a_release: 0.3,
            glide_on: true, glide_rate: 0.08, drift: 2.0,
            ..Preset::default()
        },
        // Stranger Things — evolving dark texture
        Preset {
            name: "Stranger Things", category: "Film/TV",
            osc_a_pulse: true, osc_b_tri: true,
            osc_a_pw: 0.4, filter_drive: 2.0,
            osc_a_level: 0.8, osc_b_level: 0.2, noise_level: 0.05,
            filter_cutoff: 400.0, filter_resonance: 0.4, filter_env_amount: 2000.0,
            f_attack: 1.0, f_decay: 2.0, f_sustain: 0.3, f_release: 1.5,
            a_attack: 0.8, a_decay: 1.0, a_sustain: 0.8, a_release: 2.0,
            pm_osc_b: 0.25, pm_freq_a: true, pm_filter: true,
            // FIX: always-on filter modulation
            lfo_freq: 0.1, lfo_amount: 0.3, wm_filter: true, drift: 4.0,
            ..Preset::default()
        },
        // Terminator metallic pulse
        Preset {
            name: "Terminator Pulse", category: "Film/TV",
            osc_a_pulse: true, osc_b_tri: true,
            osc_a_pw: 0.5, osc_b_semi: 14.0, // non-octave for metallic character
            osc_a_level: 0.8, osc_b_level: 0.0,
            filter_cutoff: 1500.0, filter_resonance: 0.4, filter_env_amount: 5000.0,
            filter_drive: 1.5,
            f_attack: 0.001, f_decay: 0.15, f_sustain: 0.0, f_release: 0.05,
            a_attack: 0.001, a_decay: 0.2, a_sustain: 0.0, a_release: 0.1,
            pm_filt_env: 0.5, pm_osc_b: 0.3, pm_freq_a: true, drift: 1.5,
            ..Preset::default()
        },

        // ============================ SFX ============================

        // Ghost Bell — Japan/Richard Barbieri FM metallic
        Preset {
            name: "Ghost Bell", category: "SFX",
            osc_a_pulse: true, osc_b_tri: true, // pulse carrier = cleaner FM
            osc_a_pw: 0.5,
            osc_b_semi: 19.0, // octave + fifth = 3:1 ratio area, inharmonic
            osc_a_level: 0.9, osc_b_level: 0.0, // don't hear modulator
            filter_cutoff: 8000.0, filter_resonance: 0.05, filter_env_amount: 4000.0,
            f_attack: 0.001, f_decay: 1.5, f_sustain: 0.0, f_release: 1.5,
            a_attack: 0.001, a_decay: 3.0, a_sustain: 0.0, a_release: 2.0,
            // Key: filter env controls FM depth — harmonics ring out then decay
            pm_filt_env: 0.6, pm_osc_b: 0.3, pm_freq_a: true,
            drift: 1.0,
            ..Preset::default()
        },
        // Dark FM texture — Depeche Mode "Black Celebration"
        Preset {
            name: "Dark FM", category: "SFX",
            osc_a_pulse: true, osc_b_tri: true,
            osc_a_pw: 0.45, osc_b_semi: 5.0, // non-octave ratio for inharmonicity
            osc_a_level: 0.8, osc_b_level: 0.2, noise_level: 0.05,
            filter_cutoff: 600.0, filter_resonance: 0.35, filter_env_amount: 3000.0,
            filter_drive: 2.0,
            f_attack: 0.3, f_decay: 1.5, f_sustain: 0.3, f_release: 1.0,
            a_attack: 0.2, a_decay: 0.8, a_sustain: 0.7, a_release: 1.0,
            pm_filt_env: 0.4, pm_osc_b: 0.5, pm_freq_a: true, pm_filter: true, drift: 3.0,
            ..Preset::default()
        },
        // Laser zap
        Preset {
            name: "Laser", category: "SFX",
            osc_a_saw: true,
            osc_a_level: 1.0,
            filter_cutoff: 15000.0, filter_resonance: 0.8, filter_env_amount: 15000.0,
            f_attack: 0.001, f_decay: 0.12, f_sustain: 0.0, f_release: 0.05,
            a_attack: 0.001, a_decay: 0.15, a_sustain: 0.0, a_release: 0.05,
            drift: 0.0,
            ..Preset::default()
        },

        // ========================== UTILITY ==========================

        Preset {
            name: "Init Saw", category: "Utility",
            ..Preset::default()
        },
    ]
}
