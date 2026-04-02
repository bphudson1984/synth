import { writable, derived, get } from 'svelte/store';
import { bpm, registerEngine } from './transport';

export interface SeqStep {
    notes: number[]; gate: boolean; label: string;
    velocity: number; gatePct: number; probability: number; ratchet: number; skip: boolean;
}

export interface SeqEngine {
    seqPlay(): void;
    seqStop(): void;
    seqSetBpm(bpm: number): void;
    seqClear(): void;
    noteOff?(note: number): void;
    setStepNotes(step: number, notes: number[]): void;
    setStepGate(step: number, gate: boolean): void;
    setStepVelocity(step: number, vel: number): void;
    setStepGatePct(step: number, pct: number): void;
    setStepProbability(step: number, prob: number): void;
    setStepRatchet(step: number, count: number): void;
    setStepSkip(step: number, skip: boolean): void;
    setDirection(dir: number): void;
    setSwing(swing: number): void;
    setTimeDivision(div: number): void;
    seqRotate(dir: number): void;
    setSeqLength(len: number): void;
}

const PAGE_SIZE = 16;
const MAX_PAGES = 8;

function emptyStep(): SeqStep {
    return { notes: [48], gate: false, label: '', velocity: 100, gatePct: 75, probability: 100, ratchet: 1, skip: false };
}

