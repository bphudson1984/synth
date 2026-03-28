import { VOICES } from '../constants';

export class OrbitEngine {
    private node: AudioWorkletNode | null = null;
    private _ready = false;

    get ready() { return this._ready; }

    async init(): Promise<void> {
        const ctx = new AudioContext({ sampleRate: 48000 });
        const wasmModule = await WebAssembly.compileStreaming(fetch('/tr808.wasm'));
        await ctx.audioWorklet.addModule('/worklet-processor.js');

        this.node = new AudioWorkletNode(ctx, 'tr808-processor', {
            outputChannelCount: [2],
            numberOfOutputs: 1,
        });

        await new Promise<void>((resolve) => {
            this.node!.port.onmessage = (e) => {
                if (e.data.type === 'ready') { this._ready = true; resolve(); }
            };
            this.node!.port.postMessage({ type: 'wasm-module', module: wasmModule });
        });

        this.node.connect(ctx.destination);
    }

    triggerVoice(orbitIndex: number) {
        const engineId = VOICES[orbitIndex]?.engineId;
        if (engineId !== undefined) {
            this.node?.port.postMessage({ type: 'trigger', voice: engineId });
        }
    }

    // Map ORBIT param names to engine param IDs
    // Engine: voice_id, param_id (0=level, 1=tone, 2=decay/snappy)
    setVoiceParam(orbitIndex: number, param: string, value: number) {
        const engineId = VOICES[orbitIndex]?.engineId;
        if (engineId === undefined) return;

        // Map 0-100 ORBIT range to engine ranges
        const norm = value / 100;
        let paramId: number;

        switch (param) {
            case 'level':
                paramId = 0;
                this.node?.port.postMessage({ type: 'set-param', voice: engineId, param: paramId, value: norm });
                break;
            case 'decay':
                paramId = 2; // BD decay, SD snappy, OH decay etc
                this.node?.port.postMessage({ type: 'set-param', voice: engineId, param: paramId, value: norm });
                break;
            case 'tone':
                paramId = 1;
                this.node?.port.postMessage({ type: 'set-param', voice: engineId, param: paramId, value: norm });
                break;
            case 'pitch':
                // Pitch maps to tuning on toms, or tone on others
                paramId = 1;
                this.node?.port.postMessage({ type: 'set-param', voice: engineId, param: paramId, value: norm });
                break;
        }
    }

    // Sequencer controls
    seqPlay() { this.node?.port.postMessage({ type: 'seq-play' }); }
    seqStop() { this.node?.port.postMessage({ type: 'seq-stop' }); }
    seqSetBpm(bpm: number) { this.node?.port.postMessage({ type: 'seq-bpm', value: bpm }); }
    seqToggleStep(orbitVoice: number, step: number) {
        const engineId = VOICES[orbitVoice]?.engineId;
        if (engineId !== undefined) {
            this.node?.port.postMessage({ type: 'seq-toggle', track: engineId, step });
        }
    }
    seqClear() { this.node?.port.postMessage({ type: 'seq-clear' }); }
}
