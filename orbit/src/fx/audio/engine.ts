import { getAudioContext } from '../../shared/audio/context';

export type EffectId = 0 | 1 | 2 | 3; // chorus, delay, reverb, distortion

export class FxEngine {
    private node: AudioWorkletNode | null = null;
    private _ready = false;

    // 4 bus GainNodes — each sums all engine sends for that effect
    chorusBus: GainNode | null = null;
    delayBus: GainNode | null = null;
    reverbBus: GainNode | null = null;
    distBus: GainNode | null = null;

    get ready() { return this._ready; }

    async init(): Promise<void> {
        const ctx = await getAudioContext();
        const wasmResponse = await fetch(import.meta.env.BASE_URL + 'fx-rack.wasm');
        if (!wasmResponse.ok) throw new Error(`Failed to fetch fx-rack.wasm: ${wasmResponse.status}`);
        const wasmBytes = await wasmResponse.arrayBuffer();

        this.node = new AudioWorkletNode(ctx, 'fx-processor', {
            numberOfInputs: 4,
            numberOfOutputs: 1,
            outputChannelCount: [2],
            channelCount: 2,
            channelCountMode: 'explicit',
        });

        await new Promise<void>((resolve, reject) => {
            const timeout = setTimeout(() => reject(new Error('fx-processor worklet timeout')), 10000);
            this.node!.port.onmessage = (e) => {
                if (e.data.type === 'ready') { clearTimeout(timeout); this._ready = true; resolve(); }
                if (e.data.type === 'error') { clearTimeout(timeout); reject(new Error(e.data.message)); }
            };
            this.node!.port.postMessage({ type: 'wasm-bytes', bytes: wasmBytes }, [wasmBytes]);
        });

        // Create the 4 bus GainNodes
        this.chorusBus = ctx.createGain();
        this.delayBus = ctx.createGain();
        this.reverbBus = ctx.createGain();
        this.distBus = ctx.createGain();

        // Connect buses to FX worklet inputs
        this.chorusBus.connect(this.node, 0, 0);
        this.delayBus.connect(this.node, 0, 1);
        this.reverbBus.connect(this.node, 0, 2);
        this.distBus.connect(this.node, 0, 3);

        // Connect FX output to destination
        this.node.connect(ctx.destination);
    }

    setParam(effectId: EffectId, paramId: number, value: number) {
        this.node?.port.postMessage({ type: 'set-param', effectId, paramId, value });
    }
}
