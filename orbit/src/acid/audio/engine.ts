import { getAudioContext } from '../../shared/audio/context';

export class AcidEngine {
    private node: AudioWorkletNode | null = null;
    private _ready = false;
    get ready() { return this._ready; }
    onStep: ((step: number) => void) | null = null;

    async init(): Promise<void> {
        const ctx = await getAudioContext();
        const wasmModule = await WebAssembly.compileStreaming(fetch(import.meta.env.BASE_URL + 'tb303.wasm'));
        this.node = new AudioWorkletNode(ctx, 'tb303-processor', {
            outputChannelCount: [2], numberOfOutputs: 1,
        });
        await new Promise<void>((resolve, reject) => {
            const timeout = setTimeout(() => {
                reject(new Error('TB303 worklet initialization timed out'));
            }, 10000);

            this.node!.port.onmessage = (e) => {
                if (e.data.type === 'ready') {
                    clearTimeout(timeout);
                    this._ready = true;
                    resolve();
                } else if (e.data.type === 'error') {
                    clearTimeout(timeout);
                    reject(new Error(e.data.message ?? 'TB303 worklet failed'));
                }
                if (e.data.type === 'step') { this.onStep?.(e.data.step); }
            };
            this.node!.port.postMessage({ type: 'wasm-module', module: wasmModule });
        });
        this.node.connect(ctx.destination);
    }

    setParam(id: number, value: number) { this.node?.port.postMessage({ type: 'set-param', id, value }); }
    seqPlay() { this.node?.port.postMessage({ type: 'seq-play' }); }
    seqStop() { this.node?.port.postMessage({ type: 'seq-stop' }); }
    seqSetBpm(bpm: number) { this.node?.port.postMessage({ type: 'seq-bpm', value: bpm }); }
    seqClear() { this.node?.port.postMessage({ type: 'seq-clear' }); }

    setStepNote(step: number, note: number) { this.node?.port.postMessage({ type: 'seq-set-step-note', step, note }); }
    setStepGate(step: number, gate: boolean) { this.node?.port.postMessage({ type: 'seq-set-step-gate', step, gate }); }
    setStepAccent(step: number, accent: boolean) { this.node?.port.postMessage({ type: 'seq-set-step-accent', step, accent }); }
    setStepSlide(step: number, slide: boolean) { this.node?.port.postMessage({ type: 'seq-set-step-slide', step, slide }); }
}

export const PARAM = {
    CUTOFF: 0, RESONANCE: 1, ENV_MOD: 2, DECAY: 3,
    ACCENT: 4, WAVEFORM: 5, VOLUME: 6, DISTORTION: 7,
} as const;
