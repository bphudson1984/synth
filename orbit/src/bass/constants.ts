import type { SettingsSection } from '../shared/types/settings';
import { PARAM } from './audio/engine';

export const BASS_COLOUR = '#D4A843';

// C natural minor scale pads — same as acid, recoloured for bass
export const NOTE_PADS = [
    { id: 'c',  label: 'C',  semitones: 0,  colour: '#D4A843' },
    { id: 'd',  label: 'D',  semitones: 2,  colour: '#DDB856' },
    { id: 'eb', label: 'Eb', semitones: 3,  colour: '#C49838' },
    { id: 'f',  label: 'F',  semitones: 5,  colour: '#DDB856' },
    { id: 'g',  label: 'G',  semitones: 7,  colour: '#D4A843' },
    { id: 'ab', label: 'Ab', semitones: 8,  colour: '#C49838' },
    { id: 'bb', label: 'Bb', semitones: 10, colour: '#DDB856' },
    { id: 'c2', label: "C'", semitones: 12, colour: '#D4A843' },
] as const;

export const BASS_PARAMS = ['cutoff', 'peak', 'eg int', 'lfo rate'] as const;
export type BassParamName = typeof BASS_PARAMS[number];

export const BASS_PARAM_MAP: Record<BassParamName, { id: number; min: number; max: number }> = {
    cutoff:     { id: PARAM.CUTOFF,   min: 20,   max: 12000 },
    peak:       { id: PARAM.PEAK,     min: 0,    max: 1.0 },
    'eg int':   { id: PARAM.EG_INT,   min: 0,    max: 1.0 },
    'lfo rate': { id: PARAM.LFO_RATE, min: 0.1,  max: 30 },
};

export const BASS_SETTINGS: SettingsSection[] = [
    {
        label: 'VCO 1',
        params: [
            { name: 'WAVE',   id: PARAM.VCO1_WAVE,   min: 0, max: 1, default: 0, type: 'select',
              options: [{ value: 0, label: 'SAW' }, { value: 1, label: 'SQR' }] },
            { name: 'PITCH',  id: PARAM.VCO1_PITCH,  min: -12, max: 12, default: 0, type: 'slider' },
            { name: 'ACTIVE', id: PARAM.VCO1_ACTIVE, min: 0, max: 1, default: 1, type: 'toggle' },
        ],
    },
    {
        label: 'VCO 2',
        params: [
            { name: 'WAVE',   id: PARAM.VCO2_WAVE,   min: 0, max: 1, default: 0, type: 'select',
              options: [{ value: 0, label: 'SAW' }, { value: 1, label: 'SQR' }] },
            { name: 'PITCH',  id: PARAM.VCO2_PITCH,  min: -12, max: 12, default: 0.1, type: 'slider' },
            { name: 'ACTIVE', id: PARAM.VCO2_ACTIVE, min: 0, max: 1, default: 1, type: 'toggle' },
        ],
    },
    {
        label: 'VCO 3',
        params: [
            { name: 'WAVE',   id: PARAM.VCO3_WAVE,   min: 0, max: 1, default: 0, type: 'select',
              options: [{ value: 0, label: 'SAW' }, { value: 1, label: 'SQR' }] },
            { name: 'PITCH',  id: PARAM.VCO3_PITCH,  min: -12, max: 12, default: -0.1, type: 'slider' },
            { name: 'ACTIVE', id: PARAM.VCO3_ACTIVE, min: 0, max: 1, default: 1, type: 'toggle' },
        ],
    },
    {
        label: 'FILTER',
        params: [
            { name: 'CUTOFF', id: PARAM.CUTOFF, min: 20, max: 12000, default: 2000, type: 'slider' },
            { name: 'PEAK',   id: PARAM.PEAK,   min: 0, max: 1, default: 0.3, type: 'slider' },
            { name: 'EG INT', id: PARAM.EG_INT, min: 0, max: 1, default: 0.5, type: 'slider' },
        ],
    },
    {
        label: 'ENVELOPE',
        params: [
            { name: 'ATTACK',   id: PARAM.ATTACK,        min: 0.0005, max: 10, default: 0.005, type: 'slider' },
            { name: 'DEC/REL',  id: PARAM.DECAY_RELEASE,  min: 0.01, max: 10, default: 0.3, type: 'slider' },
            { name: 'SUSTAIN',  id: PARAM.SUSTAIN_ON,     min: 0, max: 1, default: 0, type: 'toggle' },
            { name: 'EG→VCA',   id: PARAM.EG_TO_VCA,     min: 0, max: 1, default: 1, type: 'toggle' },
        ],
    },
    {
        label: 'LFO',
        params: [
            { name: 'RATE',      id: PARAM.LFO_RATE,      min: 0.1, max: 30, default: 5, type: 'slider' },
            { name: 'INT',       id: PARAM.LFO_INT,       min: 0, max: 1, default: 0, type: 'slider' },
            { name: 'WAVE',      id: PARAM.LFO_WAVE,      min: 0, max: 1, default: 0, type: 'select',
              options: [{ value: 0, label: 'TRI' }, { value: 1, label: 'SQR' }] },
            { name: '→ PITCH',   id: PARAM.LFO_TO_PITCH,  min: 0, max: 1, default: 0, type: 'toggle' },
            { name: '→ CUTOFF',  id: PARAM.LFO_TO_CUTOFF, min: 0, max: 1, default: 0, type: 'toggle' },
            { name: '→ AMP',     id: PARAM.LFO_TO_AMP,    min: 0, max: 1, default: 0, type: 'toggle' },
        ],
    },
    {
        label: 'MODE',
        params: [
            { name: 'GROUP', id: PARAM.GROUP_MODE, min: 0, max: 1, default: 0, type: 'select',
              options: [{ value: 0, label: 'ALL' }, { value: 1, label: 'POLY' }] },
        ],
    },
];

const NOTE_NAMES = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
export function midiNoteName(note: number): string {
    return `${NOTE_NAMES[note % 12]}${Math.floor(note / 12) - 1}`;
}
