import { BaseEngine } from '../../shared/audio/BaseEngine';
import { getAudioContext } from '../../shared/audio/context';

export class SamplerEngine extends BaseEngine {
    async init(): Promise<void> {
        await this.initWorklet('sampler-processor', 'sampler.wasm');
    }

    /** Load a decoded AudioBuffer into a pad slot. */
    async loadSample(padIndex: number, file: File): Promise<Float32Array> {
        const ctx = await getAudioContext();
        const arrayBuffer = await file.arrayBuffer();
        const audioBuffer = await ctx.decodeAudioData(arrayBuffer);

        // Extract PCM — keep a copy for waveform display before transferring
        const leftOrig = audioBuffer.getChannelData(0);
        const waveformCopy = new Float32Array(leftOrig); // for UI

        const left = new Float32Array(leftOrig);
        const right = audioBuffer.numberOfChannels > 1
            ? new Float32Array(audioBuffer.getChannelData(1))
            : new Float32Array(leftOrig);

        this.node?.port.postMessage({
            type: 'load-sample',
            pad: padIndex,
            left: left.buffer,
            right: right.buffer,
            sampleRate: audioBuffer.sampleRate,
            length: audioBuffer.length,
        }, [left.buffer, right.buffer]); // transferable — zero-copy to worklet

        return waveformCopy;
    }

    /** Load raw PCM data (from IndexedDB restore). */
    loadSampleFromPCM(padIndex: number, left: Float32Array, right: Float32Array, sampleRate: number) {
        const leftCopy = new Float32Array(left);
        const rightCopy = new Float32Array(right);
        this.node?.port.postMessage({
            type: 'load-sample',
            pad: padIndex,
            left: leftCopy.buffer,
            right: rightCopy.buffer,
            sampleRate,
            length: left.length,
        }, [leftCopy.buffer, rightCopy.buffer]);
    }

    /** Load from a pre-fetched ArrayBuffer (for stock kits, IndexedDB restore, etc.) */
    async loadSampleFromArrayBuffer(padIndex: number, arrayBuffer: ArrayBuffer): Promise<Float32Array> {
        const ctx = await getAudioContext();
        const audioBuffer = await ctx.decodeAudioData(arrayBuffer);
        const leftOrig = audioBuffer.getChannelData(0);
        const waveformCopy = new Float32Array(leftOrig);
        const left = new Float32Array(leftOrig);
        const right = audioBuffer.numberOfChannels > 1
            ? new Float32Array(audioBuffer.getChannelData(1))
            : new Float32Array(leftOrig);
        this.node?.port.postMessage({
            type: 'load-sample', pad: padIndex,
            left: left.buffer, right: right.buffer,
            sampleRate: audioBuffer.sampleRate, length: audioBuffer.length,
        }, [left.buffer, right.buffer]);
        return waveformCopy;
    }

    trigger(pad: number) { this.node?.port.postMessage({ type: 'trigger', pad }); }
    release(pad: number) { this.node?.port.postMessage({ type: 'release', pad }); }
    stopPad(pad: number) { this.node?.port.postMessage({ type: 'stop-pad', pad }); }

    setPadParam(pad: number, param: number, value: number) {
        this.node?.port.postMessage({ type: 'set-pad-param', pad, param, value });
    }

    noteOn(note: number, velocity: number) { this.node?.port.postMessage({ type: 'note-on', note, velocity }); }
    noteOff(note: number) { this.node?.port.postMessage({ type: 'note-off', note }); }

    // Melodic sequencer methods (same as Prophet/Braids/Bass)
    setStepNotes(step: number, notes: number[]) {
        this.node?.port.postMessage({
            type: 'seq-set-step-notes', step,
            num: notes.length, n1: notes[0] ?? 0, n2: notes[1] ?? 0, n3: notes[2] ?? 0, n4: notes[3] ?? 0,
        });
    }
    setStepGate(step: number, gate: boolean) { this.node?.port.postMessage({ type: 'seq-set-step-gate', step, gate }); }
    setStepVelocity(step: number, vel: number) { this.node?.port.postMessage({ type: 'seq-set-step-velocity', step, value: vel }); }
    setStepGatePct(step: number, pct: number) { this.node?.port.postMessage({ type: 'seq-set-step-gate-pct', step, value: pct }); }
    setStepProbability(step: number, prob: number) { this.node?.port.postMessage({ type: 'seq-set-step-probability', step, value: prob }); }
    setStepRatchet(step: number, count: number) { this.node?.port.postMessage({ type: 'seq-set-step-ratchet', step, value: count }); }
    setStepSkip(step: number, skip: boolean) { this.node?.port.postMessage({ type: 'seq-set-step-skip', step, value: skip }); }
    setDirection(dir: number) { this.node?.port.postMessage({ type: 'seq-set-direction', value: dir }); }
    setSwing(swing: number) { this.node?.port.postMessage({ type: 'seq-set-swing', value: swing }); }
    setTimeDivision(div: number) { this.node?.port.postMessage({ type: 'seq-set-time-div', value: div }); }
    seqRotate(dir: number) { this.node?.port.postMessage({ type: 'seq-rotate', value: dir }); }
    setSeqExternal(ext: boolean) { this.node?.port.postMessage({ type: 'seq-set-external', value: ext }); }
    setSeqLength(len: number) { this.node?.port.postMessage({ type: 'seq-set-length', value: len }); }
}
