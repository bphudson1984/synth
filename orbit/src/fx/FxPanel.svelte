<script lang="ts">
    import { fxParams, sendLevels, setFxParam, setSendLevel } from './stores/state';
    import type { EffectId } from './audio/engine';
    import type { ChannelId } from '../shared/stores/mixer';
    import PlayControls from '../shared/components/PlayControls.svelte';
    import GlitchSlider from '../shared/components/GlitchSlider.svelte';
    import { mixGlitchSize, setMixGlitch } from '../shared/stores/transport';

    const FX_COLOUR = '#ccc';

    interface FxParamDef {
        id: number;
        name: string;
        min: number;
        max: number;
    }

    interface EffectDef {
        effectId: EffectId;
        label: string;
        params: FxParamDef[];
    }

    const EFFECTS: EffectDef[] = [
        {
            effectId: 0, label: 'CHORUS',
            params: [
                { id: 0, name: 'RATE',  min: 0.1, max: 5 },
                { id: 1, name: 'DEPTH', min: 0,   max: 1 },
            ],
        },
        {
            effectId: 1, label: 'DELAY',
            params: [
                { id: 0, name: 'TIME',     min: 1,   max: 1000 },
                { id: 1, name: 'FEEDBACK', min: 0,   max: 0.95 },
                { id: 2, name: 'TONE',     min: 0,   max: 1 },
            ],
        },
        {
            effectId: 2, label: 'REVERB',
            params: [
                { id: 0, name: 'DECAY',   min: 0, max: 0.99 },
                { id: 1, name: 'DAMPING', min: 0, max: 1 },
            ],
        },
        {
            effectId: 3, label: 'DISTORT',
            params: [
                { id: 0, name: 'DRIVE', min: 0, max: 1 },
                { id: 1, name: 'TONE',  min: 0, max: 1 },
                { id: 2, name: 'LEVEL', min: 0, max: 1 },
            ],
        },
    ];

    const CHANNELS: { id: ChannelId; label: string; colour: string }[] = [
        { id: 'drum', label: 'DR', colour: '#378ADD' },
        { id: 'pad',  label: 'PD', colour: '#E8944A' },
        { id: 'acid', label: 'AC', colour: '#5DBE6E' },
        { id: 'lead', label: 'LD', colour: '#B56ECC' },
    ];

    let activeTab = $state(0);

    let params = $derived($fxParams);
    let sends = $derived($sendLevels);
    let effect = $derived(EFFECTS[activeTab]);

    function sliderPct(effectId: number, paramId: number, min: number, max: number): number {
        const raw = params[effectId]?.[paramId] ?? min;
        return ((raw - min) / (max - min)) * 100;
    }

    function handleSlider(effectId: EffectId, paramId: number, min: number, max: number, e: Event) {
        const pct = Number((e.target as HTMLInputElement).value);
        const value = min + (pct / 100) * (max - min);
        setFxParam(effectId, paramId, value);
    }

    function formatValue(effectId: number, paramId: number, min: number, max: number): string {
        const raw = params[effectId]?.[paramId] ?? min;
        if (max >= 100) return Math.round(raw).toString();
        if (max - min > 10) return raw.toFixed(1);
        return raw.toFixed(2);
    }

    function handleSend(channelId: ChannelId, effectIndex: number, e: Event) {
        const value = Number((e.target as HTMLInputElement).value);
        setSendLevel(channelId, effectIndex, value);
    }
</script>

