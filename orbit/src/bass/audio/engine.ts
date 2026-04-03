import { BaseEngine } from '../../shared/audio/BaseEngine';

export class BassEngine extends BaseEngine {
    async init(): Promise<void> {
        await this.initWorklet('volca-bass-processor', 'volca-bass.wasm');
        this.applyDefaultPreset();
    }

    noteOn(note: number, velocity: number) { this.node?.port.postMessage({ type: 'note-on', note, velocity }); }
    noteOff(note: number) { this.node?.port.postMessage({ type: 'note-off', note }); }

    private applyDefaultPreset() {
        // Warm unison bass
        this.setParam(PARAM.VCO1_ACTIVE, 1);
        this.setParam(PARAM.VCO2_ACTIVE, 1);
        this.setParam(PARAM.VCO3_ACTIVE, 1);
        this.setParam(PARAM.VCO1_WAVE, 0); // saw
        this.setParam(PARAM.VCO2_WAVE, 0);
        this.setParam(PARAM.VCO3_WAVE, 0);
        this.setParam(PARAM.VCO1_PITCH, 0);
        this.setParam(PARAM.VCO2_PITCH, 0.1); // slight detune
        this.setParam(PARAM.VCO3_PITCH, -0.1);
        this.setParam(PARAM.CUTOFF, 2000);
        this.setParam(PARAM.PEAK, 0.3);
        this.setParam(PARAM.EG_INT, 0.5);
        this.setParam(PARAM.ATTACK, 0.005);
        this.setParam(PARAM.DECAY_RELEASE, 0.3);
        this.setParam(PARAM.SUSTAIN_ON, 0);
        this.setParam(PARAM.EG_TO_VCA, 1);
        this.setParam(PARAM.VOLUME, 0.5);
    }

    // Sequencer methods (melodic pattern — same as Prophet/Braids)
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
    VCO1_PITCH: 0, VCO2_PITCH: 1, VCO3_PITCH: 2,
    VCO1_WAVE: 3, VCO2_WAVE: 4, VCO3_WAVE: 5,
    VCO1_ACTIVE: 6, VCO2_ACTIVE: 7, VCO3_ACTIVE: 8,
    CUTOFF: 9, PEAK: 10, EG_INT: 11,
    ATTACK: 12, DECAY_RELEASE: 13, SUSTAIN_ON: 14, EG_TO_VCA: 15,
    LFO_RATE: 16, LFO_INT: 17, LFO_WAVE: 18,
    LFO_TO_PITCH: 19, LFO_TO_CUTOFF: 20, LFO_TO_AMP: 21,
    GROUP_MODE: 22, VOLUME: 23,
} as const;
