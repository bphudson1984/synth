/**
 * All AudioWorklet processors for the synth project.
 * Combined into a single file because AudioWorklet modules loaded via
 * separate addModule() calls don't reliably share globals across browsers.
 */

// --- Factory ---
function createSynthProcessor(name, extraHandler) {
    class SynthProcessor extends AudioWorkletProcessor {
        constructor() {
            super();
            this.wasm = null;
            this.ready = false;
            this.lastStep = -1;
            this.memoryBuf = null;
            this.memoryView = null;
            this.doubleBuffer = false;
            this.bufL = null;
            this.bufR = null;
            this.bufReady = 0;
            this.bufRead = 0;
            this.port.onmessage = (e) => this.handleMessage(e.data);
        }

        handleMessage(data) {
            if (!this.ready && data.type !== 'wasm-bytes') return;

            switch (data.type) {
                case 'wasm-bytes':
                    WebAssembly.instantiate(data.bytes, {}).then(result => {
                        this.wasm = result.instance.exports;
                        this.wasm.init(sampleRate);
                        this.memoryBuf = this.wasm.memory.buffer;
                        this.memoryView = new Float32Array(this.memoryBuf);
                        this.doubleBuffer = !!data.useDoubleBuffer;
                        if (this.doubleBuffer) {
                            this.bufL = new Float32Array(512);
                            this.bufR = new Float32Array(512);
                        }
                        this.ready = true;
                        this.port.postMessage({ type: 'ready' });
                    }).catch(err => {
                        this.port.postMessage({ type: 'error', message: err.message });
                    });
                    break;
                case 'set-param':
                    if (data.voice !== undefined) {
                        this.wasm.set_param(data.voice, data.param, data.value);
                    } else {
                        this.wasm.set_param(data.id, data.value);
                    }
                    break;
                case 'note-on': this.wasm.note_on(data.note, data.velocity); break;
                case 'note-off': this.wasm.note_off(data.note); break;
                case 'seq-play': this.wasm.seq_play(); break;
                case 'seq-stop': this.wasm.seq_stop(); break;
                case 'seq-bpm': this.wasm.seq_set_bpm(data.value); break;
                case 'seq-clear': this.wasm.seq_clear(); break;
                case 'seq-set-glitch': this.wasm.seq_set_glitch(data.value); break;
                default:
                    if (extraHandler) extraHandler(this.wasm, data);
                    break;
            }
        }

        process(inputs, outputs) {
            if (!this.ready) return true;
            const output = outputs[0];
            const needed = output[0].length;

            if (this.doubleBuffer) {
                // Firefox path: batch WASM calls, copy from pre-rendered buffer
                if (this.bufReady < needed) {
                    this.wasm.process(512);
                    if (this.memoryBuf !== this.wasm.memory.buffer) {
                        this.memoryBuf = this.wasm.memory.buffer;
                        this.memoryView = new Float32Array(this.memoryBuf);
                    }
                    const lp = this.wasm.get_left_ptr() / 4;
                    const rp = this.wasm.get_right_ptr() / 4;
                    this.bufL.set(this.memoryView.subarray(lp, lp + 512));
                    this.bufR.set(this.memoryView.subarray(rp, rp + 512));
                    this.bufRead = 0;
                    this.bufReady = 512;
                }
                const r = this.bufRead;
                output[0].set(this.bufL.subarray(r, r + needed));
                if (output[1]) output[1].set(this.bufR.subarray(r, r + needed));
                this.bufRead += needed;
                this.bufReady -= needed;
            } else {
                // Chrome path: direct processing (low latency)
                const n = Math.min(needed, 256);
                this.wasm.process(n);
                if (this.memoryBuf !== this.wasm.memory.buffer) {
                    this.memoryBuf = this.wasm.memory.buffer;
                    this.memoryView = new Float32Array(this.memoryBuf);
                }
                const lp = this.wasm.get_left_ptr() / 4;
                const rp = this.wasm.get_right_ptr() / 4;
                output[0].set(this.memoryView.subarray(lp, lp + n));
                if (output[1]) output[1].set(this.memoryView.subarray(rp, rp + n));
            }

            const step = this.wasm.seq_get_current_step();
            if (step !== this.lastStep) {
                this.lastStep = step;
                this.port.postMessage({ type: 'step', step });
            }
            return true;
        }
    }

    registerProcessor(name, SynthProcessor);
}

