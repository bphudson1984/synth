import { writable, get } from 'svelte/store';
import { BASS_SETTINGS, BASS_PARAMS, BASS_PARAM_MAP, NOTE_PADS, type BassParamName } from '../constants';
import { NUM_QUICK_SLOTS, type QuickSlot, type SettingsParam } from '../../shared/types/settings';
import type { BassEngine } from '../audio/engine';
import { PARAM } from '../audio/engine';
import { SOUND_PRESETS, PATTERN_PRESETS } from '../presets';
import { bpm } from '../../shared/stores/transport';
import { registerMixerCallback } from '../../shared/stores/mixer';
import { createNoteSequencerStore } from '../../shared/stores/noteSequencer';

// Shared note sequencer for the BASS panel (same component as PAD/LEAD)
export const bassSeq = createNoteSequencerStore();

let engine: BassEngine | null = null;
let lastBassStep = -1;

export function setBassEngine(e: BassEngine) {
    engine = e;
    e.onStep = (step) => {
        bassSeq.connectOnStep(step);
        if (step === 0 && lastBassStep > 0) {
            const bank = get(bassSequenceBank);
            if (bank.length > 1) {
                if (get(bassChainMode)) {
                    switchBassSequence((get(currentBassSequenceIndex) + 1) % bank.length);
                } else if (get(bassRandomMode)) {
                    let nextIdx = get(currentBassSequenceIndex);
                    while (nextIdx === get(currentBassSequenceIndex)) nextIdx = Math.floor(Math.random() * bank.length);
                    switchBassSequence(nextIdx);
                }
            }
        }
        lastBassStep = step;
    };
    bassSeq.connectEngine(e);
    bpm.subscribe(() => {});
    registerMixerCallback('bass', (gain) => { engine?.setChannelGain(gain); }, (pan) => { engine?.setPan(pan); });
}

// Note triggering
let activeNote: number | null = null;

export function triggerNote(note: number) {
    if (activeNote !== null) engine?.noteOff(activeNote);
    engine?.noteOn(note, 127);
    activeNote = note;
    // Auto-release after 1.2s if not held by sequencer
    const noteToRelease = note;
    setTimeout(() => {
        if (activeNote === noteToRelease) {
            engine?.noteOff(noteToRelease);
            activeNote = null;
        }
    }, 1200);
}

// Presets (independent sound + pattern)
export const currentSoundPreset = writable(0);
export const currentPatternPreset = writable(0);

// Transposition
export const currentTranspose = writable(0);

// Settings
export const settingsOpen = writable(false);

function buildSettingsDefaults(): Record<number, number> {
    const vals: Record<number, number> = {};
    for (const section of BASS_SETTINGS) {
        for (const p of section.params) vals[p.id] = p.default;
    }
    return vals;
}

export const settingsValues = writable<Record<number, number>>(buildSettingsDefaults());

export function toggleSettings() { settingsOpen.update(v => !v); }

export function setSettingsParam(id: number, value: number) {
    settingsValues.update(v => { v[id] = value; return { ...v }; });
    engine?.setParam(id, value);
}

// Quick Slots
function findSettingsParam(id: number): SettingsParam | null {
    for (const section of BASS_SETTINGS) {
        for (const p of section.params) if (p.id === id) return p;
    }
    return null;
}

function buildInitialSlots(): QuickSlot[] {
    const slots: QuickSlot[] = Array(NUM_QUICK_SLOTS).fill(null);
    const ids = [PARAM.CUTOFF, PARAM.PEAK, PARAM.EG_INT, PARAM.LFO_RATE];
    ids.forEach((id, i) => { slots[i] = findSettingsParam(id); });
    return slots;
}

export const quickSlots = writable<QuickSlot[]>(buildInitialSlots());
export const activeQuickSlot = writable<number | null>(0);

export function assignQuickSlot(slotIndex: number, param: SettingsParam | null) {
    quickSlots.update(s => { s[slotIndex] = param; return [...s]; });
}
export function selectQuickSlot(slotIndex: number) {
    if (!get(quickSlots)[slotIndex]) return;
    activeQuickSlot.set(slotIndex);
}
export function setQuickSlotSliderValue(value: number) {
    const idx = get(activeQuickSlot);
    if (idx === null) return;
    const slot = get(quickSlots)[idx];
    if (!slot) return;
    const actual = slot.min + (value / 100) * (slot.max - slot.min);
    setSettingsParam(slot.id, actual);
}

