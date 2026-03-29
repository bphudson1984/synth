export const MODELS = [
    'SAW', 'SQUARE', 'MORPH', 'FOLD', 'FM', 'FBFM',
    'WTBL', 'VOWL', 'PLUK', 'BELL', 'NOIS', 'SWARM', 'CLOUD',
] as const;

export const NOTE_PADS = [
    { id: 'c',  label: 'C',  note: 48, colour: '#B56ECC' },
    { id: 'cs', label: 'C#', note: 49, colour: '#A45DBE' },
    { id: 'd',  label: 'D',  note: 50, colour: '#B56ECC' },
    { id: 'ds', label: 'D#', note: 51, colour: '#A45DBE' },
    { id: 'e',  label: 'E',  note: 52, colour: '#B56ECC' },
    { id: 'f',  label: 'F',  note: 53, colour: '#A45DBE' },
    { id: 'fs', label: 'F#', note: 54, colour: '#B56ECC' },
    { id: 'g',  label: 'G',  note: 55, colour: '#A45DBE' },
] as const;

export const LEAD_PARAMS = ['timbre', 'color', 'cutoff', 'release'] as const;
export type LeadParamName = typeof LEAD_PARAMS[number];

export const LEAD_PARAM_MAP: Record<LeadParamName, { id: number; min: number; max: number }> = {
    timbre:  { id: 1, min: 0.0, max: 1.0 },
    color:   { id: 2, min: 0.0, max: 1.0 },
    cutoff:  { id: 3, min: 100, max: 12000 },
    release: { id: 9, min: 0.01, max: 3.0 },
};

export const LEAD_COLOUR = '#B56ECC';
