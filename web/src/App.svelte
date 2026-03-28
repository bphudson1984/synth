<script lang="ts">
    import { AudioEngine } from './audio/engine';
    import { setEngine } from './stores/params';
    import * as p from './stores/params';
    import { PRESETS, CATEGORIES, type Preset } from './stores/presets';
    import Knob from './components/Knob.svelte';
    import Switch from './components/Switch.svelte';
    import Keyboard from './components/Keyboard.svelte';

    let engine: AudioEngine;
    let started = false;
    let status = 'Click to start';
    let currentPreset = 'Init';

    import { get } from 'svelte/store';

    function setArpMode(e: Event) {
        p.arpMode.set(Number((e.target as HTMLSelectElement).value));
    }
    function setArpDiv(e: Event) {
        p.arpDivision.set(Number((e.target as HTMLSelectElement).value));
    }

    function loadPreset(preset: Preset) {
        if (!engine) return;
        for (const [id, value] of preset.params) {
            engine.setParam(id, value);
        }
        currentPreset = preset.name;
    }

    async function start() {
        status = 'Loading...';
        engine = new AudioEngine();
        try {
            await engine.init();
            setEngine(engine);
            started = true;
            status = 'Playing';

            // Try MIDI
            if (navigator.requestMIDIAccess) {
                try {
                    const midi = await navigator.requestMIDIAccess();
                    for (const input of midi.inputs.values()) {
                        input.onmidimessage = handleMIDI;
                    }
                } catch {}
            }
        } catch (err: any) {
            status = 'Error: ' + err.message;
        }
    }

    function handleMIDI(e: MIDIMessageEvent) {
        const [st, d1, d2] = e.data!;
        const cmd = st & 0xf0;
        if (cmd === 0x90 && d2 > 0) engine.noteOn(d1, d2);
        else if (cmd === 0x80 || (cmd === 0x90 && d2 === 0)) engine.noteOff(d1);
        else if (cmd === 0xb0 && d1 === 1) engine.setParam(46, d2 / 127);
        else if (cmd === 0xe0) engine.setParam(47, (((d2 << 7) | d1) - 8192) / 8192);
    }

    function noteOn(e: CustomEvent<{note: number; velocity: number}>) {
        engine?.noteOn(e.detail.note, e.detail.velocity);
    }
    function noteOff(e: CustomEvent<{note: number}>) {
        engine?.noteOff(e.detail.note);
    }
</script>

