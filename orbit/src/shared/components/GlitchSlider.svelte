<script lang="ts">
    import type { Writable } from 'svelte/store';

    export let glitchSize: Writable<number>;
    export let onGlitch: (size: number) => void;
    export let colour: string = '#E8944A';

    const STOPS = [8, 4, 2, 1];
    let active = false;
    let currentSize = 0;

    function handleStart(size: number) {
        active = true;
        currentSize = size;
        onGlitch(size);
    }

    function handleMove(e: PointerEvent) {
        if (!active) return;
        const target = (e.target as HTMLElement).closest('.glitch-slider');
        if (!target) return;
        const btns = target.querySelectorAll('.glitch-btn');
        for (const btn of btns) {
            const rect = btn.getBoundingClientRect();
            if (e.clientX >= rect.left && e.clientX <= rect.right &&
                e.clientY >= rect.top && e.clientY <= rect.bottom) {
                const size = Number((btn as HTMLElement).dataset.size);
                if (size !== currentSize) {
                    currentSize = size;
                    onGlitch(size);
                }
                break;
            }
        }
    }

    function handleEnd() {
        if (!active) {
            return;
        }
        active = false;
        currentSize = 0;
        onGlitch(0);
    }

    $: activeSize = $glitchSize;
</script>

<svelte:window onpointerup={handleEnd} onpointermove={handleMove} />

<div class="glitch-slider" style="--glitch-colour: {colour}">
    <span class="glitch-label">GLT</span>
    {#each STOPS as size}
        <button
            class="glitch-btn"
            class:active={activeSize === size}
            data-size={size}
            onpointerdown={(e) => { e.preventDefault(); handleStart(size); }}
        >{size}</button>
    {/each}
</div>

<style>
    .glitch-slider {
        display: flex;
        align-items: center;
        gap: 4px;
        touch-action: none;
        user-select: none;
        -webkit-user-select: none;
    }
    .glitch-label {
        font-size: 9px;
        font-weight: 500;
        letter-spacing: 1px;
        color: var(--orbit-hint, #666);
        margin-right: 2px;
    }
    .glitch-btn {
        width: 28px;
        height: 28px;
        border-radius: 50%;
        border: 1.5px solid var(--orbit-border, #444);
        background: transparent;
        color: var(--orbit-hint, #666);
        font-family: 'JetBrains Mono', monospace;
        font-size: 10px;
        font-weight: 500;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 80ms ease-out;
        touch-action: none;
    }
    .glitch-btn.active {
        background: var(--glitch-colour);
        color: #111;
        border-color: var(--glitch-colour);
        box-shadow: 0 0 8px color-mix(in srgb, var(--glitch-colour) 50%, transparent);
    }
</style>
