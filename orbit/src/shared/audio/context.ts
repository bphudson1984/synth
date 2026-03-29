let ctx: AudioContext | null = null;

export async function getAudioContext(): Promise<AudioContext> {
    if (!ctx) ctx = new AudioContext({ sampleRate: 48000 });
    return ctx;
}
