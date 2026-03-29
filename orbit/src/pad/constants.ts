export const CHORDS = [
    { id: 'i',    label: 'Cm',   notes: [48, 51, 55], colour: '#6B5CE7' },
    { id: 'ii',   label: 'Ddm',  notes: [50, 53, 56], colour: '#8B6CE7' },
    { id: 'III',  label: 'Eb',   notes: [51, 55, 58], colour: '#5C8BE7' },
    { id: 'iv',   label: 'Fm',   notes: [53, 56, 60], colour: '#5CAEE7' },
    { id: 'v',    label: 'Gm',   notes: [55, 58, 62], colour: '#5CE7D4' },
    { id: 'VI',   label: 'Ab',   notes: [56, 60, 63], colour: '#5CE7A0' },
    { id: 'VII',  label: 'Bb',   notes: [58, 62, 65], colour: '#7CE75C' },
    { id: 'i8',   label: "Cm'",  notes: [60, 63, 67], colour: '#A0E75C' },
] as const;

export const PAD_PARAMS = ['cutoff', 'resonance', 'attack', 'release'] as const;
export type PadParamName = typeof PAD_PARAMS[number];

// Map slider 0-100 to actual Prophet-5 parameter ranges
export const PAD_PARAM_MAP: Record<PadParamName, { id: number; min: number; max: number }> = {
    cutoff:    { id: 12, min: 200,    max: 12000 },
    resonance: { id: 13, min: 0,      max: 3.0 },
    attack:    { id: 20, min: 0.005,  max: 2.0 },
    release:   { id: 23, min: 0.01,   max: 3.0 },
};
