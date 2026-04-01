import { VOICES, getEngineVoiceId, getEngineTrackId, type EngineType } from '../constants';
import { BaseEngine } from '../../shared/audio/BaseEngine';

export class OrbitEngine extends BaseEngine {
    async init(): Promise<void> {
        await this.initWorklet('tr808-processor', 'tr808.wasm');
    }

    triggerVoice(orbitIndex: number, engine: EngineType = '808') {
        const voiceId = getEngineVoiceId(orbitIndex, engine);
        this.node?.port.postMessage({ type: 'trigger', voice: voiceId });
    }

    setVoiceParam(orbitIndex: number, param: string, value: number, engine: EngineType = '808') {
        const voiceId = getEngineVoiceId(orbitIndex, engine);
        const norm = value / 100;
        let paramId: number;
        switch (param) {
            case 'level': paramId = 0; break;
            case 'decay': paramId = 2; break;
            case 'tone':  paramId = 1; break;
            case 'pitch': paramId = 1; break;
            default: return;
        }
        this.node?.port.postMessage({ type: 'set-param', voice: voiceId, param: paramId, value: norm });
    }

    seqToggleStep(orbitVoice: number, step: number) {
        const trackId = getEngineTrackId(orbitVoice);
        this.node?.port.postMessage({ type: 'seq-toggle', track: trackId, step });
    }

    setTrackEngine(orbitIndex: number, engine: EngineType) {
        const trackId = getEngineTrackId(orbitIndex);
        this.node?.port.postMessage({ type: 'set-track-engine', track: trackId, is909: engine === '909' });
    }

    setAllEngines(engine: EngineType) {
        this.node?.port.postMessage({ type: 'set-all-engines', is909: engine === '909' });
    }

    setMasterVolume(value: number) {
        this.node?.port.postMessage({ type: 'set-param', voice: 255, param: 0, value });
    }

    setTimeDivision(div: number) { this.node?.port.postMessage({ type: 'seq-set-time-div', value: div }); }
}
