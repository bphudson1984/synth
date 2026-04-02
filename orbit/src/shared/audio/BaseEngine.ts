import { getAudioContext, getLimiterNode } from './context';

/**
 * Base class for all WASM audio engines.
 * Handles common boilerplate: WASM loading, worklet setup, panning,
 * step callback, and common sequencer messages.
 */
export abstract class BaseEngine {
    protected node: AudioWorkletNode | null = null;
    protected gainNode: GainNode | null = null;
    protected panner: StereoPannerNode | null = null;
    protected _ready = false;
    get ready() { return this._ready; }
    onStep: ((step: number) => void) | null = null;

    protected async initWorklet(processorName: string, wasmFile: string): Promise<void> {
        const ctx = await getAudioContext();
        const wasmResponse = await fetch(import.meta.env.BASE_URL + wasmFile);
        if (!wasmResponse.ok) throw new Error(`Failed to fetch ${wasmFile}: ${wasmResponse.status}`);
        const wasmBytes = await wasmResponse.arrayBuffer();
        this.node = new AudioWorkletNode(ctx, processorName, {
            outputChannelCount: [2], numberOfOutputs: 1,
        });
        await new Promise<void>((resolve, reject) => {
            const timeout = setTimeout(() => reject(new Error(`${processorName} worklet timeout`)), 10000);
            this.node!.port.onmessage = (e) => {
                if (e.data.type === 'ready') { clearTimeout(timeout); this._ready = true; resolve(); }
                if (e.data.type === 'error') { clearTimeout(timeout); reject(new Error(e.data.message)); }
                if (e.data.type === 'step') { this.onStep?.(e.data.step); }
            };
            this.node!.port.postMessage({ type: 'wasm-bytes', bytes: wasmBytes }, [wasmBytes]);
        });
        // Chain: worklet → gain → panner → limiter → destination
        this.gainNode = ctx.createGain();
        this.panner = ctx.createStereoPanner();
        this.node.connect(this.gainNode);
        this.gainNode.connect(this.panner);
        const dest = getLimiterNode() ?? ctx.destination;
        this.panner.connect(dest);
    }

    setVolume(value: number) { if (this.gainNode) this.gainNode.gain.value = value; }
    setPan(value: number) { if (this.panner) this.panner.pan.value = value; }
    setParam(id: number, value: number) { this.node?.port.postMessage({ type: 'set-param', id, value }); }

    // Common sequencer methods
    seqPlay() { this.node?.port.postMessage({ type: 'seq-play' }); }
    seqStop() { this.node?.port.postMessage({ type: 'seq-stop' }); }
    seqSetBpm(bpm: number) { this.node?.port.postMessage({ type: 'seq-bpm', value: bpm }); }
    seqClear() { this.node?.port.postMessage({ type: 'seq-clear' }); }
}
