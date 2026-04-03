import type { SettingsSection } from '../shared/types/settings';

export const PAD_PARAM = {
    VOLUME: 0,
    PITCH: 1,
    PLAY_MODE: 2,
    CHOKE_GROUP: 3,
    REVERSE: 4,
    PAN: 5,
    ATTACK: 6,
    RELEASE: 7,
    START: 8,
    END: 9,
    BIT_DEPTH: 10,
    VOCODER_ON: 11,
    VOCODER_ROOT: 12,
    VOCODER_CARRIER: 13,
    VOCODER_BANDS: 14,
    VOCODER_FORMANT: 15,
    VOCODER_MIX: 16,
} as const;

export const SAMPLER_COLOUR = '#E05555';
export const NUM_PADS = 16;

// Pad labels for the PadCircle (16 pads, named 1-16)
export const SAMPLER_PADS = Array.from({ length: NUM_PADS }, (_, i) => ({
    id: `pad-${i}`,
    label: `${i + 1}`,
    colour: SAMPLER_COLOUR,
    midiNote: 36 + i,
}));

// Settings for the selected pad
export const SAMPLER_SETTINGS: SettingsSection[] = [
    {
        label: 'LEVEL',
        params: [
            { name: 'VOLUME',    id: PAD_PARAM.VOLUME,    min: 0, max: 1, default: 1.0, type: 'slider' },
            { name: 'PAN',       id: PAD_PARAM.PAN,       min: -1, max: 1, default: 0, type: 'slider' },
            { name: 'BIT CRUSH', id: PAD_PARAM.BIT_DEPTH, min: 1, max: 16, default: 16, type: 'slider' },
        ],
    },
    {
        label: 'PITCH',
        params: [
            { name: 'PITCH',  id: PAD_PARAM.PITCH,  min: -24, max: 24, default: 0, type: 'slider' },
        ],
    },
    {
        label: 'ENVELOPE',
        params: [
            { name: 'ATTACK',  id: PAD_PARAM.ATTACK,  min: 0, max: 2, default: 0.002, type: 'slider' },
            { name: 'RELEASE', id: PAD_PARAM.RELEASE, min: 0, max: 2, default: 0.005, type: 'slider' },
        ],
    },
    {
        label: 'REGION',
        params: [
            { name: 'START', id: PAD_PARAM.START, min: 0, max: 1, default: 0, type: 'slider' },
            { name: 'END',   id: PAD_PARAM.END,   min: 0, max: 1, default: 1, type: 'slider' },
        ],
    },
    {
        label: 'VOCODER',
        params: [
            { name: 'ENABLE',  id: PAD_PARAM.VOCODER_ON, min: 0, max: 1, default: 0, type: 'toggle' },
            { name: 'ROOT',    id: PAD_PARAM.VOCODER_ROOT, min: 24, max: 84, default: 60, type: 'slider' },
            { name: 'CARRIER', id: PAD_PARAM.VOCODER_CARRIER, min: 0, max: 2, default: 0, type: 'select',
              options: [
                  { value: 0, label: 'SAW' },
                  { value: 1, label: 'SQR' },
                  { value: 2, label: 'NOISE' },
              ],
            },
            { name: 'BANDS',   id: PAD_PARAM.VOCODER_BANDS, min: 4, max: 16, default: 12, type: 'slider' },
            { name: 'FORMANT', id: PAD_PARAM.VOCODER_FORMANT, min: -12, max: 12, default: 0, type: 'slider' },
            { name: 'MIX',     id: PAD_PARAM.VOCODER_MIX, min: 0, max: 1, default: 1, type: 'slider' },
        ],
    },
    {
        label: 'MODE',
        params: [
            { name: 'MODE', id: PAD_PARAM.PLAY_MODE, min: 0, max: 2, default: 0, type: 'select',
              options: [
                  { value: 0, label: 'ONE-SHOT' },
                  { value: 1, label: 'GATE' },
                  { value: 2, label: 'LOOP' },
              ],
            },
            { name: 'REVERSE', id: PAD_PARAM.REVERSE, min: 0, max: 1, default: 0, type: 'toggle' },
            { name: 'CHOKE', id: PAD_PARAM.CHOKE_GROUP, min: 0, max: 4, default: 0, type: 'select',
              options: [
                  { value: 0, label: 'OFF' },
                  { value: 1, label: '1' },
                  { value: 2, label: '2' },
                  { value: 3, label: '3' },
                  { value: 4, label: '4' },
              ],
            },
        ],
    },
];
