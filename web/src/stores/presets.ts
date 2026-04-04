import { PARAM } from '../audio/engine';

export interface Preset {
    name: string;
    category: string;
    params: [number, number][]; // [paramId, value] pairs
}

function p(overrides: [number, number][]): [number, number][] {
    // Defaults — same as WASM init
    const defaults: [number, number][] = [
        [PARAM.OSC_A_SAW, 1], [PARAM.OSC_A_PULSE, 0], [PARAM.OSC_A_PW, 0.5],
        [PARAM.OSC_B_SAW, 0], [PARAM.OSC_B_TRI, 0], [PARAM.OSC_B_PULSE, 0],
        [PARAM.OSC_B_PW, 0.5], [PARAM.OSC_B_SEMI, 0], [PARAM.OSC_B_FINE, 0],
        [PARAM.OSC_A_LEVEL, 1], [PARAM.OSC_B_LEVEL, 0], [PARAM.NOISE_LEVEL, 0],
        [PARAM.FILTER_CUTOFF, 10000], [PARAM.FILTER_RESONANCE, 0],
        [PARAM.FILTER_ENV_AMT, 5000], [PARAM.FILTER_DRIVE, 1],
        [PARAM.FILTER_ATTACK, 0.01], [PARAM.FILTER_DECAY, 0.3],
        [PARAM.FILTER_SUSTAIN, 0.2], [PARAM.FILTER_RELEASE, 0.3],
        [PARAM.AMP_ATTACK, 0.005], [PARAM.AMP_DECAY, 0.3],
        [PARAM.AMP_SUSTAIN, 0.8], [PARAM.AMP_RELEASE, 0.3],
        [PARAM.SYNC, 0],
        [PARAM.PM_FILT_ENV, 0], [PARAM.PM_OSC_B, 0],
        [PARAM.PM_FREQ_A, 0], [PARAM.PM_PW_A, 0], [PARAM.PM_FILTER, 0],
        [PARAM.LFO_FREQ, 5], [PARAM.LFO_TRI, 1], [PARAM.LFO_SAW, 0],
        [PARAM.LFO_SQUARE, 0], [PARAM.LFO_AMOUNT, 0],
        [PARAM.WM_MIX, 0], [PARAM.WM_FREQ_A, 0], [PARAM.WM_FREQ_B, 0],
        [PARAM.WM_PW_A, 0], [PARAM.WM_PW_B, 0], [PARAM.WM_FILTER, 0],
        [PARAM.MASTER_VOL, 0.5], [PARAM.GLIDE_RATE, 0.1], [PARAM.GLIDE_ON, 0],
        [PARAM.UNISON, 0], [PARAM.DRIFT, 1],
        [PARAM.CHORUS_RATE, 0.8], [PARAM.CHORUS_DEPTH, 0.5], [PARAM.CHORUS_MIX, 0],
        [PARAM.DELAY_TIME, 375], [PARAM.DELAY_FEEDBACK, 0.4],
        [PARAM.DELAY_TONE, 0.6], [PARAM.DELAY_MIX, 0],
        [PARAM.REVERB_DECAY, 0.7], [PARAM.REVERB_DAMPING, 0.7], [PARAM.REVERB_MIX, 0],
        [PARAM.DIST_DRIVE, 0.3], [PARAM.DIST_TONE, 0.5],
        [PARAM.DIST_LEVEL, 0.7], [PARAM.DIST_MIX, 0],
        [PARAM.OCTAVE_DRY, 1], [PARAM.OCTAVE_SUB, 0], [PARAM.OCTAVE_UP, 0],
    ];
    // Apply overrides
    const map = new Map(defaults);
    for (const [id, val] of overrides) map.set(id, val);
    return [...map.entries()];
}

const P = PARAM;

export const CATEGORIES = ['Brass', 'Pads', 'Leads', 'Bass', 'Keys', 'Strings', 'Film/TV', 'SFX'];

