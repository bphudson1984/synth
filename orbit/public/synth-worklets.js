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
