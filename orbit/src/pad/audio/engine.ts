import { getAudioContext } from '../../shared/audio/context';

export class ProphetEngine {
    private node: AudioWorkletNode | null = null;
    private _ready = false;
    get ready() { return this._ready; }

    async init(): Promise<void> {
        const ctx = await getAudioContext();

        const wasmResponse = await fetch(import.meta.env.BASE_URL + 'prophet-dsp.wasm');
        const wasmBytes = await wasmResponse.arrayBuffer();

        this.node = new AudioWorkletNode(ctx, 'prophet-processor', {
            outputChannelCount: [2],
            numberOfOutputs: 1,
        });

        await new Promise<void>((resolve, reject) => {
            const timeout = setTimeout(() => {
                reject(new Error('Prophet worklet initialization timed out'));
            }, 10000);

            this.node!.port.onmessage = (e) => {
                if (e.data.type === 'ready') {
                    clearTimeout(timeout);
                    this._ready = true;
                    resolve();
                } else if (e.data.type === 'error') {
                    clearTimeout(timeout);
                    reject(new Error(e.data.message ?? 'Prophet worklet failed'));
                }
            };

            this.node!.port.postMessage(
                { type: 'wasm-bytes', bytes: wasmBytes },
                [wasmBytes]
            );
        });

        this.node.connect(ctx.destination);
        this.applyDefaultPreset();
    }

    noteOn(note: number, velocity: number) {
        this.node?.port.postMessage({ type: 'note-on', note, velocity });
    }

    noteOff(note: number) {
        this.node?.port.postMessage({ type: 'note-off', note });
    }

    setParam(id: number, value: number) {
        this.node?.port.postMessage({ type: 'set-param', id, value });
    }

    private applyDefaultPreset() {
        // Warm pad preset
        // Osc A: saw
        this.setParam(PARAM.OSC_A_SAW, 1.0);
        this.setParam(PARAM.OSC_A_LEVEL, 0.7);
        // Osc B: saw, slight detune for width
        this.setParam(PARAM.OSC_B_SAW, 1.0);
        this.setParam(PARAM.OSC_B_LEVEL, 0.7);
        this.setParam(PARAM.OSC_B_FINE, 8);
        // Filter: warm cutoff, moderate resonance
        this.setParam(PARAM.FILTER_CUTOFF, 3000);
        this.setParam(PARAM.FILTER_RESONANCE, 0.8);
        this.setParam(PARAM.FILTER_ENV_AMT, 2000);
        // Filter envelope: medium sweep
        this.setParam(PARAM.FILTER_ATTACK, 0.15);
        this.setParam(PARAM.FILTER_DECAY, 0.5);
        this.setParam(PARAM.FILTER_SUSTAIN, 0.3);
        this.setParam(PARAM.FILTER_RELEASE, 0.8);
        // Amp envelope: pad character
        this.setParam(PARAM.AMP_ATTACK, 0.15);
        this.setParam(PARAM.AMP_DECAY, 0.5);
        this.setParam(PARAM.AMP_SUSTAIN, 0.7);
        this.setParam(PARAM.AMP_RELEASE, 0.8);
        // Effects: chorus + reverb for space
        this.setParam(PARAM.CHORUS_RATE, 0.8);
        this.setParam(PARAM.CHORUS_DEPTH, 0.5);
        this.setParam(PARAM.CHORUS_MIX, 0.3);
        this.setParam(PARAM.REVERB_DECAY, 0.7);
        this.setParam(PARAM.REVERB_DAMPING, 0.6);
        this.setParam(PARAM.REVERB_MIX, 0.25);
        // Master
        this.setParam(PARAM.MASTER_VOL, 0.5);
    }
}

export const PARAM = {
    OSC_A_SAW: 0,
    OSC_A_PULSE: 1,
    OSC_A_PW: 2,
    OSC_B_SAW: 3,
    OSC_B_TRI: 4,
    OSC_B_PULSE: 5,
    OSC_B_PW: 6,
    OSC_B_SEMI: 7,
    OSC_B_FINE: 8,
    OSC_A_LEVEL: 9,
    OSC_B_LEVEL: 10,
    NOISE_LEVEL: 11,
    FILTER_CUTOFF: 12,
    FILTER_RESONANCE: 13,
    FILTER_ENV_AMT: 14,
    FILTER_DRIVE: 15,
    FILTER_ATTACK: 16,
    FILTER_DECAY: 17,
    FILTER_SUSTAIN: 18,
    FILTER_RELEASE: 19,
    AMP_ATTACK: 20,
    AMP_DECAY: 21,
    AMP_SUSTAIN: 22,
    AMP_RELEASE: 23,
    SYNC: 24,
    PM_FILT_ENV: 25,
    PM_OSC_B: 26,
    PM_FREQ_A: 27,
    PM_PW_A: 28,
    PM_FILTER: 29,
    LFO_FREQ: 30,
    LFO_TRI: 31,
    LFO_SAW: 32,
    LFO_SQUARE: 33,
    LFO_AMOUNT: 34,
    WM_MIX: 35,
    WM_FREQ_A: 36,
    WM_FREQ_B: 37,
    WM_PW_A: 38,
    WM_PW_B: 39,
    WM_FILTER: 40,
    MASTER_VOL: 41,
    GLIDE_RATE: 42,
    GLIDE_ON: 43,
    UNISON: 44,
    DRIFT: 45,
    MOD_WHEEL: 46,
    PITCH_BEND: 47,
    CHORUS_RATE: 50,
    CHORUS_DEPTH: 51,
    CHORUS_MIX: 52,
    DELAY_TIME: 53,
    DELAY_FEEDBACK: 54,
    DELAY_TONE: 55,
    DELAY_MIX: 56,
    REVERB_DECAY: 57,
    REVERB_DAMPING: 58,
    REVERB_MIX: 59,
    // Arpeggiator
    ARP_MODE: 60,
    ARP_DIVISION: 61,
    ARP_BPM: 62,
    ARP_OCTAVES: 63,
    ARP_GATE: 64,
    ARP_SWING: 65,
    ARP_HOLD: 66,
    ARP_PANIC: 67,
} as const;
