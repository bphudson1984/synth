import { BaseEngine } from '../../shared/audio/BaseEngine';

export class BraidsEngine extends BaseEngine {
    async init(): Promise<void> {
        await this.initWorklet('braids-processor', 'braids-dsp.wasm');
    }

    noteOn(note: number, velocity: number) { this.node?.port.postMessage({ type: 'note-on', note, velocity }); }
    noteOff(note: number) { this.node?.port.postMessage({ type: 'note-off', note }); }

    // Sequencer step methods
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

export const PARAM = {
    MODEL: 0, TIMBRE: 1, COLOR: 2,
    FILTER_CUTOFF: 3, FILTER_RESONANCE: 4, FILTER_ENV_AMT: 5,
    AMP_ATTACK: 6, AMP_DECAY: 7, AMP_SUSTAIN: 8, AMP_RELEASE: 9,
    LFO_RATE: 10, LFO_AMOUNT: 11, LFO_DEST: 12,
    MASTER_VOL: 13, GLIDE_TIME: 14,
    FILT_ATTACK: 15, FILT_DECAY: 16, FILT_SUSTAIN: 17, FILT_RELEASE: 18,
} as const;
