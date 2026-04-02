<script lang="ts">
    import type { NoteSequencerStore } from '../stores/noteSequencer';

    export let colour: string;
    export let seq: NoteSequencerStore;

    $: ({ seqSteps: stepsStore, seqSelectedStep: selStore } = seq);
    $: sel = $selStore;
    $: steps = $stepsStore;
    $: step = steps[sel];
    $: hasStep = step?.gate;
</script>

{#if hasStep}
    <span class="lbl">VEL</span>
    <input type="range" min="0" max="100" step="1" value={step.velocity * 100 / 127}
        oninput={(e) => seq.setStepVelocity(Number((e.target as HTMLInputElement).value))}
        class="inline-slider" style="--c: {colour}; --fill-pct: {step.velocity * 100 / 127}%"
    />
    <span class="val" style="color: {colour}">{step.velocity}</span>
    <span class="sep"></span>
    <span class="lbl">LEN</span>
    {#each [1, 2, 3, 4, 8] as len}
        <button class="btn" class:active={Math.round(step.gatePct / 100) === len} onclick={() => seq.setStepGatePct(len * 100)} style="--c: {colour}">{len}</button>
    {/each}
    <span class="sep"></span>
    <span class="lbl">PROB</span>
    <input type="range" min="0" max="100" step="1" value={step.probability}
        oninput={(e) => seq.setStepProbability(Number((e.target as HTMLInputElement).value))}
        class="inline-slider" style="--c: {colour}; --fill-pct: {step.probability}%"
    />
    <span class="val" style="color: {colour}">{step.probability}%</span>
    <span class="sep"></span>
    <span class="lbl">RTCH</span>
    {#each [1, 2, 3, 4] as r}
        <button class="btn" class:active={step.ratchet === r} onclick={() => seq.setStepRatchet(r)} style="--c: {colour}">{r}x</button>
    {/each}
    <span class="sep"></span>
    <button class="btn skip" class:active={step.skip} onclick={seq.toggleStepSkip}>SKIP</button>
{:else}
    <span class="empty">Select an active step</span>
{/if}

<style>
    .lbl { font-size: 9px; font-weight: 500; letter-spacing: 1px; color: var(--orbit-hint, #666); }
    .btn { padding: 3px 8px; font-family: 'JetBrains Mono', monospace; font-size: 9px; font-weight: 500; background: transparent; color: var(--orbit-hint, #666); border: 1px solid var(--orbit-border, #444); border-radius: 8px; cursor: pointer; transition: all 100ms; }
    .btn.active { background: var(--c); color: #fff; border-color: var(--c); }
    .btn.skip.active { background: #D84040; color: #fff; border-color: #D84040; }
    .sep { width: 1px; height: 16px; background: var(--orbit-border, #333); }
    .inline-slider { width: 50px; height: 3px; -webkit-appearance: none; appearance: none; background: linear-gradient(to right, var(--c) 0%, var(--c) var(--fill-pct), var(--orbit-border) var(--fill-pct)); border-radius: 2px; outline: none; }
    .inline-slider::-webkit-slider-thumb { -webkit-appearance: none; width: 12px; height: 12px; border-radius: 50%; background: var(--orbit-surface); border: 2px solid var(--c); cursor: pointer; }
    .inline-slider::-moz-range-thumb { width: 12px; height: 12px; border-radius: 50%; background: var(--orbit-surface); border: 2px solid var(--c); cursor: pointer; }
    .val { font-size: 9px; font-weight: 500; min-width: 18px; }
    .empty { font-size: 9px; color: var(--orbit-hint, #666); }
</style>
