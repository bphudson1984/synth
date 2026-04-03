<script lang="ts">
    import { NOTE_PADS, BASS_SETTINGS } from './constants';
    import { PRESETS } from './presets';
    import {
        currentPresetIndex,
        settingsOpen, settingsValues, toggleSettings, setSettingsParam,
        triggerNote,
        quickSlots, activeQuickSlot, assignQuickSlot, selectQuickSlot,
        setQuickSlotSliderValue,
        bassSequenceBank, currentBassSequenceIndex, bassChainMode, bassRandomMode,
        bassSeq,
        loadPreset,
        switchBassSequence, addBassSequence, duplicateBassSequence, deleteBassSequence, toggleBassChain, toggleBassRandom,
    } from './stores/state';
    import { isPlaying, isRecording, bpm } from '../shared/stores/transport';
    import { get, writable } from 'svelte/store';
    import PadCircle from '../shared/components/PadCircle.svelte';
    import Slider from '../shared/components/Slider.svelte';
    import PlayControls from '../shared/components/PlayControls.svelte';
    import GlitchSlider from '../shared/components/GlitchSlider.svelte';
    import NoteSequencer from '../shared/components/NoteSequencer.svelte';
    import SeqSettingsRow from '../shared/components/SeqSettingsRow.svelte';
    import StepSettingsRow from '../shared/components/StepSettingsRow.svelte';
    import SynthSettings from '../shared/components/SynthSettings.svelte';
    import SequenceBankSelector from '../shared/components/SequenceBankSelector.svelte';

    const BASS_COLOUR = '#D4A843';

    $: presetIdx = $currentPresetIndex;
    $: seqBank = $bassSequenceBank;
    $: seqIdx = $currentBassSequenceIndex;
    $: chain = $bassChainMode;
    $: random = $bassRandomMode;
    $: showSettings = $settingsOpen;
    $: settingsVals = $settingsValues;
    $: slots = $quickSlots;
    $: activeSlot = $activeQuickSlot;
    $: activeSlotParam = activeSlot !== null ? slots[activeSlot] : null;
    $: qsLabel = activeSlotParam?.name ?? '';
    $: qsValue = activeSlotParam
        ? ((settingsVals[activeSlotParam.id] ?? activeSlotParam.default) - activeSlotParam.min) / (activeSlotParam.max - activeSlotParam.min) * 100
        : 0;
    $: ({ seqSettingsOpen: seqOpenStore, stepSettingsOpen: stepOpenStore } = bassSeq);
    $: seqOpen = $seqOpenStore;
    $: stepOpen = $stepOpenStore;

    $: activePadIndex = -1;
    const triggeredPad = writable(new Set<number>());
    $: triggered = $triggeredPad;

    let pressStartStep = -1;

    function handlePadDown(i: number) {
        const pad = NOTE_PADS[i];
        triggerNote(36 + pad.semitones);
        triggeredPad.set(new Set([i]));
        setTimeout(() => { triggeredPad.set(new Set()); }, 120);

        if (!get(isRecording)) return;
        if (get(isPlaying)) {
            pressStartStep = get(bassSeq.seqCurrentStep);
            bassSeq.setStepFromNotes(pressStartStep, [36 + pad.semitones], pad.label);
        } else {
            pressStartStep = get(bassSeq.seqSelectedStep);
            bassSeq.setSeqStepFromNotes([36 + pad.semitones], pad.label);
        }
    }

    function handlePadClick(_i: number, durationMs: number) {
        if (!get(isRecording) || pressStartStep < 0) { pressStartStep = -1; return; }
        const div = get(bassSeq.seqTimeDivision);
        const stepMs = 60000 / get(bpm) / Math.pow(2, div);
        const gatePct = Math.max(5, Math.round(durationMs / stepMs * 100));
        bassSeq.setStepGatePctAt(pressStartStep, gatePct);
        if (!get(isPlaying)) {
            const numSteps = get(bassSeq.seqSteps).length;
            const next = (pressStartStep + Math.max(1, Math.ceil(gatePct / 100))) % numSteps;
            bassSeq.selectSeqStep(next);
        }
        pressStartStep = -1;
    }

    function handlePresetChange(e: Event) {
        loadPreset(Number((e.target as HTMLSelectElement).value));
    }
</script>

<div class="bass-panel">
    <div class="top-bar">
        <select class="top-select" value={presetIdx} onchange={handlePresetChange}>
            {#each PRESETS as preset, i}
                <option value={i}>{preset.name}</option>
            {/each}
        </select>
        <button class="bar-btn" class:active={seqOpen} onclick={bassSeq.toggleSeqSettings}>SEQ</button>
        <button class="bar-btn" class:active={stepOpen} onclick={bassSeq.toggleStepSettings}>STEP</button>
        <button class="bar-btn" class:active={showSettings} onclick={toggleSettings}>SETTINGS</button>
    </div>
    {#if showSettings}
        <SynthSettings
            sections={BASS_SETTINGS}
            colour={BASS_COLOUR}
            values={settingsVals}
            onParamChange={setSettingsParam}
            quickSlots={slots}
            onAssignQuickSlot={assignQuickSlot}
        />
    {:else}
        {#if seqOpen}
            <div class="drawer-row"><SeqSettingsRow colour={BASS_COLOUR} seq={bassSeq} /></div>
        {:else if stepOpen}
            <div class="drawer-row"><StepSettingsRow colour={BASS_COLOUR} seq={bassSeq} /></div>
        {/if}
        <NoteSequencer colour={BASS_COLOUR} seq={bassSeq} />
        <SequenceBankSelector
            currentIndex={seqIdx}
            count={seqBank.length}
            maxCount={8}
            colour={BASS_COLOUR}
            onSelect={switchBassSequence}
            onAdd={addBassSequence}
            onDuplicate={duplicateBassSequence}
            onDelete={deleteBassSequence}
            chainActive={chain}
            randomActive={random}
            onToggleChain={toggleBassChain}
            onToggleRandom={toggleBassRandom}
        />
    {/if}
    <PadCircle
        voices={NOTE_PADS.map(p => ({ id: p.id, label: p.label, colour: p.colour }))}
        params={[]}
        selectedVoice={activePadIndex}
        selectedParam=""
        triggeredVoices={triggered}
        onPadClick={handlePadClick}
        onPadDown={handlePadDown}
        onParamSelect={() => {}}
        quickSlots={slots}
        activeQuickSlot={activeSlot}
        colour={BASS_COLOUR}
        onQuickSlotSelect={selectQuickSlot}
    />
    <PlayControls>
        <GlitchSlider glitchSize={bassSeq.glitchSize} onGlitch={bassSeq.setGlitch} colour={BASS_COLOUR} />
    </PlayControls>
    <Slider
        label={qsLabel}
        value={qsValue}
        colour={BASS_COLOUR}
        onChange={setQuickSlotSliderValue}
    />
</div>

<style>
    .bass-panel { flex: 1; display: flex; flex-direction: column; }
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
