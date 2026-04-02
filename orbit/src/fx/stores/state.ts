import { writable, get } from 'svelte/store';
import type { FxEngine, EffectId } from '../audio/engine';
import type { ChannelId } from '../../shared/stores/mixer';

// Per-engine send levels for each effect: sendLevels[channelId][effectIndex] = 0..100
type SendLevels = Record<ChannelId, [number, number, number, number]>;

export const sendLevels = writable<SendLevels>({
    drum: [0, 0, 0, 0],
    pad:  [0, 0, 0, 0],
    acid: [0, 0, 0, 0],
    lead: [0, 0, 0, 0],
    bass: [0, 0, 0, 0],
});

// Effect parameters (raw values)
// Chorus: 0=rate, 1=depth
// Delay: 0=time_ms, 1=feedback, 2=tone
// Reverb: 0=decay, 1=damping
// Distortion: 0=drive, 1=tone, 2=level
export const fxParams = writable<Record<number, Record<number, number>>>({
    0: { 0: 0.8, 1: 0.5 },
    1: { 0: 375, 1: 0.4, 2: 0.6 },
    2: { 0: 0.7, 1: 0.7 },
    3: { 0: 0.3, 1: 0.5, 2: 0.7 },
});

let fxEngine: FxEngine | null = null;
const engineSendSetters: Record<string, (effectIndex: number, level: number) => void> = {};

export function setFxEngine(e: FxEngine) {
    fxEngine = e;
    // Apply default params
    const params = get(fxParams);
    for (const [effectId, paramMap] of Object.entries(params)) {
        for (const [paramId, value] of Object.entries(paramMap)) {
            e.setParam(Number(effectId) as EffectId, Number(paramId), value);
        }
    }
}

export function registerEngineSends(id: ChannelId, setter: (effectIndex: number, level: number) => void) {
    engineSendSetters[id] = setter;
}

export function setFxParam(effectId: EffectId, paramId: number, value: number) {
    fxParams.update(p => {
        p[effectId] = { ...p[effectId], [paramId]: value };
        return { ...p };
    });
    fxEngine?.setParam(effectId, paramId, value);
}

export function setSendLevel(channelId: ChannelId, effectIndex: number, level: number) {
    sendLevels.update(s => {
        const arr = [...s[channelId]] as [number, number, number, number];
        arr[effectIndex] = level;
        return { ...s, [channelId]: arr };
    });
    // Convert 0-100 to 0-1 gain
    engineSendSetters[channelId]?.(effectIndex, level / 100);
}
