<script lang="ts">
    import { VOICES } from '../constants';
    import { selectedVoice, selectedParam, sliderValue, setSliderValue } from '../stores/state';

    $: voice = $selectedVoice;
    $: param = $selectedParam;
    $: value = $sliderValue;
    $: colour = VOICES[voice].colour;

    function handleInput(e: Event) {
        setSliderValue(Number((e.target as HTMLInputElement).value));
    }
</script>

<div class="slider-area">
    <div class="slider-header">
        <span class="slider-label">{param.toUpperCase()}</span>
        <span class="slider-value" style="color: {colour}">{Math.round(value)}</span>
    </div>
    <div class="slider-track-container">
        <input
            type="range"
            min="0"
            max="100"
            step="1"
            value={value}
            oninput={handleInput}
            class="slider"
            style="--fill-color: {colour}; --fill-pct: {value}%"
        />
    </div>
</div>

<style>
    .slider-area {
        padding: 12px 24px 24px;
        padding-bottom: calc(24px + env(safe-area-inset-bottom, 0px));
    }
    .slider-header {
        display: flex;
        justify-content: space-between;
        align-items: baseline;
        margin-bottom: 8px;
    }
    .slider-label {
        font-size: 11px;
        font-weight: 400;
        text-transform: uppercase;
        letter-spacing: 2px;
        color: var(--orbit-hint, #888);
    }
    .slider-value {
        font-size: 16px;
        font-weight: 500;
        letter-spacing: -0.5px;
    }
    .slider-track-container { width: 100%; }
    .slider {
        -webkit-appearance: none;
        appearance: none;
        width: 100%;
        height: 3px;
        background: linear-gradient(
            to right,
            var(--fill-color) 0%,
            var(--fill-color) var(--fill-pct),
            var(--orbit-well, #1a1a1a) var(--fill-pct),
            var(--orbit-well, #1a1a1a) 100%
        );
        border-radius: 2px;
        outline: none;
        cursor: pointer;
    }
    .slider::-webkit-slider-thumb {
        -webkit-appearance: none;
        width: 16px; height: 16px;
        border-radius: 50%;
        background: var(--orbit-surface, #111);
        border: 2px solid var(--fill-color);
        cursor: pointer;
    }
    .slider::-moz-range-thumb {
        width: 16px; height: 16px;
        border-radius: 50%;
        background: var(--orbit-surface, #111);
        border: 2px solid var(--fill-color);
        cursor: pointer;
    }
</style>
