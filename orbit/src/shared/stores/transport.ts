import { writable } from 'svelte/store';

export const DEFAULT_BPM = 120;
export const MIN_BPM = 60;
export const MAX_BPM = 200;

export const bpm = writable(DEFAULT_BPM);
