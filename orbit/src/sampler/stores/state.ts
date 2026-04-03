import { writable, get } from 'svelte/store';
import { NUM_PADS, SAMPLER_SETTINGS } from '../constants';
import { NUM_QUICK_SLOTS, type QuickSlot, type SettingsParam } from '../../shared/types/settings';
import type { SamplerEngine } from '../audio/engine';
import { PAD_PARAM } from '../constants';
import { bpm } from '../../shared/stores/transport';
import { registerMixerCallback } from '../../shared/stores/mixer';
import { createNoteSequencerStore } from '../../shared/stores/noteSequencer';
import { saveSample, loadAllSamples, deleteSample } from '../audio/storage';

// Shared note sequencer (same component as PAD/LEAD/BASS)
export const samplerSeq = createNoteSequencerStore();

let engine: SamplerEngine | null = null;
let lastStep = -1;

// Per-pad state
export interface PadState {
    name: string;
    loaded: boolean;
    waveform: Float32Array | null; // for display
}

function emptyPadStates(): PadState[] {
    return Array.from({ length: NUM_PADS }, () => ({ name: '', loaded: false, waveform: null }));
}

export const padStates = writable<PadState[]>(emptyPadStates());
export const selectedPad = writable(0);
export const settingsOpen = writable(false);

// Per-pad settings values (keyed by pad index, then param id)
function defaultPadSettings(): Record<number, number> {
    return {
        [PAD_PARAM.VOLUME]: 1.0,
        [PAD_PARAM.PITCH]: 0,
        [PAD_PARAM.PLAY_MODE]: 0,
        [PAD_PARAM.CHOKE_GROUP]: 0,
        [PAD_PARAM.REVERSE]: 0,
        [PAD_PARAM.PAN]: 0,
        [PAD_PARAM.ATTACK]: 0.002,
        [PAD_PARAM.RELEASE]: 0.005,
        [PAD_PARAM.START]: 0,
        [PAD_PARAM.END]: 1,
        [PAD_PARAM.BIT_DEPTH]: 16,
        [PAD_PARAM.VOCODER_ON]: 0,
        [PAD_PARAM.VOCODER_ROOT]: 60,
        [PAD_PARAM.VOCODER_CARRIER]: 0,
        [PAD_PARAM.VOCODER_BANDS]: 12,
        [PAD_PARAM.VOCODER_FORMANT]: 0,
        [PAD_PARAM.VOCODER_MIX]: 1,
    };
}

export const padSettings = writable<Record<number, Record<number, number>>>(
    Object.fromEntries(Array.from({ length: NUM_PADS }, (_, i) => [i, defaultPadSettings()]))
);

// Sequence bank
const MAX_SEQUENCES = 8;
export const samplerSequenceBank = writable<ReturnType<typeof samplerSeq.captureSequence>[]>([samplerSeq.captureSequence()]);
export const currentSequenceIndex = writable(0);
export const samplerChainMode = writable(false);
export const samplerRandomMode = writable(false);

export function toggleSamplerChain() { samplerChainMode.update(v => { if (!v) samplerRandomMode.set(false); return !v; }); }
export function toggleSamplerRandom() { samplerRandomMode.update(v => { if (!v) samplerChainMode.set(false); return !v; }); }

export function setSamplerEngine(e: SamplerEngine) {
    engine = e;
    e.onStep = (step) => {
        samplerSeq.connectOnStep(step);
        if (step === 0 && lastStep > 0) {
            const bank = get(samplerSequenceBank);
            if (bank.length > 1) {
                if (get(samplerChainMode)) {
                    switchSequence((get(currentSequenceIndex) + 1) % bank.length);
                } else if (get(samplerRandomMode)) {
                    let next = get(currentSequenceIndex);
                    while (next === get(currentSequenceIndex)) next = Math.floor(Math.random() * bank.length);
                    switchSequence(next);
                }
            }
        }
        lastStep = step;
    };
    samplerSeq.connectEngine(e);
    bpm.subscribe(() => {});
    registerMixerCallback('sampler',
        (gain) => { engine?.setParam(200, gain); },
        (pan) => { engine?.setPan(pan); },
    );

    // Restore samples from IndexedDB
    loadAllSamples().then(samples => {
        for (const [padIndex, data] of samples) {
            engine?.loadSampleFromPCM(padIndex, data.left, data.right, data.sampleRate);
            padStates.update(ps => {
                ps[padIndex] = { name: data.name, loaded: true, waveform: new Float32Array(data.left) };
                return [...ps];
            });
        }
    }).catch(err => console.warn('Failed to restore samples:', err));
}

// Actions
export function selectPad(index: number) { selectedPad.set(index); }
export function toggleSettings() { settingsOpen.update(v => !v); }

export function triggerPad(index: number) {
    engine?.trigger(index);
}

export function releasePad(index: number) {
    engine?.release(index);
}

// Mic recording
export const isRecordingMic = writable(false);
let mediaRecorder: MediaRecorder | null = null;
let micStream: MediaStream | null = null;

export async function startMicRecording() {
    try {
        micStream = await navigator.mediaDevices.getUserMedia({ audio: true });
        const chunks: Blob[] = [];
        mediaRecorder = new MediaRecorder(micStream);
        mediaRecorder.ondataavailable = (e) => { if (e.data.size > 0) chunks.push(e.data); };
        mediaRecorder.onstop = async () => {
            micStream?.getTracks().forEach(t => t.stop());
            micStream = null;
            const blob = new Blob(chunks, { type: 'audio/webm' });
            const file = new File([blob], `rec-${Date.now()}.webm`, { type: blob.type });
            await loadSampleFile(get(selectedPad), file);
        };
        mediaRecorder.start();
        isRecordingMic.set(true);
    } catch (err) {
        console.error('Mic access denied:', err);
    }
}

