import { writable, derived, get } from 'svelte/store';
import type { BraidsEngine } from '../audio/engine';
import { PARAM } from '../audio/engine';
import {
    MODELS, LEAD_PARAMS, LEAD_PARAM_MAP, LEAD_SETTINGS, SCALE_NOTES, SCALE_CHORDS,
    NUM_STEPS, type LeadParamName, type PadMode, type ArpMode, type ArpDivision,
} from '../constants';
import { NUM_QUICK_SLOTS, type QuickSlot, type SettingsParam } from '../../shared/types/settings';
import { LEAD_PRESETS } from '../presets';
import { registerMixerCallback } from '../../shared/stores/mixer';
import type { NoteSequencerStore } from '../../shared/stores/noteSequencer';
import { bpm, isPlaying, isRecording, registerEngine } from '../../shared/stores/transport';

let engine: BraidsEngine | null = null;
export function setLeadEngine(e: BraidsEngine) {
    engine = e;
    e.onStep = (step) => {
        seqCurrentStep.set(step);
        if (get(arpMode) !== 'off') {
            // Feed sequencer chord into the arp
            const steps = get(seqSteps);
            if (steps[step]?.gate && steps[step].notes.length > 0) {
                updateArpChord(steps[step].notes);
            }
            // Clock the arp from the sequencer step
            arpOnStep();
        }
    };
    registerMixerCallback('lead',
        (gain) => { engine?.setParam(PARAM.MASTER_VOL, gain); },
        (pan) => { engine?.setPan(pan); }
    );
    registerEngine({
        play: () => {
            const arpOn = get(arpMode) !== 'off';
            engine?.setSeqExternal(arpOn);
            engine?.seqSetBpm(get(bpm)); engine?.seqPlay();
            if (arpOn) {
                const steps = get(seqSteps);
                const first = steps.find(s => s.gate && s.notes.length > 0);
                if (first) startArp(first.notes);
            }
        },
        stop: () => {
            engine?.seqStop(); seqCurrentStep.set(0);
            engine?.setSeqExternal(false);
            stopArp();
            // Belt-and-suspenders: release any note the engine might be holding
            for (let n = 0; n < 128; n++) engine?.noteOff(n);
        },
    });
    bpm.subscribe((value) => { engine?.seqSetBpm(value); });
    // React to global play/stop changes for arp sync
    isPlaying.subscribe((playing) => {
        if (playing && get(arpMode) !== 'off') {
            engine?.setSeqExternal(true);
            const steps = get(seqSteps);
            const first = steps.find(s => s.gate && s.notes.length > 0);
            if (first && arpNotes.length === 0) startArp(first.notes);
        } else if (!playing) {
            engine?.setSeqExternal(false);
            stopArp();
        }
    });
}

// --- Model ---
export const selectedModel = writable(0);
export function selectModel(index: number) {
    selectedModel.set(index);
    engine?.setParam(PARAM.MODEL, index);
}

// --- Pad mode ---
export const padMode = writable<PadMode>('note');
export function togglePadMode() {
    padMode.update(m => m === 'note' ? 'chord' : 'note');
}

// --- Params ---
export const selectedParam = writable<LeadParamName>('timbre');
export const synthParams = writable<Record<LeadParamName, number>>({
    timbre: 50, color: 50, cutoff: 65, release: 30,
});
export const sliderValue = derived(
    [selectedParam, synthParams],
    ([$p, $ps]) => $ps[$p]
);
export function selectLeadParam(param: string) { selectedParam.set(param as LeadParamName); }
export function setSliderValue(value: number) {
    const param = get(selectedParam);
    synthParams.update(p => { p[param] = value; return p; });
    const m = LEAD_PARAM_MAP[param];
    engine?.setParam(m.id, m.min + (value / 100) * (m.max - m.min));
}

// --- Settings ---
export const settingsOpen = writable(false);