// Transpose
export function transposePattern(semitones: number) {
    const prev = get(currentTranspose);
    const delta = semitones - prev;
    if (delta === 0) return;
    currentTranspose.set(semitones);
    bassSeq.seqSteps.update(steps => {
        for (const s of steps) {
            if (s.gate) {
                s.notes = s.notes.map(n => Math.max(0, Math.min(127, n + delta)));
            }
        }
        return [...steps];
    });
    // Sync to engine
    const steps = get(bassSeq.seqSteps);
    for (let i = 0; i < steps.length; i++) {
        if (steps[i].gate) engine?.setStepNotes(i, steps[i].notes);
    }
}

// Sound presets (synth params only)
export function loadSoundPreset(index: number) {
    const preset = SOUND_PRESETS[index];
    if (!preset || !engine) return;
    currentSoundPreset.set(index);
    for (const [id, value] of preset.params) {
        engine.setParam(id, value);
    }
    settingsValues.update(v => {
        for (const [id, value] of preset.params) {
            if (id in v) v[id] = value;
        }
        return { ...v };
    });
}

// Pattern presets (step sequence only)
export function loadPatternPreset(index: number) {
    const preset = PATTERN_PRESETS[index];
    if (!preset || !engine) return;
    currentPatternPreset.set(index);
    bassSeq.clearSequence();
    const steps = preset.steps.map(s => ({ ...s, notes: [...s.notes] }));
    bassSeq.seqSteps.set(steps);
    for (let i = 0; i < steps.length; i++) {
        const s = steps[i];
        if (s.gate) {
            engine.setStepNotes(i, s.notes);
            engine.setStepGate(i, true);
            engine.setStepVelocity(i, Math.round(s.velocity * 1.27));
            engine.setStepGatePct(i, s.gatePct);
            engine.setStepProbability(i, s.probability);
            engine.setStepRatchet(i, s.ratchet);
            if (s.skip) engine.setStepSkip(i, true);
        } else {
            engine.setStepGate(i, false);
        }
    }
}

// Sequence bank
const MAX_SEQUENCES = 8;
export const bassSequenceBank = writable<ReturnType<typeof bassSeq.captureSequence>[]>([bassSeq.captureSequence()]);
export const currentBassSequenceIndex = writable(0);
export const bassChainMode = writable(false);
export const bassRandomMode = writable(false);

export function toggleBassChain() { bassChainMode.update(v => { if (!v) bassRandomMode.set(false); return !v; }); }
export function toggleBassRandom() { bassRandomMode.update(v => { if (!v) bassChainMode.set(false); return !v; }); }

export function switchBassSequence(index: number) {
    const bank = get(bassSequenceBank);
    if (index < 0 || index >= bank.length || index === get(currentBassSequenceIndex)) return;
    bank[get(currentBassSequenceIndex)] = bassSeq.captureSequence();
    bassSequenceBank.set(bank);
    bassSeq.restoreSequence(bank[index]);
    currentBassSequenceIndex.set(index);
}
export function addBassSequence() {
    const bank = get(bassSequenceBank);
    if (bank.length >= MAX_SEQUENCES) return;
    bank[get(currentBassSequenceIndex)] = bassSeq.captureSequence();
    bassSeq.clearSequence();
    bank.push(bassSeq.captureSequence());
    bassSequenceBank.set(bank);
    currentBassSequenceIndex.set(bank.length - 1);
}
export function deleteBassSequence() {
    const bank = get(bassSequenceBank);
    if (bank.length <= 1) return;
    const idx = get(currentBassSequenceIndex);
    bank.splice(idx, 1);
    const newIdx = Math.min(idx, bank.length - 1);
    bassSequenceBank.set(bank);
    bassSeq.restoreSequence(bank[newIdx]);
    currentBassSequenceIndex.set(newIdx);
}
export function duplicateBassSequence() {
    const bank = get(bassSequenceBank);
    if (bank.length >= MAX_SEQUENCES) return;
    const current = bassSeq.captureSequence();
    bank[get(currentBassSequenceIndex)] = current;
    const clone = { ...current, steps: current.steps.map(s => ({ ...s, notes: [...s.notes] })) };
    bank.push(clone);
    bassSequenceBank.set(bank);
    currentBassSequenceIndex.set(bank.length - 1);
}
