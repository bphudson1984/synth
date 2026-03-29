import { writable, derived, get } from 'svelte/store';
import { NUM_VOICES, NUM_STEPS, type ParamName, type EngineType } from '../constants';
import type { OrbitEngine } from '../audio/engine';
import { bpm, registerEngine } from '../../shared/stores/transport';
import { PRESETS } from '../presets';

let engine: OrbitEngine | null = null;
export function setDrumEngine(e: OrbitEngine) {
    engine = e;
    e.onStep = (step) => currentStep.set(step);
    registerEngine({
        play: () => { engine?.seqSetBpm(get(bpm)); engine?.seqPlay(); },
        stop: () => { engine?.seqStop(); currentStep.set(0); },
    });
}

// Selection
export const selectedVoice = writable(0);
export const selectedParam = writable<ParamName>('level');

// Engine selection
export const globalEngine = writable<EngineType>('808');
export const perPadEngine = writable<Record<number, EngineType>>(
    Object.fromEntries(Array.from({ length: NUM_VOICES }, (_, i) => [i, '808' as EngineType]))
);

// Per-voice parameters
const initParams = () => {
    const p: Record<number, Record<ParamName, number>> = {};
    for (let i = 0; i < NUM_VOICES; i++) {
        p[i] = { level: 75, decay: 50, tone: 50, pitch: 50 };
    }
    return p;
};
export const voiceParams = writable(initParams());

// Patterns
const initPatterns = () => {
    const p: Record<number, boolean[]> = {};
    for (let i = 0; i < NUM_VOICES; i++) {
        p[i] = Array(NUM_STEPS).fill(false);
    }
    return p;
};
export const patterns = writable(initPatterns());

// Transport
export { isPlaying, bpm, togglePlay } from '../../shared/stores/transport';
export const currentStep = writable(0);
export const triggeredVoices = writable(new Set<number>());

// Derived
export const sliderValue = derived(
    [selectedVoice, selectedParam, voiceParams],
    ([$voice, $param, $params]) => $params[$voice][$param]
);

// Actions
export function selectVoice(index: number) {
    selectedVoice.set(index);
    selectedParam.set('level');
}

export function selectParam(param: ParamName) {
    selectedParam.set(param);
}

export function setSliderValue(value: number) {
    const voice = get(selectedVoice);
    const param = get(selectedParam);
    voiceParams.update(p => { p[voice][param] = value; return p; });
    const eng = get(perPadEngine)[voice];
    engine?.setVoiceParam(voice, param, value, eng);
}

export function toggleStep(step: number) {
    const voice = get(selectedVoice);
    patterns.update(p => { p[voice][step] = !p[voice][step]; return p; });
    engine?.seqToggleStep(voice, step);
}

export function setBpm(value: number) {
    bpm.set(value);
    engine?.seqSetBpm(value);
}

export function triggerPad(index: number) {
    const eng = get(perPadEngine)[index];
    engine?.triggerVoice(index, eng);
    triggeredVoices.update(s => { s.add(index); return new Set(s); });
    setTimeout(() => {
        triggeredVoices.update(s => { s.delete(index); return new Set(s); });
    }, 120);
}

export function setGlobalEngine(eng: EngineType) {
    globalEngine.set(eng);
    perPadEngine.update(p => {
        for (let i = 0; i < NUM_VOICES; i++) p[i] = eng;
        return p;
    });
    engine?.setAllEngines(eng);
}

export function setPadEngine(padIndex: number, eng: EngineType) {
    perPadEngine.update(p => { p[padIndex] = eng; return p; });
    engine?.setTrackEngine(padIndex, eng);
}

export function togglePadEngine(padIndex: number) {
    const current = get(perPadEngine)[padIndex];
    setPadEngine(padIndex, current === '808' ? '909' : '808');
}

// Presets
export const currentDrumPreset = writable(-1);

export function loadDrumPreset(index: number) {
    const preset = PRESETS[index];
    if (!preset || !engine) return;
    currentDrumPreset.set(index);
    // Clear existing pattern in engine
    engine.seqClear();
    // Load new pattern
    const newPatterns: Record<number, boolean[]> = {};
    for (let voice = 0; voice < NUM_VOICES; voice++) {
        newPatterns[voice] = [...preset.pattern[voice]];
        for (let step = 0; step < NUM_STEPS; step++) {
            if (preset.pattern[voice][step]) {
                engine.seqToggleStep(voice, step);
            }
        }
    }
    patterns.set(newPatterns);
}

export function randomizeDrumPattern() {
    if (!engine) return;
    engine.seqClear();
    const newPatterns: Record<number, boolean[]> = {};
    for (let voice = 0; voice < NUM_VOICES; voice++) {
        newPatterns[voice] = Array(NUM_STEPS).fill(false);
        // Different density per voice type
        const density = voice === 0 ? 0.25  // kick: sparse
            : voice === 1 ? 0.12            // snare: very sparse
            : voice === 2 ? 0.5             // chh: dense
            : voice === 3 ? 0.06            // ohh: rare
            : voice === 4 ? 0.12            // clap: sparse
            : voice === 5 ? 0.08            // tom
            : voice === 6 ? 0.1             // rim
            : 0.15;                          // perc
        for (let step = 0; step < NUM_STEPS; step++) {
            if (Math.random() < density) {
                newPatterns[voice][step] = true;
                engine.seqToggleStep(voice, step);
            }
        }
    }
    patterns.set(newPatterns);
    currentDrumPreset.set(-1);
}