// --- Melodic sequencer handler (shared by Prophet + Braids) ---
function melodicSeqHandler(wasm, data) {
    switch (data.type) {
        case 'seq-set-step-notes': wasm.seq_set_step_notes(data.step, data.num, data.n1, data.n2, data.n3, data.n4); break;
        case 'seq-set-step-gate': wasm.seq_set_step_gate(data.step, data.gate ? 1 : 0); break;
        case 'seq-set-step-velocity': wasm.seq_set_step_velocity(data.step, data.value); break;
        case 'seq-set-step-gate-pct': wasm.seq_set_step_gate_pct(data.step, data.value); break;
        case 'seq-set-step-probability': wasm.seq_set_step_probability(data.step, data.value); break;
        case 'seq-set-step-ratchet': wasm.seq_set_step_ratchet(data.step, data.value); break;
        case 'seq-set-step-skip': wasm.seq_set_step_skip(data.step, data.value ? 1 : 0); break;
        case 'seq-set-direction': wasm.seq_set_direction(data.value); break;
        case 'seq-set-swing': wasm.seq_set_swing(data.value); break;
        case 'seq-set-time-div': wasm.seq_set_time_div(data.value); break;
        case 'seq-set-length': wasm.seq_set_length(data.value); break;
        case 'seq-set-external': wasm.seq_set_external(data.value ? 1 : 0); break;
        case 'seq-rotate': wasm.seq_rotate(data.value); break;
    }
}

// --- Register all processors ---

createSynthProcessor('prophet-processor', melodicSeqHandler);

createSynthProcessor('braids-processor', melodicSeqHandler);

createSynthProcessor('volca-bass-processor', melodicSeqHandler);

// --- Sampler processor (custom: handles sample data loading + melodic seq) ---

class SamplerProcessor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.wasm = null;
        this.ready = false;
        this.lastStep = -1;
        this.memoryBuf = null;
        this.memoryView = null;
        this.port.onmessage = (e) => this.handleMessage(e.data);
    }

    handleMessage(data) {
        if (!this.ready && data.type !== 'wasm-bytes') return;

        switch (data.type) {
            case 'wasm-bytes':
                WebAssembly.instantiate(data.bytes, {}).then(result => {
                    this.wasm = result.instance.exports;
                    this.wasm.init(sampleRate);
                    this.memoryBuf = this.wasm.memory.buffer;
                    this.memoryView = new Float32Array(this.memoryBuf);
                    this.ready = true;
                    this.port.postMessage({ type: 'ready' });
                }).catch(err => {
                    this.port.postMessage({ type: 'error', message: err.message });
                });
                break;

            // Sample loading: allocate WASM buffer, copy data, register
            case 'load-sample': {
                const left = new Float32Array(data.left);
                const right = new Float32Array(data.right);
                const len = left.length;
                // Allocate two buffers in WASM memory
                const leftPtr = this.wasm.alloc_sample_buffer(len);
                const rightPtr = this.wasm.alloc_sample_buffer(len);
                // Refresh memory view (alloc may have grown memory)
                if (this.memoryBuf !== this.wasm.memory.buffer) {
                    this.memoryBuf = this.wasm.memory.buffer;
                    this.memoryView = new Float32Array(this.memoryBuf);
                }
                // Copy PCM data into WASM memory
                this.memoryView.set(left, leftPtr / 4);
                this.memoryView.set(right, rightPtr / 4);
                // Register with engine
                this.wasm.load_sample(data.pad, leftPtr, rightPtr, len, data.sampleRate);
                this.port.postMessage({ type: 'sample-loaded', pad: data.pad });
                break;
            }

            // Trigger / release
            case 'trigger': this.wasm.trigger(data.pad); break;
            case 'release': this.wasm.release(data.pad); break;
            case 'stop-pad': this.wasm.stop(data.pad); break;

            // Per-pad params
            case 'set-pad-param': this.wasm.set_pad_param(data.pad, data.param, data.value); break;

            // Global params
            case 'set-param':
                if (data.voice !== undefined) {
                    this.wasm.set_param(data.voice, data.param, data.value);
                } else {
                    this.wasm.set_param(data.id, data.value);
                }
                break;

            // Note on/off (for sequencer / MIDI)
            case 'note-on': this.wasm.note_on(data.note, data.velocity); break;
            case 'note-off': this.wasm.note_off(data.note); break;

            // Sequencer (reuse melodic handler cases)
            case 'seq-play': this.wasm.seq_play(); break;
            case 'seq-stop': this.wasm.seq_stop(); break;
            case 'seq-bpm': this.wasm.seq_set_bpm(data.value); break;
            case 'seq-clear': this.wasm.seq_clear(); break;
            case 'seq-set-glitch': this.wasm.seq_set_glitch(data.value); break;

            default:
                // Melodic sequencer messages
                melodicSeqHandler(this.wasm, data);
                break;
        }
    }

    process(inputs, outputs) {
        if (!this.ready) return true;
        const output = outputs[0];
        const n = Math.min(output[0].length, 256);

        this.wasm.process(n);
        if (this.memoryBuf !== this.wasm.memory.buffer) {
            this.memoryBuf = this.wasm.memory.buffer;
            this.memoryView = new Float32Array(this.memoryBuf);
        }
        const lp = this.wasm.get_left_ptr() / 4;
        const rp = this.wasm.get_right_ptr() / 4;
        output[0].set(this.memoryView.subarray(lp, lp + n));
        if (output[1]) output[1].set(this.memoryView.subarray(rp, rp + n));

        const step = this.wasm.seq_get_current_step();
        if (step !== this.lastStep) {
            this.lastStep = step;
            this.port.postMessage({ type: 'step', step });
        }
        return true;
    }
}

