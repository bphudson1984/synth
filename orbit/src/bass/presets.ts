import { PARAM } from './audio/engine';
import type { SeqStep } from '../shared/stores/noteSequencer';

export interface BassPreset {
    name: string;
    params: [number, number][];
    steps: SeqStep[];
}

const P = PARAM;

function defaults(overrides: [number, number][]): [number, number][] {
    const base: [number, number][] = [
        [P.VCO1_WAVE, 0], [P.VCO2_WAVE, 0], [P.VCO3_WAVE, 0],
        [P.VCO1_PITCH, 0], [P.VCO2_PITCH, 0.1], [P.VCO3_PITCH, -0.1],
        [P.VCO1_ACTIVE, 1], [P.VCO2_ACTIVE, 1], [P.VCO3_ACTIVE, 1],
        [P.CUTOFF, 2000], [P.PEAK, 0.3], [P.EG_INT, 0.5],
        [P.ATTACK, 0.005], [P.DECAY_RELEASE, 0.3], [P.SUSTAIN_ON, 0], [P.EG_TO_VCA, 1],
        [P.LFO_RATE, 5], [P.LFO_INT, 0], [P.LFO_WAVE, 0],
        [P.LFO_TO_PITCH, 0], [P.LFO_TO_CUTOFF, 0], [P.LFO_TO_AMP, 0],
        [P.GROUP_MODE, 0], [P.VOLUME, 0.5],
    ];
    const map = new Map(base);
    for (const [id, val] of overrides) map.set(id, val);
    return [...map.entries()];
}

// Step helpers
function n(note: number, gatePct = 75, velocity = 100): SeqStep {
    return { notes: [note], gate: true, label: '', velocity, gatePct, probability: 100, ratchet: 1, skip: false };
}
// Rest
const R: SeqStep = { notes: [36], gate: false, label: '', velocity: 100, gatePct: 75, probability: 100, ratchet: 1, skip: false };
// Legato / tied note (gate > 100% for slide behavior)
function leg(note: number, gatePct = 150): SeqStep {
    return { notes: [note], gate: true, label: '', velocity: 100, gatePct, probability: 100, ratchet: 1, skip: false };
}
// Ghost note (low velocity)
function gh(note: number): SeqStep {
    return { notes: [note], gate: true, label: '', velocity: 50, gatePct: 40, probability: 100, ratchet: 1, skip: false };
}

