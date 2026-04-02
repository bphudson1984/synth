<script lang="ts">
    import { SCALE_NOTES, SCALE_CHORDS, LEAD_COLOUR, LEAD_SETTINGS, MODELS } from './constants';
    import {
        selectedModel, triggeredNotes, padMode, latchEnabled,
        currentLeadPreset, leadSeq,
        arpSettingsOpen, seqSettingsOpen, stepSettingsOpen,
        settingsOpen, settingsValues, toggleSettings, setSettingsParam,
        quickSlots, activeQuickSlot, assignQuickSlot, selectQuickSlot,
        setQuickSlotSliderValue,
        leadSequenceBank, currentLeadSequenceIndex, leadChainMode, leadRandomMode,
        selectModel, triggerPad,
        togglePadMode, toggleLatch, setSeqStepFromPad,
        loadLeadPreset, toggleArpSettings, toggleSeqSettings, toggleStepSettings,
        switchLeadSequence, addLeadSequence, duplicateLeadSequence, deleteLeadSequence, toggleLeadChain, toggleLeadRandom,
    } from './stores/state';
    import { LEAD_PRESETS } from './presets';
    import { get } from 'svelte/store';
    import { isPlaying, isRecording, bpm } from '../shared/stores/transport';
    import { seqTimeDivision } from './stores/state';
    import PadCircle from '../shared/components/PadCircle.svelte';
    import Slider from '../shared/components/Slider.svelte';
    import PlayControls from '../shared/components/PlayControls.svelte';
    import NoteSequencer from '../shared/components/NoteSequencer.svelte';
    import SeqSettingsRow from '../shared/components/SeqSettingsRow.svelte';
    import StepSettingsRow from '../shared/components/StepSettingsRow.svelte';
    import ArpSettings from './ArpSettings.svelte';
    import SynthSettings from '../shared/components/SynthSettings.svelte';
    import SequenceBankSelector from '../shared/components/SequenceBankSelector.svelte';

    $: model = $selectedModel;
    $: triggered = $triggeredNotes;
    $: mode = $padMode;
    $: latch = $latchEnabled;
    $: presetIdx = $currentLeadPreset;
    $: arpOpen = $arpSettingsOpen;
    $: seqOpen = $seqSettingsOpen;
    $: stepOpen = $stepSettingsOpen;
    $: showSettings = $settingsOpen;
    $: settingsVals = $settingsValues;
    $: slots = $quickSlots;
    $: activeSlot = $activeQuickSlot;
    $: activeSlotParam = activeSlot !== null ? slots[activeSlot] : null;
    $: qsLabel = activeSlotParam?.name ?? '';
    $: qsValue = activeSlotParam
        ? ((settingsVals[activeSlotParam.id] ?? activeSlotParam.default) - activeSlotParam.min) / (activeSlotParam.max - activeSlotParam.min) * 100
        : 0;
    $: anyDrawerOpen = arpOpen || seqOpen || stepOpen;
    $: seqBank = $leadSequenceBank;
    $: seqIdx = $currentLeadSequenceIndex;
    $: chain = $leadChainMode;
    $: random = $leadRandomMode;

    function handlePresetChange(e: Event) {
        loadLeadPreset(Number((e.target as HTMLSelectElement).value));
    }

    $: pads = mode === 'chord'
        ? SCALE_CHORDS.map(c => ({ id: c.id, label: c.label, colour: c.colour }))
        : SCALE_NOTES.map(n => ({ id: n.id, label: n.label, colour: n.colour }));

    function handleModelChange(e: Event) {
        selectModel(Number((e.target as HTMLSelectElement).value));
    }

    let pressStartStep = -1;

    function handlePadDown(i: number) {
        triggerPad(i);

        if (!get(isRecording)) return;

        // Record the note immediately at the current step
        if (get(isPlaying)) {
            pressStartStep = get(leadSeq.seqCurrentStep);
        } else {
            pressStartStep = get(leadSeq.seqSelectedStep);
        }
        setSeqStepFromPad(i, pressStartStep);
    }

    function handlePadClick(_i: number, durationMs: number) {
        if (!get(isRecording) || pressStartStep < 0) { pressStartStep = -1; return; }

        // Calculate gate as percentage of step duration — can exceed 100 to sustain across steps
        const stepMs = 60000 / get(bpm) / Math.pow(2, get(seqTimeDivision));
        const gatePct = Math.max(5, Math.round(durationMs / stepMs * 100));

        // Update the gate length on the step that was recorded on press
        leadSeq.setStepGatePctAt(pressStartStep, gatePct);

        // Advance cursor past the sustained steps (step-entry mode)
        if (!get(isPlaying)) {
            const totalSteps = Math.max(1, Math.ceil(gatePct / 100));
            const numSteps = get(leadSeq.seqSteps).length;
            const endStep = (pressStartStep + totalSteps) % numSteps;
            const page = get(leadSeq.seqCurrentPage);
            const pageStart = page * 16;
            const pageEnd = pageStart + 15;
            const next = endStep > pageEnd ? pageStart : endStep;
            leadSeq.selectSeqStep(next);
        }

        pressStartStep = -1;
    }
