import type { SettingsSection } from '../shared/types/settings';
import { PAD_PARAM } from './audio/engine';

export const SAMPLER_COLOUR = '#E05555';
export const NUM_PADS = 16;

// Pad labels for the PadCircle (16 pads, named 1-16)
export const SAMPLER_PADS = Array.from({ length: NUM_PADS }, (_, i) => ({
    id: `pad-${i}`,
    label: `${i + 1}`,
    colour: SAMPLER_COLOUR,
    midiNote: 36 + i, // MIDI notes 36-51 map to pads 0-15
}));

// Settings for the selected pad
export const SAMPLER_SETTINGS: SettingsSection[] = [
    {
        label: 'PAD',
        params: [
            { name: 'VOLUME', id: PAD_PARAM.VOLUME, min: 0, max: 1, default: 1.0, type: 'slider' },
            { name: 'PITCH',  id: PAD_PARAM.PITCH,  min: -24, max: 24, default: 0, type: 'slider' },
            { name: 'MODE',   id: PAD_PARAM.PLAY_MODE, min: 0, max: 2, default: 0, type: 'select',
              options: [
                  { value: 0, label: 'ONE-SHOT' },
                  { value: 1, label: 'GATE' },
                  { value: 2, label: 'LOOP' },
              ],
            },
            { name: 'CHOKE',  id: PAD_PARAM.CHOKE_GROUP, min: 0, max: 4, default: 0, type: 'select',
              options: [
                  { value: 0, label: 'OFF' },
                  { value: 1, label: '1' },
                  { value: 2, label: '2' },
                  { value: 3, label: '3' },
                  { value: 4, label: '4' },
              ],
            },
            { name: 'REVERSE', id: PAD_PARAM.REVERSE, min: 0, max: 1, default: 0, type: 'toggle' },
        ],
    },
];
