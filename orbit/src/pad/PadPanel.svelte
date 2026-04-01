<script lang="ts">
    import { CHORDS, PAD_PARAMS } from './constants';
    import { PRESETS } from './presets';
    import {
        selectedChord, selectedPadParam, triggeredChords, padSliderValue,
        currentPresetIndex, arpEnabled, padSeq,
        selectChord, selectPadParam, triggerChord, setPadSliderValue, loadPreset, toggleArp,
    } from './stores/state';
    import { isPlaying, isRecording } from '../shared/stores/transport';
    import { get } from 'svelte/store';
    import PadCircle from '../shared/components/PadCircle.svelte';
    import Slider from '../shared/components/Slider.svelte';
    import PlayControls from '../shared/components/PlayControls.svelte';
    import NoteSequencer from '../shared/components/NoteSequencer.svelte';
    import SeqSettingsRow from '../shared/components/SeqSettingsRow.svelte';
    import StepSettingsRow from '../shared/components/StepSettingsRow.svelte';

    const PAD_COLOUR = '#E8944A';

    $: selChord = $selectedChord;
    $: selParam = $selectedPadParam;
    $: triggered = $triggeredChords;
    $: sliderVal = $padSliderValue;
    $: colour = CHORDS[selChord]?.colour ?? PAD_COLOUR;
    $: presetIdx = $currentPresetIndex;
    $: arp = $arpEnabled;

    $: ({ seqSettingsOpen: seqOpenStore, stepSettingsOpen: stepOpenStore } = padSeq);
    $: seqOpen = $seqOpenStore;
    $: stepOpen = $stepOpenStore;

    function handlePadClick(i: number) {
        selectChord(i);
        triggerChord(i);

        if (!get(isRecording)) return; // record off: just play sound

        const chord = CHORDS[i];
        if (get(isPlaying)) {
            // Real-time record: write to the step the playhead is on
            padSeq.setStepFromNotes(get(padSeq.seqCurrentStep), [...chord.notes], chord.label);
        } else {
            // Step-entry: write to selected step, advance
            padSeq.setSeqStepFromNotes([...chord.notes], chord.label);
            const cur = get(padSeq.seqSelectedStep);
            const page = get(padSeq.seqCurrentPage);
            const pageStart = page * 16;
            const pageEnd = pageStart + 15;
            if (cur < pageEnd) { padSeq.selectSeqStep(cur + 1); }
            else { padSeq.selectSeqStep(pageStart); }
        }
    }

    function handlePresetChange(e: Event) {
        loadPreset(Number((e.target as HTMLSelectElement).value));
    }
</script>

<div class="pad-panel">
    <div class="top-bar">
        <select class="top-select" value={presetIdx} onchange={handlePresetChange}>
            {#each PRESETS as preset, i}
                <option value={i}>{preset.category} — {preset.name}</option>
            {/each}
        </select>
        <button class="bar-btn" class:active={arp} onclick={toggleArp}>ARP</button>
        <button class="bar-btn" class:active={seqOpen} onclick={padSeq.toggleSeqSettings}>SEQ</button>
        <button class="bar-btn" class:active={stepOpen} onclick={padSeq.toggleStepSettings}>STEP</button>
    </div>
    {#if seqOpen}
        <div class="drawer-row"><SeqSettingsRow colour={PAD_COLOUR} seq={padSeq} /></div>
    {:else if stepOpen}
        <div class="drawer-row"><StepSettingsRow colour={PAD_COLOUR} seq={padSeq} /></div>
    {/if}
    <NoteSequencer colour={PAD_COLOUR} seq={padSeq} />
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
    .pad-panel { flex: 1; display: flex; flex-direction: column; }
    .top-bar {
        display: flex; justify-content: center; align-items: center;
        gap: 6px; padding: 8px 16px 4px; flex-wrap: wrap;
    }
    .top-select {
        padding: 4px 10px;
        font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500;
        letter-spacing: 0.5px; text-transform: uppercase;
        background: var(--orbit-well, #1a1a1a); color: var(--orbit-ink, #eee);
        border: 1px solid var(--orbit-border, #333); border-radius: 12px;
        outline: none; cursor: pointer;
        -webkit-appearance: none; appearance: none;
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='6'%3E%3Cpath d='M0 0l5 6 5-6z' fill='%23666'/%3E%3C/svg%3E");
        background-repeat: no-repeat; background-position: right 10px center; padding-right: 28px;
    }
    .top-select:focus { border-color: var(--orbit-ink, #eee); }
    .top-select option { background: var(--orbit-surface, #111); color: var(--orbit-ink, #eee); }
    .bar-btn {
        padding: 4px 12px;
        font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500;
        letter-spacing: 1px; background: transparent; color: var(--orbit-hint, #666);
        border: 1.5px solid var(--orbit-border, #444); border-radius: 12px;
        cursor: pointer; transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1);
    }
    .bar-btn.active { background: var(--orbit-ink, #eee); color: var(--orbit-surface, #111); border-color: var(--orbit-ink, #eee); }
    .drawer-row {
        display: flex; flex-wrap: wrap; gap: 6px;
        padding: 6px 16px; align-items: center; justify-content: center;
    }
</style>
