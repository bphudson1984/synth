// Voice definitions — maps ORBIT voices to TR-808 engine voice IDs
export const VOICES = [
    { id: 'kick',     label: 'KCK', colour: '#378ADD', engineId: 0 },
    { id: 'snare',    label: 'SNR', colour: '#D85A30', engineId: 1 },
    { id: 'closedHH', label: 'CHH', colour: '#1D9E75', engineId: 7 },
    { id: 'openHH',   label: 'OHH', colour: '#5DCAA5', engineId: 8 },
    { id: 'clap',     label: 'CLP', colour: '#D4537E', engineId: 6 },
    { id: 'tom',      label: 'TOM', colour: '#EF9F27', engineId: 3 },
    { id: 'rim',      label: 'RIM', colour: '#7F77DD', engineId: 5 },
    { id: 'perc',     label: 'PRC', colour: '#639922', engineId: 12 },
] as const;

export const PARAMS = ['level', 'decay', 'tone', 'pitch'] as const;
export type ParamName = typeof PARAMS[number];

export const NUM_STEPS = 16;
export const NUM_VOICES = 8;

export const DEFAULT_BPM = 120;
export const MIN_BPM = 60;
export const MAX_BPM = 200;
