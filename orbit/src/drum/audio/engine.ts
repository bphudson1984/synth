import { VOICES, getEngineVoiceId, getEngineTrackId, type EngineType } from '../constants';
import { getAudioContext } from '../../shared/audio/context';

export class OrbitEngine {
    private node: AudioWorkletNode | null = null;
    private panner: StereoPannerNode | null = null;
    private _ready = false;
    get ready() { return this._ready; }
    onStep: ((step: number) => void) | null = null;

    async init(): Promise<void> {
        const ctx = await getAudioContext();
        const wasmResponse = await fetch(import.meta.env.BASE_URL + 'tr808.wasm');
        if (!wasmResponse.ok) {
            throw new Error(`Failed to fetch tr808.wasm: ${wasmResponse.status} ${wasmResponse.statusText}`);
        }
        const wasmModule = await WebAssembly.compile(await wasmResponse.arrayBuffer());
        this.node = new AudioWorkletNode(ctx, 'tr808-processor', {
            outputChannelCount: [2], numberOfOutputs: 1,
        });
        await new Promise<void>((resolve, reject) => {
            const timeout = setTimeout(() => {
                reject(new Error('TR808 worklet initialization timed out'));
            }, 10000);

            this.node!.port.onmessage = (e) => {
                if (e.data.type === 'ready') {
                    clearTimeout(timeout);
                    this._ready = true;
                    resolve();
                } else if (e.data.type === 'error') {
                    clearTimeout(timeout);
                    reject(new Error(e.data.message ?? 'TR808 worklet failed'));
                }
                if (e.data.type === 'step') { this.onStep?.(e.data.step); }
            };
            this.node!.port.postMessage({ type: 'wasm-module', module: wasmModule });
        });
        this.panner = ctx.createStereoPanner();
        this.node.connect(this.panner);
        this.panner.connect(ctx.destination);
    }

    setPan(value: number) { if (this.panner) this.panner.pan.value = value; }

    triggerVoice(orbitIndex: number, engine: EngineType = '808') {
        const voiceId = getEngineVoiceId(orbitIndex, engine);
        this.node?.port.postMessage({ type: 'trigger', voice: voiceId });
    }

    setVoiceParam(orbitIndex: number, param: string, value: number, engine: EngineType = '808') {
        const voiceId = getEngineVoiceId(orbitIndex, engine);
        const norm = value / 100;
        let paramId: number;
        switch (param) {
            case 'level': paramId = 0; break;
            case 'decay': paramId = 2; break;
            case 'tone':  paramId = 1; break;
            case 'pitch': paramId = 1; break;
            default: return;
        }
        this.node?.port.postMessage({ type: 'set-param', voice: voiceId, param: paramId, value: norm });
    }

    seqPlay() { this.node?.port.postMessage({ type: 'seq-play' }); }
    seqStop() { this.node?.port.postMessage({ type: 'seq-stop' }); }
    seqSetBpm(bpm: number) { this.node?.port.postMessage({ type: 'seq-bpm', value: bpm }); }
    seqToggleStep(orbitVoice: number, step: number) {
        const trackId = getEngineTrackId(orbitVoice);
        this.node?.port.postMessage({ type: 'seq-toggle', track: trackId, step });
    }
    seqClear() { this.node?.port.postMessage({ type: 'seq-clear' }); }

    setTrackEngine(orbitIndex: number, engine: EngineType) {
        const trackId = getEngineTrackId(orbitIndex);
        this.node?.port.postMessage({ type: 'set-track-engine', track: trackId, is909: engine === '909' });
    }

    setAllEngines(engine: EngineType) {
        this.node?.port.postMessage({ type: 'set-all-engines', is909: engine === '909' });
    }

    setMasterVolume(value: number) {
        this.node?.port.postMessage({ type: 'set-param', voice: 255, param: 0, value });
    }
}
