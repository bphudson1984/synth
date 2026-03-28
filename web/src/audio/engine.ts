export class AudioEngine {
    private ctx: AudioContext | null = null;
    private node: AudioWorkletNode | null = null;
    private _ready = false;

    get ready() { return this._ready; }

    async init(): Promise<void> {
        this.ctx = new AudioContext({ sampleRate: 48000 });

        // Compile WASM module on main thread
        const wasmModule = await WebAssembly.compileStreaming(
            fetch(import.meta.env.BASE_URL + 'prophet-dsp.wasm')
        );

        // Load worklet processor
        await this.ctx.audioWorklet.addModule(import.meta.env.BASE_URL + 'worklet-processor.js');

        // Create node with stereo output
        this.node = new AudioWorkletNode(this.ctx, 'prophet-processor', {
            outputChannelCount: [2],
            numberOfOutputs: 1,
        });

        // Wait for WASM to initialize in the worklet
        await new Promise<void>((resolve) => {
            this.node!.port.onmessage = (e) => {
                if (e.data.type === 'ready') {
                    this._ready = true;
                    resolve();
                }
            };
            // Send the compiled module to the worklet
            this.node!.port.postMessage({ type: 'wasm-module', module: wasmModule });
        });

        this.node.connect(this.ctx.destination);
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

    async resume() {
        await this.ctx?.resume();
    }
}

// Parameter ID constants matching the Rust set_param function
export const PARAM = {
    // Oscillator A
    OSC_A_SAW: 0,
    OSC_A_PULSE: 1,
    OSC_A_PW: 2,
    // Oscillator B
    OSC_B_SAW: 3,
    OSC_B_TRI: 4,
    OSC_B_PULSE: 5,
    OSC_B_PW: 6,
    OSC_B_SEMI: 7,
    OSC_B_FINE: 8,
    // Mixer
    OSC_A_LEVEL: 9,
    OSC_B_LEVEL: 10,
    NOISE_LEVEL: 11,
    // Filter
    FILTER_CUTOFF: 12,
    FILTER_RESONANCE: 13,
    FILTER_ENV_AMT: 14,
    FILTER_DRIVE: 15,
    // Filter Envelope
    FILTER_ATTACK: 16,
    FILTER_DECAY: 17,
    FILTER_SUSTAIN: 18,
    FILTER_RELEASE: 19,
    // Amp Envelope
    AMP_ATTACK: 20,
    AMP_DECAY: 21,
    AMP_SUSTAIN: 22,
    AMP_RELEASE: 23,
    // Sync
    SYNC: 24,
    // Poly Mod
    PM_FILT_ENV: 25,
    PM_OSC_B: 26,
    PM_FREQ_A: 27,
    PM_PW_A: 28,
    PM_FILTER: 29,
    // LFO
    LFO_FREQ: 30,
    LFO_TRI: 31,
    LFO_SAW: 32,
    LFO_SQUARE: 33,
    LFO_AMOUNT: 34,
    // Wheel Mod
    WM_MIX: 35,
    WM_FREQ_A: 36,
    WM_FREQ_B: 37,
    WM_PW_A: 38,
    WM_PW_B: 39,
    WM_FILTER: 40,
    // Master
    MASTER_VOL: 41,
    GLIDE_RATE: 42,
    GLIDE_ON: 43,
    UNISON: 44,
    DRIFT: 45,
    MOD_WHEEL: 46,
    PITCH_BEND: 47,
    // Chorus
    CHORUS_RATE: 50,
    CHORUS_DEPTH: 51,
    CHORUS_MIX: 52,
    // Delay
    DELAY_TIME: 53,
    DELAY_FEEDBACK: 54,
    DELAY_TONE: 55,
    DELAY_MIX: 56,
    // Reverb
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
