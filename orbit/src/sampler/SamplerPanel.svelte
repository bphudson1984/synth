<script lang="ts">
    import { SAMPLER_PADS, SAMPLER_SETTINGS, SAMPLER_COLOUR } from './constants';
    import {
        padStates, selectedPad, settingsOpen, padSettings,
        isRecordingMic, startMicRecording, stopMicRecording,
        isLoadingKit, loadStockKit,
        samplerSeq, samplerSequenceBank, currentSequenceIndex,
        samplerChainMode, samplerRandomMode,
        quickSlots, activeQuickSlot, assignQuickSlot, selectQuickSlot,
        setQuickSlotSliderValue, getSelectedPadSettings,
        selectPad, toggleSettings, triggerPad, loadSampleFile,
        setSelectedPadParam,
        switchSequence, addSequence, duplicateSequence, deleteSequence,
        toggleSamplerChain, toggleSamplerRandom,
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
    import WaveformDisplay from './components/WaveformDisplay.svelte';

    $: pads = $padStates;
    $: selPad = $selectedPad;
    $: seqBank = $samplerSequenceBank;
    $: seqIdx = $currentSequenceIndex;
    $: chain = $samplerChainMode;
    $: random = $samplerRandomMode;
    $: showSettings = $settingsOpen;
    $: micRec = $isRecordingMic;
    $: loadingKit = $isLoadingKit;
    $: slots = $quickSlots;
    $: activeSlot = $activeQuickSlot;
    $: activeSlotParam = activeSlot !== null ? slots[activeSlot] : null;
    $: selectedPadVals = getSelectedPadSettings();
    $: qsLabel = activeSlotParam?.name ?? '';
    $: qsValue = activeSlotParam
        ? ((selectedPadVals[activeSlotParam.id] ?? activeSlotParam.default) - activeSlotParam.min) / (activeSlotParam.max - activeSlotParam.min) * 100
        : 0;
    $: ({ seqSettingsOpen: seqOpenStore, stepSettingsOpen: stepOpenStore } = samplerSeq);
    $: seqOpen = $seqOpenStore;
    $: stepOpen = $stepOpenStore;

    const triggeredPad = writable(new Set<number>());
    $: triggered = $triggeredPad;

    function handlePadDown(i: number) {
        selectPad(i);
        if (pads[i].loaded) {
            triggerPad(i);
        }
        triggeredPad.set(new Set([i]));
        setTimeout(() => { triggeredPad.set(new Set()); }, 120);

        if (!get(isRecording)) return;
        // Record pad trigger as note (MIDI 36 + padIndex)
        const note = SAMPLER_PADS[i].midiNote;
        if (get(isPlaying)) {
            const step = get(samplerSeq.seqCurrentStep);
            samplerSeq.setStepFromNotes(step, [note], `P${i + 1}`);
        } else {
            samplerSeq.setSeqStepFromNotes([note], `P${i + 1}`);
        }
    }

    function handlePadClick(_i: number, _durationMs: number) {}

    // File drop handling
    function handleDrop(e: DragEvent) {
        e.preventDefault();
        const file = e.dataTransfer?.files[0];
        if (file && file.type.startsWith('audio/')) {
            loadSampleFile(selPad, file);
        }
    }

    function handleDragOver(e: DragEvent) { e.preventDefault(); }

    function handleFileInput(e: Event) {
        const file = (e.target as HTMLInputElement).files?.[0];
        if (file) loadSampleFile(selPad, file);
    }
</script>

<div class="sampler-panel" ondrop={handleDrop} ondragover={handleDragOver}>
    <div class="top-bar">
        <label class="load-btn">
            LOAD
            <input type="file" accept="audio/*" onchange={handleFileInput} style="display:none" />
        </label>
        <button class="rec-btn" class:recording={micRec} onclick={() => micRec ? stopMicRecording() : startMicRecording()}>
            {micRec ? 'STOP' : 'REC'}
        </button>
        <button class="kit-btn" onclick={loadStockKit} disabled={loadingKit}>
            {loadingKit ? 'LOADING...' : 'STOCK KIT'}
        </button>
        <span class="pad-label">PAD {selPad + 1}{pads[selPad].loaded ? ` — ${pads[selPad].name}` : ''}</span>
        <button class="bar-btn" class:active={seqOpen} onclick={samplerSeq.toggleSeqSettings}>SEQ</button>
        <button class="bar-btn" class:active={stepOpen} onclick={samplerSeq.toggleStepSettings}>STEP</button>
        <button class="bar-btn" class:active={showSettings} onclick={toggleSettings}>SETTINGS</button>
    </div>
    {#if showSettings}
        <SynthSettings
            sections={SAMPLER_SETTINGS}
            colour={SAMPLER_COLOUR}
            values={selectedPadVals}
            onParamChange={setSelectedPadParam}
            quickSlots={slots}
            onAssignQuickSlot={assignQuickSlot}
        />
    {:else}
        <WaveformDisplay
            waveform={pads[selPad].waveform}
            colour={SAMPLER_COLOUR}
            name={pads[selPad].name}
        />
        {#if seqOpen}
            <div class="drawer-row"><SeqSettingsRow colour={SAMPLER_COLOUR} seq={samplerSeq} /></div>
        {:else if stepOpen}
            <div class="drawer-row"><StepSettingsRow colour={SAMPLER_COLOUR} seq={samplerSeq} /></div>
        {/if}
        <NoteSequencer colour={SAMPLER_COLOUR} seq={samplerSeq} />
        <SequenceBankSelector
            currentIndex={seqIdx}
            count={seqBank.length}
            maxCount={8}
            colour={SAMPLER_COLOUR}
            onSelect={switchSequence}
            onAdd={addSequence}
            onDuplicate={duplicateSequence}
            onDelete={deleteSequence}
            chainActive={chain}
            randomActive={random}
            onToggleChain={toggleSamplerChain}
            onToggleRandom={toggleSamplerRandom}
        />
    {/if}
    <PadCircle
        voices={SAMPLER_PADS.map((p, i) => ({
            id: p.id,
            label: pads[i].loaded ? p.label : `${p.label}`,
            colour: pads[i].loaded ? SAMPLER_COLOUR : '#444',
        }))}
        params={[]}
        selectedVoice={selPad}
        selectedParam=""
        triggeredVoices={triggered}
        onPadClick={handlePadClick}
        onPadDown={handlePadDown}
        onParamSelect={() => {}}
        quickSlots={slots}
        activeQuickSlot={activeSlot}
        colour={SAMPLER_COLOUR}
        onQuickSlotSelect={selectQuickSlot}
    />
    <PlayControls>
        <GlitchSlider glitchSize={samplerSeq.glitchSize} onGlitch={samplerSeq.setGlitch} colour={SAMPLER_COLOUR} />
    </PlayControls>
    <Slider
        label={qsLabel}
        value={qsValue}
        colour={SAMPLER_COLOUR}
        onChange={setQuickSlotSliderValue}
    />
</div>

<style>
    .sampler-panel { flex: 1; display: flex; flex-direction: column; }
    .top-bar {
        display: flex; justify-content: center; align-items: center;
        gap: 6px; padding: 8px 16px 4px; flex-wrap: wrap;
    }
    .load-btn {
        padding: 4px 12px;
        font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500;
        letter-spacing: 1px; background: transparent; color: #E05555;
        border: 1.5px solid #E05555; border-radius: 12px;
        cursor: pointer; transition: all 120ms;
    }
    .load-btn:hover { background: #E05555; color: #111; }
    .rec-btn {
        padding: 4px 12px;
        font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500;
        letter-spacing: 1px; background: transparent; color: #D84040;
        border: 1.5px solid #D84040; border-radius: 12px;
        cursor: pointer; transition: all 120ms;
    }
    .rec-btn:hover { background: #D84040; color: #fff; }
    .rec-btn.recording { background: #D84040; color: #fff; animation: pulse-rec 1s infinite; }
    @keyframes pulse-rec { 0%, 100% { opacity: 1; } 50% { opacity: 0.5; } }
    .kit-btn {
        padding: 4px 12px;
        font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500;
        letter-spacing: 1px; background: transparent; color: var(--orbit-hint, #666);
        border: 1.5px solid var(--orbit-border, #444); border-radius: 12px;
        cursor: pointer; transition: all 120ms;
    }
    .kit-btn:hover { background: var(--orbit-ink, #eee); color: var(--orbit-surface, #111); }
    .kit-btn:disabled { opacity: 0.5; cursor: wait; }
    .pad-label {
        font-family: 'JetBrains Mono', monospace; font-size: 10px;
        color: var(--orbit-label, #aaa); letter-spacing: 0.5px;
        overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
        max-width: 140px;
    }
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
