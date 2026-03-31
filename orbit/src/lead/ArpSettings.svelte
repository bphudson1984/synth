<script lang="ts">
    import { LEAD_COLOUR } from './constants';
    import {
        arpMode, arpDivision, arpOctaves,
        setArpMode, setArpDivision, setArpOctaves,
    } from './stores/state';

    $: mode = $arpMode;
    $: div = $arpDivision;
    $: oct = $arpOctaves;

    const modes = [
        { value: 'off', label: 'OFF' }, { value: 'up', label: 'UP' },
        { value: 'down', label: 'DN' }, { value: 'updown', label: 'U/D' },
        { value: 'random', label: 'RND' },
    ];
    const divisions = ['1/4', '1/8', '1/16', '1/32'];
</script>

<span class="lbl">MODE</span>
{#each modes as m}
    <button class="btn" class:active={mode === m.value} onclick={() => setArpMode(m.value)} style="--c: {LEAD_COLOUR}">{m.label}</button>
{/each}
<span class="sep"></span>
<span class="lbl">DIV</span>
{#each divisions as d}
    <button class="btn" class:active={div === d} onclick={() => setArpDivision(d)} style="--c: {LEAD_COLOUR}">{d}</button>
{/each}
<span class="sep"></span>
<span class="lbl">OCT</span>
{#each [1, 2, 3] as o}
    <button class="btn" class:active={oct === o} onclick={() => setArpOctaves(o)} style="--c: {LEAD_COLOUR}">{o}</button>
{/each}

<style>
    .lbl { font-size: 9px; font-weight: 500; letter-spacing: 1px; color: var(--orbit-hint, #666); }
    .btn {
        padding: 3px 8px;
        font-family: 'JetBrains Mono', monospace; font-size: 9px; font-weight: 500;
        letter-spacing: 0.3px; background: transparent; color: var(--orbit-hint, #666);
        border: 1px solid var(--orbit-border, #444); border-radius: 8px;
        cursor: pointer; transition: all 100ms;
    }
    .btn.active { background: var(--c); color: #fff; border-color: var(--c); }
    .sep { width: 1px; height: 16px; background: var(--orbit-border, #333); }
</style>
