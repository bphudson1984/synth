import { BaseEngine } from '../../shared/audio/BaseEngine';

export class ProphetEngine extends BaseEngine {
    async init(): Promise<void> {
        await this.initWorklet('prophet-processor', 'prophet-dsp.wasm');
        this.applyDefaultPreset();
    }

    noteOn(note: number, velocity: number) { this.node?.port.postMessage({ type: 'note-on', note, velocity }); }
    noteOff(note: number) { this.node?.port.postMessage({ type: 'note-off', note }); }

    private applyDefaultPreset() {
        // Warm pad preset
        this.setParam(PARAM.OSC_A_SAW, 1.0);
        this.setParam(PARAM.OSC_A_LEVEL, 0.7);
        this.setParam(PARAM.OSC_B_SAW, 1.0);
        this.setParam(PARAM.OSC_B_LEVEL, 0.7);
        this.setParam(PARAM.OSC_B_FINE, 8);
        this.setParam(PARAM.FILTER_CUTOFF, 3000);
        this.setParam(PARAM.FILTER_RESONANCE, 0.8);
        this.setParam(PARAM.FILTER_ENV_AMT, 2000);
        this.setParam(PARAM.FILTER_ATTACK, 0.15);
        this.setParam(PARAM.FILTER_DECAY, 0.5);
        this.setParam(PARAM.FILTER_SUSTAIN, 0.3);
        this.setParam(PARAM.FILTER_RELEASE, 0.8);
        this.setParam(PARAM.AMP_ATTACK, 0.15);
        this.setParam(PARAM.AMP_DECAY, 0.5);
        this.setParam(PARAM.AMP_SUSTAIN, 0.7);
        this.setParam(PARAM.AMP_RELEASE, 0.8);
        this.setParam(PARAM.MASTER_VOL, 0.5);
    }

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
    OSC_A_SAW: 0, OSC_A_PULSE: 1, OSC_A_PW: 2,
    OSC_B_SAW: 3, OSC_B_TRI: 4, OSC_B_PULSE: 5, OSC_B_PW: 6, OSC_B_SEMI: 7, OSC_B_FINE: 8,
    OSC_A_LEVEL: 9, OSC_B_LEVEL: 10, NOISE_LEVEL: 11,
    FILTER_CUTOFF: 12, FILTER_RESONANCE: 13, FILTER_ENV_AMT: 14, FILTER_DRIVE: 15,
    FILTER_ATTACK: 16, FILTER_DECAY: 17, FILTER_SUSTAIN: 18, FILTER_RELEASE: 19,
    AMP_ATTACK: 20, AMP_DECAY: 21, AMP_SUSTAIN: 22, AMP_RELEASE: 23,
    SYNC: 24,
    PM_FILT_ENV: 25, PM_OSC_B: 26, PM_FREQ_A: 27, PM_PW_A: 28, PM_FILTER: 29,
    LFO_FREQ: 30, LFO_TRI: 31, LFO_SAW: 32, LFO_SQUARE: 33, LFO_AMOUNT: 34,
    WM_MIX: 35, WM_FREQ_A: 36, WM_FREQ_B: 37, WM_PW_A: 38, WM_PW_B: 39, WM_FILTER: 40,
    MASTER_VOL: 41, GLIDE_RATE: 42, GLIDE_ON: 43, UNISON: 44, DRIFT: 45,
    MOD_WHEEL: 46, PITCH_BEND: 47,
    ARP_MODE: 60, ARP_DIVISION: 61, ARP_BPM: 62, ARP_OCTAVES: 63, ARP_GATE: 64, ARP_SWING: 65, ARP_HOLD: 66, ARP_PANIC: 67,
} as const;
