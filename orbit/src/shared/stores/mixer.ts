import { writable, get } from 'svelte/store';

export type ChannelId = 'drum' | 'pad' | 'acid';

interface ChannelState {
    volume: number;  // 0-100
    pan: number;     // -100 to 100 (left to right)
    mute: boolean;
    solo: boolean;
}

export const channels = writable<Record<ChannelId, ChannelState>>({
    drum: { volume: 75, pan: 0, mute: false, solo: false },
    pad:  { volume: 75, pan: 0, mute: false, solo: false },
    acid: { volume: 75, pan: 0, mute: false, solo: false },
});

const volumeCallbacks: Record<string, (gain: number) => void> = {};
const panCallbacks: Record<string, (pan: number) => void> = {};

export function registerMixerCallback(id: ChannelId, volumeCb: (gain: number) => void, panCb: (pan: number) => void) {
    volumeCallbacks[id] = volumeCb;
    panCallbacks[id] = panCb;
    applyMix();
}

function applyMix() {
    const state = get(channels);
    const anySolo = Object.values(state).some(c => c.solo);

    for (const id of ['drum', 'pad', 'acid'] as ChannelId[]) {
        const ch = state[id];
        let gain: number;
        if (ch.mute) {
            gain = 0;
        } else if (anySolo && !ch.solo) {
            gain = 0;
        } else {
            gain = ch.volume / 100;
        }
        volumeCallbacks[id]?.(gain);
        panCallbacks[id]?.(state[id].pan / 100); // -1 to 1
    }
}

export function setVolume(id: ChannelId, volume: number) {
    channels.update(c => { c[id].volume = Math.round(volume); return { ...c }; });
    applyMix();
}

export function toggleMute(id: ChannelId) {
    channels.update(c => { c[id].mute = !c[id].mute; return { ...c }; });
    applyMix();
}

export function toggleSolo(id: ChannelId) {
    channels.update(c => { c[id].solo = !c[id].solo; return { ...c }; });
    applyMix();
}

export function setPan(id: ChannelId, pan: number) {
    channels.update(c => { c[id].pan = Math.round(pan); return { ...c }; });
    panCallbacks[id]?.(pan / 100);
}
