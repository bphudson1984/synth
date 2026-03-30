<script lang="ts">
    import { globalEngine, setGlobalEngine } from './stores/state';

    $: engine = $globalEngine;
</script>

<div class="transport">
    <div class="engine-toggle">
        <button class="eng-btn" class:active={engine === '808'} onclick={() => setGlobalEngine('808')}>808</button>
        <button class="eng-btn" class:active={engine === '909'} onclick={() => setGlobalEngine('909')}>909</button>
    </div>
</div>

<style>
    .transport {
        display: flex;
        justify-content: center;
        align-items: center;
        padding: 8px 24px;
    }
    .engine-toggle { display: flex; gap: 0; }
    .eng-btn {
        padding: 4px 12px;
        font-family: 'JetBrains Mono', monospace;
        font-size: 11px; font-weight: 500; letter-spacing: 0.5px;
        background: transparent; color: var(--orbit-hint, #666);
        border: 1.5px solid var(--orbit-border, #444);
        cursor: pointer; transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1);
    }
    .eng-btn:first-child { border-radius: 12px 0 0 12px; border-right: none; }
    .eng-btn:last-child { border-radius: 0 12px 12px 0; }
    .eng-btn.active { background: var(--orbit-ink, #eee); color: var(--orbit-surface, #111); border-color: var(--orbit-ink, #eee); }
</style>
