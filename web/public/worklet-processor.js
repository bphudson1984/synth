class ProphetProcessor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.wasm = null;
        this.ready = false;
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
        }
    }

    process(inputs, outputs, params) {
        if (!this.ready) return true;

        const output = outputs[0];
        const numSamples = output[0].length;

        this.wasm.process(numSamples);

        // Read stereo output from WASM linear memory
        const memory = new Float32Array(this.wasm.memory.buffer);
        const leftPtr = this.wasm.get_left_ptr() / 4;
        const rightPtr = this.wasm.get_right_ptr() / 4;

        output[0].set(memory.subarray(leftPtr, leftPtr + numSamples));
        if (output[1]) {
            output[1].set(memory.subarray(rightPtr, rightPtr + numSamples));
        }

        return true;
    }
}

registerProcessor('prophet-processor', ProphetProcessor);
