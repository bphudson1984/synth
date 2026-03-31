class ProphetProcessor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.wasm = null;
        this.ready = false;
        this.lastStep = -1;
        this.port.onmessage = (e) => this.handleMessage(e.data);
    }

    handleMessage(data) {
        switch (data.type) {
            case 'wasm-bytes':
                WebAssembly.instantiate(data.bytes, {}).then(result => {
                    this.wasm = result.instance.exports;
                    this.wasm.init(sampleRate);
                    this.ready = true;
                    this.port.postMessage({ type: 'ready' });
                }).catch(err => {
                    this.port.postMessage({ type: 'error', message: err.message });
                });
                break;
            case 'note-on':
                if (this.ready) this.wasm.note_on(data.note, data.velocity);
                break;
            case 'note-off':
                if (this.ready) this.wasm.note_off(data.note);
                break;
            case 'set-param':
                if (this.ready) this.wasm.set_param(data.id, data.value);
                break;
            // --- Sequencer ---
            case 'seq-play': if (this.ready) this.wasm.seq_play(); break;
            case 'seq-stop': if (this.ready) this.wasm.seq_stop(); break;
            case 'seq-bpm': if (this.ready) this.wasm.seq_set_bpm(data.value); break;
            case 'seq-clear': if (this.ready) this.wasm.seq_clear(); break;
            case 'seq-set-length': if (this.ready) this.wasm.seq_set_length(data.value); break;
            case 'seq-set-external': if (this.ready) this.wasm.seq_set_external(data.value ? 1 : 0); break;
            case 'seq-set-step-notes':
                if (this.ready) this.wasm.seq_set_step_notes(data.step, data.num, data.n1, data.n2, data.n3, data.n4);
                break;
            case 'seq-set-step-gate': if (this.ready) this.wasm.seq_set_step_gate(data.step, data.gate ? 1 : 0); break;
            case 'seq-set-step-velocity': if (this.ready) this.wasm.seq_set_step_velocity(data.step, data.value); break;
            case 'seq-set-step-gate-pct': if (this.ready) this.wasm.seq_set_step_gate_pct(data.step, data.value); break;
            case 'seq-set-step-probability': if (this.ready) this.wasm.seq_set_step_probability(data.step, data.value); break;
            case 'seq-set-step-ratchet': if (this.ready) this.wasm.seq_set_step_ratchet(data.step, data.value); break;
            case 'seq-set-step-skip': if (this.ready) this.wasm.seq_set_step_skip(data.step, data.value ? 1 : 0); break;
            case 'seq-set-direction': if (this.ready) this.wasm.seq_set_direction(data.value); break;
            case 'seq-set-swing': if (this.ready) this.wasm.seq_set_swing(data.value); break;
            case 'seq-set-time-div': if (this.ready) this.wasm.seq_set_time_div(data.value); break;
            case 'seq-rotate': if (this.ready) this.wasm.seq_rotate(data.value); break;
        }
    }

    process(inputs, outputs) {
        if (!this.ready) return true;
        const output = outputs[0];
        const numSamples = output[0].length;
        this.wasm.process(numSamples);
        const memory = new Float32Array(this.wasm.memory.buffer);
        const leftPtr = this.wasm.get_left_ptr() / 4;
        const rightPtr = this.wasm.get_right_ptr() / 4;
        output[0].set(memory.subarray(leftPtr, leftPtr + numSamples));
        if (output[1]) output[1].set(memory.subarray(rightPtr, rightPtr + numSamples));

        const step = this.wasm.seq_get_current_step();
        if (step !== this.lastStep) {
            this.lastStep = step;
            this.port.postMessage({ type: 'step', step });
        }
        return true;
    }
}

registerProcessor('prophet-processor', ProphetProcessor);
