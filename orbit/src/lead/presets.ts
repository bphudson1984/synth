import type { SeqStep } from './stores/state';

export interface LeadPreset {
    name: string;
    steps: SeqStep[];
    model: number;
    timbre: number;
    color: number;
    cutoff: number;
    release: number;
}

function n(note: number, label: string): SeqStep { return { notes: [note], gate: true, label }; }
function c(notes: number[], label: string): SeqStep { return { notes, gate: true, label }; }
const R: SeqStep = { notes: [48], gate: false, label: '' };

export const LEAD_PRESETS: LeadPreset[] = [
    {
        name: 'Arp Progression',
        model: 0, timbre: 50, color: 40, cutoff: 60, release: 35,
        steps: [
            c([48,51,55,58], 'Cm7'), c([48,51,55,58], 'Cm7'), c([48,51,55,58], 'Cm7'), c([48,51,55,58], 'Cm7'),
            c([53,56,60,63], 'Fm7'), c([53,56,60,63], 'Fm7'), c([53,56,60,63], 'Fm7'), c([53,56,60,63], 'Fm7'),
            c([56,60,63,67], 'AbM7'), c([56,60,63,67], 'AbM7'), c([55,58,62,65], 'Gm7'), c([55,58,62,65], 'Gm7'),
            c([48,51,55,58], 'Cm7'), c([48,51,55,58], 'Cm7'), c([48,51,55,58], 'Cm7'), c([48,51,55,58], 'Cm7'),
        ],
    },
    {
        name: 'Melodic Lead',
        model: 2, timbre: 60, color: 30, cutoff: 70, release: 25,
        steps: [
            n(60, "C'"), n(63, "Eb'"), n(67, "G'"), n(65, "F'"),
            n(63, "Eb'"), n(60, "C'"), R, n(58, 'Bb'),
            n(60, "C'"), n(63, "Eb'"), n(67, "G'"), n(70, "Bb'"),
            n(67, "G'"), n(63, "Eb'"), n(60, "C'"), R,
        ],
    },
    {
        name: 'FM Bells',
        model: 4, timbre: 45, color: 55, cutoff: 80, release: 60,
        steps: [
            n(60, "C'"), R, n(67, "G'"), R,
            n(63, "Eb'"), R, n(58, 'Bb'), R,
            n(60, "C'"), R, n(67, "G'"), n(70, "Bb'"),
            R, n(63, "Eb'"), R, n(55, 'G'),
        ],
    },
    {
        name: 'Pluck Pattern',
        model: 8, timbre: 30, color: 50, cutoff: 75, release: 20,
        steps: [
            n(48, 'C'), n(55, 'G'), n(51, 'Eb'), n(55, 'G'),
            n(53, 'F'), n(60, "C'"), n(56, 'Ab'), n(60, "C'"),
            n(48, 'C'), n(55, 'G'), n(51, 'Eb'), n(58, 'Bb'),
            n(55, 'G'), n(51, 'Eb'), n(48, 'C'), R,
        ],
    },
    {
        name: 'Swarm Chords',
        model: 11, timbre: 65, color: 20, cutoff: 55, release: 45,
        steps: [
            c([48,51,55,58], 'Cm7'), R, R, R,
            c([53,56,60,63], 'Fm7'), R, R, R,
            c([56,60,63,67], 'AbM7'), R, R, R,
            c([55,58,62,65], 'Gm7'), R, R, R,
        ],
    },
    {
        name: 'Vowel Sequence',
        model: 7, timbre: 25, color: 50, cutoff: 65, release: 30,
        steps: [
            n(48, 'C'), n(48, 'C'), n(51, 'Eb'), n(51, 'Eb'),
            n(55, 'G'), n(55, 'G'), n(53, 'F'), n(53, 'F'),
            n(48, 'C'), n(51, 'Eb'), n(55, 'G'), n(58, 'Bb'),
            n(55, 'G'), n(53, 'F'), n(51, 'Eb'), n(48, 'C'),
        ],
    },
    {
        name: 'Cloud Texture',
        model: 12, timbre: 70, color: 60, cutoff: 50, release: 70,
        steps: [
            n(48, 'C'), R, R, n(55, 'G'),
            R, R, n(51, 'Eb'), R,
            R, n(48, 'C'), R, R,
            n(53, 'F'), R, R, n(55, 'G'),
        ],
    },
    {
        name: 'Fold Bass',
        model: 3, timbre: 80, color: 30, cutoff: 40, release: 15,
        steps: [
            n(36, 'C2'), n(36, 'C2'), R, n(36, 'C2'),
            R, n(39, 'Eb2'), n(36, 'C2'), R,
            n(36, 'C2'), R, n(41, 'F2'), n(36, 'C2'),
            R, n(36, 'C2'), n(43, 'G2'), R,
        ],
    },
];
