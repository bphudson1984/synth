class TR808Processor extends AudioWorkletProcessor {
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
        if (data.type === 'wasm-bytes') {
            // Compile + instantiate from raw bytes (works in all browsers)
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
        } else if (data.type === 'wasm-module') {
            // Legacy path: pre-compiled module (may fail in Chrome AudioWorklet)
            WebAssembly.instantiate(data.module, {}).then(instance => {
                this.wasm = instance.exports;
                this.wasm.init(sampleRate);
                this.ready = true;
                this.port.postMessage({ type: 'ready' });
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
        }
    }

    process(inputs, outputs) {
        if (!this.ready) return true;
        const output = outputs[0];
        const needed = output[0].length;

        if (this.doubleBuffer) {
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

registerProcessor('tr808-processor', TR808Processor);
