let initPromise: Promise<AudioContext> | null = null;

export function getAudioContext(): Promise<AudioContext> {
    if (!initPromise) {
        initPromise = (async () => {
            const ctx = new AudioContext({ sampleRate: 48000 });

            // iOS Safari (and Chrome on iOS, which uses WebKit) may create
            // the AudioContext in a "suspended" state even inside a user-gesture
            // handler.  Explicitly resuming guarantees audio output.
            if (ctx.state === 'suspended') {
                await ctx.resume();
            }

            const base = import.meta.env.BASE_URL;
            // Load ALL worklet modules sequentially in one place
            // to avoid race conditions from parallel addModule calls
            // Single worklet file registers all processors
            await ctx.audioWorklet.addModule(base + 'synth-worklets.js');

            // Re-resume the context when the page becomes visible again.
            // iOS suspends the AudioContext when the browser tab is backgrounded.
            document.addEventListener('visibilitychange', () => {
                if (document.visibilityState === 'visible' && ctx.state === 'suspended') {
                    ctx.resume();
                }
            });

            return ctx;
        })();
    }
    return initPromise;
}
