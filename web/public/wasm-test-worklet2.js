class WasmTest2Processor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.port.onmessage = (e) => {
            const data = e.data;
            if (data.type === 'wasm-bytes') {
                const bytesType = typeof data.bytes;
                const isArrayBuffer = data.bytes instanceof ArrayBuffer;
                const byteLength = data.bytes.byteLength;
                this.port.postMessage({ type: 'bytes-received', bytesType, isArrayBuffer, byteLength });
                
                WebAssembly.instantiate(data.bytes, {}).then(result => {
                    const exports = Object.keys(result.instance.exports);
                    this.port.postMessage({ type: 'instantiated', exports });
                }).catch(err => {
                    this.port.postMessage({ type: 'instantiate-error', error: err.message });
                });
            }
        };
    }
    process() { return true; }
}
registerProcessor('wasm-test2-processor', WasmTest2Processor);
