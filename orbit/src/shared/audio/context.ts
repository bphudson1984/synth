let initPromise: Promise<AudioContext> | null = null;

export function getAudioContext(): Promise<AudioContext> {
    if (!initPromise) {
        initPromise = (async () => {
            const ctx = new AudioContext({ sampleRate: 48000 });
            const base = import.meta.env.BASE_URL;
            // Load ALL worklet modules sequentially in one place
            // to avoid race conditions from parallel addModule calls
            await ctx.audioWorklet.addModule(base + 'worklet-processor.js');
            await ctx.audioWorklet.addModule(base + 'prophet-worklet.js');
            await ctx.audioWorklet.addModule(base + 'tb303-worklet.js');
            return ctx;
        })().catch((err) => {
            // Reset so the next user tap can retry
            initPromise = null;
            throw err;
        });
    }
    return initPromise;
}
