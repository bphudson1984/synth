import { writable, derived, get } from 'svelte/store';
import type { BraidsEngine } from '../audio/engine';
import { PARAM } from '../audio/engine';
import {
    MODELS, LEAD_PARAMS, LEAD_PARAM_MAP, SCALE_NOTES, SCALE_CHORDS,
    NUM_STEPS, type LeadParamName, type PadMode, type ArpMode, type ArpDivision,
} from '../constants';
import { LEAD_PRESETS } from '../presets';
import { registerMixerCallback } from '../../shared/stores/mixer';
import { bpm, isPlaying, registerEngine } from '../../shared/stores/transport';

let engine: BraidsEngine | null = null;
export function setLeadEngine(e: BraidsEngine) {
    engine = e;
    e.onStep = (step) => {
        seqCurrentStep.set(step);
        // When arp is active, feed sequencer chord into the running arp
        if (get(arpMode) !== 'off') {
            const steps = get(seqSteps);
            if (steps[step]?.gate && steps[step].notes.length > 0) {
                updateArpChord(steps[step].notes);
            }
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
                else ensureArpRunning();
            }
        },
        stop: () => {
            engine?.seqStop(); seqCurrentStep.set(0);
            engine?.setSeqExternal(false);
            stopArp();
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
            else ensureArpRunning();
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

    // When playing, pads only write to sequencer — don't play directly
    if (get(isPlaying)) return;

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
        for (const n of notes) engine?.noteOn(n, 100);

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
export interface SeqStep { notes: number[]; gate: boolean; label: string; }

const PAGE_SIZE = 16;
const MAX_PAGES = 8;

export const seqSteps = writable<SeqStep[]>(
    Array.from({ length: PAGE_SIZE }, () => ({ notes: [48], gate: false, label: '' }))
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
            s.push({ notes: [48], gate: false, label: '' });
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

export function setSeqStepFromPad(padIndex: number) {
    const step = get(seqSelectedStep);
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
    seqSteps.update(s => { s[step] = { notes, gate: true, label }; return [...s]; });
    engine?.setStepNotes(step, notes);
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

export function clearSequence() {
    const emptySteps = Array.from({ length: PAGE_SIZE }, (): SeqStep => ({ notes: [48], gate: false, label: '' }));
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
        if (newSteps[i].gate) {
            engine.setStepNotes(i, newSteps[i].notes);
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
        s[fromIdx] = { notes: [48], gate: false, label: '' };
        s[toIdx] = moved;
        return [...s];
    });
    // Sync both steps to WASM
    const steps = get(seqSteps);
    engine?.setStepGate(fromIdx, false);
    if (steps[toIdx].gate) {
        engine?.setStepNotes(toIdx, steps[toIdx].notes);
    } else {
        engine?.setStepGate(toIdx, false);
    }
}

// --- Arpeggiator (JS-side) ---
export const arpMode = writable<ArpMode>('off');
export const arpDivision = writable<ArpDivision>('1/8');
export const arpOctaves = writable(1);
export const arpSettingsOpen = writable(false);

let arpTimerId: ReturnType<typeof setInterval> | null = null;
let arpNotes: number[] = [];
let arpIndex = 0;
let arpDirection = 1;
let arpOctave = 0;

export function setArpMode(mode: ArpMode) {
    arpMode.set(mode);
    if (mode === 'off') {
        stopArp();
        engine?.setSeqExternal(false); // sequencer plays notes directly
    } else {
        engine?.setSeqExternal(true); // sequencer only reports steps, arp plays notes
    }
}
export function setArpDivision(div: ArpDivision) {
    arpDivision.set(div);
    if (arpTimerId && arpNotes.length > 0) restartArpTimer();
}
export function setArpOctaves(oct: number) { arpOctaves.set(oct); }
export function toggleArpSettings() { arpSettingsOpen.update(v => !v); }

function startArp(notes: number[]) {
    stopArp();
    arpNotes = [...notes].sort((a, b) => a - b);
    arpIndex = 0;
    arpDirection = 1;
    arpOctave = 0;
    arpTick();
    arpTimerId = setInterval(() => arpTick(), getArpIntervalMs());
}

function updateArpChord(notes: number[]) {
    const wasEmpty = arpNotes.length === 0;
    arpNotes = [...notes].sort((a, b) => a - b);
    if (arpIndex >= arpNotes.length) arpIndex = 0;
    // If arp timer is running but had no notes, kick it
    if (wasEmpty && arpNotes.length > 0 && arpTimerId) {
        arpTick();
    }
}

function stopArp() {
    if (arpTimerId) { clearInterval(arpTimerId); arpTimerId = null; }
    releaseAll();
    arpNotes = [];
    arpIndex = 0;
    arpOctave = 0;
}

function ensureArpRunning() {
    if (get(arpMode) === 'off') return;
    if (!arpTimerId) {
        arpTimerId = setInterval(() => arpTick(), getArpIntervalMs());
    }
}

function getArpIntervalMs(): number {
    const q = 60000 / get(bpm);
    switch (get(arpDivision)) {
        case '1/4': return q;
        case '1/8': return q / 2;
        case '1/16': return q / 4;
        case '1/32': return q / 8;
    }
}

function arpTick() {
    if (arpNotes.length === 0 || !engine) return;
    if (get(arpMode) === 'off') return;

    // Release previous note
    for (const n of activeNotes) engine.noteOff(n);

    const octaves = get(arpOctaves);
    const note = arpNotes[arpIndex % arpNotes.length] + arpOctave * 12;
    activeNotes = [note];
    engine.noteOn(note, 100);

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
