let initPromise: Promise<AudioContext> | null = null;
let limiterNode: DynamicsCompressorNode | null = null;

export function getAudioContext(): Promise<AudioContext> {
    if (!initPromise) {
        initPromise = (async () => {
            // Use the device's native sample rate to avoid resampling overhead on mobile
            const ctx = new AudioContext();
            const base = import.meta.env.BASE_URL;
            // Load ALL worklet modules sequentially in one place
            // to avoid race conditions from parallel addModule calls
            // Single worklet file registers all processors
            await ctx.audioWorklet.addModule(base + 'synth-worklets.js');

            // Insert a limiter (compressor as brick-wall) before destination
            // to prevent clipping when multiple channels sum above 1.0
            limiterNode = ctx.createDynamicsCompressor();
            limiterNode.threshold.value = -3;
            limiterNode.knee.value = 3;
            limiterNode.ratio.value = 20;
            limiterNode.attack.value = 0.001;
            limiterNode.release.value = 0.05;
            limiterNode.connect(ctx.destination);

            return ctx;
        })();
    }
    return initPromise;
}

/** Returns the limiter node that all engines should connect to instead of destination */
export function getLimiterNode(): DynamicsCompressorNode | null {
    return limiterNode;
}
