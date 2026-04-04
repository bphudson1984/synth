import { writable } from 'svelte/store';
import { PARAM, type AudioEngine } from '../audio/engine';

let engine: AudioEngine | null = null;

export function setEngine(e: AudioEngine) {
    engine = e;
}

function paramStore(id: number, initial: number) {
    const store = writable(initial);
    store.subscribe(value => {
        engine?.setParam(id, value);
    });
    return store;
}

function boolStore(id: number, initial: boolean) {
    const store = writable(initial);
    store.subscribe(value => {
        engine?.setParam(id, value ? 1.0 : 0.0);
    });
    return store;
}

// Oscillator A
export const oscASaw = boolStore(PARAM.OSC_A_SAW, true);
export const oscAPulse = boolStore(PARAM.OSC_A_PULSE, false);
export const oscAPW = paramStore(PARAM.OSC_A_PW, 0.5);

// Oscillator B
export const oscBSaw = boolStore(PARAM.OSC_B_SAW, false);
export const oscBTri = boolStore(PARAM.OSC_B_TRI, false);
export const oscBPulse = boolStore(PARAM.OSC_B_PULSE, false);
export const oscBPW = paramStore(PARAM.OSC_B_PW, 0.5);
export const oscBSemi = paramStore(PARAM.OSC_B_SEMI, 0);
export const oscBFine = paramStore(PARAM.OSC_B_FINE, 0);

// Mixer
export const oscALevel = paramStore(PARAM.OSC_A_LEVEL, 1.0);
export const oscBLevel = paramStore(PARAM.OSC_B_LEVEL, 0.0);
export const noiseLevel = paramStore(PARAM.NOISE_LEVEL, 0.0);

// Filter
export const filterCutoff = paramStore(PARAM.FILTER_CUTOFF, 10000);
export const filterRes = paramStore(PARAM.FILTER_RESONANCE, 0.0);
export const filterEnvAmt = paramStore(PARAM.FILTER_ENV_AMT, 5000);
export const filterDrive = paramStore(PARAM.FILTER_DRIVE, 1.0);

// Filter Envelope
export const fAttack = paramStore(PARAM.FILTER_ATTACK, 0.01);
export const fDecay = paramStore(PARAM.FILTER_DECAY, 0.3);
export const fSustain = paramStore(PARAM.FILTER_SUSTAIN, 0.2);
export const fRelease = paramStore(PARAM.FILTER_RELEASE, 0.3);

// Amp Envelope
export const aAttack = paramStore(PARAM.AMP_ATTACK, 0.005);
export const aDecay = paramStore(PARAM.AMP_DECAY, 0.3);
export const aSustain = paramStore(PARAM.AMP_SUSTAIN, 0.8);
export const aRelease = paramStore(PARAM.AMP_RELEASE, 0.3);

// Sync
export const sync = boolStore(PARAM.SYNC, false);

// Poly Mod
export const pmFiltEnv = paramStore(PARAM.PM_FILT_ENV, 0.0);
export const pmOscB = paramStore(PARAM.PM_OSC_B, 0.0);
export const pmFreqA = boolStore(PARAM.PM_FREQ_A, false);
export const pmPWA = boolStore(PARAM.PM_PW_A, false);
export const pmFilter = boolStore(PARAM.PM_FILTER, false);

// LFO
export const lfoFreq = paramStore(PARAM.LFO_FREQ, 5.0);
export const lfoTri = boolStore(PARAM.LFO_TRI, true);
export const lfoSaw = boolStore(PARAM.LFO_SAW, false);
export const lfoSquare = boolStore(PARAM.LFO_SQUARE, false);
export const lfoAmount = paramStore(PARAM.LFO_AMOUNT, 0.0);

// Wheel Mod
export const wmMix = paramStore(PARAM.WM_MIX, 0.0);
export const wmFreqA = boolStore(PARAM.WM_FREQ_A, false);
export const wmFreqB = boolStore(PARAM.WM_FREQ_B, false);
export const wmPWA = boolStore(PARAM.WM_PW_A, false);
export const wmPWB = boolStore(PARAM.WM_PW_B, false);
export const wmFilter = boolStore(PARAM.WM_FILTER, false);

// Master
export const masterVol = paramStore(PARAM.MASTER_VOL, 0.5);
export const glideRate = paramStore(PARAM.GLIDE_RATE, 0.1);
export const glideOn = boolStore(PARAM.GLIDE_ON, false);
export const unison = boolStore(PARAM.UNISON, false);
export const drift = paramStore(PARAM.DRIFT, 1.0);

// Effects
export const chorusRate = paramStore(PARAM.CHORUS_RATE, 0.8);
export const chorusDepth = paramStore(PARAM.CHORUS_DEPTH, 0.5);
export const chorusMix = paramStore(PARAM.CHORUS_MIX, 0.0);

export const delayTime = paramStore(PARAM.DELAY_TIME, 375);
export const delayFeedback = paramStore(PARAM.DELAY_FEEDBACK, 0.4);
export const delayTone = paramStore(PARAM.DELAY_TONE, 0.6);
export const delayMix = paramStore(PARAM.DELAY_MIX, 0.0);

export const reverbDecay = paramStore(PARAM.REVERB_DECAY, 0.7);
export const reverbDamping = paramStore(PARAM.REVERB_DAMPING, 0.7);
export const reverbMix = paramStore(PARAM.REVERB_MIX, 0.0);

// FX Chain order: [chorus=0, delay=1, reverb=2] — indices into the effect slots
export const fxOrder = writable([0, 1, 2]);
fxOrder.subscribe(order => {
    engine?.setFxOrder(order);
});

// Arpeggiator
export const arpMode = paramStore(PARAM.ARP_MODE, 0); // 0=off, 1=up, 2=down, 3=updown, 4=updown excl, 5=random, 6=order
export const arpDivision = paramStore(PARAM.ARP_DIVISION, 1); // 0=1/4, 1=1/8, 2=1/16, 3=1/32, 4=dotted 1/8, 5=triplet 1/8
export const arpBpm = paramStore(PARAM.ARP_BPM, 120);
export const arpOctaves = paramStore(PARAM.ARP_OCTAVES, 1);
export const arpGate = paramStore(PARAM.ARP_GATE, 0.5);
export const arpSwing = paramStore(PARAM.ARP_SWING, 0.5);
export const arpHold = boolStore(PARAM.ARP_HOLD, false);
