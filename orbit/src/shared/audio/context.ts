let initPromise: Promise<AudioContext> | null = null;

export function getAudioContext(): Promise<AudioContext> {
    if (!initPromise) {
        initPromise = (async () => {
            const isFirefox = /Firefox/.test(navigator.userAgent);
            const ctx = new AudioContext({ sampleRate: 48000, latencyHint: isFirefox ? 0.08 : 'playback' });
            const base = import.meta.env.BASE_URL;
            // Load ALL worklet modules sequentially in one place
            // to avoid race conditions from parallel addModule calls
            // Single worklet file registers all processors
            await ctx.audioWorklet.addModule(base + 'synth-worklets.js');
            return ctx;
        })();
    }
    return initPromise;
}
