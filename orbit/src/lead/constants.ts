export const MODELS = [
    'SAW', 'SQUARE', 'MORPH', 'FOLD', 'FM', 'FBFM',
    'WTBL', 'VOWL', 'PLUK', 'BELL', 'NOIS', 'SWARM', 'CLOUD',
] as const;

// C minor scale notes (for NOTE mode)
export const SCALE_NOTES = [
    { id: 'c',  label: 'C',  note: 48, colour: '#B56ECC' },
    { id: 'd',  label: 'D',  note: 50, colour: '#A45DBE' },
    { id: 'eb', label: 'Eb', note: 51, colour: '#B56ECC' },
    { id: 'f',  label: 'F',  note: 53, colour: '#A45DBE' },
    { id: 'g',  label: 'G',  note: 55, colour: '#B56ECC' },
    { id: 'ab', label: 'Ab', note: 56, colour: '#A45DBE' },
    { id: 'bb', label: 'Bb', note: 58, colour: '#B56ECC' },
    { id: 'c2', label: "C'", note: 60, colour: '#A45DBE' },
] as const;

// C minor harmonized 7th chords (for CHORD mode)
export const SCALE_CHORDS = [
    { id: 'i7',   label: 'Cm7',    notes: [48, 51, 55, 58], colour: '#B56ECC' },
    { id: 'ii7',  label: 'Dm7b5',  notes: [50, 53, 56, 60], colour: '#A45DBE' },
    { id: 'III7', label: 'EbM7',   notes: [51, 55, 58, 62], colour: '#B56ECC' },
    { id: 'iv7',  label: 'Fm7',    notes: [53, 56, 60, 63], colour: '#A45DBE' },
    { id: 'v7',   label: 'Gm7',    notes: [55, 58, 62, 65], colour: '#B56ECC' },
    { id: 'VI7',  label: 'AbM7',   notes: [56, 60, 63, 67], colour: '#A45DBE' },
    { id: 'VII7', label: 'Bb7',    notes: [58, 62, 65, 68], colour: '#B56ECC' },
    { id: 'i78',  label: "Cm7'",   notes: [60, 63, 67, 70], colour: '#A45DBE' },
] as const;

export type PadMode = 'note' | 'chord';

export const LEAD_PARAMS = ['timbre', 'color', 'cutoff', 'release'] as const;
export type LeadParamName = typeof LEAD_PARAMS[number];

export const LEAD_PARAM_MAP: Record<LeadParamName, { id: number; min: number; max: number }> = {
    timbre:  { id: 1, min: 0.0, max: 1.0 },
    color:   { id: 2, min: 0.0, max: 1.0 },
    cutoff:  { id: 3, min: 100, max: 12000 },
    release: { id: 9, min: 0.01, max: 3.0 },
};

export const LEAD_COLOUR = '#B56ECC';
export const NUM_STEPS = 16;

export type ArpMode = 'off' | 'up' | 'down' | 'updown' | 'random';
export type ArpDivision = '1/4' | '1/8' | '1/16' | '1/32';

import type { SettingsSection } from '../shared/types/settings';
import { PARAM } from './audio/engine';

export const LEAD_SETTINGS: SettingsSection[] = [
    {
        label: 'FILTER',
        params: [
            { name: 'RES',     id: PARAM.FILTER_RESONANCE, min: 0, max: 1, default: 0, type: 'slider' },
            { name: 'ENV AMT', id: PARAM.FILTER_ENV_AMT,   min: 0, max: 1, default: 0, type: 'slider' },
        ],
    },
    {
        label: 'FILT ENV',
        params: [
            { name: 'ATTACK',  id: PARAM.FILT_ATTACK,  min: 0.005, max: 2, default: 0.01, type: 'slider' },
            { name: 'DECAY',   id: PARAM.FILT_DECAY,   min: 0.01, max: 3, default: 0.3, type: 'slider' },
            { name: 'SUSTAIN', id: PARAM.FILT_SUSTAIN,  min: 0, max: 1, default: 0, type: 'slider' },
            { name: 'RELEASE', id: PARAM.FILT_RELEASE,  min: 0.01, max: 3, default: 0.5, type: 'slider' },
        ],
    },
    {
        label: 'AMP ENV',
        params: [
            { name: 'ATTACK',  id: PARAM.AMP_ATTACK,   min: 0.005, max: 2, default: 0.01, type: 'slider' },
            { name: 'DECAY',   id: PARAM.AMP_DECAY,    min: 0.01, max: 3, default: 0.3, type: 'slider' },
            { name: 'SUSTAIN', id: PARAM.AMP_SUSTAIN,   min: 0, max: 1, default: 0.8, type: 'slider' },
        ],
    },
    {
        label: 'LFO',
        params: [
            { name: 'RATE',   id: PARAM.LFO_RATE,    min: 0, max: 20, default: 0, type: 'slider' },
            { name: 'AMOUNT', id: PARAM.LFO_AMOUNT,   min: 0, max: 1, default: 0, type: 'slider' },
            { name: 'DEST',   id: PARAM.LFO_DEST,     min: 0, max: 4, default: 0, type: 'select',
              options: [
                  { value: 0, label: 'OFF' }, { value: 1, label: 'TIMBRE' },
                  { value: 2, label: 'COLOR' }, { value: 3, label: 'CUTOFF' },
                  { value: 4, label: 'PITCH' },
              ],
            },
        ],
    },
    {
        label: 'MASTER',
        params: [
            { name: 'GLIDE', id: PARAM.GLIDE_TIME, min: 0, max: 1, default: 0, type: 'slider' },
        ],
    },
];
