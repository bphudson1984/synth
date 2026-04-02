import { PARAM } from './audio/engine';

export interface BassPreset {
    name: string;
    params: [number, number][];
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

export const PRESETS: BassPreset[] = [
    { name: 'Warm Unison', params: defaults([
        [P.VCO2_PITCH, 0.08], [P.VCO3_PITCH, -0.08],
        [P.CUTOFF, 2000], [P.PEAK, 0.3], [P.EG_INT, 0.5],
        [P.ATTACK, 0.005], [P.DECAY_RELEASE, 0.3],
    ])},
    { name: 'Sub Rumble', params: defaults([
        [P.VCO1_WAVE, 1], [P.VCO2_ACTIVE, 0], [P.VCO3_ACTIVE, 0],
        [P.CUTOFF, 400], [P.PEAK, 0.5], [P.EG_INT, 0.3],
        [P.ATTACK, 0.001], [P.DECAY_RELEASE, 0.8],
    ])},
    { name: 'Acid Bass', params: defaults([
        [P.VCO2_ACTIVE, 0], [P.VCO3_ACTIVE, 0],
        [P.CUTOFF, 500], [P.PEAK, 0.7], [P.EG_INT, 0.9],
        [P.ATTACK, 0.001], [P.DECAY_RELEASE, 0.15],
    ])},
    { name: 'Deep Throb', params: defaults([
        [P.VCO1_WAVE, 1], [P.VCO2_WAVE, 0], [P.VCO3_WAVE, 1],
        [P.VCO2_PITCH, -12], [P.VCO3_PITCH, 0],
        [P.CUTOFF, 800], [P.PEAK, 0.4], [P.EG_INT, 0.6],
        [P.ATTACK, 0.01], [P.DECAY_RELEASE, 0.5],
        [P.LFO_INT, 0.3], [P.LFO_RATE, 3], [P.LFO_TO_CUTOFF, 1],
    ])},
    { name: 'Growl', params: defaults([
        [P.CUTOFF, 600], [P.PEAK, 0.8], [P.EG_INT, 1.0],
        [P.ATTACK, 0.001], [P.DECAY_RELEASE, 0.2],
        [P.VCO2_PITCH, 7], [P.VCO3_PITCH, 12],
    ])},
    { name: 'Pulse Pad', params: defaults([
        [P.VCO1_WAVE, 1], [P.VCO2_WAVE, 1], [P.VCO3_WAVE, 1],
        [P.VCO2_PITCH, 0.15], [P.VCO3_PITCH, -0.15],
        [P.CUTOFF, 3000], [P.PEAK, 0.2], [P.EG_INT, 0.3],
        [P.ATTACK, 0.1], [P.DECAY_RELEASE, 1.0], [P.SUSTAIN_ON, 1],
    ])},
];
