import { writable, derived, get } from 'svelte/store';
import { NUM_STEPS, ACID_PARAMS, ACID_PARAM_MAP, ACID_SETTINGS, type AcidParamName } from '../constants';
import { NUM_QUICK_SLOTS, type QuickSlot, type SettingsParam } from '../../shared/types/settings';
import type { AcidEngine } from '../audio/engine';
import { PARAM } from '../audio/engine';
import { PRESETS } from '../presets';
import { bpm, registerEngine } from '../../shared/stores/transport';
import { registerMixerCallback } from '../../shared/stores/mixer';

let engine: AcidEngine | null = null;
let lastAcidStep = -1;

export function setAcidEngine(e: AcidEngine) {
    engine = e;
    e.onStep = (step) => {
        currentStep.set(step);
        if (step === 0 && lastAcidStep > 0) {
            const bank = get(acidSequenceBank);
            if (bank.length > 1) {
                if (get(acidChainMode)) {
                    switchAcidSequence((get(currentAcidSequenceIndex) + 1) % bank.length);
                } else if (get(acidRandomMode)) {
                    let nextIdx = get(currentAcidSequenceIndex);
                    while (nextIdx === get(currentAcidSequenceIndex)) nextIdx = Math.floor(Math.random() * bank.length);
                    switchAcidSequence(nextIdx);
                }
            }
        }
        lastAcidStep = step;
    };
    bpm.subscribe((value) => { engine?.seqSetBpm(value); });
    registerMixerCallback('acid', (gain) => { engine?.setParam(PARAM.VOLUME, gain); }, (pan) => { engine?.setPan(pan); });
    registerEngine({
        play: () => { lastAcidStep = -1; engine?.seqSetBpm(get(bpm)); engine?.seqPlay(); },
        stop: () => { engine?.seqStop(); currentStep.set(0); lastAcidStep = -1; },
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

// --- Settings ---
export const settingsOpen = writable(false);

function buildSettingsDefaults(): Record<number, number> {
    const vals: Record<number, number> = {};
    for (const section of ACID_SETTINGS) {
        for (const p of section.params) {
            vals[p.id] = p.default;
        }
    }
    return vals;
}

export const settingsValues = writable<Record<number, number>>(buildSettingsDefaults());

export function toggleSettings() {
    settingsOpen.update(v => !v);
}

export function setSettingsParam(id: number, value: number) {
    settingsValues.update(v => { v[id] = value; return { ...v }; });
    engine?.setParam(id, value);
}

// --- Quick Slots ---
function findSettingsParam(id: number): SettingsParam | null {
    for (const section of ACID_SETTINGS) {
        for (const p of section.params) if (p.id === id) return p;
    }
    return null;
}

function buildInitialSlots(): QuickSlot[] {
    const slots: QuickSlot[] = Array(NUM_QUICK_SLOTS).fill(null);
    const frontPanelIds = [PARAM.CUTOFF, PARAM.RESONANCE, PARAM.ENV_MOD, PARAM.DISTORTION];
    frontPanelIds.forEach((id, i) => { slots[i] = findSettingsParam(id); });
    return slots;
}

export const quickSlots = writable<QuickSlot[]>(buildInitialSlots());
export const activeQuickSlot = writable<number | null>(0);

export function assignQuickSlot(slotIndex: number, param: SettingsParam | null) {
    quickSlots.update(s => { s[slotIndex] = param; return [...s]; });
}

export function selectQuickSlot(slotIndex: number) {
    const slots = get(quickSlots);
    if (!slots[slotIndex]) return;
    activeQuickSlot.set(slotIndex);
}

export function clearQuickSlotSelection() {
    activeQuickSlot.set(null);
}

export function getQuickSlotSliderValue(): number {
    const idx = get(activeQuickSlot);
    if (idx === null) return 0;
    const slot = get(quickSlots)[idx];
    if (!slot) return 0;
    const raw = get(settingsValues)[slot.id] ?? slot.default;
    return ((raw - slot.min) / (slot.max - slot.min)) * 100;
}

export function setQuickSlotSliderValue(value: number) {
    const idx = get(activeQuickSlot);
    if (idx === null) return;
    const slot = get(quickSlots)[idx];
    if (!slot) return;
    const actual = slot.min + (value / 100) * (slot.max - slot.min);
    setSettingsParam(slot.id, actual);
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
    const decayVal = 0.03 + (preset.decay / 100) * 2.97;
    const accentVal = preset.accent / 100;
    const waveVal = preset.waveform === 'saw' ? 0 : 1;
    engine.setParam(PARAM.DECAY, decayVal);
    engine.setParam(PARAM.ACCENT, accentVal);
    waveform.set(preset.waveform);
    engine.setParam(PARAM.WAVEFORM, waveVal);
    settingsValues.set({ [PARAM.DECAY]: decayVal, [PARAM.ACCENT]: accentVal, [PARAM.WAVEFORM]: waveVal });
    currentTranspose.set(0);
}

// --- Sequence bank ---
const MAX_SEQUENCES = 8;

interface AcidSequenceSnapshot {
    stepNotes: number[];
    stepGates: boolean[];
    stepAccents: boolean[];
    stepSlides: boolean[];
}

function captureAcidSequence(): AcidSequenceSnapshot {
    return {
        stepNotes: [...get(stepNotes)],
        stepGates: [...get(stepGates)],
        stepAccents: [...get(stepAccents)],
        stepSlides: [...get(stepSlides)],
    };
}

function restoreAcidSequence(snapshot: AcidSequenceSnapshot) {
    engine?.seqClear();
    const notes = [...snapshot.stepNotes];
    const gates = [...snapshot.stepGates];
    const accents = [...snapshot.stepAccents];
    const slides = [...snapshot.stepSlides];
    for (let i = 0; i < NUM_STEPS; i++) {
        engine?.setStepNote(i, notes[i]);
        engine?.setStepGate(i, gates[i]);
        engine?.setStepAccent(i, accents[i]);
        engine?.setStepSlide(i, slides[i]);
    }
    stepNotes.set(notes);
    stepGates.set(gates);
    stepAccents.set(accents);
    stepSlides.set(slides);
}

export const acidSequenceBank = writable<AcidSequenceSnapshot[]>([captureAcidSequence()]);
export const currentAcidSequenceIndex = writable(0);
export const acidChainMode = writable(false);
export const acidRandomMode = writable(false);

export function toggleAcidChain() {
    acidChainMode.update(v => { if (!v) acidRandomMode.set(false); return !v; });
}
export function toggleAcidRandom() {
    acidRandomMode.update(v => { if (!v) acidChainMode.set(false); return !v; });
}

export function switchAcidSequence(index: number) {
    const bank = get(acidSequenceBank);
    if (index < 0 || index >= bank.length || index === get(currentAcidSequenceIndex)) return;
    bank[get(currentAcidSequenceIndex)] = captureAcidSequence();
    acidSequenceBank.set(bank);
    restoreAcidSequence(bank[index]);
    currentAcidSequenceIndex.set(index);
}

export function addAcidSequence() {
    const bank = get(acidSequenceBank);
    if (bank.length >= MAX_SEQUENCES) return;
    bank[get(currentAcidSequenceIndex)] = captureAcidSequence();
    bank.push({
        stepNotes: Array(NUM_STEPS).fill(48),
        stepGates: Array(NUM_STEPS).fill(false),
        stepAccents: Array(NUM_STEPS).fill(false),
        stepSlides: Array(NUM_STEPS).fill(false),
    });
    acidSequenceBank.set(bank);
    const newIdx = bank.length - 1;
    restoreAcidSequence(bank[newIdx]);
    currentAcidSequenceIndex.set(newIdx);
}

export function deleteAcidSequence() {
    const bank = get(acidSequenceBank);
    if (bank.length <= 1) return;
    const idx = get(currentAcidSequenceIndex);
    bank.splice(idx, 1);
    const newIdx = Math.min(idx, bank.length - 1);
    acidSequenceBank.set(bank);
    restoreAcidSequence(bank[newIdx]);
    currentAcidSequenceIndex.set(newIdx);
}

export function duplicateAcidSequence() {
    const bank = get(acidSequenceBank);
    if (bank.length >= MAX_SEQUENCES) return;
    const current = captureAcidSequence();
    bank[get(currentAcidSequenceIndex)] = current;
    bank.push({
        stepNotes: [...current.stepNotes],
        stepGates: [...current.stepGates],
        stepAccents: [...current.stepAccents],
        stepSlides: [...current.stepSlides],
    });
    acidSequenceBank.set(bank);
    currentAcidSequenceIndex.set(bank.length - 1);
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