registerProcessor('sampler-processor', SamplerProcessor);

createSynthProcessor('tb303-processor', (wasm, data) => {
    switch (data.type) {
        case 'seq-set-step-note': wasm.seq_set_step_note(data.step, data.note); break;
        case 'seq-set-step-gate': wasm.seq_set_step_gate(data.step, data.gate ? 1 : 0); break;
        case 'seq-set-step-accent': wasm.seq_set_step_accent(data.step, data.accent ? 1 : 0); break;
        case 'seq-set-step-slide': wasm.seq_set_step_slide(data.step, data.slide ? 1 : 0); break;
        case 'seq-set-direction': wasm.seq_set_direction(data.value); break;
        case 'seq-set-swing': wasm.seq_set_swing(data.value); break;
        case 'seq-set-time-div': wasm.seq_set_time_div(data.value); break;
    }
});

createSynthProcessor('tr808-processor', (wasm, data) => {
    switch (data.type) {
        case 'trigger': wasm.trigger(data.voice); break;
        case 'seq-toggle': wasm.seq_toggle_step(data.track, data.step); break;
        case 'seq-swing': wasm.seq_set_swing(data.value); break;
        case 'seq-set-time-div': wasm.seq_set_time_div(data.value); break;
        case 'seq-set-length': wasm.seq_set_length(data.value); break;
        case 'set-track-engine': wasm.set_track_engine(data.track, data.is909 ? 1 : 0); break;
        case 'set-all-engines': wasm.set_all_engines(data.is909 ? 1 : 0); break;
    }
});

// --- FX Rack processor (5 stereo inputs → 1 stereo output) ---

class FxProcessor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.wasm = null;
        this.ready = false;
        this.memoryBuf = null;
        this.memoryView = null;
        this.ptrs = null;
        this.port.onmessage = (e) => this.handleMessage(e.data);
    }

    handleMessage(data) {
        if (data.type === 'wasm-bytes') {
            WebAssembly.instantiate(data.bytes, {}).then(result => {
                this.wasm = result.instance.exports;
                this.wasm.init(sampleRate);
                this.memoryBuf = this.wasm.memory.buffer;
                this.memoryView = new Float32Array(this.memoryBuf);
                // Cache buffer pointers (stable after init)
                this.ptrs = [
                    [this.wasm.get_chorus_in_l_ptr() / 4, this.wasm.get_chorus_in_r_ptr() / 4],
                    [this.wasm.get_delay_in_l_ptr() / 4,  this.wasm.get_delay_in_r_ptr() / 4],
                    [this.wasm.get_reverb_in_l_ptr() / 4, this.wasm.get_reverb_in_r_ptr() / 4],
                    [this.wasm.get_dist_in_l_ptr() / 4,   this.wasm.get_dist_in_r_ptr() / 4],
                    [this.wasm.get_octave_in_l_ptr() / 4, this.wasm.get_octave_in_r_ptr() / 4],
                ];
                this.outL = this.wasm.get_out_l_ptr() / 4;
                this.outR = this.wasm.get_out_r_ptr() / 4;
                this.ready = true;
                this.port.postMessage({ type: 'ready' });
            }).catch(err => {
                this.port.postMessage({ type: 'error', message: err.message });
            });
        } else if (data.type === 'set-param' && this.ready) {
            this.wasm.set_param(data.effectId, data.paramId, data.value);
        }
    }

    process(inputs, outputs) {
        if (!this.ready) return true;
        const out = outputs[0];
        if (!out || !out[0]) return true;
        const n = out[0].length;

        // Refresh memory view if WASM memory grew
        if (this.memoryBuf !== this.wasm.memory.buffer) {
            this.memoryBuf = this.wasm.memory.buffer;
            this.memoryView = new Float32Array(this.memoryBuf);
        }

        // Copy 5 stereo inputs into WASM buffers
        for (let i = 0; i < 5; i++) {
            const inp = inputs[i];
            const [ptrL, ptrR] = this.ptrs[i];
            if (inp && inp.length > 0 && inp[0] && inp[0].length > 0) {
                this.memoryView.set(inp[0].subarray(0, n), ptrL);
                if (inp[1] && inp[1].length > 0) {
                    this.memoryView.set(inp[1].subarray(0, n), ptrR);
                } else {
                    this.memoryView.set(inp[0].subarray(0, n), ptrR);
                }
            } else {
                this.memoryView.fill(0, ptrL, ptrL + n);
                this.memoryView.fill(0, ptrR, ptrR + n);
            }
        }

        // Process all effects
        this.wasm.process(n);

        // Copy output
        out[0].set(this.memoryView.subarray(this.outL, this.outL + n));
        if (out[1]) out[1].set(this.memoryView.subarray(this.outR, this.outR + n));

        return true;
    }
}

registerProcessor('fx-processor', FxProcessor);
