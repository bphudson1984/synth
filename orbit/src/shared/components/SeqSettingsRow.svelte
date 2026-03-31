<script lang="ts">
    import type { NoteSequencerStore } from '../stores/noteSequencer';

    export let colour: string;
    export let seq: NoteSequencerStore;

    $: ({ seqDirection: dirStore, seqSwing: swingStore, seqTimeDivision: tdivStore } = seq);
    $: dir = $dirStore;
    $: swing = $swingStore;
    $: tdiv = $tdivStore;

    const directions = [
        { value: 0, label: 'FWD' }, { value: 1, label: 'REV' },
        { value: 2, label: 'PING' }, { value: 3, label: 'RND' },
    ];
    const timeDivs = [
        { value: 0, label: '1/4' }, { value: 1, label: '1/8' },
        { value: 2, label: '1/16' }, { value: 3, label: '1/32' },
    ];
</script>

<span class="lbl">DIR</span>
{#each directions as d}
    <button class="btn" class:active={dir === d.value} onclick={() => seq.setSeqDirection(d.value)} style="--c: {colour}">{d.label}</button>
{/each}
<span class="sep"></span>
<span class="lbl">RATE</span>
{#each timeDivs as t}
    <button class="btn" class:active={tdiv === t.value} onclick={() => seq.setSeqTimeDivision(t.value)} style="--c: {colour}">{t.label}</button>
{/each}
<span class="sep"></span>
<span class="lbl">SWG</span>
<input type="range" min="0" max="100" step="1" value={swing}
    oninput={(e) => seq.setSeqSwing(Number((e.target as HTMLInputElement).value))}
    class="inline-slider" style="--c: {colour}; --fill-pct: {swing}%"
/>
<span class="val" style="color: {colour}">{swing}</span>
<span class="sep"></span>
<button class="btn" onclick={() => seq.rotatePattern(-1)} style="--c: {colour}">←</button>
<button class="btn" onclick={() => seq.rotatePattern(1)} style="--c: {colour}">→</button>
<button class="btn" onclick={seq.randomizeGates} style="--c: {colour}">RND</button>
<button class="btn clr" onclick={seq.clearSequence}>CLR</button>

<style>
    .lbl { font-size: 9px; font-weight: 500; letter-spacing: 1px; color: var(--orbit-hint, #666); }
    .btn { padding: 3px 8px; font-family: 'JetBrains Mono', monospace; font-size: 9px; font-weight: 500; letter-spacing: 0.3px; background: transparent; color: var(--orbit-hint, #666); border: 1px solid var(--orbit-border, #444); border-radius: 8px; cursor: pointer; transition: all 100ms; }
    .btn.active { background: var(--c); color: #fff; border-color: var(--c); }
    .btn:active { background: var(--orbit-ink, #eee); color: var(--orbit-surface, #111); }
    .btn.clr:active { background: #D84040; color: #fff; border-color: #D84040; }
    .sep { width: 1px; height: 16px; background: var(--orbit-border, #333); }
    .inline-slider { width: 60px; height: 3px; -webkit-appearance: none; appearance: none; background: linear-gradient(to right, var(--c) 0%, var(--c) var(--fill-pct), var(--orbit-border) var(--fill-pct)); border-radius: 2px; outline: none; }
    .inline-slider::-webkit-slider-thumb { -webkit-appearance: none; width: 12px; height: 12px; border-radius: 50%; background: var(--orbit-surface); border: 2px solid var(--c); cursor: pointer; }
    .inline-slider::-moz-range-thumb { width: 12px; height: 12px; border-radius: 50%; background: var(--orbit-surface); border: 2px solid var(--c); cursor: pointer; }
    .val { font-size: 9px; font-weight: 500; min-width: 18px; }
</style>
