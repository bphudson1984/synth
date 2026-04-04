import { getAudioContext } from './context';
import type { FxEngine } from '../../fx/audio/engine';

/**
 * Base class for all WASM audio engines.
 * Handles common boilerplate: WASM loading, worklet setup, panning,
 * step callback, and common sequencer messages.
 */
export abstract class BaseEngine {
    protected node: AudioWorkletNode | null = null;
    protected channelGain: GainNode | null = null;
    protected panner: StereoPannerNode | null = null;
    protected sendGains: GainNode[] = [];
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
            const isFirefox = /Firefox/.test(navigator.userAgent);
            this.node!.port.postMessage({ type: 'wasm-bytes', bytes: wasmBytes, useDoubleBuffer: isFirefox }, [wasmBytes]);
        });
        this.channelGain = ctx.createGain();
        this.panner = ctx.createStereoPanner();
        this.node.connect(this.channelGain);
        this.channelGain.connect(this.panner);
        this.panner.connect(ctx.destination);
    }

    /** Set the mixer channel gain (0-1). Used by the mixer faders for pre-mix level control. */
    setChannelGain(gain: number) { if (this.channelGain) this.channelGain.gain.value = Math.max(0, Math.min(1, gain)); }
    setPan(value: number) { if (this.panner) this.panner.pan.value = value; }
    setParam(id: number, value: number) { this.node?.port.postMessage({ type: 'set-param', id, value }); }

    /** Create 4 send gain nodes from this engine to the FX rack buses. */
    connectSends(fxEngine: FxEngine): void {
        if (!this.panner) return;
        const ctx = this.panner.context as AudioContext;
        const buses = [fxEngine.chorusBus, fxEngine.delayBus, fxEngine.reverbBus, fxEngine.distBus];
        for (const bus of buses) {
            const send = ctx.createGain();
            send.gain.value = 0; // sends start muted
            this.panner.connect(send);
            send.connect(bus!);
            this.sendGains.push(send);
        }
    }

    /** Set the send level for one effect (0=chorus, 1=delay, 2=reverb, 3=distortion). */
    setSendLevel(effectIndex: number, level: number): void {
        if (this.sendGains[effectIndex]) {
            this.sendGains[effectIndex].gain.value = level;
        }
    }

    // Common sequencer methods
    seqPlay() { this.node?.port.postMessage({ type: 'seq-play' }); }
    seqStop() { this.node?.port.postMessage({ type: 'seq-stop' }); }
    seqSetBpm(bpm: number) { this.node?.port.postMessage({ type: 'seq-bpm', value: bpm }); }
    seqClear() { this.node?.port.postMessage({ type: 'seq-clear' }); }
    seqSetGlitch(size: number) { this.node?.port.postMessage({ type: 'seq-set-glitch', value: size }); }
}
