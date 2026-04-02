<script lang="ts">
    import { channels, setVolume, setPan, toggleMute, toggleSolo, type ChannelId } from '../shared/stores/mixer';
    import { mixGlitchSize, setMixGlitch } from '../shared/stores/transport';
    import PlayControls from '../shared/components/PlayControls.svelte';
    import GlitchSlider from '../shared/components/GlitchSlider.svelte';

    $: ch = $channels;

    const STRIPS: { id: ChannelId; label: string; colour: string }[] = [
        { id: 'drum', label: 'DRUM', colour: '#378ADD' },
        { id: 'pad',  label: 'PAD',  colour: '#E8944A' },
        { id: 'acid', label: 'ACID', colour: '#5DBE6E' },
        { id: 'lead', label: 'LEAD', colour: '#B56ECC' },
    ];

    function handleVolume(id: ChannelId, e: Event) {
        setVolume(id, Number((e.target as HTMLInputElement).value));
    }

    function handlePan(id: ChannelId, e: Event) {
        setPan(id, Number((e.target as HTMLInputElement).value));
    }

    // Convert pan -100..100 to rotation angle -135..135 degrees
    function panAngle(pan: number): number {
        return (pan / 100) * 135;
    }

    function panLabel(pan: number): string {
        if (pan === 0) return 'C';
        return pan < 0 ? `L${Math.abs(pan)}` : `R${pan}`;
    }
</script>

<div class="mix-panel">
    <div class="mix-header">MIX</div>
    <div class="strips">
        {#each STRIPS as strip}
            {@const state = ch[strip.id]}
            <div class="strip">
                <span class="strip-label" style="color: {strip.colour}">{strip.label}</span>
                <div class="knob-container">
                    <div class="knob" style="--knob-colour: {strip.colour}">
                        <div class="knob-body" style="transform: rotate({panAngle(state.pan)}deg)">
                            <div class="knob-indicator"></div>
                        </div>
                        <input
                            type="range"
                            min="-100"
                            max="100"
                            step="1"
                            value={state.pan}
                            oninput={(e) => handlePan(strip.id, e)}
                            class="knob-input"
                        />
                    </div>
                    <span class="pan-label">{panLabel(state.pan)}</span>
                </div>
                <div class="fader-track">
                    <input
                        type="range"
                        min="0"
                        max="100"
                        step="1"
                        value={state.volume}
                        oninput={(e) => handleVolume(strip.id, e)}
                        class="fader"
                        orient="vertical"
                        style="--fill-color: {strip.colour}; --fill-pct: {state.volume}%"
                    />
                </div>
                <span class="vol-value" style="color: {strip.colour}">{state.volume}</span>
                <div class="strip-buttons">
                    <button
                        class="strip-btn mute-btn"
                        class:active={state.mute}
                        onclick={() => toggleMute(strip.id)}
                    >M</button>
                    <button
                        class="strip-btn solo-btn"
                        class:active={state.solo}
                        onclick={() => toggleSolo(strip.id)}
                    >S</button>
                </div>
            </div>
        {/each}
    </div>
    <PlayControls>
        <GlitchSlider glitchSize={mixGlitchSize} onGlitch={setMixGlitch} colour="#eee" />
    </PlayControls>
</div>

<style>
    .mix-panel {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 24px 24px;
        padding-bottom: calc(24px + env(safe-area-inset-bottom, 0px));
    }
    .mix-header {
        font-size: 11px;
        font-weight: 400;
        letter-spacing: 2px;
        text-transform: uppercase;
        color: var(--orbit-hint, #666);
        margin-bottom: 20px;
    }
    .strips {
        display: flex;
        gap: 40px;
        justify-content: center;
    }
    .strip {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 10px;
        min-width: 60px;
    }
    .strip-label {
        font-size: 11px;
        font-weight: 500;
        letter-spacing: 2px;
        text-transform: uppercase;
    }
    .knob-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 4px;
    }
    .knob {
        position: relative;
        width: 36px;
        height: 36px;
    }
    .knob-body {
        width: 100%;
        height: 100%;
        border-radius: 50%;
        background: var(--orbit-well, #1a1a1a);
        border: 1.5px solid var(--orbit-border, #444);
        display: flex;
        align-items: flex-start;
        justify-content: center;
        transition: transform 30ms linear;
    }
    .knob-indicator {
        width: 2px;
        height: 10px;
        background: var(--knob-colour);
        border-radius: 1px;
        margin-top: 4px;
    }
    .knob-input {
        position: absolute;
        top: 0; left: 0;
        width: 100%;
        height: 100%;
        opacity: 0;
        cursor: pointer;
        margin: 0;
    }
    .pan-label {
        font-size: 9px;
        font-weight: 500;
        color: var(--orbit-hint, #666);
        letter-spacing: 0.3px;
    }
    .fader-track {
        display: flex;
        align-items: center;
        justify-content: center;
        height: 200px;
    }
    .fader {
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
    .fader::-webkit-slider-thumb {
        -webkit-appearance: none;
        width: 24px;
        height: 10px;
        border-radius: 3px;
        background: var(--orbit-surface, #111);
        border: 2px solid var(--fill-color);
        cursor: pointer;
    }
    .fader::-moz-range-thumb {
        width: 24px;
        height: 10px;
        border-radius: 3px;
        background: var(--orbit-surface, #111);
        border: 2px solid var(--fill-color);
        cursor: pointer;
    }
    .vol-value {
        font-size: 16px;
        font-weight: 500;
        letter-spacing: -0.5px;
    }
    .strip-buttons {
        display: flex;
        gap: 6px;
    }
    .strip-btn {
        width: 32px;
        height: 32px;
        border-radius: 50%;
        font-family: 'JetBrains Mono', monospace;
        font-size: 11px;
        font-weight: 500;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1);
    }
    .mute-btn {
        background: transparent;
        color: var(--orbit-hint, #666);
        border: 1.5px solid var(--orbit-border, #444);
    }
    .mute-btn.active {
        background: #D84040;
        color: #fff;
        border-color: #D84040;
    }
    .solo-btn {
        background: transparent;
        color: var(--orbit-hint, #666);
        border: 1.5px solid var(--orbit-border, #444);
    }
    .solo-btn.active {
        background: #D4B830;
        color: #111;
        border-color: #D4B830;
    }
</style>
