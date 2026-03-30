<script lang="ts">
    import { SCALE_NOTES, SCALE_CHORDS, LEAD_PARAMS, LEAD_COLOUR, MODELS } from './constants';
    import {
        selectedModel, selectedParam, sliderValue, triggeredNotes, padMode, latchEnabled,
        seqNumPages, seqCurrentPage, seqSelectedStep, currentLeadPreset,
        selectModel, selectLeadParam, setSliderValue, triggerPad,
        togglePadMode, toggleLatch, clearSequence, setSeqStepFromPad, selectSeqStep, setSeqPage,
        loadLeadPreset,
    } from './stores/state';
    import { LEAD_PRESETS } from './presets';
    import { get } from 'svelte/store';
    import PadCircle from '../shared/components/PadCircle.svelte';
    import Slider from '../shared/components/Slider.svelte';
    import PlayControls from '../shared/components/PlayControls.svelte';
    import LeadSequencer from './LeadSequencer.svelte';
    import ArpSettings from './ArpSettings.svelte';

    $: model = $selectedModel;
    $: selParam = $selectedParam;
    $: sliderVal = $sliderValue;
    $: triggered = $triggeredNotes;
    $: mode = $padMode;
    $: latch = $latchEnabled;
    $: presetIdx = $currentLeadPreset;

    function handlePresetChange(e: Event) {
        loadLeadPreset(Number((e.target as HTMLSelectElement).value));
    }

    $: pads = mode === 'chord'
        ? SCALE_CHORDS.map(c => ({ id: c.id, label: c.label, colour: c.colour }))
        : SCALE_NOTES.map(n => ({ id: n.id, label: n.label, colour: n.colour }));

    function handleModelChange(e: Event) {
        selectModel(Number((e.target as HTMLSelectElement).value));
    }

    function handlePadClick(i: number) {
        triggerPad(i);
        setSeqStepFromPad(i);
        // Auto-advance within current page, wrap to start of page at end
        const cur = get(seqSelectedStep);
        const page = get(seqCurrentPage);
        const pageStart = page * 16;
        const pageEnd = pageStart + 15;
        if (cur < pageEnd) {
            selectSeqStep(cur + 1);
        } else {
            // Wrap to start of current page
            selectSeqStep(pageStart);
        }
    }
</script>

<div class="lead-panel">
    <div class="top-bar">
        <select class="model-select" value={model} onchange={handleModelChange}>
            {#each MODELS as name, i}
                <option value={i}>{name}</option>
            {/each}
        </select>
        <select class="preset-select" value={presetIdx} onchange={handlePresetChange}>
            <option value={-1} disabled>PRESET</option>
            {#each LEAD_PRESETS as p, i}
                <option value={i}>{p.name}</option>
            {/each}
        </select>
        <button class="mode-btn" onclick={togglePadMode}>
            {mode === 'note' ? 'NOTE' : 'CHRD'}
        </button>
        <button class="latch-btn" class:active={latch} onclick={toggleLatch}>LATCH</button>
        <button class="clr-btn" onclick={clearSequence}>CLR</button>
        <ArpSettings />
    </div>
    <LeadSequencer />
    <PadCircle
        voices={pads}
        params={[...LEAD_PARAMS]}
        selectedVoice={-1}
        selectedParam={selParam}
        triggeredVoices={triggered}
        onPadClick={handlePadClick}
        onParamSelect={selectLeadParam}
    />
    <PlayControls />
    <Slider
        label={selParam}
        value={sliderVal}
        colour={LEAD_COLOUR}
        onChange={setSliderValue}
    />
</div>

<style>
    .lead-panel { flex: 1; display: flex; flex-direction: column; }
    .top-bar {
        display: flex; justify-content: center; align-items: flex-start;
        gap: 8px; padding: 8px 24px 4px; flex-wrap: wrap;
    }
    .model-select {
        flex: 1; max-width: 180px; padding: 4px 10px;
        font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500;
        letter-spacing: 0.5px; text-transform: uppercase;
        background: var(--orbit-well, #1a1a1a); color: var(--orbit-ink, #eee);
        border: 1px solid var(--orbit-border, #333); border-radius: 12px;
        outline: none; cursor: pointer;
        -webkit-appearance: none; appearance: none;
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='6'%3E%3Cpath d='M0 0l5 6 5-6z' fill='%23666'/%3E%3C/svg%3E");
        background-repeat: no-repeat; background-position: right 10px center; padding-right: 28px;
    }
    .model-select:focus { border-color: var(--orbit-ink, #eee); }
    .model-select option { background: var(--orbit-surface, #111); color: var(--orbit-ink, #eee); }
    .preset-select {
        flex: 1; max-width: 140px; padding: 4px 10px;
        font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500;
        letter-spacing: 0.5px; text-transform: uppercase;
        background: var(--orbit-well, #1a1a1a); color: var(--orbit-ink, #eee);
        border: 1px solid var(--orbit-border, #333); border-radius: 12px;
        outline: none; cursor: pointer;
        -webkit-appearance: none; appearance: none;
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='6'%3E%3Cpath d='M0 0l5 6 5-6z' fill='%23666'/%3E%3C/svg%3E");
        background-repeat: no-repeat; background-position: right 10px center; padding-right: 28px;
    }
    .preset-select:focus { border-color: var(--orbit-ink, #eee); }
    .preset-select option { background: var(--orbit-surface, #111); color: var(--orbit-ink, #eee); }
    .mode-btn {
        padding: 4px 12px;
        font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500;
        letter-spacing: 1px; background: transparent; color: var(--orbit-hint, #666);
        border: 1.5px solid var(--orbit-border, #444); border-radius: 12px;
        cursor: pointer; transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1);
    }
    .mode-btn:active { background: var(--orbit-ink, #eee); color: var(--orbit-surface, #111); }
    .latch-btn {
        padding: 4px 10px;
        font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500;
        letter-spacing: 0.5px; background: transparent; color: var(--orbit-hint, #666);
        border: 1.5px solid var(--orbit-border, #444); border-radius: 12px;
        cursor: pointer; transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1);
    }
    .latch-btn.active { background: #D4B830; color: #111; border-color: #D4B830; }
    .clr-btn {
        padding: 4px 10px;
        font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500;
        letter-spacing: 0.5px; background: transparent; color: var(--orbit-hint, #666);
        border: 1.5px solid var(--orbit-border, #444); border-radius: 12px;
        cursor: pointer; transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1);
    }
    .clr-btn:active { background: #D84040; color: #fff; border-color: #D84040; }
</style>
