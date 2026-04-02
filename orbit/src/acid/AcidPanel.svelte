<script lang="ts">
    import { get, writable } from 'svelte/store';
    import { NOTE_PADS, ACID_COLOUR, ACID_SETTINGS } from './constants';
    import { PRESETS } from './presets';
    import {
        currentPresetIndex, currentTranspose,
        settingsOpen, settingsValues, toggleSettings, setSettingsParam,
        quickSlots, activeQuickSlot, assignQuickSlot, selectQuickSlot,
        setQuickSlotSliderValue,
        loadPreset, randomizePattern, transposePattern,
    } from './stores/state';
    import PadCircle from '../shared/components/PadCircle.svelte';
    import Slider from '../shared/components/Slider.svelte';
    import PlayControls from '../shared/components/PlayControls.svelte';
    import AcidSequencer from './AcidSequencer.svelte';
    import AcidTransport from './AcidTransport.svelte';
    import SynthSettings from '../shared/components/SynthSettings.svelte';

    $: presetIdx = $currentPresetIndex;
    $: transpose = $currentTranspose;
    $: showSettings = $settingsOpen;
    $: settingsVals = $settingsValues;
    $: slots = $quickSlots;
    $: activeSlot = $activeQuickSlot;
    $: activeSlotParam = activeSlot !== null ? slots[activeSlot] : null;
    $: qsLabel = activeSlotParam?.name ?? '';
    $: qsValue = activeSlotParam
        ? ((settingsVals[activeSlotParam.id] ?? activeSlotParam.default) - activeSlotParam.min) / (activeSlotParam.max - activeSlotParam.min) * 100
        : 0;

    // Highlight the pad matching the current transposition
    $: activePadIndex = NOTE_PADS.findIndex(p => p.semitones === transpose);
    const triggeredPad = writable(new Set<number>());
    $: triggered = $triggeredPad;

    function handlePadDown(i: number) {
        transposePattern(NOTE_PADS[i].semitones);
        triggeredPad.set(new Set([i]));
        setTimeout(() => { triggeredPad.set(new Set()); }, 120);
    }

    function handlePadClick(_i: number, _durationMs: number) {}

    function handlePresetChange(e: Event) {
        loadPreset(Number((e.target as HTMLSelectElement).value));
    }
</script>

<div class="acid-panel">
    <div class="preset-bar">
        <select class="preset-select" value={presetIdx} onchange={handlePresetChange}>
            {#each PRESETS as preset, i}
                <option value={i}>{preset.name}</option>
            {/each}
        </select>
        <button class="rnd-btn" onclick={randomizePattern}>RND</button>
        <button class="rnd-btn" class:active={showSettings} onclick={toggleSettings}>SETTINGS</button>
    </div>
    {#if showSettings}
        <SynthSettings
            sections={ACID_SETTINGS}
            colour={ACID_COLOUR}
            values={settingsVals}
            onParamChange={setSettingsParam}
            quickSlots={slots}
            onAssignQuickSlot={assignQuickSlot}
        />
    {:else}
        <AcidSequencer />
        <AcidTransport />
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
        colour={ACID_COLOUR}
        onQuickSlotSelect={selectQuickSlot}
    />
    <PlayControls />
    <Slider label={qsLabel} value={qsValue} colour={ACID_COLOUR} onChange={setQuickSlotSliderValue} />
</div>

<style>
    .acid-panel { flex: 1; display: flex; flex-direction: column; }
    .preset-bar { display: flex; justify-content: center; align-items: center; gap: 8px; padding: 8px 24px 4px; }
    .preset-select { flex: 1; max-width: 280px; padding: 6px 12px; font-family: 'JetBrains Mono', monospace; font-size: 11px; font-weight: 500; letter-spacing: 0.5px; text-transform: uppercase; background: var(--orbit-well, #1a1a1a); color: var(--orbit-ink, #eee); border: 1px solid var(--orbit-border, #333); border-radius: 12px; outline: none; cursor: pointer; -webkit-appearance: none; appearance: none; background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='6'%3E%3Cpath d='M0 0l5 6 5-6z' fill='%23666'/%3E%3C/svg%3E"); background-repeat: no-repeat; background-position: right 12px center; padding-right: 32px; }
    .preset-select:focus { border-color: var(--orbit-ink, #eee); }
    .preset-select option { background: var(--orbit-surface, #111); color: var(--orbit-ink, #eee); }
    .rnd-btn { padding: 6px 14px; font-family: 'JetBrains Mono', monospace; font-size: 11px; font-weight: 500; letter-spacing: 1px; background: transparent; color: var(--orbit-hint, #666); border: 1.5px solid var(--orbit-border, #444); border-radius: 12px; cursor: pointer; transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1); }
    .rnd-btn:active { background: var(--orbit-ink, #eee); color: var(--orbit-surface, #111); }
    .rnd-btn.active { background: var(--orbit-ink, #eee); color: var(--orbit-surface, #111); border-color: var(--orbit-ink, #eee); }
</style>
