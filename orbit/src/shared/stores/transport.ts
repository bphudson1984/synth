import { writable, get } from 'svelte/store';

export const DEFAULT_BPM = 120;
export const MIN_BPM = 60;
export const MAX_BPM = 200;

export const bpm = writable(DEFAULT_BPM);
export const isPlaying = writable(false);
export const isRecording = writable(false);

// Registry of play/stop callbacks from each engine
type TransportCallbacks = {
    play: () => void;
    stop: () => void;
};

const engines: TransportCallbacks[] = [];

export function registerEngine(callbacks: TransportCallbacks) {
    engines.push(callbacks);
}

export function togglePlay() {
    const playing = !get(isPlaying);
    isPlaying.set(playing);
    if (playing) {
        for (const e of engines) e.play();
    } else {
        for (const e of engines) e.stop();
    }
}

export function toggleRecord() {
    isRecording.update(v => !v);
}
