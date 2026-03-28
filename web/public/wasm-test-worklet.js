class WasmTestProcessor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.port.onmessage = (e) => {
            const data = e.data;
            if (data.type === 'wasm-module') {
                const moduleType = typeof data.module;
                const isModule = data.module instanceof WebAssembly.Module;
                this.port.postMessage({ type: 'module-received', moduleType, isModule });
                
                WebAssembly.instantiate(data.module, {}).then(instance => {
                    const exports = Object.keys(instance.exports);
                    this.port.postMessage({ type: 'instantiated', exports });
                }).catch(err => {
                    this.port.postMessage({ type: 'instantiate-error', error: err.message });
                });
            }
        };
    }
    process() { return true; }
}
registerProcessor('wasm-test-processor', WasmTestProcessor);