{#if !started}
    <div class="splash">
        <h1>SEQUENTIAL CIRCUITS</h1>
        <h2>PROPHET-5</h2>
        <button onclick={start}>{status}</button>
    </div>
{:else}
    <div class="synth">
        <header>
            <span class="brand">SEQUENTIAL CIRCUITS</span>
            <span class="model">PROPHET-5</span>
            <div class="preset-selector">
                <label>PROGRAM</label>
                <select onchange={(e) => {
                    const name = (e.target as HTMLSelectElement).value;
                    const preset = PRESETS.find(p => p.name === name);
                    if (preset) loadPreset(preset);
                }}>
                    <option value="">-- Select Preset --</option>
                    {#each CATEGORIES as cat}
                        <optgroup label={cat}>
                            {#each PRESETS.filter(pr => pr.category === cat) as preset}
                                <option value={preset.name} selected={currentPreset === preset.name}>
                                    {preset.name}
                                </option>
                            {/each}
                        </optgroup>
                    {/each}
                </select>
            </div>
        </header>

        <div class="panel">
            <div class="wood left"></div>

            <div class="controls">
                <!-- Synth sections row -->
                <div class="sections-row">
                    <section>
                        <h3>POLY MOD</h3>
                        <div class="row">
                            <Knob label="FILT ENV" store={p.pmFiltEnv} />
                            <Knob label="OSC B" store={p.pmOscB} />
                        </div>
                        <div class="row">
                            <Switch label="FREQ A" store={p.pmFreqA} />
                            <Switch label="PW A" store={p.pmPWA} />
                            <Switch label="FILT" store={p.pmFilter} />
                        </div>
                    </section>

                    <div class="divider"></div>

                    <section>
                        <h3>LFO</h3>
                        <div class="row">
                            <Knob label="FREQ" store={p.lfoFreq} min={0.1} max={20} />
                            <Knob label="AMT" store={p.lfoAmount} />
                        </div>
                        <div class="row">
                            <Switch label="TRI" store={p.lfoTri} />
                            <Switch label="SAW" store={p.lfoSaw} />
                            <Switch label="SQR" store={p.lfoSquare} />
                        </div>
                    </section>

                    <div class="divider"></div>

                    <section>
                        <h3>WHEEL MOD</h3>
                        <div class="row">
                            <Knob label="MIX" store={p.wmMix} />
                        </div>
                        <div class="row">
                            <Switch label="F.A" store={p.wmFreqA} />
                            <Switch label="F.B" store={p.wmFreqB} />
                            <Switch label="FILT" store={p.wmFilter} />
                        </div>
                        <div class="row">
                            <Switch label="PW.A" store={p.wmPWA} />
                            <Switch label="PW.B" store={p.wmPWB} />
                        </div>
                    </section>

                    <div class="divider"></div>

                    <section>
                        <h3>OSCILLATOR A</h3>
                        <div class="row">
                            <Switch label="SAW" store={p.oscASaw} />
                            <Switch label="PULSE" store={p.oscAPulse} />
                            <Switch label="SYNC" store={p.sync} />
                        </div>
                        <div class="row">
                            <Knob label="PW" store={p.oscAPW} min={0.01} max={0.99} />
                        </div>
                    </section>

                    <div class="divider"></div>

                    <section>
                        <h3>OSCILLATOR B</h3>
                        <div class="row">
                            <Knob label="FREQ" store={p.oscBSemi} min={-24} max={24} />
                            <Knob label="FINE" store={p.oscBFine} min={-100} max={100} />
                            <Knob label="PW" store={p.oscBPW} min={0.01} max={0.99} />
                        </div>
                        <div class="row">
                            <Switch label="SAW" store={p.oscBSaw} />
                            <Switch label="TRI" store={p.oscBTri} />
                            <Switch label="PULSE" store={p.oscBPulse} />
                        </div>
                    </section>

                    <div class="divider"></div>

                    <section>
                        <h3>MIXER</h3>
                        <div class="row">
                            <Knob label="OSC A" store={p.oscALevel} />
                            <Knob label="OSC B" store={p.oscBLevel} />
                            <Knob label="NOISE" store={p.noiseLevel} />
                        </div>
                    </section>

                    <div class="divider"></div>

                    <section>
                        <h3>FILTER</h3>
                        <div class="row">
                            <Knob label="CUTOFF" store={p.filterCutoff} min={20} max={20000} size={48} />
                            <Knob label="RES" store={p.filterRes} size={48} />
                        </div>
                        <div class="row">
                            <Knob label="ENV AMT" store={p.filterEnvAmt} min={0} max={20000} />
                            <Knob label="DRIVE" store={p.filterDrive} min={0.5} max={5} />
                        </div>
                    </section>

                    <div class="divider"></div>

                    <section>
                        <h3>FILTER ENVELOPE</h3>
                        <div class="row">
                            <Knob label="A" store={p.fAttack} min={0.001} max={10} />
                            <Knob label="D" store={p.fDecay} min={0.001} max={10} />
                            <Knob label="S" store={p.fSustain} />
                            <Knob label="R" store={p.fRelease} min={0.001} max={10} />
                        </div>
                    </section>

                    <div class="divider"></div>

                    <section>
                        <h3>AMPLIFIER</h3>
                        <div class="row">
                            <Knob label="A" store={p.aAttack} min={0.001} max={10} />
                            <Knob label="D" store={p.aDecay} min={0.001} max={10} />
                            <Knob label="S" store={p.aSustain} />
                            <Knob label="R" store={p.aRelease} min={0.001} max={10} />
                        </div>
                    </section>

                    <div class="divider"></div>

                    <section>
                        <h3>MASTER</h3>
                        <div class="row">
                            <Knob label="VOL" store={p.masterVol} size={48} silver />
                            <Knob label="GLIDE" store={p.glideRate} min={0.001} max={2} />
                            <Knob label="DRIFT" store={p.drift} min={0} max={10} />
                        </div>
                        <div class="row">
                            <Switch label="GLIDE" store={p.glideOn} />
                            <Switch label="UNISN" store={p.unison} />
                        </div>
                    </section>
                </div>

                <!-- Effects row -->
                <div class="fx-divider"></div>
                <div class="sections-row fx-row">
                    <section>
                        <h3>ARPEGGIATOR</h3>
                        <div class="row">
                            <div class="select-group">
                                <label>MODE</label>
                                <select onchange={setArpMode}>
                                    <option value="0">Off</option>
                                    <option value="1">Up</option>
                                    <option value="2">Down</option>
                                    <option value="3">Up/Down</option>
                                    <option value="4">Up/Dn Excl</option>
                                    <option value="5">Random</option>
                                    <option value="6">Order</option>
                                </select>
                            </div>
                            <div class="select-group">
                                <label>RATE</label>
                                <select onchange={setArpDiv}>
                                    <option value="0">1/4</option>
                                    <option value="1" selected>1/8</option>
                                    <option value="2">1/16</option>
                                    <option value="3">1/32</option>
                                    <option value="4">Dot 1/8</option>
                                    <option value="5">Trip 1/8</option>
                                </select>
                            </div>
                        </div>
                        <div class="row">
                            <Knob label="BPM" store={p.arpBpm} min={30} max={300} />
                            <Knob label="OCT" store={p.arpOctaves} min={1} max={4} />
                            <Knob label="GATE" store={p.arpGate} min={0.05} max={1} />
                            <Knob label="SWING" store={p.arpSwing} />
                        </div>
                        <div class="row">
                            <Switch label="HOLD" store={p.arpHold} />
                        </div>
                    </section>

                    <div class="divider"></div>

                    <section>
                        <h3>CHORUS</h3>
                        <div class="row">
                            <Knob label="RATE" store={p.chorusRate} min={0.1} max={5} />
                            <Knob label="DEPTH" store={p.chorusDepth} />
                            <Knob label="MIX" store={p.chorusMix} />
                        </div>
                    </section>
                    <div class="divider"></div>
                    <section>
                        <h3>DELAY</h3>
                        <div class="row">
                            <Knob label="TIME" store={p.delayTime} min={1} max={2000} />
                            <Knob label="FDBK" store={p.delayFeedback} min={0} max={0.95} />
                            <Knob label="TONE" store={p.delayTone} />
                            <Knob label="MIX" store={p.delayMix} />
                        </div>
                    </section>
                    <div class="divider"></div>
                    <section>
                        <h3>REVERB</h3>
                        <div class="row">
                            <Knob label="DECAY" store={p.reverbDecay} min={0} max={0.99} />
                            <Knob label="DAMP" store={p.reverbDamping} />
                            <Knob label="MIX" store={p.reverbMix} />
                        </div>
                    </section>
                </div>
            </div>

            <div class="wood right"></div>
        </div>

        <Keyboard onnoteon={noteOn} onnoteoff={noteOff} />
    </div>
{/if}

<style>
    .splash {
        display: flex; flex-direction: column; align-items: center;
        justify-content: center; height: 100vh; gap: 12px;
    }
    .splash h1 { font-size: 14px; color: #b0a890; letter-spacing: 3px; }
    .splash h2 { font-size: 28px; color: #e8e0d0; letter-spacing: 2px; margin-bottom: 24px; }
    .splash button {
        padding: 14px 48px; font-size: 16px;
        background: #3a3530; color: #e0d8c8;
        border: 1px solid #5a5245; border-radius: 4px;
        cursor: pointer; letter-spacing: 1px;
    }
    .splash button:hover { background: #4a4540; }

    .synth { max-width: 100vw; overflow-x: auto; }

    header {
        padding: 8px 40px;
        background: #1c1c1b;
        display: flex; align-items: baseline; gap: 10px;
    }
    .brand { font-size: 10px; color: #afa598; letter-spacing: 2px; }
    .model { font-size: 15px; color: #e8e2d4; font-weight: bold; letter-spacing: 1px; }

    .preset-selector {
        margin-left: 30px;
        display: flex; align-items: center; gap: 8px;
    }
    .preset-selector label {
        font-size: 8px; color: #8a8578; letter-spacing: 1px;
    }
    .preset-selector select {
        background: #2a2820; color: #d8d0c0; border: 1px solid #4a4538;
        padding: 4px 8px; font-size: 12px; border-radius: 3px;
        min-width: 180px; cursor: pointer;
    }
    .preset-selector select:focus { outline: 1px solid #6a6050; }
    .preset-selector optgroup { color: #b0a080; font-style: normal; }
    .preset-selector option { background: #2a2820; color: #d8d0c0; }

    .panel { display: flex; }
    .wood {
        width: 28px; min-height: 100%;
        background: linear-gradient(90deg, #503018, #6e4228, #8a5535, #6e4228, #503018);
    }

    .controls {
        flex: 1;
        background: #1c1c1b;
        padding: 8px 10px;
    }

    .sections-row {
        display: flex;
        align-items: flex-start;
        gap: 0;
    }

    section {
        padding: 0 4px;
    }
    section h3 {
        font-size: 9px;
        color: #c3c0b6;
        text-align: center;
        letter-spacing: 0.5px;
        margin-bottom: 6px;
        white-space: nowrap;
    }

    .row {
        display: flex;
        align-items: flex-start;
        gap: 2px;
        margin-bottom: 2px;
    }

    .divider {
        width: 1px;
        align-self: stretch;
        background: #302e2a;
        margin: 0 4px;
    }

    .fx-divider {
        height: 1px;
        background: #373530;
        margin: 8px 0;
    }

    .fx-row {
        padding-top: 4px;
    }

    .select-group {
        display: flex; flex-direction: column; align-items: center; gap: 2px;
    }
    .select-group label {
        font-size: 8px; color: #b0ada5; letter-spacing: 0.5px;
    }
    .select-group select {
        background: #2a2820; color: #d0c8b8; border: 1px solid #3a3530;
        padding: 3px 6px; font-size: 10px; border-radius: 2px; cursor: pointer;
        min-width: 70px;
    }
</style>
