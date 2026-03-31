let initPromise: Promise<AudioContext> | null = null;

export function getAudioContext(): Promise<AudioContext> {
    if (!initPromise) {
        initPromise = (async () => {
            const ctx = new AudioContext({ sampleRate: 48000 });
            // iOS Safari requires an explicit resume() during a user gesture —
            // without it the context stays "suspended" and produces no audio.
            if (ctx.state === 'suspended') await ctx.resume();
            const base = import.meta.env.BASE_URL;
            // Load ALL worklet modules sequentially in one place
            // to avoid race conditions from parallel addModule calls
            await ctx.audioWorklet.addModule(base + 'worklet-processor.js');
            await ctx.audioWorklet.addModule(base + 'prophet-worklet.js');
            await ctx.audioWorklet.addModule(base + 'tb303-worklet.js');
            await ctx.audioWorklet.addModule(base + 'braids-worklet.js');
            return ctx;
        })();
    }
    return initPromise;
}
