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
            });
        } else if (data.type === 'trigger') {
            if (this.ready) this.wasm.trigger(data.voice);
        } else if (data.type === 'set-param') {
            if (this.ready) this.wasm.set_param(data.voice, data.param, data.value);
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
        return true;
    }
}

registerProcessor('tr808-processor', TR808Processor);
