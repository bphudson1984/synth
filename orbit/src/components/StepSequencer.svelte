<script lang="ts">
    import { VOICES, NUM_STEPS } from '../constants';
    import { selectedVoice, patterns, currentStep, isPlaying, toggleStep } from '../stores/state';

    // Get current voice's pattern and colour
    $: voice = $selectedVoice;
    $: colour = VOICES[voice].colour;
    $: pattern = $patterns[voice];
    $: step = $currentStep;
    $: playing = $isPlaying;
</script>

<div class="sequencer" style="--step-colour: {colour}">
    <div class="grid">
        {#each Array(NUM_STEPS) as _, i}
            <button
                class="step"
                class:active={pattern[i]}
                class:playhead={playing && step === i}
                style={pattern[i] ? `background: ${colour}` : ''}
                onclick={() => toggleStep(i)}
                aria-label="Step {i + 1}"
            ></button>
        {/each}
    </div>
</div>

<style>
    .sequencer { padding: 0 24px; }
    .grid {
        display: grid;
        grid-template-columns: repeat(8, 1fr);
        gap: 4px;
    }
    .step {
        aspect-ratio: 1;
        border-radius: 4px;
        border: 0.5px solid var(--orbit-border, #333);
        background: var(--orbit-well, #1a1a1a);
        cursor: pointer;
        transition: background 80ms cubic-bezier(0.2, 0.8, 0.3, 1);
        position: relative;
    }
    .step:active { transform: scale(0.95); }
    .step.active { border: none; }
    .step.playhead {
        box-shadow: 0 0 0 1.5px var(--orbit-ink, #eee);
    }
    .step.playhead:not(.active) {
        background: var(--orbit-well-bright, #262626);
    }
    .step.playhead.active {
        box-shadow: 0 0 0 1.5px var(--orbit-ink, #eee), 0 0 10px var(--step-colour);
    }
    .step.playhead::after {
        content: '';
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        height: 2px;
        background: var(--orbit-ink, #eee);
    }
</style>
