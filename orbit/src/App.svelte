<script lang="ts">
    import { OrbitEngine } from './drum/audio/engine';
    import { setDrumEngine } from './drum/stores/state';
    import { ProphetEngine } from './pad/audio/engine';
    import { setPadEngine } from './pad/stores/state';
    import { AcidEngine } from './acid/audio/engine';
    import { setAcidEngine } from './acid/stores/state';
    import DrumPanel from './drum/DrumPanel.svelte';
    import PadPanel from './pad/PadPanel.svelte';
    import AcidPanel from './acid/AcidPanel.svelte';

    let started = $state(false);
    let loading = $state(false);
    let panel = $state<'drum' | 'pad' | 'acid'>('drum');

    async function start() {
        loading = true;
        const drumEngine = new OrbitEngine();
        const padEngine = new ProphetEngine();
        const acidEngine = new AcidEngine();
        await Promise.all([drumEngine.init(), padEngine.init(), acidEngine.init()]);
        setDrumEngine(drumEngine);
        setPadEngine(padEngine);
        setAcidEngine(acidEngine);
        started = true;
        loading = false;
    }
</script>

{#if !started}
    <div class="splash">
        <div class="brand">Hudsonic</div>
        <div class="logo">ORBIT</div>
        <button class="start-btn" onclick={start} disabled={loading}>
            {loading ? 'LOADING...' : 'TAP TO START'}
        </button>
    </div>
{:else}
    <div class="app">
        <nav class="panel-tabs">
            <button class="tab-btn" class:active={panel === 'drum'} onclick={() => panel = 'drum'}>DRUM</button>
            <button class="tab-btn" class:active={panel === 'pad'} onclick={() => panel = 'pad'}>PAD</button>
            <button class="tab-btn" class:active={panel === 'acid'} onclick={() => panel = 'acid'}>ACID</button>
        </nav>
        {#if panel === 'drum'}
            <DrumPanel />
        {:else if panel === 'pad'}
            <PadPanel />
        {:else}
            <AcidPanel />
        {/if}
    </div>
{/if}

<style>
    :global(body) {
        margin: 0;
        padding: 0;
        overflow: hidden;
        background: var(--orbit-canvas, #0a0a0a);
        color: var(--orbit-ink, #eee);
        font-family: 'JetBrains Mono', monospace;
        -webkit-font-smoothing: antialiased;
    }
    :global(:root) {
        --orbit-surface: #111;
        --orbit-well: #1a1a1a;
        --orbit-well-bright: #262626;
        --orbit-canvas: #0a0a0a;
        --orbit-ink: #eee;
        --orbit-label: #aaa;
        --orbit-hint: #666;
        --orbit-border: #333;
    }

    .splash {
        height: 100dvh;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 32px;
    }
    .brand {
        font-size: 13px;
        font-weight: 300;
        letter-spacing: 6px;
        text-transform: uppercase;
        color: var(--orbit-hint, #666);
        font-family: 'JetBrains Mono', monospace;
        margin-bottom: -20px;
    }
    .logo {
        font-size: 48px;
        font-weight: 500;
        letter-spacing: 12px;
        color: var(--orbit-ink);
    }
    .start-btn {
        padding: 16px 40px;
        font-family: 'JetBrains Mono', monospace;
        font-size: 12px;
        font-weight: 400;
        letter-spacing: 3px;
        text-transform: uppercase;
        background: transparent;
        color: var(--orbit-ink);
        border: 1.5px solid var(--orbit-ink);
        border-radius: 24px;
        cursor: pointer;
        transition: all 120ms;
    }
    .start-btn:hover { background: var(--orbit-ink); color: var(--orbit-surface); }
    .start-btn:disabled { opacity: 0.5; cursor: wait; }

    .app {
        height: 100dvh;
        display: flex;
        flex-direction: column;
        padding-top: env(safe-area-inset-top, 16px);
        overflow: hidden;
    }

    .panel-tabs {
        display: flex;
        justify-content: center;
        gap: 0;
        padding: 8px 24px 4px;
    }
    .tab-btn {
        padding: 4px 16px;
        font-family: 'JetBrains Mono', monospace;
        font-size: 11px;
        font-weight: 500;
        letter-spacing: 2px;
        text-transform: uppercase;
        background: transparent;
        color: var(--orbit-hint, #666);
        border: 1.5px solid var(--orbit-border, #444);
        cursor: pointer;
        transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1);
    }
    .tab-btn:first-child { border-radius: 12px 0 0 12px; border-right: none; }
    .tab-btn:not(:first-child):not(:last-child) { border-radius: 0; border-right: none; }
    .tab-btn:last-child { border-radius: 0 12px 12px 0; }
    .tab-btn.active {
        background: var(--orbit-ink, #eee);
        color: var(--orbit-surface, #111);
        border-color: var(--orbit-ink, #eee);
    }
</style>