</script>

<div class="lead-panel">
    <div class="top-bar">
        <select class="top-select" value={model} onchange={handleModelChange}>
            {#each MODELS as name, i}
                <option value={i}>{name}</option>
            {/each}
        </select>
        <select class="top-select" value={presetIdx} onchange={handlePresetChange}>
            <option value={-1} disabled>PRESET</option>
            {#each LEAD_PRESETS as p, i}
                <option value={i}>{p.name}</option>
            {/each}
        </select>
        <button class="bar-btn" class:active={arpOpen} onclick={toggleArpSettings}>ARP</button>
        <button class="bar-btn" class:active={seqOpen} onclick={toggleSeqSettings}>SEQ</button>
        <button class="bar-btn" class:active={stepOpen} onclick={toggleStepSettings}>STEP</button>
        <button class="bar-btn" class:active={showSettings} onclick={toggleSettings}>SETTINGS</button>
    </div>
    {#if showSettings}
        <SynthSettings
            sections={LEAD_SETTINGS}
            colour={LEAD_COLOUR}
            values={settingsVals}
            onParamChange={setSettingsParam}
            quickSlots={slots}
            onAssignQuickSlot={assignQuickSlot}
        />
    {:else}
        {#if arpOpen}
            <div class="drawer-row"><ArpSettings /></div>
        {:else if seqOpen}
            <div class="drawer-row"><SeqSettingsRow colour={LEAD_COLOUR} seq={leadSeq} /></div>
        {:else if stepOpen}
            <div class="drawer-row"><StepSettingsRow colour={LEAD_COLOUR} seq={leadSeq} /></div>
        {/if}
        <NoteSequencer colour={LEAD_COLOUR} seq={leadSeq} />
        <SequenceBankSelector
            currentIndex={seqIdx}
            count={seqBank.length}
            maxCount={8}
            colour={LEAD_COLOUR}
            onSelect={switchLeadSequence}
            onAdd={addLeadSequence}
            onDuplicate={duplicateLeadSequence}
            onDelete={deleteLeadSequence}
            chainActive={chain}
            randomActive={random}
            onToggleChain={toggleLeadChain}
            onToggleRandom={toggleLeadRandom}
        />
    {/if}
    <PadCircle
        voices={pads}
        params={[]}
        selectedVoice={-1}
        selectedParam=""
        triggeredVoices={triggered}
        onPadClick={handlePadClick}
        onPadDown={handlePadDown}
        onParamSelect={() => {}}
        quickSlots={slots}
        activeQuickSlot={activeSlot}
        colour={LEAD_COLOUR}
        onQuickSlotSelect={selectQuickSlot}
    />
    <div class="pad-controls">
        <button class="pill-btn" class:active={mode === 'chord'} onclick={togglePadMode}>
            {mode === 'note' ? 'NOTE' : 'CHRD'}
        </button>
        <button class="pill-btn latch" class:active={latch} onclick={toggleLatch}>LATCH</button>
    </div>
    <PlayControls />
    <Slider
        label={qsLabel}
        value={qsValue}
        colour={LEAD_COLOUR}
        onChange={setQuickSlotSliderValue}
    />
</div>

<style>
    .lead-panel { flex: 1; display: flex; flex-direction: column; }
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
    .pad-controls {
        display: flex; justify-content: center; gap: 8px; padding: 0 24px;
    }
    .pill-btn {
        padding: 4px 14px;
        font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500;
        letter-spacing: 1px; background: transparent; color: var(--orbit-hint, #666);
        border: 1.5px solid var(--orbit-border, #444); border-radius: 12px;
        cursor: pointer; transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1);
    }
    .pill-btn:active { background: var(--orbit-ink, #eee); color: var(--orbit-surface, #111); }
    .pill-btn.latch.active { background: #D4B830; color: #111; border-color: #D4B830; }
</style>
