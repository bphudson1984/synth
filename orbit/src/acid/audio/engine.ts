import { BaseEngine } from '../../shared/audio/BaseEngine';

export class AcidEngine extends BaseEngine {
    async init(): Promise<void> {
        await this.initWorklet('tb303-processor', 'tb303.wasm');
    }

    noteOn(note: number, velocity: number) { this.node?.port.postMessage({ type: 'note-on', note, velocity }); }
    noteOff(note: number) { this.node?.port.postMessage({ type: 'note-off', note }); }

    // Acid-specific step methods
    setStepNote(step: number, note: number) { this.node?.port.postMessage({ type: 'seq-set-step-note', step, note }); }
    setStepGate(step: number, gate: boolean) { this.node?.port.postMessage({ type: 'seq-set-step-gate', step, gate }); }
    setStepAccent(step: number, accent: boolean) { this.node?.port.postMessage({ type: 'seq-set-step-accent', step, accent }); }
    setStepSlide(step: number, slide: boolean) { this.node?.port.postMessage({ type: 'seq-set-step-slide', step, slide }); }

    // Pattern controls (new — matches melodic sequencers)
    setDirection(dir: number) { this.node?.port.postMessage({ type: 'seq-set-direction', value: dir }); }
    setSwing(swing: number) { this.node?.port.postMessage({ type: 'seq-set-swing', value: swing }); }
    setTimeDivision(div: number) { this.node?.port.postMessage({ type: 'seq-set-time-div', value: div }); }
}

export const PARAM = {
    CUTOFF: 0, RESONANCE: 1, ENV_MOD: 2, DECAY: 3,
    ACCENT: 4, WAVEFORM: 5, VOLUME: 6, DISTORTION: 7,
} as const;
