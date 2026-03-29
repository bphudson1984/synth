<script lang="ts">
    import { OrbitEngine } from './audio/engine';
    import { setEngine } from './stores/state';
    import StepSequencer from './components/StepSequencer.svelte';
    import Transport from './components/Transport.svelte';
    import PadCircle from './components/PadCircle.svelte';
    import Slider from './components/Slider.svelte';

    let started = $state(false);
    let loading = $state(false);

    async function start() {
        loading = true;
        const engine = new OrbitEngine();
        await engine.init();
        setEngine(engine);
        started = true;
        loading = false;
    }
</script>

{#if !started}
    <div class="splash">
        <div class="logo">ORBIT</div>
        <button class="start-btn" onclick={start} disabled={loading}>
            {loading ? 'LOADING...' : 'TAP TO START'}
        </button>
    </div>
{:else}
    <div class="app">
        <StepSequencer />
        <Transport />
        <PadCircle />
        <Slider />
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
</style>
