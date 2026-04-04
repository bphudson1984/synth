class ProphetProcessor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.wasm = null;
        this.ready = false;
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
            case 'note-on':
                if (this.ready) this.wasm.note_on(data.note, data.velocity);
                break;
            case 'note-off':
                if (this.ready) this.wasm.note_off(data.note);
                break;
            case 'set-param':
                if (this.ready) this.wasm.set_param(data.id, data.value);
                break;
            case 'set-fx-order':
                if (this.ready) this.wasm.set_fx_order(data.order[0], data.order[1], data.order[2], data.order[3], data.order[4]);
                break;
        }
    }

    process(inputs, outputs, params) {
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

        return true;
    }
}

registerProcessor('prophet-processor', ProphetProcessor);
