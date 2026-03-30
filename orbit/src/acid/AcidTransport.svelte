<script lang="ts">
    import {
        waveform, selectedStep, stepAccents, stepSlides,
        toggleWaveform, toggleStepAccent, toggleStepSlide, shiftOctave,
    } from './stores/state';
    import { ACID_COLOUR } from './constants';

    $: wave = $waveform;
    $: sel = $selectedStep;
    $: accents = $stepAccents;
    $: slides = $stepSlides;
    $: selAccent = accents[sel];
    $: selSlide = slides[sel];
</script>

<div class="transport">
    <div class="wave-toggle">
        <button class="eng-btn" class:active={wave === 'saw'} onclick={toggleWaveform}>SAW</button>
        <button class="eng-btn" class:active={wave === 'square'} onclick={toggleWaveform}>SQR</button>
    </div>
    <button class="mod-btn" class:active={selAccent} onclick={toggleStepAccent} style="--mod-colour: {ACID_COLOUR}">ACC</button>
    <button class="mod-btn" class:active={selSlide} onclick={toggleStepSlide} style="--mod-colour: {ACID_COLOUR}">SLD</button>
    <div class="oct-toggle">
        <button class="oct-btn" onclick={() => shiftOctave(-1)}>OCT-</button>
        <button class="oct-btn" onclick={() => shiftOctave(1)}>OCT+</button>
    </div>
</div>

<style>
    .transport { display: flex; justify-content: center; align-items: center; gap: 6px; padding: 8px 24px; flex-wrap: wrap; }
    .wave-toggle { display: flex; gap: 0; }
    .eng-btn { padding: 4px 10px; font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500; letter-spacing: 0.5px; background: transparent; color: var(--orbit-hint, #666); border: 1.5px solid var(--orbit-border, #444); cursor: pointer; transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1); }
    .eng-btn:first-child { border-radius: 12px 0 0 12px; border-right: none; }
    .eng-btn:last-child { border-radius: 0 12px 12px 0; }
    .eng-btn.active { background: var(--orbit-ink, #eee); color: var(--orbit-surface, #111); border-color: var(--orbit-ink, #eee); }
    .mod-btn { padding: 4px 8px; font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500; letter-spacing: 0.5px; background: transparent; color: var(--orbit-hint, #666); border: 1.5px solid var(--orbit-border, #444); border-radius: 12px; cursor: pointer; transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1); }
    .mod-btn.active { background: var(--mod-colour); color: #fff; border-color: var(--mod-colour); }
    .oct-toggle { display: flex; gap: 0; }
    .oct-btn { padding: 4px 6px; font-family: 'JetBrains Mono', monospace; font-size: 9px; font-weight: 500; letter-spacing: 0.3px; background: transparent; color: var(--orbit-hint, #666); border: 1.5px solid var(--orbit-border, #444); cursor: pointer; transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1); }
    .oct-btn:first-child { border-radius: 12px 0 0 12px; border-right: none; }
    .oct-btn:last-child { border-radius: 0 12px 12px 0; }
    .oct-btn:active { background: var(--orbit-ink, #eee); color: var(--orbit-surface, #111); }
</style>
