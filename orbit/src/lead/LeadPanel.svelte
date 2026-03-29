<script lang="ts">
    import { NOTE_PADS, LEAD_PARAMS, LEAD_COLOUR, MODELS } from './constants';
    import {
        selectedModel, selectedParam, sliderValue, triggeredNotes,
        selectModel, selectLeadParam, setSliderValue, triggerNote,
    } from './stores/state';
    import PadCircle from '../shared/components/PadCircle.svelte';
    import Slider from '../shared/components/Slider.svelte';

    $: model = $selectedModel;
    $: selParam = $selectedParam;
    $: sliderVal = $sliderValue;
    $: triggered = $triggeredNotes;

    function handleModelChange(e: Event) {
        selectModel(Number((e.target as HTMLSelectElement).value));
    }

    function handlePadClick(i: number) {
        triggerNote(i, NOTE_PADS[i].note);
    }
</script>

<div class="lead-panel">
    <div class="model-bar">
        <select class="model-select" value={model} onchange={handleModelChange}>
            {#each MODELS as name, i}
                <option value={i}>{name}</option>
            {/each}
        </select>
    </div>
    <PadCircle
        voices={NOTE_PADS.map(p => ({ id: p.id, label: p.label, colour: p.colour }))}
        params={[...LEAD_PARAMS]}
        selectedVoice={-1}
        selectedParam={selParam}
        triggeredVoices={triggered}
        onPadClick={handlePadClick}
        onParamSelect={selectLeadParam}
    />
    <Slider
        label={selParam}
        value={sliderVal}
        colour={LEAD_COLOUR}
        onChange={setSliderValue}
    />
</div>

<style>
    .lead-panel { flex: 1; display: flex; flex-direction: column; }
    .model-bar { display: flex; justify-content: center; padding: 8px 24px 4px; }
    .model-select {
        width: 100%; max-width: 320px; padding: 6px 12px;
        font-family: 'JetBrains Mono', monospace; font-size: 11px; font-weight: 500;
        letter-spacing: 0.5px; text-transform: uppercase;
        background: var(--orbit-well, #1a1a1a); color: var(--orbit-ink, #eee);
        border: 1px solid var(--orbit-border, #333); border-radius: 12px;
        outline: none; cursor: pointer;
        -webkit-appearance: none; appearance: none;
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='6'%3E%3Cpath d='M0 0l5 6 5-6z' fill='%23666'/%3E%3C/svg%3E");
        background-repeat: no-repeat; background-position: right 12px center; padding-right: 32px;
    }
    .model-select:focus { border-color: var(--orbit-ink, #eee); }
    .model-select option { background: var(--orbit-surface, #111); color: var(--orbit-ink, #eee); }
</style>
