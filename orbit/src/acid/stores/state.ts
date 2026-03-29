import { writable, derived, get } from 'svelte/store';
import { NUM_STEPS, ACID_PARAMS, ACID_PARAM_MAP, type AcidParamName } from '../constants';
import type { AcidEngine } from '../audio/engine';
import { PARAM } from '../audio/engine';
import { PRESETS } from '../presets';
import { bpm, registerEngine } from '../../shared/stores/transport';

let engine: AcidEngine | null = null;
export function setAcidEngine(e: AcidEngine) {
    engine = e;
    e.onStep = (step) => currentStep.set(step);
    bpm.subscribe((value) => { engine?.seqSetBpm(value); });
    registerEngine({
        play: () => { engine?.seqSetBpm(get(bpm)); engine?.seqPlay(); },
        stop: () => { engine?.seqStop(); currentStep.set(0); },
    });
}

export const currentPresetIndex = writable(0);
export const currentTranspose = writable(0); // semitones from C
export const selectedStep = writable(0);
export const selectedParam = writable<AcidParamName>('cutoff');
export const stepNotes = writable<number[]>(Array(NUM_STEPS).fill(48));
export const stepGates = writable<boolean[]>(Array(NUM_STEPS).fill(false));
export const stepAccents = writable<boolean[]>(Array(NUM_STEPS).fill(false));
export const stepSlides = writable<boolean[]>(Array(NUM_STEPS).fill(false));
export const synthParams = writable<Record<AcidParamName, number>>({
    cutoff: 35, resonance: 60, 'env mod': 70, dist: 30,
});
export { isPlaying, togglePlay } from '../../shared/stores/transport';
export const currentStep = writable(0);
export const waveform = writable<'saw' | 'square'>('saw');

export const sliderValue = derived(
    [selectedParam, synthParams],
    ([$param, $params]) => $params[$param]
);

export function selectStep(step: number) { selectedStep.set(step); }
export function selectAcidParam(param: string) { selectedParam.set(param as AcidParamName); }

export function setStepNote(note: number) {
    const step = get(selectedStep);
    stepNotes.update(n => { n[step] = note; return [...n]; });
    stepGates.update(g => { g[step] = true; return [...g]; });
    engine?.setStepNote(step, note);
    engine?.setStepGate(step, true);
}

export function toggleStepGate(step: number) {
    stepGates.update(g => { g[step] = !g[step]; return [...g]; });
    engine?.setStepGate(step, get(stepGates)[step]);
}

export function toggleStepAccent() {
    const step = get(selectedStep);
    stepAccents.update(a => { a[step] = !a[step]; return [...a]; });
    engine?.setStepAccent(step, get(stepAccents)[step]);
}

export function toggleStepSlide() {
    const step = get(selectedStep);
    stepSlides.update(s => { s[step] = !s[step]; return [...s]; });
    engine?.setStepSlide(step, get(stepSlides)[step]);
}

export function setSliderValue(value: number) {
    const param = get(selectedParam);
    synthParams.update(p => { p[param] = value; return p; });
    const mapping = ACID_PARAM_MAP[param];
    const actual = mapping.min + (value / 100) * (mapping.max - mapping.min);
    engine?.setParam(mapping.id, actual);
}

export function toggleWaveform() {
    const next = get(waveform) === 'saw' ? 'square' : 'saw';
    waveform.set(next);
    engine?.setParam(PARAM.WAVEFORM, next === 'saw' ? 0 : 1);
}

export function loadPreset(index: number) {
    const preset = PRESETS[index];
    if (!preset || !engine) return;
    currentPresetIndex.set(index);
    const notes: number[] = [], gates: boolean[] = [], accents: boolean[] = [], slides: boolean[] = [];
    for (let i = 0; i < NUM_STEPS; i++) {
        const s = preset.steps[i];
        notes.push(s.note); gates.push(s.gate); accents.push(s.accent); slides.push(s.slide);
        engine.setStepNote(i, s.note); engine.setStepGate(i, s.gate);
        engine.setStepAccent(i, s.accent); engine.setStepSlide(i, s.slide);
    }
    stepNotes.set(notes); stepGates.set(gates); stepAccents.set(accents); stepSlides.set(slides);
    synthParams.set({ cutoff: preset.cutoff, resonance: preset.resonance, 'env mod': preset.envMod, dist: preset.dist });
    for (const paramName of ACID_PARAMS) {
        const mapping = ACID_PARAM_MAP[paramName];
        const sv = paramName === 'cutoff' ? preset.cutoff : paramName === 'resonance' ? preset.resonance
            : paramName === 'env mod' ? preset.envMod : preset.dist;
        engine.setParam(mapping.id, mapping.min + (sv / 100) * (mapping.max - mapping.min));
    }
    engine.setParam(PARAM.DECAY, 0.03 + (preset.decay / 100) * 2.97);
    engine.setParam(PARAM.ACCENT, preset.accent / 100);
    waveform.set(preset.waveform);
    engine.setParam(PARAM.WAVEFORM, preset.waveform === 'saw' ? 0 : 1);
    currentTranspose.set(0);
}

export function randomizePattern() {
    if (!engine) return;
    const notes: number[] = [], gates: boolean[] = [], accents: boolean[] = [], slides: boolean[] = [];
    for (let i = 0; i < NUM_STEPS; i++) {
        const gate = Math.random() < 0.7;
        const accent = gate && Math.random() < 0.25;
        const slide = gate && Math.random() < 0.3;
        let note = 36 + Math.floor(Math.random() * 12);
        if (Math.random() < 0.15) note += 12;
        notes.push(note); gates.push(gate); accents.push(accent); slides.push(slide);
        engine.setStepNote(i, note); engine.setStepGate(i, gate);
        engine.setStepAccent(i, accent); engine.setStepSlide(i, slide);
    }
    stepNotes.set(notes); stepGates.set(gates); stepAccents.set(accents); stepSlides.set(slides);
    currentTranspose.set(0);
}

export function shiftOctave(direction: number) {
    transposePattern(get(currentTranspose) + direction * 12);
}

export function transposePattern(semitones: number) {
    if (!engine) return;
    const delta = semitones - get(currentTranspose);
    if (delta === 0) return;
    currentTranspose.set(semitones);
    stepNotes.update(notes => {
        const transposed = notes.map(n => Math.max(24, Math.min(96, n + delta)));
        for (let i = 0; i < NUM_STEPS; i++) {
            engine!.setStepNote(i, transposed[i]);
        }
        return transposed;
    });
}
