class TestProcessor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.port.onmessage = (e) => {
            this.port.postMessage({ type: 'echo', data: e.data });
        };
        this.port.postMessage({ type: 'constructed' });
    }
    process() { return true; }
}
registerProcessor('test-processor', TestProcessor);
