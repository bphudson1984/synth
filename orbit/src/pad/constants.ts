export const CHORDS = [
    { id: 'i',    label: 'Cm',   notes: [48, 51, 55], colour: '#E8944A' },
    { id: 'ii',   label: 'Ddm',  notes: [50, 53, 56], colour: '#EDA462' },
    { id: 'III',  label: 'Eb',   notes: [51, 55, 58], colour: '#D9843A' },
    { id: 'iv',   label: 'Fm',   notes: [53, 56, 60], colour: '#EDA462' },
    { id: 'v',    label: 'Gm',   notes: [55, 58, 62], colour: '#E8944A' },
    { id: 'VI',   label: 'Ab',   notes: [56, 60, 63], colour: '#D9843A' },
    { id: 'VII',  label: 'Bb',   notes: [58, 62, 65], colour: '#EDA462' },
    { id: 'i8',   label: "Cm'",  notes: [60, 63, 67], colour: '#E8944A' },
] as const;

export const PAD_PARAMS = ['cutoff', 'resonance', 'attack', 'release'] as const;

import type { SettingsSection } from '../shared/types/settings';
import { PARAM } from './audio/engine';

export const PAD_SETTINGS: SettingsSection[] = [
    {
        label: 'OSC A',
        params: [
            { name: 'SAW',   id: PARAM.OSC_A_SAW,   min: 0, max: 1, default: 1, type: 'toggle' },
            { name: 'PULSE', id: PARAM.OSC_A_PULSE,  min: 0, max: 1, default: 0, type: 'toggle' },
            { name: 'PW',    id: PARAM.OSC_A_PW,     min: 0, max: 1, default: 0.5, type: 'slider' },
        ],
    },
    {
        label: 'OSC B',
        params: [
            { name: 'SAW',   id: PARAM.OSC_B_SAW,    min: 0, max: 1, default: 1, type: 'toggle' },
            { name: 'TRI',   id: PARAM.OSC_B_TRI,    min: 0, max: 1, default: 0, type: 'toggle' },
            { name: 'PULSE', id: PARAM.OSC_B_PULSE,   min: 0, max: 1, default: 0, type: 'toggle' },
            { name: 'PW',    id: PARAM.OSC_B_PW,      min: 0, max: 1, default: 0.5, type: 'slider' },
            { name: 'SEMI',  id: PARAM.OSC_B_SEMI,    min: -24, max: 24, default: 0, type: 'slider' },
            { name: 'FINE',  id: PARAM.OSC_B_FINE,    min: -50, max: 50, default: 8, type: 'slider' },
            { name: 'SYNC',  id: PARAM.SYNC,          min: 0, max: 1, default: 0, type: 'toggle' },
        ],
    },
    {
        label: 'MIXER',
        params: [
            { name: 'OSC A', id: PARAM.OSC_A_LEVEL,   min: 0, max: 1, default: 0.7, type: 'slider' },
            { name: 'OSC B', id: PARAM.OSC_B_LEVEL,    min: 0, max: 1, default: 0.7, type: 'slider' },
            { name: 'NOISE', id: PARAM.NOISE_LEVEL,    min: 0, max: 1, default: 0, type: 'slider' },
        ],
    },
    {
        label: 'FILTER',
        params: [
            { name: 'CUTOFF',  id: PARAM.FILTER_CUTOFF,    min: 200, max: 12000, default: 3000, type: 'slider' },
            { name: 'RES',     id: PARAM.FILTER_RESONANCE,  min: 0, max: 3, default: 0.8, type: 'slider' },
            { name: 'ENV AMT', id: PARAM.FILTER_ENV_AMT,    min: 0, max: 10000, default: 2000, type: 'slider' },
            { name: 'DRIVE',   id: PARAM.FILTER_DRIVE,      min: 0, max: 1, default: 0, type: 'slider' },
        ],
    },
    {
        label: 'FILT ENV',
        params: [
            { name: 'ATTACK',  id: PARAM.FILTER_ATTACK,   min: 0.005, max: 2, default: 0.15, type: 'slider' },
            { name: 'DECAY',   id: PARAM.FILTER_DECAY,    min: 0.01, max: 3, default: 0.5, type: 'slider' },
            { name: 'SUSTAIN', id: PARAM.FILTER_SUSTAIN,  min: 0, max: 1, default: 0.3, type: 'slider' },
            { name: 'RELEASE', id: PARAM.FILTER_RELEASE,  min: 0.01, max: 3, default: 0.8, type: 'slider' },
        ],
    },
    {
        label: 'AMP ENV',
        params: [
            { name: 'ATTACK',  id: PARAM.AMP_ATTACK,   min: 0.005, max: 2, default: 0.15, type: 'slider' },
            { name: 'DECAY',   id: PARAM.AMP_DECAY,    min: 0.01, max: 3, default: 0.5, type: 'slider' },
            { name: 'SUSTAIN', id: PARAM.AMP_SUSTAIN,   min: 0, max: 1, default: 0.7, type: 'slider' },
            { name: 'RELEASE', id: PARAM.AMP_RELEASE,   min: 0.01, max: 3, default: 0.8, type: 'slider' },
        ],
    },
    {
        label: 'POLY MOD',
        params: [
            { name: 'FILT ENV', id: PARAM.PM_FILT_ENV,  min: 0, max: 1, default: 0, type: 'slider' },
            { name: 'OSC B',    id: PARAM.PM_OSC_B,     min: 0, max: 1, default: 0, type: 'slider' },
            { name: '→ FREQ A', id: PARAM.PM_FREQ_A,    min: 0, max: 1, default: 0, type: 'toggle' },
            { name: '→ PW A',   id: PARAM.PM_PW_A,      min: 0, max: 1, default: 0, type: 'toggle' },
            { name: '→ FILTER', id: PARAM.PM_FILTER,     min: 0, max: 1, default: 0, type: 'toggle' },
        ],
    },
    {
        label: 'LFO',
        params: [
            { name: 'FREQ',   id: PARAM.LFO_FREQ,    min: 0, max: 20, default: 0, type: 'slider' },
            { name: 'TRI',    id: PARAM.LFO_TRI,     min: 0, max: 1, default: 0, type: 'toggle' },
            { name: 'SAW',    id: PARAM.LFO_SAW,     min: 0, max: 1, default: 0, type: 'toggle' },
            { name: 'SQUARE', id: PARAM.LFO_SQUARE,   min: 0, max: 1, default: 0, type: 'toggle' },
            { name: 'AMOUNT', id: PARAM.LFO_AMOUNT,   min: 0, max: 1, default: 0, type: 'slider' },
        ],
    },
    {
        label: 'WHEEL MOD',
        params: [
            { name: 'SRC MIX',  id: PARAM.WM_MIX,     min: 0, max: 1, default: 0, type: 'slider' },
            { name: '→ FREQ A', id: PARAM.WM_FREQ_A,   min: 0, max: 1, default: 0, type: 'toggle' },
            { name: '→ FREQ B', id: PARAM.WM_FREQ_B,   min: 0, max: 1, default: 0, type: 'toggle' },
            { name: '→ PW A',   id: PARAM.WM_PW_A,     min: 0, max: 1, default: 0, type: 'toggle' },
            { name: '→ PW B',   id: PARAM.WM_PW_B,     min: 0, max: 1, default: 0, type: 'toggle' },
            { name: '→ FILTER', id: PARAM.WM_FILTER,    min: 0, max: 1, default: 0, type: 'toggle' },
        ],
    },
    {
        label: 'MASTER',
        params: [
            { name: 'GLIDE RATE', id: PARAM.GLIDE_RATE,  min: 0, max: 1, default: 0, type: 'slider' },
            { name: 'GLIDE',      id: PARAM.GLIDE_ON,    min: 0, max: 1, default: 0, type: 'toggle' },
            { name: 'UNISON',     id: PARAM.UNISON,      min: 0, max: 1, default: 0, type: 'toggle' },
            { name: 'DRIFT',      id: PARAM.DRIFT,       min: 0, max: 0.1, default: 0, type: 'slider' },
        ],
    },
    {
        label: 'CHORUS',
        params: [
            { name: 'RATE',  id: PARAM.CHORUS_RATE,   min: 0, max: 5, default: 0.8, type: 'slider' },
            { name: 'DEPTH', id: PARAM.CHORUS_DEPTH,  min: 0, max: 1, default: 0.5, type: 'slider' },
            { name: 'MIX',   id: PARAM.CHORUS_MIX,    min: 0, max: 1, default: 0.3, type: 'slider' },
        ],
    },
    {
        label: 'DELAY',
        params: [
            { name: 'TIME',     id: PARAM.DELAY_TIME,      min: 0, max: 1000, default: 0, type: 'slider' },
            { name: 'FEEDBACK', id: PARAM.DELAY_FEEDBACK,   min: 0, max: 1, default: 0, type: 'slider' },
            { name: 'TONE',     id: PARAM.DELAY_TONE,       min: 0, max: 1, default: 0.5, type: 'slider' },
            { name: 'MIX',      id: PARAM.DELAY_MIX,        min: 0, max: 1, default: 0, type: 'slider' },
        ],
    },
    {
        label: 'REVERB',
        params: [
            { name: 'DECAY',   id: PARAM.REVERB_DECAY,    min: 0, max: 1, default: 0.7, type: 'slider' },
            { name: 'DAMPING', id: PARAM.REVERB_DAMPING,  min: 0, max: 1, default: 0.6, type: 'slider' },
            { name: 'MIX',     id: PARAM.REVERB_MIX,      min: 0, max: 1, default: 0.25, type: 'slider' },
        ],
    },
];
export type PadParamName = typeof PAD_PARAMS[number];

// Map slider 0-100 to actual Prophet-5 parameter ranges
export const PAD_PARAM_MAP: Record<PadParamName, { id: number; min: number; max: number }> = {
    cutoff:    { id: 12, min: 200,    max: 12000 },
    resonance: { id: 13, min: 0,      max: 3.0 },
    attack:    { id: 20, min: 0.005,  max: 2.0 },
    release:   { id: 23, min: 0.01,   max: 3.0 },
};
