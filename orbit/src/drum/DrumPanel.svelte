<script lang="ts">
    import { VOICES, PARAMS } from './constants';
    import {
        selectedVoice, selectedParam, triggeredVoices, perPadEngine,
        sliderValue, currentDrumPreset,
        selectVoice, selectParam, triggerPad, togglePadEngine, setSliderValue,
        loadDrumPreset, randomizeDrumPattern,
    } from './stores/state';
    import { PRESETS } from './presets';
    import PadCircle from '../shared/components/PadCircle.svelte';
    import Slider from '../shared/components/Slider.svelte';
    import StepSequencer from './StepSequencer.svelte';
    import Transport from './Transport.svelte';

    $: selVoice = $selectedVoice;
    $: selParam = $selectedParam;
    $: triggered = $triggeredVoices;
    $: padEngines = $perPadEngine;
    $: sliderVal = $sliderValue;
    $: colour = VOICES[selVoice].colour;
    $: presetIdx = $currentDrumPreset;

    function handlePresetChange(e: Event) {
        loadDrumPreset(Number((e.target as HTMLSelectElement).value));
    }

    function handlePadClick(i: number) {
        selectVoice(i);
        triggerPad(i);
    }

    function badge(i: number): string | null {
        const eng = padEngines[i] ?? '808';
        return eng === '909' ? '9' : '8';
    }
</script>

<div class="preset-bar">
    <select class="preset-select" value={presetIdx} onchange={handlePresetChange}>
        <option value={-1} disabled>SELECT PATTERN</option>
        {#each PRESETS as preset, i}
            <option value={i}>{preset.name}</option>
        {/each}
    </select>
    <button class="rnd-btn" onclick={randomizeDrumPattern}>RND</button>
</div>
<StepSequencer />
<Transport />
<PadCircle
    voices={VOICES.map(v => ({ id: v.id, label: v.label, colour: v.colour }))}
    params={[...PARAMS]}
    selectedVoice={selVoice}
    selectedParam={selParam}
    triggeredVoices={triggered}
    onPadClick={handlePadClick}
    onPadDblClick={togglePadEngine}
    onParamSelect={selectParam}
    {badge}
/>
<Slider
    label={selParam}
    value={sliderVal}
    {colour}
    onChange={setSliderValue}
/>

<style>
    .preset-bar {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 8px;
        padding: 0 24px 4px;
    }
    .preset-select {
        flex: 1;
        max-width: 280px;
        padding: 6px 12px;
        font-family: 'JetBrains Mono', monospace;
        font-size: 11px;
        font-weight: 500;
        letter-spacing: 0.5px;
        text-transform: uppercase;
        background: var(--orbit-well, #1a1a1a);
        color: var(--orbit-ink, #eee);
        border: 1px solid var(--orbit-border, #333);
        border-radius: 12px;
        outline: none;
        cursor: pointer;
        -webkit-appearance: none;
        appearance: none;
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='6'%3E%3Cpath d='M0 0l5 6 5-6z' fill='%23666'/%3E%3C/svg%3E");
        background-repeat: no-repeat;
        background-position: right 12px center;
        padding-right: 32px;
    }
    .preset-select:focus { border-color: var(--orbit-ink, #eee); }
    .preset-select option { background: var(--orbit-surface, #111); color: var(--orbit-ink, #eee); }
    .rnd-btn {
        padding: 6px 14px;
        font-family: 'JetBrains Mono', monospace;
        font-size: 11px;
        font-weight: 500;
        letter-spacing: 1px;
        background: transparent;
        color: var(--orbit-hint, #666);
        border: 1.5px solid var(--orbit-border, #444);
        border-radius: 12px;
        cursor: pointer;
        transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1);
    }
    .rnd-btn:active { background: var(--orbit-ink, #eee); color: var(--orbit-surface, #111); }
</style>