export const PRESETS: Preset[] = [
    // BRASS
    { name: 'Prophet Brass', category: 'Brass', params: p([
        [P.OSC_A_SAW, 1], [P.OSC_B_SAW, 1], [P.OSC_B_FINE, 5],
        [P.OSC_A_LEVEL, 0.8], [P.OSC_B_LEVEL, 0.8],
        [P.FILTER_CUTOFF, 400], [P.FILTER_RESONANCE, 0.15], [P.FILTER_ENV_AMT, 6000],
        [P.FILTER_ATTACK, 0.003], [P.FILTER_DECAY, 0.35], [P.FILTER_SUSTAIN, 0.5], [P.FILTER_RELEASE, 0.15],
        [P.AMP_ATTACK, 0.003], [P.AMP_DECAY, 0.3], [P.AMP_SUSTAIN, 0.8], [P.AMP_RELEASE, 0.15],
        [P.LFO_FREQ, 5.5], [P.WM_FREQ_A, 1], [P.WM_FREQ_B, 1], [P.DRIFT, 1.5],
    ])},
    { name: 'Soft Brass', category: 'Brass', params: p([
        [P.OSC_A_SAW, 1], [P.OSC_B_SAW, 1], [P.OSC_B_FINE, 7],
        [P.OSC_A_LEVEL, 0.7], [P.OSC_B_LEVEL, 0.7],
        [P.FILTER_CUTOFF, 350], [P.FILTER_RESONANCE, 0.2], [P.FILTER_ENV_AMT, 4000],
        [P.FILTER_ATTACK, 0.05], [P.FILTER_DECAY, 0.5], [P.FILTER_SUSTAIN, 0.6], [P.FILTER_RELEASE, 0.3],
        [P.AMP_ATTACK, 0.05], [P.AMP_DECAY, 0.4], [P.AMP_SUSTAIN, 0.85], [P.AMP_RELEASE, 0.25],
        [P.LFO_FREQ, 5.5], [P.WM_FREQ_A, 1], [P.WM_FREQ_B, 1], [P.DRIFT, 2],
    ])},

    // PADS
    { name: 'In The Air', category: 'Pads', params: p([
        [P.OSC_A_SAW, 0], [P.OSC_A_PULSE, 1], [P.OSC_A_PW, 0.35],
        [P.OSC_B_SAW, 1], [P.OSC_B_FINE, 8],
        [P.OSC_A_LEVEL, 0.7], [P.OSC_B_LEVEL, 0.5],
        [P.FILTER_CUTOFF, 2000], [P.FILTER_RESONANCE, 0.08], [P.FILTER_ENV_AMT, 1500],
        [P.FILTER_ATTACK, 0.5], [P.FILTER_DECAY, 1.5], [P.FILTER_SUSTAIN, 0.7], [P.FILTER_RELEASE, 1.5],
        [P.AMP_ATTACK, 0.6], [P.AMP_SUSTAIN, 1.0], [P.AMP_RELEASE, 1.8],
        [P.LFO_FREQ, 1.2], [P.LFO_AMOUNT, 0.6], [P.WM_PW_A, 1],
        [P.DRIFT, 2], [P.CHORUS_MIX, 0.4], [P.REVERB_DECAY, 0.8], [P.REVERB_MIX, 0.3],
    ])},
    { name: 'Blade Runner', category: 'Pads', params: p([
        [P.OSC_A_SAW, 1], [P.OSC_B_SAW, 1], [P.OSC_B_FINE, 5],
        [P.OSC_A_LEVEL, 0.7], [P.OSC_B_LEVEL, 0.7], [P.NOISE_LEVEL, 0.05],
        [P.FILTER_CUTOFF, 800], [P.FILTER_RESONANCE, 0.18], [P.FILTER_ENV_AMT, 2000],
        [P.FILTER_ATTACK, 1.0], [P.FILTER_DECAY, 3.0], [P.FILTER_SUSTAIN, 0.5], [P.FILTER_RELEASE, 2.5],
        [P.AMP_ATTACK, 0.8], [P.AMP_SUSTAIN, 1.0], [P.AMP_RELEASE, 3.0],
        [P.LFO_FREQ, 0.2], [P.LFO_AMOUNT, 0.3], [P.WM_FILTER, 1],
        [P.DRIFT, 3], [P.REVERB_DECAY, 0.9], [P.REVERB_DAMPING, 0.5], [P.REVERB_MIX, 0.4],
    ])},
    { name: 'Human Nature', category: 'Pads', params: p([
        [P.OSC_A_SAW, 0], [P.OSC_A_PULSE, 1], [P.OSC_A_PW, 0.35],
        [P.OSC_B_PULSE, 1], [P.OSC_B_PW, 0.6], [P.OSC_B_FINE, 6],
        [P.OSC_A_LEVEL, 0.6], [P.OSC_B_LEVEL, 0.5],
        [P.FILTER_CUTOFF, 1500], [P.FILTER_RESONANCE, 0.1], [P.FILTER_ENV_AMT, 800],
        [P.FILTER_ATTACK, 0.8], [P.FILTER_DECAY, 2.0], [P.FILTER_SUSTAIN, 0.6], [P.FILTER_RELEASE, 2.0],
        [P.AMP_ATTACK, 0.7], [P.AMP_SUSTAIN, 1.0], [P.AMP_RELEASE, 2.5],
        [P.LFO_FREQ, 0.8], [P.LFO_AMOUNT, 0.5], [P.WM_PW_A, 1], [P.WM_PW_B, 1],
        [P.DRIFT, 2.5], [P.CHORUS_MIX, 0.3], [P.REVERB_DECAY, 0.85], [P.REVERB_MIX, 0.35],
    ])},

    // LEADS
    { name: 'Sync Lead', category: 'Leads', params: p([
        [P.OSC_A_SAW, 1], [P.OSC_B_SAW, 1],
        [P.OSC_A_LEVEL, 1.0], [P.OSC_B_LEVEL, 0],
        [P.FILTER_CUTOFF, 8000], [P.FILTER_RESONANCE, 0.1], [P.FILTER_ENV_AMT, 3000],
        [P.FILTER_ATTACK, 0.001], [P.FILTER_DECAY, 0.5], [P.FILTER_SUSTAIN, 0.3], [P.FILTER_RELEASE, 0.2],
        [P.AMP_ATTACK, 0.001], [P.AMP_SUSTAIN, 0.85], [P.AMP_RELEASE, 0.2],
        [P.SYNC, 1], [P.PM_FILT_ENV, 0.6], [P.PM_FREQ_A, 1],
        [P.DRIFT, 1], [P.DELAY_TIME, 350], [P.DELAY_FEEDBACK, 0.3], [P.DELAY_MIX, 0.2],
        [P.REVERB_MIX, 0.15],
    ])},
    { name: 'Unison Lead', category: 'Leads', params: p([
        [P.OSC_A_SAW, 1], [P.OSC_B_SAW, 1], [P.OSC_B_FINE, 5],
        [P.OSC_A_LEVEL, 0.8], [P.OSC_B_LEVEL, 0.8],
        [P.FILTER_CUTOFF, 800], [P.FILTER_RESONANCE, 0.2], [P.FILTER_ENV_AMT, 6000],
        [P.FILTER_ATTACK, 0.001], [P.FILTER_DECAY, 0.5], [P.FILTER_SUSTAIN, 0.5],
        [P.AMP_ATTACK, 0.001], [P.AMP_SUSTAIN, 0.9],
        [P.UNISON, 1], [P.GLIDE_ON, 1], [P.GLIDE_RATE, 0.06],
        [P.LFO_FREQ, 5.5], [P.WM_FREQ_A, 1], [P.WM_FREQ_B, 1], [P.DRIFT, 2.5],
    ])},
    { name: 'Kids', category: 'Leads', params: p([
        [P.OSC_A_SAW, 0], [P.OSC_A_PULSE, 1], [P.OSC_A_PW, 0.5],
        [P.FILTER_CUTOFF, 1687], [P.FILTER_RESONANCE, 0.57], [P.FILTER_ENV_AMT, 0],
        [P.AMP_ATTACK, 0.01], [P.AMP_DECAY, 0.3], [P.AMP_SUSTAIN, 0.85],
        [P.LFO_FREQ, 5.65], [P.LFO_AMOUNT, 0.3], [P.WM_FREQ_A, 1], [P.DRIFT, 1],
    ])},
    { name: "Don't Go", category: 'Leads', params: p([
        [P.OSC_A_SAW, 1], [P.OSC_B_SAW, 1], [P.OSC_B_SEMI, 24],
        [P.OSC_A_LEVEL, 0.2], [P.OSC_B_LEVEL, 0.8],
        [P.FILTER_CUTOFF, 3200], [P.FILTER_ENV_AMT, 2750],
        [P.FILTER_ATTACK, 0.02], [P.FILTER_DECAY, 0.15], [P.FILTER_SUSTAIN, 0],
        [P.AMP_ATTACK, 0.01], [P.AMP_SUSTAIN, 1.0], [P.AMP_RELEASE, 0.015],
    ])},
    { name: 'Cars', category: 'Leads', params: p([
        [P.FILTER_CUTOFF, 1600], [P.FILTER_RESONANCE, 0.3], [P.FILTER_ENV_AMT, 0],
        [P.AMP_ATTACK, 0.14], [P.AMP_SUSTAIN, 1.0], [P.AMP_RELEASE, 0.65],
        [P.LFO_FREQ, 5], [P.LFO_AMOUNT, 0.15], [P.WM_FREQ_A, 1], [P.DRIFT, 1.5],
    ])},

    // BASS
    { name: 'Fat Bass', category: 'Bass', params: p([
        [P.OSC_A_SAW, 1], [P.OSC_B_SAW, 1], [P.OSC_B_SEMI, -12],
        [P.OSC_A_LEVEL, 0.6], [P.OSC_B_LEVEL, 1.0],
        [P.FILTER_CUTOFF, 300], [P.FILTER_RESONANCE, 0.2], [P.FILTER_ENV_AMT, 4000],
        [P.FILTER_ATTACK, 0.001], [P.FILTER_DECAY, 0.2], [P.FILTER_SUSTAIN, 0.15], [P.FILTER_RELEASE, 0.05],
        [P.AMP_ATTACK, 0.001], [P.AMP_DECAY, 0.4], [P.AMP_SUSTAIN, 0.7], [P.AMP_RELEASE, 0.08],
        [P.DRIFT, 0.5],
    ])},
    { name: 'Acid Bass', category: 'Bass', params: p([
        [P.FILTER_CUTOFF, 180], [P.FILTER_RESONANCE, 0.7], [P.FILTER_ENV_AMT, 7000],
        [P.FILTER_ATTACK, 0.001], [P.FILTER_DECAY, 0.2], [P.FILTER_SUSTAIN, 0],
        [P.AMP_ATTACK, 0.001], [P.AMP_DECAY, 0.3], [P.AMP_SUSTAIN, 0.6], [P.AMP_RELEASE, 0.05],
        [P.DRIFT, 0.5],
    ])},
    { name: 'Personal Jesus', category: 'Bass', params: p([
        [P.OSC_A_SAW, 1], [P.OSC_B_PULSE, 1], [P.OSC_B_PW, 0.5], [P.OSC_B_SEMI, -12],
        [P.OSC_A_LEVEL, 0.8], [P.OSC_B_LEVEL, 0.6],
        [P.FILTER_CUTOFF, 350], [P.FILTER_RESONANCE, 0.65], [P.FILTER_ENV_AMT, 5000],
        [P.FILTER_ATTACK, 0.001], [P.FILTER_DECAY, 0.25], [P.FILTER_SUSTAIN, 0.1],
        [P.AMP_ATTACK, 0.001], [P.AMP_DECAY, 0.3], [P.AMP_SUSTAIN, 0.7], [P.AMP_RELEASE, 0.08],
    ])},

    // KEYS
    { name: 'Organ', category: 'Keys', params: p([
        [P.OSC_A_SAW, 0], [P.OSC_A_PULSE, 1], [P.OSC_B_PULSE, 1],
        [P.OSC_B_SEMI, 12], [P.OSC_A_LEVEL, 0.7], [P.OSC_B_LEVEL, 0.5],
        [P.FILTER_CUTOFF, 6000], [P.FILTER_ENV_AMT, 0],
        [P.FILTER_SUSTAIN, 1], [P.AMP_ATTACK, 0.001], [P.AMP_SUSTAIN, 1.0], [P.AMP_RELEASE, 0.03],
        [P.LFO_FREQ, 6], [P.WM_FREQ_A, 1], [P.WM_FREQ_B, 1], [P.DRIFT, 0],
    ])},
    { name: 'Take On Me', category: 'Keys', params: p([
        [P.OSC_A_SAW, 0], [P.OSC_A_PULSE, 1], [P.OSC_A_PW, 0.4],
        [P.OSC_B_SAW, 1], [P.OSC_B_SEMI, 12],
        [P.OSC_A_LEVEL, 0.8], [P.OSC_B_LEVEL, 0.5],
        [P.FILTER_CUTOFF, 5000], [P.FILTER_RESONANCE, 0.15], [P.FILTER_ENV_AMT, 2000],
        [P.FILTER_ATTACK, 0.001], [P.FILTER_DECAY, 0.3], [P.FILTER_SUSTAIN, 0.2],
        [P.AMP_ATTACK, 0.001], [P.AMP_DECAY, 0.25], [P.AMP_SUSTAIN, 0], [P.AMP_RELEASE, 0.1],
    ])},

    // STRINGS
    { name: 'Prophet Strings', category: 'Strings', params: p([
        [P.OSC_A_SAW, 0], [P.OSC_A_PULSE, 1], [P.OSC_A_PW, 0.4],
        [P.OSC_B_SAW, 1], [P.OSC_B_FINE, 7],
        [P.OSC_A_LEVEL, 0.65], [P.OSC_B_LEVEL, 0.65],
        [P.FILTER_CUTOFF, 2500], [P.FILTER_RESONANCE, 0.05], [P.FILTER_ENV_AMT, 1000],
        [P.FILTER_ATTACK, 0.4], [P.FILTER_DECAY, 0.8], [P.FILTER_SUSTAIN, 0.7],
        [P.AMP_ATTACK, 0.4], [P.AMP_SUSTAIN, 0.9], [P.AMP_RELEASE, 0.7],
        [P.LFO_FREQ, 1.5], [P.LFO_AMOUNT, 0.5], [P.WM_PW_A, 1],
        [P.DRIFT, 2], [P.CHORUS_MIX, 0.35], [P.REVERB_MIX, 0.25],
    ])},

    // FILM/TV
    { name: 'Carpenter Horror', category: 'Film/TV', params: p([
        [P.OSC_A_SAW, 1], [P.OSC_B_TRI, 1], [P.FILTER_DRIVE, 2.5],
        [P.OSC_A_LEVEL, 0.8], [P.OSC_B_LEVEL, 0.3], [P.NOISE_LEVEL, 0.08],
        [P.FILTER_CUTOFF, 250], [P.FILTER_RESONANCE, 0.6], [P.FILTER_ENV_AMT, 1500],
        [P.FILTER_ATTACK, 2.0], [P.FILTER_DECAY, 3.0], [P.FILTER_SUSTAIN, 0.3],
        [P.AMP_ATTACK, 1.5], [P.AMP_SUSTAIN, 0.8], [P.AMP_RELEASE, 2.5],
        [P.PM_OSC_B, 0.2], [P.PM_FREQ_A, 1], [P.PM_FILTER, 1],
        [P.LFO_FREQ, 0.15], [P.LFO_AMOUNT, 0.4], [P.WM_FILTER, 1],
        [P.DRIFT, 4], [P.REVERB_DECAY, 0.85], [P.REVERB_MIX, 0.35],
    ])},
    { name: 'Stranger Things', category: 'Film/TV', params: p([
        [P.OSC_A_SAW, 0], [P.OSC_A_PULSE, 1], [P.OSC_A_PW, 0.4],
        [P.OSC_B_TRI, 1], [P.FILTER_DRIVE, 2],
        [P.OSC_A_LEVEL, 0.8], [P.OSC_B_LEVEL, 0.2], [P.NOISE_LEVEL, 0.05],
        [P.FILTER_CUTOFF, 400], [P.FILTER_RESONANCE, 0.4], [P.FILTER_ENV_AMT, 2000],
        [P.FILTER_ATTACK, 1.0], [P.FILTER_DECAY, 2.0], [P.FILTER_SUSTAIN, 0.3],
        [P.AMP_ATTACK, 0.8], [P.AMP_SUSTAIN, 0.8], [P.AMP_RELEASE, 2.0],
        [P.PM_OSC_B, 0.25], [P.PM_FREQ_A, 1], [P.PM_FILTER, 1],
        [P.LFO_FREQ, 0.1], [P.LFO_AMOUNT, 0.3], [P.WM_FILTER, 1],
        [P.DRIFT, 4], [P.REVERB_DECAY, 0.9], [P.REVERB_MIX, 0.4],
    ])},

    // SFX
    { name: 'Ghost Bell', category: 'SFX', params: p([
        [P.OSC_A_SAW, 0], [P.OSC_A_PULSE, 1], [P.OSC_A_PW, 0.5],
        [P.OSC_B_TRI, 1], [P.OSC_B_SEMI, 19],
        [P.OSC_A_LEVEL, 0.9], [P.OSC_B_LEVEL, 0],
        [P.FILTER_CUTOFF, 8000], [P.FILTER_ENV_AMT, 4000],
        [P.FILTER_ATTACK, 0.001], [P.FILTER_DECAY, 1.5], [P.FILTER_SUSTAIN, 0], [P.FILTER_RELEASE, 1.5],
        [P.AMP_ATTACK, 0.001], [P.AMP_DECAY, 3.0], [P.AMP_SUSTAIN, 0], [P.AMP_RELEASE, 2.0],
        [P.PM_FILT_ENV, 0.6], [P.PM_OSC_B, 0.3], [P.PM_FREQ_A, 1],
        [P.REVERB_DECAY, 0.85], [P.REVERB_MIX, 0.35],
    ])},
    { name: 'Laser', category: 'SFX', params: p([
        [P.FILTER_CUTOFF, 15000], [P.FILTER_RESONANCE, 0.8], [P.FILTER_ENV_AMT, 15000],
        [P.FILTER_ATTACK, 0.001], [P.FILTER_DECAY, 0.12], [P.FILTER_SUSTAIN, 0],
        [P.AMP_ATTACK, 0.001], [P.AMP_DECAY, 0.15], [P.AMP_SUSTAIN, 0], [P.AMP_RELEASE, 0.05],
        [P.DRIFT, 0],
    ])},
];
