export type EngineType = '808' | '909';

export const VOICES = [
    { id: 'kick',     label: 'KCK', colour: '#378ADD', engineId808: 0, engineId909: 100 },
    { id: 'snare',    label: 'SNR', colour: '#D85A30', engineId808: 1, engineId909: 101 },
    { id: 'closedHH', label: 'CHH', colour: '#1D9E75', engineId808: 7, engineId909: 107 },
    { id: 'openHH',   label: 'OHH', colour: '#5DCAA5', engineId808: 8, engineId909: 108 },
    { id: 'clap',     label: 'CLP', colour: '#D4537E', engineId808: 6, engineId909: 106 },
    { id: 'tom',      label: 'TOM', colour: '#EF9F27', engineId808: 3, engineId909: 103 },
    { id: 'rim',      label: 'RIM', colour: '#7F77DD', engineId808: 5, engineId909: 105 },
    { id: 'perc',     label: 'PRC', colour: '#639922', engineId808: 12, engineId909: 109 },
] as const;

export const PARAMS = ['level', 'decay', 'tone', 'pitch'] as const;
export type ParamName = typeof PARAMS[number];

export const NUM_STEPS = 16;
export const NUM_VOICES = 8;

export const DEFAULT_BPM = 120;
export const MIN_BPM = 60;
export const MAX_BPM = 200;

// Map ORBIT voice index to engine track ID (for sequencer)
export function getEngineTrackId(orbitIndex: number): number {
    return VOICES[orbitIndex]?.engineId808 ?? 0; // sequencer always uses 808 track IDs
}

export function getEngineVoiceId(orbitIndex: number, engine: EngineType): number {
    const v = VOICES[orbitIndex];
    if (!v) return 0;
    return engine === '909' ? v.engineId909 : v.engineId808;
}
