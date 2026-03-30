<script lang="ts">
    import { CHORDS, PAD_PARAMS } from './constants';
    import { PRESETS } from './presets';
    import {
        selectedChord, selectedPadParam, triggeredChords, padSliderValue,
        currentPresetIndex, arpEnabled,
        selectChord, selectPadParam, triggerChord, setPadSliderValue, loadPreset, toggleArp,
    } from './stores/state';
    import PadCircle from '../shared/components/PadCircle.svelte';
    import Slider from '../shared/components/Slider.svelte';
    import PlayControls from '../shared/components/PlayControls.svelte';

    $: selChord = $selectedChord;
    $: selParam = $selectedPadParam;
    $: triggered = $triggeredChords;
    $: sliderVal = $padSliderValue;
    $: colour = CHORDS[selChord].colour;
    $: presetIdx = $currentPresetIndex;
    $: arp = $arpEnabled;

    function handlePadClick(i: number) {
        selectChord(i);
        triggerChord(i);
    }

    function handlePresetChange(e: Event) {
        const idx = Number((e.target as HTMLSelectElement).value);
        loadPreset(idx);
    }
</script>

<div class="pad-panel">
    <div class="preset-bar">
        <select class="preset-select" value={presetIdx} onchange={handlePresetChange}>
            {#each PRESETS as preset, i}
                <option value={i}>{preset.category} — {preset.name}</option>
            {/each}
        </select>
        <button class="arp-btn" class:active={arp} onclick={toggleArp}>ARP</button>
    </div>
    <PadCircle
        voices={CHORDS.map(c => ({ id: c.id, label: c.label, colour: c.colour }))}
        params={[...PAD_PARAMS]}
        selectedVoice={selChord}
        selectedParam={selParam}
        triggeredVoices={triggered}
        onPadClick={handlePadClick}
        onParamSelect={selectPadParam}
    />
    <PlayControls />
    <Slider
        label={selParam}
        value={sliderVal}
        {colour}
        onChange={setPadSliderValue}
    />
</div>

<style>
    .pad-panel {
        flex: 1;
        display: flex;
        flex-direction: column;
    }
    .preset-bar {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 8px;
        padding: 8px 24px 0;
    }
    .preset-select {
        width: 100%;
        max-width: 320px;
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
    .preset-select:focus {
        border-color: var(--orbit-ink, #eee);
    }
    .preset-select option {
        background: var(--orbit-surface, #111);
        color: var(--orbit-ink, #eee);
    }
    .arp-btn {
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
        white-space: nowrap;
    }
    .arp-btn.active {
        background: var(--orbit-ink, #eee);
        color: var(--orbit-surface, #111);
        border-color: var(--orbit-ink, #eee);
    }
</style>
