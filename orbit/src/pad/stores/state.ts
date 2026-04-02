import { writable, derived, get } from 'svelte/store';
import { CHORDS, PAD_PARAMS, PAD_PARAM_MAP, PAD_SETTINGS, type PadParamName } from '../constants';
import { NUM_QUICK_SLOTS, type QuickSlot, type SettingsParam } from '../../shared/types/settings';
import type { ProphetEngine } from '../audio/engine';
import { PARAM } from '../audio/engine';
import { PRESETS, type Preset } from '../presets';
import { bpm } from '../../shared/stores/transport';
import { registerMixerCallback } from '../../shared/stores/mixer';
import { createNoteSequencerStore } from '../../shared/stores/noteSequencer';

// Shared note sequencer for the PAD panel
export const padSeq = createNoteSequencerStore();

let engine: ProphetEngine | null = null;
export function setPadEngine(e: ProphetEngine) {
    engine = e;
    e.onStep = (step) => padSeq.connectOnStep(step);
    padSeq.connectEngine(e);
    bpm.subscribe((value) => {
        if (engine && get(arpEnabled)) {
            engine.setParam(PARAM.ARP_BPM, value);
        }
    });
    registerMixerCallback('pad', (gain) => { engine?.setParam(PARAM.MASTER_VOL, gain); }, (pan) => { engine?.setPan(pan); });
}

// Preset
export const currentPresetIndex = writable(0);

// Arpeggiator
export const arpEnabled = writable(false);

// Selection
export const selectedChord = writable(0);
export const selectedPadParam = writable<PadParamName>('cutoff');

// Global synth parameters (0-100 slider values)
export const padParams = writable<Record<PadParamName, number>>({
    cutoff: 70,
    resonance: 25,
    attack: 40,
    release: 60,
});

// Trigger flash
export const triggeredChords = writable(new Set<number>());

// Track active notes for proper note-off
let activeNotes: number[] = [];

// Derived
export const padSliderValue = derived(
    [selectedPadParam, padParams],
    ([$param, $params]) => $params[$param]
);

// Actions
export function selectChord(index: number) {
    selectedChord.set(index);
}

export function selectPadParam(param: string) {
    selectedPadParam.set(param as PadParamName);
}

export function triggerChord(index: number) {
    const chord = CHORDS[index];

    // Release any currently held notes
    for (const note of activeNotes) {
        engine?.noteOff(note);
    }

    // Play the new chord
    activeNotes = [...chord.notes];
    for (const note of chord.notes) {
        engine?.noteOn(note, 127);
    }

    // Visual trigger pulse
    triggeredChords.update(s => { s.add(index); return new Set(s); });
    setTimeout(() => {
        triggeredChords.update(s => { s.delete(index); return new Set(s); });
    }, 200);

    // When arp is on, keep notes held so the arpeggiator cycles through them.
    // They get released when the next chord is triggered (see noteOff loop above).
    if (!get(arpEnabled)) {
        const notesToRelease = [...chord.notes];
        setTimeout(() => {
            if (activeNotes.length === notesToRelease.length &&
                activeNotes.every((n, i) => n === notesToRelease[i])) {
                for (const note of notesToRelease) {
                    engine?.noteOff(note);
                }
                activeNotes = [];
            }
        }, 1200);
    }
}

export function setPadSliderValue(value: number) {
    const param = get(selectedPadParam);
    padParams.update(p => { p[param] = value; return p; });

    // Map 0-100 to actual parameter range
    const mapping = PAD_PARAM_MAP[param];
    const actual = mapping.min + (value / 100) * (mapping.max - mapping.min);
    engine?.setParam(mapping.id, actual);
}

export function toggleArp() {
    const on = !get(arpEnabled);
    arpEnabled.set(on);

    if (on) {
        // Release any notes currently playing in the synth voices
        for (const note of activeNotes) {
            engine?.noteOff(note);
        }
        // Enable arp mode BEFORE sending notes so they route into the arpeggiator
        engine?.setParam(PARAM.ARP_MODE, 1);
        engine?.setParam(PARAM.ARP_BPM, get(bpm));
        engine?.setParam(PARAM.ARP_OCTAVES, 2);
        engine?.setParam(PARAM.ARP_GATE, 0.5);
        // Re-send held notes so the arpeggiator picks them up
        for (const note of activeNotes) {
            engine?.noteOn(note, 127);
        }
    } else {
        // Release held notes from the arpeggiator, then disable
        for (const note of activeNotes) {
            engine?.noteOff(note);
        }
        engine?.setParam(PARAM.ARP_MODE, 0);
        activeNotes = [];
    }
}

// --- Settings ---
export const settingsOpen = writable(false);

// Build initial values from defaults
function buildSettingsDefaults(): Record<number, number> {
    const vals: Record<number, number> = {};
    for (const section of PAD_SETTINGS) {
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
    for (const section of PAD_SETTINGS) {
        for (const p of section.params) if (p.id === id) return p;
    }
    return null;
}

function buildInitialSlots(): QuickSlot[] {
    const slots: QuickSlot[] = Array(NUM_QUICK_SLOTS).fill(null);
    // Pre-populate with front-panel params
    const frontPanelIds = [PARAM.FILTER_CUTOFF, PARAM.FILTER_RESONANCE, PARAM.AMP_ATTACK, PARAM.AMP_RELEASE];
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

export function loadPreset(index: number) {
    const preset = PRESETS[index];
    if (!preset || !engine) return;
    currentPresetIndex.set(index);
    for (const [id, value] of preset.params) {
        engine.setParam(id, value);
    }
    // Update settings values to reflect preset
    settingsValues.update(v => {
        for (const [id, value] of preset.params) {
            if (id in v) v[id] = value;
        }
        return { ...v };
    });
    // Update slider values to reflect new preset's mapped params
    padParams.update(p => {
        for (const paramName of PAD_PARAMS) {
            const mapping = PAD_PARAM_MAP[paramName];
            const presetEntry = preset.params.find(([id]) => id === mapping.id);
            if (presetEntry) {
                const [, actual] = presetEntry;
                p[paramName] = Math.round(((actual - mapping.min) / (mapping.max - mapping.min)) * 100);
            }
        }
        return p;
    });
}
