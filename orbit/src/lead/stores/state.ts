import { writable, derived, get } from 'svelte/store';
import type { BraidsEngine } from '../audio/engine';
import { PARAM } from '../audio/engine';
import { MODELS, LEAD_PARAMS, LEAD_PARAM_MAP, type LeadParamName } from '../constants';
import { registerMixerCallback } from '../../shared/stores/mixer';

let engine: BraidsEngine | null = null;
export function setLeadEngine(e: BraidsEngine) {
    engine = e;
    registerMixerCallback('lead',
        (gain) => { engine?.setParam(PARAM.MASTER_VOL, gain); },
        (pan) => { engine?.setPan(pan); }
    );
}

export const selectedModel = writable(0);
export const selectedParam = writable<LeadParamName>('timbre');
export const synthParams = writable<Record<LeadParamName, number>>({
    timbre: 50, color: 50, cutoff: 65, release: 30,
});
export const triggeredNotes = writable(new Set<number>());

export const sliderValue = derived(
    [selectedParam, synthParams],
    ([$param, $params]) => $params[$param]
);

export function selectModel(index: number) {
    selectedModel.set(index);
    engine?.setParam(PARAM.MODEL, index);
}

export function selectLeadParam(param: string) {
    selectedParam.set(param as LeadParamName);
}

export function setSliderValue(value: number) {
    const param = get(selectedParam);
    synthParams.update(p => { p[param] = value; return p; });
    const mapping = LEAD_PARAM_MAP[param];
    const actual = mapping.min + (value / 100) * (mapping.max - mapping.min);
    engine?.setParam(mapping.id, actual);
}

export function triggerNote(padIndex: number, note: number) {
    engine?.noteOn(note, 100);
    triggeredNotes.update(s => { s.add(padIndex); return new Set(s); });
    setTimeout(() => {
        triggeredNotes.update(s => { s.delete(padIndex); return new Set(s); });
    }, 150);
    // Release after sustain (if not held)
    setTimeout(() => { engine?.noteOff(note); }, 800);
}