export function createNoteSequencerStore() {
    let engine: SeqEngine | null = null;

    const seqSteps = writable<SeqStep[]>(Array.from({ length: PAGE_SIZE }, emptyStep));
    const seqNumPages = writable(1);
    const seqCurrentPage = writable(0);
    const seqSelectedStep = writable(0);
    const seqCurrentStep = writable(0);
    const seqDirection = writable(0);
    const seqSwing = writable(0);
    const seqTimeDivision = writable(2);
    const seqSettingsOpen = writable(false);
    const stepSettingsOpen = writable(false);
    const lenMode = writable(false);

    function connectEngine(e: SeqEngine) {
        engine = e;
        registerEngine({
            play: () => { engine?.seqSetBpm(get(bpm)); engine?.seqPlay(); },
            stop: () => {
                engine?.seqStop(); seqCurrentStep.set(0);
                // Release all notes to prevent hanging
                for (let n = 0; n < 128; n++) engine?.noteOff?.(n);
            },
        });
        bpm.subscribe((value) => { engine?.seqSetBpm(value); });
    }

    function connectOnStep(step: number) {
        seqCurrentStep.set(step);
    }

    function addSeqPage() {
        const pages = get(seqNumPages);
        if (pages >= MAX_PAGES) return;
        seqNumPages.set(pages + 1);
        seqSteps.update(s => { for (let i = 0; i < PAGE_SIZE; i++) s.push(emptyStep()); return [...s]; });
        engine?.setSeqLength((pages + 1) * PAGE_SIZE);
        seqCurrentPage.set(pages);
    }

    function setSeqPage(page: number) {
        seqCurrentPage.set(Math.max(0, Math.min(page, get(seqNumPages) - 1)));
    }

    function selectSeqStep(step: number) { seqSelectedStep.set(step); }

    function setSeqStepFromNotes(notes: number[], label: string) {
        const step = get(seqSelectedStep);
        seqSteps.update(s => { s[step] = { ...s[step], notes, gate: true, label }; return [...s]; });
        engine?.setStepNotes(step, notes);
    }

    function setStepFromNotes(stepIndex: number, notes: number[], label: string) {
        seqSteps.update(s => { s[stepIndex] = { ...s[stepIndex], notes, gate: true, label }; return [...s]; });
        engine?.setStepNotes(stepIndex, notes);
    }

    function toggleSeqStepGate(step: number) {
        seqSteps.update(s => { s[step].gate = !s[step].gate; return [...s]; });
        engine?.setStepGate(step, get(seqSteps)[step].gate);
    }

    function removeStepGate(step: number) {
        seqSteps.update(s => { s[step].gate = false; return [...s]; });
        engine?.setStepGate(step, false);
    }

    function moveStep(fromIdx: number, toIdx: number) {
        seqSteps.update(s => {
            if (fromIdx < 0 || fromIdx >= s.length || toIdx < 0 || toIdx >= s.length) return s;
            const moved = { ...s[fromIdx] };
            s[fromIdx] = emptyStep();
            s[toIdx] = moved;
            return [...s];
        });
        engine?.setStepGate(fromIdx, false);
        const steps = get(seqSteps);
        if (steps[toIdx].gate) {
            engine?.setStepNotes(toIdx, steps[toIdx].notes);
            engine?.setStepGatePct(toIdx, steps[toIdx].gatePct);
        } else {
            engine?.setStepGate(toIdx, false);
        }
    }

    function clearSequence() {
        seqSteps.set(Array.from({ length: PAGE_SIZE }, emptyStep));
        seqNumPages.set(1);
        seqCurrentPage.set(0);
        seqSelectedStep.set(0);
        engine?.seqClear();
        engine?.setSeqLength(PAGE_SIZE);
    }

    function clearSelectedStep() {
        const step = get(seqSelectedStep);
        seqSteps.update(s => { s[step].gate = false; return [...s]; });
        engine?.setStepGate(step, false);
    }

    // Per-step editing
    function setStepVelocity(val: number) {
        const step = get(seqSelectedStep);
        seqSteps.update(s => { s[step].velocity = val; return [...s]; });
        engine?.setStepVelocity(step, Math.round(val * 1.27));
    }
    function setStepGatePct(val: number) {
        const step = get(seqSelectedStep);
        seqSteps.update(s => { s[step].gatePct = val; return [...s]; });
        engine?.setStepGatePct(step, val);
    }
    function setStepGatePctAt(step: number, val: number) {
        seqSteps.update(s => { s[step].gatePct = val; return [...s]; });
        engine?.setStepGatePct(step, val);
    }
    function setStepProbability(val: number) {
        const step = get(seqSelectedStep);
        seqSteps.update(s => { s[step].probability = val; return [...s]; });
        engine?.setStepProbability(step, val);
    }
    function setStepRatchet(val: number) {
        const step = get(seqSelectedStep);
        seqSteps.update(s => { s[step].ratchet = val; return [...s]; });
        engine?.setStepRatchet(step, val);
    }
    function toggleStepSkip() {
        const step = get(seqSelectedStep);
        seqSteps.update(s => { s[step].skip = !s[step].skip; return [...s]; });
        engine?.setStepSkip(step, get(seqSteps)[step].skip);
    }

    // Pattern settings
    function setSeqDirection(dir: number) { seqDirection.set(dir); engine?.setDirection(dir); }
    function setSeqSwing(val: number) { seqSwing.set(val); engine?.setSwing(val / 100); }
    function setSeqTimeDivision(div: number) { seqTimeDivision.set(div); engine?.setTimeDivision(div); }
    function rotatePattern(dir: number) {
        engine?.seqRotate(dir);
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
    function randomizeGates() {
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

    // Drawer toggles (only one open at a time, caller handles arp drawer)
    function toggleSeqSettings() {
        seqSettingsOpen.update(v => { if (!v) stepSettingsOpen.set(false); return !v; });
    }
    function toggleStepSettings() {
        stepSettingsOpen.update(v => { if (!v) seqSettingsOpen.set(false); return !v; });
    }
    function toggleLenMode() { lenMode.update(v => !v); }

    function setLenFromStep(endStep: number) {
        const start = get(seqSelectedStep);
        const step = get(seqSteps)[start];
        if (!step?.gate || endStep === start) { lenMode.set(false); return; }
        const len = endStep > start ? endStep - start : (get(seqSteps).length - start + endStep);
        const gatePct = Math.max(100, len * 100);
        seqSteps.update(s => { s[start].gatePct = gatePct; return [...s]; });
        engine?.setStepGatePct(start, gatePct);
        lenMode.set(false);
    }

    function closeAllDrawers() {
        seqSettingsOpen.set(false);
        stepSettingsOpen.set(false);
        lenMode.set(false);
    }

    return {
        // Stores
        seqSteps, seqNumPages, seqCurrentPage, seqSelectedStep, seqCurrentStep,
        seqDirection, seqSwing, seqTimeDivision, seqSettingsOpen, stepSettingsOpen, lenMode,
        // Connection
        connectEngine, connectOnStep,
        // Actions
        addSeqPage, setSeqPage, selectSeqStep, setSeqStepFromNotes, setStepFromNotes,
        toggleSeqStepGate, removeStepGate, moveStep, clearSequence, clearSelectedStep,
        setStepVelocity, setStepGatePct, setStepGatePctAt, setStepProbability, setStepRatchet, toggleStepSkip,
        setSeqDirection, setSeqSwing, setSeqTimeDivision, rotatePattern, randomizeGates,
        toggleLenMode, setLenFromStep,
        toggleSeqSettings, toggleStepSettings, closeAllDrawers,
    };
}

export type NoteSequencerStore = ReturnType<typeof createNoteSequencerStore>;