<div class="fx-panel">
    <div class="tabs">
        {#each EFFECTS as effect, i}
            <button
                class="tab"
                class:active={activeTab === i}
                onclick={() => activeTab = i}
            >{effect.label}</button>
        {/each}
    </div>

    <div class="content">
        <div class="section-label">PARAMETERS</div>
        <div class="params">
            {#each effect.params as param}
                <div class="param-row">
                    <span class="param-label">{param.name}</span>
                    <input
                        type="range"
                        min="0"
                        max="100"
                        step="1"
                        value={sliderPct(effect.effectId, param.id, param.min, param.max)}
                        oninput={(e) => handleSlider(effect.effectId, param.id, param.min, param.max, e)}
                        class="param-slider"
                        style="--fill-pct: {sliderPct(effect.effectId, param.id, param.min, param.max)}%"
                    />
                    <span class="param-value">{formatValue(effect.effectId, param.id, param.min, param.max)}</span>
                </div>
            {/each}
        </div>

        <div class="section-label">SENDS</div>
        <div class="sends">
            {#each CHANNELS as ch}
                <div class="send-strip">
                    <span class="send-label" style="color: {ch.colour}">{ch.label}</span>
                    <div class="send-fader-track">
                        <input
                            type="range"
                            min="0"
                            max="100"
                            step="1"
                            value={sends[ch.id][activeTab]}
                            oninput={(e) => handleSend(ch.id, activeTab, e)}
                            class="send-fader"
                            orient="vertical"
                            style="--fill-color: {ch.colour}; --fill-pct: {sends[ch.id][activeTab]}%"
                        />
                    </div>
                    <span class="send-value" style="color: {ch.colour}">{sends[ch.id][activeTab]}</span>
                </div>
            {/each}
        </div>
    </div>
    <PlayControls>
        <GlitchSlider glitchSize={mixGlitchSize} onGlitch={setMixGlitch} colour="#eee" />
    </PlayControls>
</div>

<style>
    .fx-panel {
        flex: 1;
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }
    .tabs {
        display: flex;
        gap: 0;
        padding: 8px 24px 4px;
        justify-content: center;
    }
    .tab {
        padding: 4px 16px;
        font-family: 'JetBrains Mono', monospace;
        font-size: 10px;
        font-weight: 500;
        letter-spacing: 1.5px;
        text-transform: uppercase;
        background: transparent;
        color: var(--orbit-hint, #666);
        border: 1.5px solid var(--orbit-border, #444);
        cursor: pointer;
        transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1);
    }
    .tab:first-child { border-radius: 12px 0 0 12px; border-right: none; }
    .tab:not(:first-child):not(:last-child) { border-radius: 0; border-right: none; }
    .tab:last-child { border-radius: 0 12px 12px 0; }
    .tab.active {
        background: var(--orbit-ink, #eee);
        color: var(--orbit-surface, #111);
        border-color: var(--orbit-ink, #eee);
    }
    .content {
        flex: 1;
        overflow-y: auto;
        padding: 12px 24px;
        display: flex;
        flex-direction: column;
        gap: 16px;
    }
    .section-label {
        font-size: 9px;
        font-weight: 500;
        letter-spacing: 2px;
        color: var(--orbit-hint, #666);
        text-align: center;
    }
    .params {
        display: flex;
        flex-direction: column;
        gap: 14px;
    }
    .param-row {
        display: flex;
        align-items: center;
        gap: 12px;
    }
    .param-label {
        font-size: 10px;
        font-weight: 500;
        letter-spacing: 0.5px;
        color: var(--orbit-label, #aaa);
        min-width: 70px;
        flex-shrink: 0;
    }
    .param-slider {
        flex: 1;
        height: 3px;
        -webkit-appearance: none;
        appearance: none;
        background: linear-gradient(
            to right,
            #ccc 0%, #ccc var(--fill-pct),
            var(--orbit-well, #1a1a1a) var(--fill-pct)
        );
        border-radius: 2px;
        outline: none;
        cursor: pointer;
    }
    .param-slider::-webkit-slider-thumb {
        -webkit-appearance: none;
        width: 14px; height: 14px;
        border-radius: 50%;
        background: var(--orbit-surface, #111);
        border: 2px solid #ccc;
        cursor: pointer;
    }
    .param-slider::-moz-range-thumb {
        width: 14px; height: 14px;
        border-radius: 50%;
        background: var(--orbit-surface, #111);
        border: 2px solid #ccc;
        cursor: pointer;
    }
    .param-value {
        font-size: 10px;
        font-weight: 500;
        color: #ccc;
        min-width: 36px;
        text-align: right;
    }
    .sends {
        display: flex;
        gap: 32px;
        justify-content: center;
        padding: 8px 0;
    }
    .send-strip {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 8px;
        min-width: 40px;
    }
    .send-label {
        font-size: 10px;
        font-weight: 600;
        letter-spacing: 1px;
    }
    .send-fader-track {
        display: flex;
        align-items: center;
        justify-content: center;
        height: 140px;
    }
    .send-fader {
        -webkit-appearance: none;
        appearance: none;
        writing-mode: vertical-lr;
        direction: rtl;
        width: 3px;
        height: 100%;
        background: linear-gradient(
            to top,
            var(--fill-color) 0%,
            var(--fill-color) var(--fill-pct),
            var(--orbit-well, #1a1a1a) var(--fill-pct),
            var(--orbit-well, #1a1a1a) 100%
        );
        border-radius: 2px;
        outline: none;
        cursor: pointer;
    }
    .send-fader::-webkit-slider-thumb {
        -webkit-appearance: none;
        width: 20px;
        height: 8px;
        border-radius: 3px;
        background: var(--orbit-surface, #111);
        border: 2px solid var(--fill-color);
        cursor: pointer;
    }
    .send-fader::-moz-range-thumb {
        width: 20px;
        height: 8px;
        border-radius: 3px;
        background: var(--orbit-surface, #111);
        border: 2px solid var(--fill-color);
        cursor: pointer;
    }
    .send-value {
        font-size: 14px;
        font-weight: 500;
        letter-spacing: -0.5px;
    }
</style>
