import { writable, derived, get } from 'svelte/store';
import { NUM_VOICES, NUM_STEPS, DEFAULT_BPM, type ParamName } from '../constants';
import type { OrbitEngine } from '../audio/engine';

let engine: OrbitEngine | null = null;
export function setEngine(e: OrbitEngine) { engine = e; }

// Selection
export const selectedVoice = writable(0);
export const selectedParam = writable<ParamName>('level');

// Per-voice parameters: voiceParams[voiceIndex][param] = 0-100
const initParams = () => {
    const p: Record<number, Record<ParamName, number>> = {};
    for (let i = 0; i < NUM_VOICES; i++) {
        p[i] = { level: 75, decay: 50, tone: 50, pitch: 50 };
    }
    return p;
};
export const voiceParams = writable(initParams());

// Patterns: patterns[voiceIndex] = boolean[16]
const initPatterns = () => {
    const p: Record<number, boolean[]> = {};
    for (let i = 0; i < NUM_VOICES; i++) {
        p[i] = Array(NUM_STEPS).fill(false);
    }
    return p;
};
export const patterns = writable(initPatterns());

// Transport
export const isPlaying = writable(false);
export const bpm = writable(DEFAULT_BPM);
export const currentStep = writable(0);

// Triggered voices (for visual pulse)
export const triggeredVoices = writable(new Set<number>());

// Derived: current slider value
export const sliderValue = derived(
    [selectedVoice, selectedParam, voiceParams],
    ([$voice, $param, $params]) => $params[$voice][$param]
);

// Actions
export function selectVoice(index: number) {
    selectedVoice.set(index);
    selectedParam.set('level'); // reset to level on voice change
}

export function selectParam(param: ParamName) {
    selectedParam.set(param);
}

export function setSliderValue(value: number) {
    const voice = get(selectedVoice);
    const param = get(selectedParam);
    voiceParams.update(p => {
        p[voice][param] = value;
        return p;
    });
    engine?.setVoiceParam(voice, param, value);
}

export function toggleStep(step: number) {
    const voice = get(selectedVoice);
    patterns.update(p => {
        p[voice][step] = !p[voice][step];
        return p;
    });
    engine?.seqToggleStep(voice, step);
}

export function togglePlay() {
    const playing = !get(isPlaying);
    isPlaying.set(playing);
    if (playing) {
        engine?.seqSetBpm(get(bpm));
        engine?.seqPlay();
    } else {
        engine?.seqStop();
        currentStep.set(0);
    }
}

export function setBpm(value: number) {
    bpm.set(value);
    engine?.seqSetBpm(value);
}

export function triggerPad(index: number) {
    engine?.triggerVoice(index);
    // Visual pulse
    triggeredVoices.update(s => { s.add(index); return new Set(s); });
    setTimeout(() => {
        triggeredVoices.update(s => { s.delete(index); return new Set(s); });
    }, 120);
}
