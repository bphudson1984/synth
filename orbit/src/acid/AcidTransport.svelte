<script lang="ts">
    import {
        isPlaying, waveform, selectedStep, stepAccents, stepSlides,
        togglePlay, toggleWaveform, toggleStepAccent, toggleStepSlide, shiftOctave,
    } from './stores/state';
    import { bpm, MIN_BPM, MAX_BPM } from '../shared/stores/transport';
    import { ACID_COLOUR } from './constants';

    function adjustBpm(delta: number) {
        bpm.update(b => Math.max(MIN_BPM, Math.min(MAX_BPM, b + delta)));
    }

    $: playing = $isPlaying;
    $: wave = $waveform;
    $: currentBpm = $bpm;
    $: sel = $selectedStep;
    $: accents = $stepAccents;
    $: slides = $stepSlides;
    $: selAccent = accents[sel];
    $: selSlide = slides[sel];
</script>

<div class="transport">
    <div class="left-controls">
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
    <div class="bpm-section">
        <button class="bpm-btn" onclick={() => adjustBpm(-1)}>−</button>
        <span class="bpm-value">{currentBpm}</span>
        <button class="bpm-btn" onclick={() => adjustBpm(1)}>+</button>
    </div>
    <div class="transport-buttons">
        <button class="transport-btn" class:active={!playing} onclick={togglePlay} aria-label="Stop"><div class="stop-icon"></div></button>
        <button class="transport-btn" class:active={playing} onclick={togglePlay} aria-label="Play"><div class="play-icon"></div></button>
    </div>
</div>

<style>
    .transport { display: flex; justify-content: space-between; align-items: center; padding: 8px 24px; }
    .left-controls { display: flex; gap: 6px; align-items: center; }
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
    .bpm-section { display: flex; align-items: center; gap: 8px; }
    .bpm-value { font-size: 22px; font-weight: 500; color: var(--orbit-ink, #eee); letter-spacing: -0.5px; min-width: 42px; text-align: center; }
    .bpm-btn { width: 28px; height: 28px; border-radius: 50%; border: 1px solid var(--orbit-border, #444); background: transparent; color: var(--orbit-ink, #eee); font-size: 16px; cursor: pointer; display: flex; align-items: center; justify-content: center; }
    .transport-buttons { display: flex; gap: 12px; }
    .transport-btn { width: 44px; height: 44px; border-radius: 50%; border: 1.5px solid var(--orbit-ink, #eee); background: transparent; cursor: pointer; display: flex; align-items: center; justify-content: center; transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1); }
    .transport-btn.active { background: var(--orbit-ink, #eee); }
    .stop-icon { width: 12px; height: 12px; background: var(--orbit-ink, #eee); border-radius: 1px; }
    .transport-btn.active .stop-icon { background: var(--orbit-surface, #111); }
    .play-icon { width: 0; height: 0; border-left: 10px solid var(--orbit-ink, #eee); border-top: 7px solid transparent; border-bottom: 7px solid transparent; margin-left: 3px; }
    .transport-btn.active .play-icon { border-left-color: var(--orbit-surface, #111); }
</style>
