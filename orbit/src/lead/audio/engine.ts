import { getAudioContext } from '../../shared/audio/context';

export class BraidsEngine {
    private node: AudioWorkletNode | null = null;
    private panner: StereoPannerNode | null = null;
    private _ready = false;
    get ready() { return this._ready; }

    async init(): Promise<void> {
        const ctx = await getAudioContext();
        const wasmResponse = await fetch(import.meta.env.BASE_URL + 'braids-dsp.wasm');
        if (!wasmResponse.ok) throw new Error(`Failed to fetch braids-dsp.wasm: ${wasmResponse.status}`);
        const wasmBytes = await wasmResponse.arrayBuffer();
        this.node = new AudioWorkletNode(ctx, 'braids-processor', {
            outputChannelCount: [2], numberOfOutputs: 1,
        });
        await new Promise<void>((resolve, reject) => {
            const timeout = setTimeout(() => reject(new Error('Braids worklet timeout')), 10000);
            this.node!.port.onmessage = (e) => {
                if (e.data.type === 'ready') { clearTimeout(timeout); this._ready = true; resolve(); }
                if (e.data.type === 'error') { clearTimeout(timeout); reject(new Error(e.data.message)); }
            };
            this.node!.port.postMessage({ type: 'wasm-bytes', bytes: wasmBytes }, [wasmBytes]);
        });
        this.panner = ctx.createStereoPanner();
        this.node.connect(this.panner);
        this.panner.connect(ctx.destination);
    }

    setPan(value: number) { if (this.panner) this.panner.pan.value = value; }
    noteOn(note: number, velocity: number) { this.node?.port.postMessage({ type: 'note-on', note, velocity }); }
    noteOff(note: number) { this.node?.port.postMessage({ type: 'note-off', note }); }
    setParam(id: number, value: number) { this.node?.port.postMessage({ type: 'set-param', id, value }); }
}

export const PARAM = {
    MODEL: 0, TIMBRE: 1, COLOR: 2,
    FILTER_CUTOFF: 3, FILTER_RESONANCE: 4, FILTER_ENV_AMT: 5,
    AMP_ATTACK: 6, AMP_DECAY: 7, AMP_SUSTAIN: 8, AMP_RELEASE: 9,
    LFO_RATE: 10, LFO_AMOUNT: 11, LFO_DEST: 12,
    MASTER_VOL: 13, GLIDE_TIME: 14,
    FILT_ATTACK: 15, FILT_DECAY: 16, FILT_SUSTAIN: 17, FILT_RELEASE: 18,
} as const;