export const PRESETS: BassPreset[] = [
    // --- DEEP & HEAVY ---
    {
        name: 'House Foundation',
        params: defaults([
            [P.VCO2_PITCH, 0.08], [P.VCO3_PITCH, -0.08],
            [P.CUTOFF, 1200], [P.PEAK, 0.25], [P.EG_INT, 0.4],
            [P.ATTACK, 0.003], [P.DECAY_RELEASE, 0.4],
        ]),
        steps: [
            n(36), R, R, gh(36),
            R, R, n(36), R,
            R, gh(36), R, R,
            n(36), R, gh(36), R,
        ],
    },
    {
        name: 'Sub Weight',
        params: defaults([
            [P.VCO1_WAVE, 1], [P.VCO2_ACTIVE, 0], [P.VCO3_ACTIVE, 0],
            [P.CUTOFF, 300], [P.PEAK, 0.4], [P.EG_INT, 0.2],
            [P.ATTACK, 0.001], [P.DECAY_RELEASE, 0.8],
        ]),
        steps: [
            n(36, 200), R, R, R,
            R, R, R, n(36, 50),
            n(36, 200), R, R, R,
            R, R, n(43, 75), n(36, 50),
        ],
    },
    {
        name: 'Dub Pressure',
        params: defaults([
            [P.VCO2_PITCH, -12], [P.VCO3_ACTIVE, 0],
            [P.CUTOFF, 600], [P.PEAK, 0.5], [P.EG_INT, 0.6],
            [P.ATTACK, 0.005], [P.DECAY_RELEASE, 0.6],
        ]),
        steps: [
            n(36), R, R, leg(43),
            R, R, n(36), R,
            R, n(41, 50), R, R,
            n(36), R, R, leg(31),
        ],
    },

    // --- DRIVING & RHYTHMIC ---
    {
        name: 'Techno Pulse',
        params: defaults([
            [P.CUTOFF, 800], [P.PEAK, 0.6], [P.EG_INT, 0.8],
            [P.ATTACK, 0.001], [P.DECAY_RELEASE, 0.15],
            [P.VCO2_PITCH, 0.05], [P.VCO3_PITCH, -0.05],
        ]),
        steps: [
            n(36), n(36, 40), R, n(36),
            n(36, 40), R, n(36), n(36, 40),
            n(36), n(36, 40), R, n(36),
            n(36, 40), R, n(48), R,
        ],
    },
    {
        name: 'Motorik',
        params: defaults([
            [P.VCO1_WAVE, 1], [P.VCO2_WAVE, 0], [P.VCO3_WAVE, 1],
            [P.VCO2_PITCH, -12], [P.VCO3_PITCH, 0],
            [P.CUTOFF, 1500], [P.PEAK, 0.3], [P.EG_INT, 0.5],
            [P.ATTACK, 0.002], [P.DECAY_RELEASE, 0.25],
        ]),
        steps: [
            n(36), gh(36), n(36), gh(36),
            n(36), gh(36), n(36), gh(36),
            n(36), gh(36), n(36), gh(36),
            n(36), gh(36), n(43), n(41),
        ],
    },
    {
        name: 'Syncopated Funk',
        params: defaults([
            [P.CUTOFF, 1800], [P.PEAK, 0.35], [P.EG_INT, 0.6],
            [P.ATTACK, 0.001], [P.DECAY_RELEASE, 0.2],
        ]),
        steps: [
            n(36), R, gh(36), n(43),
            R, n(36), R, gh(43),
            n(41), R, n(36), R,
            gh(36), n(43), R, n(36, 40),
        ],
    },

    // --- ACID & SQUELCHY ---
    {
        name: 'Acid Unison',
        params: defaults([
            [P.CUTOFF, 400], [P.PEAK, 0.75], [P.EG_INT, 0.95],
            [P.ATTACK, 0.001], [P.DECAY_RELEASE, 0.12],
        ]),
        steps: [
            n(36), n(36, 40), R, n(36),
            leg(48), R, n(36), gh(36),
            n(36), R, n(36), leg(43),
            n(36), R, n(48), n(36, 40),
        ],
    },
    {
        name: 'Squelch Box',
        params: defaults([
            [P.VCO2_ACTIVE, 0], [P.VCO3_ACTIVE, 0],
            [P.CUTOFF, 300], [P.PEAK, 0.85], [P.EG_INT, 1.0],
            [P.ATTACK, 0.001], [P.DECAY_RELEASE, 0.1],
        ]),
        steps: [
            n(36), R, n(36, 40), n(48),
            R, leg(43), n(36), R,
            n(36), n(48, 40), R, n(36),
            leg(41), R, n(36), gh(36),
        ],
    },

    // --- MELODIC ---
    {
        name: 'Minor Walk',
        params: defaults([
            [P.CUTOFF, 2500], [P.PEAK, 0.2], [P.EG_INT, 0.4],
            [P.ATTACK, 0.003], [P.DECAY_RELEASE, 0.35],
        ]),
        steps: [
            n(36), R, n(39), n(41),
            n(43), R, n(41), leg(39),
            n(36), R, n(34), leg(36),
            n(39), n(41), n(43), n(41),
        ],
    },
    {
        name: 'Octave Jump',
        params: defaults([
            [P.CUTOFF, 1600], [P.PEAK, 0.4], [P.EG_INT, 0.6],
            [P.ATTACK, 0.001], [P.DECAY_RELEASE, 0.2],
        ]),
        steps: [
            n(36), R, n(48, 40), R,
            n(36), R, n(48, 40), n(43),
            n(36), R, n(48, 40), R,
            n(36), n(41), n(48, 40), leg(43),
        ],
    },
    {
        name: 'Chromatic Descent',
        params: defaults([
            [P.CUTOFF, 1000], [P.PEAK, 0.5], [P.EG_INT, 0.7],
            [P.ATTACK, 0.001], [P.DECAY_RELEASE, 0.18],
        ]),
        steps: [
            n(48), leg(47), leg(46), leg(45),
            n(44), leg(43), leg(42), leg(41),
            n(40), leg(39), leg(38), leg(37),
            n(36), R, n(48), R,
        ],
    },

    // --- THICK & DETUNED ---
    {
        name: 'Unison Throb',
        params: defaults([
            [P.VCO2_PITCH, 0.15], [P.VCO3_PITCH, -0.15],
            [P.CUTOFF, 800], [P.PEAK, 0.45], [P.EG_INT, 0.7],
            [P.ATTACK, 0.008], [P.DECAY_RELEASE, 0.5],
            [P.LFO_INT, 0.2], [P.LFO_RATE, 2], [P.LFO_TO_CUTOFF, 1],
        ]),
        steps: [
            n(36, 200), R, R, R,
            gh(36), R, n(36, 200), R,
            R, R, gh(36), R,
            n(36, 200), R, R, n(43, 50),
        ],
    },
    {
        name: 'Fifth Stack',
        params: defaults([
            [P.VCO2_PITCH, 7], [P.VCO3_PITCH, -12],
            [P.CUTOFF, 1200], [P.PEAK, 0.3], [P.EG_INT, 0.5],
            [P.ATTACK, 0.005], [P.DECAY_RELEASE, 0.35],
        ]),
        steps: [
            n(36), R, gh(36), n(36),
            R, n(41), R, gh(41),
            n(43), R, gh(43), n(43),
            R, n(41), leg(39), n(36, 40),
        ],
    },

    // --- DARK & ATMOSPHERIC ---
    {
        name: 'Haunted',
        params: defaults([
            [P.VCO1_WAVE, 1], [P.VCO2_WAVE, 1], [P.VCO3_WAVE, 1],
            [P.VCO2_PITCH, 0.12], [P.VCO3_PITCH, -0.12],
            [P.CUTOFF, 500], [P.PEAK, 0.6], [P.EG_INT, 0.3],
            [P.ATTACK, 0.05], [P.DECAY_RELEASE, 1.5], [P.SUSTAIN_ON, 1],
            [P.LFO_INT, 0.15], [P.LFO_RATE, 0.5], [P.LFO_TO_CUTOFF, 1],
        ]),
        steps: [
            n(36, 300), R, R, R,
            R, R, R, R,
            n(34, 300), R, R, R,
            R, R, R, R,
        ],
    },
    {
        name: 'Growl Machine',
        params: defaults([
            [P.CUTOFF, 500], [P.PEAK, 0.8], [P.EG_INT, 1.0],
            [P.ATTACK, 0.001], [P.DECAY_RELEASE, 0.15],
            [P.VCO2_PITCH, 7], [P.VCO3_PITCH, 12],
        ]),
        steps: [
            n(36), n(36, 40), R, n(36),
            R, n(36), n(36, 40), R,
            n(36), R, n(36), n(36, 40),
            R, n(36), R, n(48),
        ],
    },
];
