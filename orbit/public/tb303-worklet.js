class TB303Processor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.wasm = null;
        this.ready = false;
        this.lastStep = -1;
        this.port.onmessage = (e) => this.handleMessage(e.data);
    }

    handleMessage(data) {
        if (data.type === 'wasm-bytes') {
            // Compile + instantiate from raw bytes (works in all browsers)
            WebAssembly.instantiate(data.bytes, {}).then(result => {
                this.wasm = result.instance.exports;
                this.wasm.init(sampleRate);
                this.ready = true;
                this.port.postMessage({ type: 'ready' });
            }).catch(err => {
                this.port.postMessage({ type: 'error', message: err.message });
            });
        } else if (data.type === 'wasm-module') {
            // Legacy path: pre-compiled module (may fail in Chrome AudioWorklet)
            WebAssembly.instantiate(data.module, {}).then(instance => {
                this.wasm = instance.exports;
                this.wasm.init(sampleRate);
                this.ready = true;
                this.port.postMessage({ type: 'ready' });
            });
        } else if (data.type === 'set-param') {
            if (this.ready) this.wasm.set_param(data.id, data.value);
        } else if (data.type === 'seq-play') {
            if (this.ready) this.wasm.seq_play();
        } else if (data.type === 'seq-stop') {
            if (this.ready) this.wasm.seq_stop();
        } else if (data.type === 'seq-bpm') {
            if (this.ready) this.wasm.seq_set_bpm(data.value);
        } else if (data.type === 'seq-set-step-note') {
            if (this.ready) this.wasm.seq_set_step_note(data.step, data.note);
        } else if (data.type === 'seq-set-step-gate') {
            if (this.ready) this.wasm.seq_set_step_gate(data.step, data.gate ? 1 : 0);
        } else if (data.type === 'seq-set-step-accent') {
            if (this.ready) this.wasm.seq_set_step_accent(data.step, data.accent ? 1 : 0);
        } else if (data.type === 'seq-set-step-slide') {
            if (this.ready) this.wasm.seq_set_step_slide(data.step, data.slide ? 1 : 0);
        } else if (data.type === 'seq-clear') {
            if (this.ready) this.wasm.seq_clear();
        }
    }

    process(inputs, outputs) {
        if (!this.ready) return true;
        const output = outputs[0];
        const n = output[0].length;
        this.wasm.process(n);
        const memory = new Float32Array(this.wasm.memory.buffer);
        const lp = this.wasm.get_left_ptr() / 4;
        const rp = this.wasm.get_right_ptr() / 4;
        output[0].set(memory.subarray(lp, lp + n));
        if (output[1]) output[1].set(memory.subarray(rp, rp + n));

        const step = this.wasm.seq_get_current_step();
        if (step !== this.lastStep) {
            this.lastStep = step;
            this.port.postMessage({ type: 'step', step });
        }
        return true;
    }
}

registerProcessor('tb303-processor', TB303Processor);
