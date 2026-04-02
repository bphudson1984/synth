// C natural minor scale degrees — each pad transposes the pattern to that root
// Intervals from C: 0, 2, 3, 5, 7, 8, 10, 12
export const NOTE_PADS = [
    { id: 'c',  label: 'C',  semitones: 0,  colour: '#5DBE6E' },
    { id: 'd',  label: 'D',  semitones: 2,  colour: '#6ECC7E' },
    { id: 'eb', label: 'Eb', semitones: 3,  colour: '#4DAE5E' },
    { id: 'f',  label: 'F',  semitones: 5,  colour: '#6ECC7E' },
    { id: 'g',  label: 'G',  semitones: 7,  colour: '#5DBE6E' },
    { id: 'ab', label: 'Ab', semitones: 8,  colour: '#4DAE5E' },
    { id: 'bb', label: 'Bb', semitones: 10, colour: '#6ECC7E' },
    { id: 'c2', label: "C'", semitones: 12, colour: '#5DBE6E' },
] as const;

export const ACID_PARAMS = ['cutoff', 'resonance', 'env mod', 'dist'] as const;
export type AcidParamName = typeof ACID_PARAMS[number];

export const ACID_PARAM_MAP: Record<AcidParamName, { id: number; min: number; max: number }> = {
    cutoff:    { id: 0, min: 100,  max: 5000 },
    resonance: { id: 1, min: 0,    max: 1.0 },
    'env mod': { id: 2, min: 0,    max: 1.0 },
    dist:      { id: 7, min: 0,    max: 1.0 },
};

export const NUM_STEPS = 16;
export const ACID_COLOUR = '#5DBE6E';

import type { SettingsSection } from '../shared/types/settings';
import { PARAM } from './audio/engine';

export const ACID_SETTINGS: SettingsSection[] = [
    {
        label: 'FILTER',
        params: [
            { name: 'CUTOFF',    id: PARAM.CUTOFF,    min: 100, max: 5000, default: 1500, type: 'slider' },
            { name: 'RESONANCE', id: PARAM.RESONANCE,  min: 0, max: 1, default: 0.6, type: 'slider' },
            { name: 'ENV MOD',   id: PARAM.ENV_MOD,    min: 0, max: 1, default: 0.7, type: 'slider' },
        ],
    },
    {
        label: 'TONE',
        params: [
            { name: 'DECAY',    id: PARAM.DECAY,    min: 0.03, max: 3, default: 0.3, type: 'slider' },
            { name: 'ACCENT',   id: PARAM.ACCENT,   min: 0, max: 1, default: 0.5, type: 'slider' },
            { name: 'DIST',     id: PARAM.DISTORTION, min: 0, max: 1, default: 0.3, type: 'slider' },
            { name: 'WAVEFORM', id: PARAM.WAVEFORM,  min: 0, max: 1, default: 0, type: 'select',
              options: [
                  { value: 0, label: 'SAW' }, { value: 1, label: 'SQR' },
              ],
            },
        ],
    },
];

const NOTE_NAMES = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
export function midiNoteName(note: number): string {
    return `${NOTE_NAMES[note % 12]}${Math.floor(note / 12) - 1}`;
}