export function stopMicRecording() {
    if (mediaRecorder && mediaRecorder.state !== 'inactive') {
        mediaRecorder.stop();
    }
    isRecordingMic.set(false);
}

export async function loadSampleFile(padIndex: number, file: File) {
    if (!engine) return;
    const waveform = await engine.loadSample(padIndex, file);
    padStates.update(ps => {
        ps[padIndex] = { name: file.name, loaded: true, waveform };
        return [...ps];
    });
    // Persist to IndexedDB (need the raw PCM — use the waveform copy as mono L=R)
    await saveSample(padIndex, waveform, waveform, 48000, file.name);
}

// Stock sample kit — fetches from Tone.js CDN, loads into pads, persists
const STOCK_KIT = [
    { name: 'Kick',    url: 'https://tonejs.github.io/audio/drum-machine/kick.mp3' },
    { name: 'Snare',   url: 'https://tonejs.github.io/audio/drum-machine/snare.mp3' },
    { name: 'HiHat',   url: 'https://tonejs.github.io/audio/drum-machine/hihat.mp3' },
    { name: 'Clap',    url: 'https://tonejs.github.io/audio/drum-machine/clap.mp3' },
    { name: 'Tom 1',   url: 'https://tonejs.github.io/audio/drum-machine/tom1.mp3' },
    { name: 'Tom 2',   url: 'https://tonejs.github.io/audio/drum-machine/tom2.mp3' },
    { name: 'Tom 3',   url: 'https://tonejs.github.io/audio/drum-machine/tom3.mp3' },
    { name: 'Casio 1', url: 'https://tonejs.github.io/audio/casio/A1.mp3' },
    { name: 'Casio 2', url: 'https://tonejs.github.io/audio/casio/C2.mp3' },
    { name: 'Casio 3', url: 'https://tonejs.github.io/audio/casio/E2.mp3' },
    { name: 'Casio 4', url: 'https://tonejs.github.io/audio/casio/G2.mp3' },
    { name: 'Casio 5', url: 'https://tonejs.github.io/audio/casio/A2.mp3' },
];

export const isLoadingKit = writable(false);

export async function loadStockKit() {
    if (!engine) return;
    isLoadingKit.set(true);
    try {
        for (let i = 0; i < STOCK_KIT.length && i < NUM_PADS; i++) {
            const { name, url } = STOCK_KIT[i];
            try {
                const response = await fetch(url);
                const arrayBuffer = await response.arrayBuffer();
                const waveform = await engine.loadSampleFromArrayBuffer(i, arrayBuffer);
                padStates.update(ps => {
                    ps[i] = { name, loaded: true, waveform };
                    return [...ps];
                });
                await saveSample(i, waveform, waveform, 48000, name);
            } catch (err) {
                console.warn(`Failed to load stock sample ${name}:`, err);
            }
        }
    } finally {
        isLoadingKit.set(false);
    }
}

export async function clearPad(padIndex: number) {
    engine?.stopPad(padIndex);
    padStates.update(ps => {
        ps[padIndex] = { name: '', loaded: false, waveform: null };
        return [...ps];
    });
    await deleteSample(padIndex);
}

export function setPadParam(padIndex: number, paramId: number, value: number) {
    padSettings.update(ps => {
        ps[padIndex] = { ...ps[padIndex], [paramId]: value };
        return { ...ps };
    });
    engine?.setPadParam(padIndex, paramId, value);
}

// Settings convenience (operates on selected pad)
export function setSelectedPadParam(paramId: number, value: number) {
    const pad = get(selectedPad);
    setPadParam(pad, paramId, value);
}

// Quick slots
function findParam(id: number): SettingsParam | null {
    for (const section of SAMPLER_SETTINGS) {
        for (const p of section.params) if (p.id === id) return p;
    }
    return null;
}

export const quickSlots = writable<QuickSlot[]>([
    findParam(PAD_PARAM.VOLUME),
    findParam(PAD_PARAM.PITCH),
    null, null, null, null, null, null,
]);
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
    setSelectedPadParam(slot.id, actual);
}

// Sequence bank
export function switchSequence(index: number) {
    const bank = get(samplerSequenceBank);
    if (index < 0 || index >= bank.length || index === get(currentSequenceIndex)) return;
    bank[get(currentSequenceIndex)] = samplerSeq.captureSequence();
    samplerSequenceBank.set(bank);
    samplerSeq.restoreSequence(bank[index]);
    currentSequenceIndex.set(index);
}
export function addSequence() {
    const bank = get(samplerSequenceBank);
    if (bank.length >= MAX_SEQUENCES) return;
    bank[get(currentSequenceIndex)] = samplerSeq.captureSequence();
    samplerSeq.clearSequence();
    bank.push(samplerSeq.captureSequence());
    samplerSequenceBank.set(bank);
    currentSequenceIndex.set(bank.length - 1);
}
export function deleteSequence() {
    const bank = get(samplerSequenceBank);
    if (bank.length <= 1) return;
    const idx = get(currentSequenceIndex);
    bank.splice(idx, 1);
    const newIdx = Math.min(idx, bank.length - 1);
    samplerSequenceBank.set(bank);
    samplerSeq.restoreSequence(bank[newIdx]);
    currentSequenceIndex.set(newIdx);
}
export function duplicateSequence() {
    const bank = get(samplerSequenceBank);
    if (bank.length >= MAX_SEQUENCES) return;
    const current = samplerSeq.captureSequence();
    bank[get(currentSequenceIndex)] = current;
    const clone = { ...current, steps: current.steps.map(s => ({ ...s, notes: [...s.notes] })) };
    bank.push(clone);
    samplerSequenceBank.set(bank);
    currentSequenceIndex.set(bank.length - 1);
}
