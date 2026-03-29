<script lang="ts">
    import { NUM_STEPS, ACID_COLOUR, midiNoteName } from './constants';
    import {
        selectedStep, stepNotes, stepGates, stepAccents, stepSlides,
        currentStep, isPlaying, selectStep, toggleStepGate,
    } from './stores/state';

    $: sel = $selectedStep;
    $: notes = $stepNotes;
    $: gates = $stepGates;
    $: accents = $stepAccents;
    $: slides = $stepSlides;
    $: step = $currentStep;
    $: playing = $isPlaying;
</script>

<div class="acid-seq" style="--acid-colour: {ACID_COLOUR}">
    <div class="grid">
        {#each Array(NUM_STEPS) as _, i}
            <button
                class="cell"
                class:active={gates[i]}
                class:selected={sel === i}
                class:playhead={playing && step === i}
                onclick={() => selectStep(i)}
                ondblclick={() => toggleStepGate(i)}
                aria-label="Step {i + 1}"
            >
                {#if accents[i] && gates[i]}<span class="accent-bar"></span>{/if}
                {#if gates[i]}<span class="note-label">{midiNoteName(notes[i])}</span>{/if}
                {#if slides[i] && gates[i]}<span class="slide-mark">&#x25B8;</span>{/if}
            </button>
        {/each}
    </div>
</div>

<style>
    .acid-seq { padding: 0 12px; }
    .grid { display: grid; grid-template-columns: repeat(8, 1fr); gap: 4px; }
    .cell {
        aspect-ratio: 1.2; border-radius: 4px;
        border: 0.5px solid var(--orbit-border, #333);
        background: var(--orbit-well, #1a1a1a);
        cursor: pointer; position: relative;
        display: flex; align-items: center; justify-content: center;
        transition: background 80ms cubic-bezier(0.2, 0.8, 0.3, 1); padding: 0;
    }
    .cell:active { transform: scale(0.95); }
    .cell.active { background: color-mix(in srgb, var(--acid-colour) 30%, var(--orbit-well)); border-color: var(--acid-colour); }
    .cell.selected { box-shadow: 0 0 0 1.5px var(--acid-colour); }
    .cell.playhead { box-shadow: 0 0 0 1.5px var(--orbit-ink, #eee); }
    .cell.playhead.selected { box-shadow: 0 0 0 1.5px var(--orbit-ink, #eee), 0 0 8px var(--acid-colour); }
    .cell.playhead:not(.active) { background: var(--orbit-well-bright, #262626); }
    .cell.playhead::after { content: ''; position: absolute; bottom: 0; left: 0; right: 0; height: 2px; background: var(--orbit-ink, #eee); }
    .note-label { font-size: 9px; font-weight: 500; color: var(--orbit-ink, #eee); pointer-events: none; letter-spacing: 0.3px; }
    .accent-bar { position: absolute; top: 1px; left: 2px; right: 2px; height: 2px; background: var(--acid-colour); border-radius: 1px; pointer-events: none; }
    .slide-mark { position: absolute; right: 1px; bottom: 0; font-size: 8px; color: var(--acid-colour); pointer-events: none; line-height: 1; }
</style>