function buildSettingsDefaults(): Record<number, number> {
    const vals: Record<number, number> = {};
    for (const section of LEAD_SETTINGS) {
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
    for (const section of LEAD_SETTINGS) {
        for (const p of section.params) if (p.id === id) return p;
    }
    return null;
}

function buildInitialSlots(): QuickSlot[] {
    const slots: QuickSlot[] = Array(NUM_QUICK_SLOTS).fill(null);
    const frontPanelIds = [PARAM.TIMBRE, PARAM.COLOR, PARAM.FILTER_CUTOFF, PARAM.AMP_RELEASE];
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

// --- Latch ---
export const latchEnabled = writable(false);
export function toggleLatch() { latchEnabled.update(v => !v); }

// --- Note triggering ---
export const triggeredNotes = writable(new Set<number>());
let activeNotes: number[] = [];

function releaseAll() {
    for (const n of activeNotes) engine?.noteOff(n);
    activeNotes = [];
}

export function triggerPad(padIndex: number) {
    // Visual flash always
    triggeredNotes.update(s => { s.add(padIndex); return new Set(s); });
    setTimeout(() => { triggeredNotes.update(s => { s.delete(padIndex); return new Set(s); }); }, 150);

    const mode = get(padMode);
    let notes: number[];
    if (mode === 'chord') {
        notes = [...SCALE_CHORDS[padIndex].notes];
    } else {
        notes = [SCALE_NOTES[padIndex].note];
    }

    if (get(arpMode) !== 'off') {
        // Arp mode: feed notes to arp, let arp handle all noteOn/noteOff
        startArp(notes);
    } else {
        // Direct mode: play notes immediately
        releaseAll();
        activeNotes = notes;
        for (const n of notes) engine?.noteOn(n, 127);

        if (!get(latchEnabled)) {
            const toRelease = [...notes];
            setTimeout(() => {
                if (activeNotes.length === toRelease.length && activeNotes.every((n, i) => n === toRelease[i])) {
                    releaseAll();
                }
            }, 800);
        }
    }
}

// --- Sequencer (WASM-side, sample-accurate) ---
export interface SeqStep {
    notes: number[]; gate: boolean; label: string;
    velocity: number; gatePct: number; probability: number; ratchet: number; skip: boolean;
}

const PAGE_SIZE = 16;
const MAX_PAGES = 8;

export const seqSteps = writable<SeqStep[]>(
    Array.from({ length: PAGE_SIZE }, () => ({ notes: [48], gate: false, label: '', velocity: 100, gatePct: 75, probability: 100, ratchet: 1, skip: false }))
);
export const seqNumPages = writable(1);
export const seqCurrentPage = writable(0);
export const seqSelectedStep = writable(0); // global step index across all pages
export const seqCurrentStep = writable(0);  // playback position (global)

export function addSeqPage() {
    const pages = get(seqNumPages);
    if (pages >= MAX_PAGES) return;
    seqNumPages.set(pages + 1);
    seqSteps.update(s => {
        for (let i = 0; i < PAGE_SIZE; i++) {
            s.push({ notes: [48], gate: false, label: '', velocity: 100, gatePct: 75, probability: 100, ratchet: 1, skip: false });
        }
        return [...s];
    });
    engine?.setSeqLength((pages + 1) * PAGE_SIZE);
    // Navigate to the new page
    seqCurrentPage.set(pages);
}

export function setSeqPage(page: number) {
    const pages = get(seqNumPages);
    seqCurrentPage.set(Math.max(0, Math.min(page, pages - 1)));
}

export function setSeqStepFromPad(padIndex: number, targetStep?: number, gatePct?: number) {
    const step = targetStep ?? get(seqSelectedStep);
    const mode = get(padMode);
    let notes: number[];
    let label: string;
    if (mode === 'chord') {
        const chord = SCALE_CHORDS[padIndex];
        notes = [...chord.notes];
        label = chord.label;
    } else {
        const n = SCALE_NOTES[padIndex];
        notes = [n.note];
        label = n.label;
    }
    const gp = gatePct ?? 75;
    seqSteps.update(s => { s[step] = { ...s[step], notes, gate: true, label, gatePct: gp }; return [...s]; });
    engine?.setStepNotes(step, notes);
    engine?.setStepGatePct(step, gp);
}

export function toggleSeqStepGate(step: number) {
    seqSteps.update(s => { s[step].gate = !s[step].gate; return [...s]; });
    engine?.setStepGate(step, get(seqSteps)[step].gate);
}

export function selectSeqStep(step: number) { seqSelectedStep.set(step); }

export function clearSelectedStep() {
    const step = get(seqSelectedStep);
    seqSteps.update(s => { s[step].gate = false; return [...s]; });
    engine?.setStepGate(step, false);
}

// --- Seq pattern settings ---
export const seqDirection = writable(0);  // 0=fwd, 1=rev, 2=ping, 3=rnd
export const seqSwing = writable(0);      // 0-100
export const seqTimeDivision = writable(2); // 0=1/4, 1=1/8, 2=1/16, 3=1/32
export const seqSettingsOpen = writable(false);
export const stepSettingsOpen = writable(false);
export const lenMode = writable(false);

export function toggleLenMode() { lenMode.update(v => !v); }
export function setLenFromStep(endStep: number) {
    const start = get(seqSelectedStep);
    const step = get(seqSteps)[start];
    if (!step?.gate || endStep === start) { lenMode.set(false); return; }
    const len = endStep > start ? endStep - start : (get(seqSteps).length - start + endStep);
    const gatePct = Math.max(100, len * 100);
    seqSteps.update(s => { s[start].gatePct = gatePct; return [...s]; });
    engine?.setStepGatePct(start, gatePct);
    lenMode.set(false);
}

export function toggleSeqSettings() {
    seqSettingsOpen.update(v => { if (!v) { stepSettingsOpen.set(false); arpSettingsOpen.set(false); } return !v; });
}
export function toggleStepSettings() {
    stepSettingsOpen.update(v => { if (!v) { seqSettingsOpen.set(false); arpSettingsOpen.set(false); } return !v; });
}
export function setSeqDirection(dir: number) { seqDirection.set(dir); engine?.setDirection(dir); }
export function setSeqSwing(val: number) { seqSwing.set(val); engine?.setSwing(val / 100); }
export function setSeqTimeDivision(div: number) { seqTimeDivision.set(div); engine?.setTimeDivision(div); }
export function rotatePattern(dir: number) {
    engine?.seqRotate(dir);
    // Sync the local step data
    seqSteps.update(s => {
        const len = s.length;
        const buf = [...s];
        for (let i = 0; i < len; i++) {
            const src = dir > 0 ? (i === 0 ? len - 1 : i - 1) : (i + 1) % len;
            s[i] = buf[src];
        }
        return [...s];
    });
}
export function randomizeGates() {
    seqSteps.update(s => {
        for (let i = 0; i < s.length; i++) {
            if (s[i].gate) {
                s[i].probability = 30 + Math.floor(Math.random() * 70);
                engine?.setStepProbability(i, s[i].probability);
            }
        }
        return [...s];
    });
}

// --- Per-step editing ---
export function setStepVelocity(val: number) {
    const step = get(seqSelectedStep);
    seqSteps.update(s => { s[step].velocity = val; return [...s]; });
    engine?.setStepVelocity(step, Math.round(val * 1.27)); // 0-100 → 0-127
}
export function setStepGatePct(val: number) {
    const step = get(seqSelectedStep);
    seqSteps.update(s => { s[step].gatePct = val; return [...s]; });
    engine?.setStepGatePct(step, val);
}
export function setStepGatePctAt(step: number, val: number) {
    seqSteps.update(s => { s[step].gatePct = val; return [...s]; });
    engine?.setStepGatePct(step, val);
}
export function setStepProbability(val: number) {
    const step = get(seqSelectedStep);
    seqSteps.update(s => { s[step].probability = val; return [...s]; });
    engine?.setStepProbability(step, val);
}
export function setStepRatchet(val: number) {
    const step = get(seqSelectedStep);
    seqSteps.update(s => { s[step].ratchet = val; return [...s]; });
    engine?.setStepRatchet(step, val);
}
export function toggleStepSkip() {
    const step = get(seqSelectedStep);
    seqSteps.update(s => { s[step].skip = !s[step].skip; return [...s]; });
    engine?.setStepSkip(step, get(seqSteps)[step].skip);
}

export function clearSequence() {
    const emptySteps = Array.from({ length: PAGE_SIZE }, (): SeqStep => ({ notes: [48], gate: false, label: '', velocity: 100, gatePct: 75, probability: 100, ratchet: 1, skip: false }));
    seqSteps.set(emptySteps);
    seqNumPages.set(1);
    seqCurrentPage.set(0);
    seqSelectedStep.set(0);
    engine?.seqClear();
    engine?.setSeqLength(PAGE_SIZE);
}

export function removeStepGate(step: number) {
    seqSteps.update(s => { s[step].gate = false; return [...s]; });
    engine?.setStepGate(step, false);
}

export const currentLeadPreset = writable(-1);

export function loadLeadPreset(index: number) {
    const preset = LEAD_PRESETS[index];
    if (!preset || !engine) return;
    currentLeadPreset.set(index);
    clearSequence();
    // Load steps
    const newSteps = preset.steps.map(s => ({ ...s }));
    seqSteps.set(newSteps);
    for (let i = 0; i < newSteps.length; i++) {
        const s = newSteps[i];
        if (s.gate) {
            engine.setStepNotes(i, s.notes);
            engine.setStepVelocity(i, Math.round(s.velocity * 1.27));
            engine.setStepGatePct(i, s.gatePct);
            engine.setStepProbability(i, s.probability);
            engine.setStepRatchet(i, s.ratchet);
            if (s.skip) engine.setStepSkip(i, true);
        }
    }
    // Load synth params
    selectModel(preset.model);
    synthParams.set({
        timbre: preset.timbre, color: preset.color,
        cutoff: preset.cutoff, release: preset.release,
    });
    for (const paramName of LEAD_PARAMS) {
        const m = LEAD_PARAM_MAP[paramName];
        const sv = paramName === 'timbre' ? preset.timbre : paramName === 'color' ? preset.color
            : paramName === 'cutoff' ? preset.cutoff : preset.release;
        engine.setParam(m.id, m.min + (sv / 100) * (m.max - m.min));
    }
}

export function moveStep(fromIdx: number, toIdx: number) {
    seqSteps.update(s => {
        const totalSteps = s.length;
        if (fromIdx < 0 || fromIdx >= totalSteps || toIdx < 0 || toIdx >= totalSteps) return s;
        const moved = { ...s[fromIdx] };
        s[fromIdx] = { notes: [48], gate: false, label: '', velocity: 100, gatePct: 75, probability: 100, ratchet: 1, skip: false };
        s[toIdx] = moved;
        return [...s];
    });
    // Sync both steps to WASM
    const steps = get(seqSteps);
    engine?.setStepGate(fromIdx, false);
    if (steps[toIdx].gate) {
        engine?.setStepNotes(toIdx, steps[toIdx].notes);
        engine?.setStepGatePct(toIdx, steps[toIdx].gatePct);
    } else {
        engine?.setStepGate(toIdx, false);
    }
}

// --- Arpeggiator (clocked by WASM sequencer steps, not setInterval) ---
export const arpMode = writable<ArpMode>('off');
export const arpDivision = writable<ArpDivision>('1/8');
export const arpOctaves = writable(1);
export const arpSettingsOpen = writable(false);

let arpRunning = false;
let arpNotes: number[] = [];
let arpIndex = 0;
let arpDirection = 1;
let arpOctave = 0;
let arpStepCounter = 0; // counts sequencer steps to determine when to tick

// How many sequencer steps per arp tick, based on seq time_div and arp division
// Both are relative to the beat. Seq time_div: 0=1/4, 1=1/8, 2=1/16, 3=1/32
// Arp division: '1/4', '1/8', '1/16', '1/32'
function getArpStepsPerTick(): number {
    const seqDiv = get(seqTimeDivision);
    // Sequencer steps per quarter note
    const seqPerQuarter = seqDiv === 0 ? 1 : seqDiv === 1 ? 2 : seqDiv === 3 ? 8 : 4;
    const arpDiv = get(arpDivision);
    // Arp ticks per quarter note
    const arpPerQuarter = arpDiv === '1/4' ? 1 : arpDiv === '1/8' ? 2 : arpDiv === '1/16' ? 4 : 8;
    // Steps per arp tick (can be fractional for fast arps)
    return seqPerQuarter / arpPerQuarter;
}

// Called from onStep — this is the master clock
function arpOnStep() {
    if (!arpRunning || arpNotes.length === 0) return;
    const stepsPerTick = getArpStepsPerTick();

    if (stepsPerTick >= 1) {
        // Arp is slower than or equal to sequencer rate
        arpStepCounter++;
        if (arpStepCounter >= stepsPerTick) {
            arpStepCounter = 0;
            arpTick();
        }
    } else {
        // Arp is faster than sequencer — multiple ticks per step
        const ticksPerStep = Math.round(1 / stepsPerTick);
        for (let i = 0; i < ticksPerStep; i++) {
            arpTick();
        }
    }
}

export function setArpMode(mode: ArpMode) {
    arpMode.set(mode);
    if (mode === 'off') {
        stopArp();
        engine?.setSeqExternal(false);
    } else {
        engine?.setSeqExternal(true);
        if (get(isPlaying)) {
            const steps = get(seqSteps);
            const first = steps.find(s => s.gate && s.notes.length > 0);
            if (first) startArp(first.notes);
        }
    }
}
export function setArpDivision(div: ArpDivision) {
    arpDivision.set(div);
    arpStepCounter = 0; // reset counter so new division takes effect immediately
}
export function setArpOctaves(oct: number) {
    arpOctaves.set(oct);
    if (arpOctave >= oct) arpOctave = 0;
}
export function toggleArpSettings() {
    arpSettingsOpen.update(v => { if (!v) { seqSettingsOpen.set(false); stepSettingsOpen.set(false); } return !v; });
}

let arpLiveTimerId: ReturnType<typeof setInterval> | null = null;

function startArp(notes: number[]) {
    stopArp();
    arpNotes = [...notes].sort((a, b) => a - b);
    arpIndex = 0;
    arpDirection = 1;
    arpOctave = 0;
    arpStepCounter = 0;
    arpRunning = true;
    arpTick();
    // In live mode (not playing), use a timer since there's no sequencer clock
    if (!get(isPlaying)) {
        const q = 60000 / get(bpm);
        const div = get(arpDivision);
        const ms = div === '1/4' ? q : div === '1/8' ? q / 2 : div === '1/16' ? q / 4 : q / 8;
        arpLiveTimerId = setInterval(() => arpTick(), ms);
    }
}

function updateArpChord(notes: number[]) {
    arpNotes = [...notes].sort((a, b) => a - b);
    if (arpIndex >= arpNotes.length) arpIndex = 0;
    if (!arpRunning && arpNotes.length > 0) {
        arpRunning = true;
        arpStepCounter = 0;
        arpTick();
    }
}

function stopArp() {
    arpRunning = false;
    if (arpLiveTimerId) { clearInterval(arpLiveTimerId); arpLiveTimerId = null; }
    releaseAll();
    arpNotes = [];
    arpIndex = 0;
    arpOctave = 0;
    arpStepCounter = 0;
}

function arpTick() {
    if (arpNotes.length === 0 || !engine) return;
    if (get(arpMode) === 'off') return;

    // Release previous note
    for (const n of activeNotes) engine.noteOff(n);

    const octaves = get(arpOctaves);
    const note = arpNotes[arpIndex % arpNotes.length] + arpOctave * 12;
    activeNotes = [note];
    engine.noteOn(note, 127);

    // Advance
    const mode = get(arpMode);
    switch (mode) {
        case 'up':
            arpIndex++;
            if (arpIndex >= arpNotes.length) { arpIndex = 0; arpOctave = (arpOctave + 1) % octaves; }
            break;
        case 'down':
            arpIndex--;
            if (arpIndex < 0) { arpIndex = arpNotes.length - 1; arpOctave = arpOctave > 0 ? arpOctave - 1 : octaves - 1; }
            break;
        case 'updown':
            arpIndex += arpDirection;
            if (arpIndex >= arpNotes.length) { arpDirection = -1; arpIndex = Math.max(arpNotes.length - 2, 0); }
            else if (arpIndex < 0) { arpDirection = 1; arpIndex = Math.min(1, arpNotes.length - 1); arpOctave = (arpOctave + 1) % octaves; }
            break;
        case 'random':
            arpIndex = Math.floor(Math.random() * arpNotes.length);
            arpOctave = Math.floor(Math.random() * octaves);
            break;
    }
}

// --- Adapter: bundle sequencer stores for shared NoteSequencer components ---
export const leadSeq: NoteSequencerStore = {
    seqSteps, seqNumPages, seqCurrentPage, seqSelectedStep, seqCurrentStep,
    seqDirection, seqSwing, seqTimeDivision, seqSettingsOpen, stepSettingsOpen, lenMode,
    connectEngine: () => {}, connectOnStep: () => {},
    addSeqPage, setSeqPage, selectSeqStep,
    setSeqStepFromNotes: (notes: number[], label: string) => {
        const step = get(seqSelectedStep);
        seqSteps.update(s => { s[step] = { ...s[step], notes, gate: true, label }; return [...s]; });
        engine?.setStepNotes(step, notes);
    },
    toggleSeqStepGate, removeStepGate, moveStep, clearSequence, clearSelectedStep,
    setStepVelocity, setStepGatePct, setStepGatePctAt, setStepProbability, setStepRatchet, toggleStepSkip,
    setSeqDirection, setSeqSwing, setSeqTimeDivision, rotatePattern, randomizeGates,
    toggleLenMode, setLenFromStep,
    toggleSeqSettings, toggleStepSettings,
    closeAllDrawers: () => { seqSettingsOpen.set(false); stepSettingsOpen.set(false); lenMode.set(false); arpSettingsOpen.set(false); },
};
