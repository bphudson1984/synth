class TR808Processor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.wasm = null;
        this.ready = false;
        this.port.onmessage = (e) => this.handleMessage(e.data);
    }

    handleMessage(data) {
        if (data.type === 'wasm-module') {
            WebAssembly.instantiate(data.module, {}).then(instance => {
                this.wasm = instance.exports;
                this.wasm.init(sampleRate);
                this.ready = true;
                this.port.postMessage({ type: 'ready' });
            }).catch(err => {
                this.port.postMessage({ type: 'error', message: err.message });
            });
        } else if (data.type === 'trigger') {
            if (this.ready) this.wasm.trigger(data.voice);
        } else if (data.type === 'set-param') {
            if (this.ready) this.wasm.set_param(data.voice, data.param, data.value);
        } else if (data.type === 'seq-play') {
            if (this.ready) this.wasm.seq_play();
        } else if (data.type === 'seq-stop') {
            if (this.ready) this.wasm.seq_stop();
        } else if (data.type === 'seq-bpm') {
            if (this.ready) this.wasm.seq_set_bpm(data.value);
        } else if (data.type === 'seq-swing') {
            if (this.ready) this.wasm.seq_set_swing(data.value);
        } else if (data.type === 'seq-toggle') {
            if (this.ready) this.wasm.seq_toggle_step(data.track, data.step);
        } else if (data.type === 'seq-clear') {
            if (this.ready) this.wasm.seq_clear();
        } else if (data.type === 'set-track-engine') {
            if (this.ready) this.wasm.set_track_engine(data.track, data.is909 ? 1 : 0);
        } else if (data.type === 'set-all-engines') {
            if (this.ready) this.wasm.set_all_engines(data.is909 ? 1 : 0);
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

registerProcessor('tr808-processor', TR808Processor);
