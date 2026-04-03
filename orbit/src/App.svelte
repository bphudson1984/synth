<script lang="ts">
    import { OrbitEngine } from './drum/audio/engine';
    import { setDrumEngine } from './drum/stores/state';
    import { ProphetEngine } from './pad/audio/engine';
    import { setPadEngine } from './pad/stores/state';
    import { AcidEngine } from './acid/audio/engine';
    import { setAcidEngine } from './acid/stores/state';
    import { BraidsEngine } from './lead/audio/engine';
    import { setLeadEngine } from './lead/stores/state';
    import { BassEngine } from './bass/audio/engine';
    import { setBassEngine } from './bass/stores/state';
    import { SamplerEngine } from './sampler/audio/engine';
    import { setSamplerEngine } from './sampler/stores/state';
    import { FxEngine } from './fx/audio/engine';
    import { setFxEngine, registerEngineSends } from './fx/stores/state';
    import DrumPanel from './drum/DrumPanel.svelte';
    import PadPanel from './pad/PadPanel.svelte';
    import AcidPanel from './acid/AcidPanel.svelte';
    import LeadPanel from './lead/LeadPanel.svelte';
    import BassPanel from './bass/BassPanel.svelte';
    import SamplerPanel from './sampler/SamplerPanel.svelte';
    import FxPanel from './fx/FxPanel.svelte';
    import MixPanel from './mix/MixPanel.svelte';
    import HelpPanel from './help/HelpPanel.svelte';

    let started = $state(false);
    let loading = $state(false);
    let showHelp = $state(false);
    let panel = $state<'drum' | 'pad' | 'acid' | 'lead' | 'bass' | 'sampler' | 'fx' | 'mix'>('drum');

    function openHelp() {
        showHelp = true;
        window.history.pushState({}, '', '/help');
    }

    function closeHelp() {
        showHelp = false;
        window.history.pushState({}, '', '/');
    }

    async function start() {
        loading = true;
        try {
            const drumEngine = new OrbitEngine();
            const padEngine = new ProphetEngine();
            const acidEngine = new AcidEngine();
            const leadEngine = new BraidsEngine();
            const bassEngine = new BassEngine();
            const samplerEngine = new SamplerEngine();
            const fxEngine = new FxEngine();
            await Promise.all([drumEngine.init(), padEngine.init(), acidEngine.init(), leadEngine.init(), bassEngine.init(), samplerEngine.init(), fxEngine.init()]);
            setDrumEngine(drumEngine);
            setPadEngine(padEngine);
            setAcidEngine(acidEngine);
            setLeadEngine(leadEngine);
            setBassEngine(bassEngine);
            setSamplerEngine(samplerEngine);
            setFxEngine(fxEngine);
            // Connect send routing from each engine to the FX rack
            drumEngine.connectSends(fxEngine);
            padEngine.connectSends(fxEngine);
            acidEngine.connectSends(fxEngine);
            leadEngine.connectSends(fxEngine);
            bassEngine.connectSends(fxEngine);
            samplerEngine.connectSends(fxEngine);
            registerEngineSends('drum', (i, l) => drumEngine.setSendLevel(i, l));
            registerEngineSends('pad', (i, l) => padEngine.setSendLevel(i, l));
            registerEngineSends('acid', (i, l) => acidEngine.setSendLevel(i, l));
            registerEngineSends('lead', (i, l) => leadEngine.setSendLevel(i, l));
            registerEngineSends('bass', (i, l) => bassEngine.setSendLevel(i, l));
            registerEngineSends('sampler', (i, l) => samplerEngine.setSendLevel(i, l));
            started = true;
            loading = false;
        } catch (err) {
            console.error('Engine initialization failed:', err);
        } finally {
            loading = false;
        }
    }
</script>

{#if showHelp}
    <HelpPanel onBack={closeHelp} />
{:else if !started}
    <div class="splash">
        <div class="brand">Hudsonic</div>
        <div class="logo">ORBIT</div>
        <button class="start-btn" onclick={start} disabled={loading}>
            {loading ? 'LOADING...' : 'TAP TO START'}
        </button>
        <button class="splash-help-btn" onclick={openHelp}>?</button>
    </div>
{:else}
    <div class="app">
        <nav class="panel-tabs">
            <button class="tab-btn" class:active={panel === 'drum'} onclick={() => panel = 'drum'}>DRUM</button>
            <button class="tab-btn" class:active={panel === 'bass'} onclick={() => panel = 'bass'}>BASS</button>
            <button class="tab-btn" class:active={panel === 'pad'} onclick={() => panel = 'pad'}>PAD</button>
            <button class="tab-btn" class:active={panel === 'acid'} onclick={() => panel = 'acid'}>ACID</button>
            <button class="tab-btn" class:active={panel === 'lead'} onclick={() => panel = 'lead'}>LEAD</button>
            <button class="tab-btn" class:active={panel === 'sampler'} onclick={() => panel = 'sampler'}>SMPL</button>
            <button class="tab-btn" class:active={panel === 'fx'} onclick={() => panel = 'fx'}>FX</button>
            <button class="tab-btn" class:active={panel === 'mix'} onclick={() => panel = 'mix'}>MIX</button>
        </nav>
        {#if panel === 'drum'}
            <DrumPanel />
        {:else if panel === 'pad'}
            <PadPanel />
        {:else if panel === 'acid'}
            <AcidPanel />
        {:else if panel === 'lead'}
            <LeadPanel />
        {:else if panel === 'bass'}
            <BassPanel />
        {:else if panel === 'sampler'}
            <SamplerPanel />
        {:else if panel === 'fx'}
            <FxPanel />
        {:else}
            <MixPanel />
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
    .splash-help-btn {
        width: 32px;
        height: 32px;
        border-radius: 50%;
        font-family: 'JetBrains Mono', monospace;
        font-size: 13px;
        font-weight: 500;
        background: transparent;
        color: var(--orbit-hint, #666);
        border: 1.5px solid var(--orbit-border, #333);
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 0;
        transition: all 120ms;
    }
    .splash-help-btn:hover {
        background: var(--orbit-ink, #eee);
        color: var(--orbit-surface, #111);
        border-color: var(--orbit-ink, #eee);
    }

    .app {
        height: 100dvh;
        display: flex;
        flex-direction: column;
        padding-top: env(safe-area-inset-top, 16px);
        overflow: hidden;
    }

    .panel-tabs {
        display: flex;
        gap: 0;
        padding: 8px 16px 4px;
        flex-shrink: 0;
    }
    .tab-btn {
        flex: 1;
        padding: 6px 0;
        font-family: 'JetBrains Mono', monospace;
        font-size: 10px;
        font-weight: 500;
        letter-spacing: 1.5px;
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
    .help-btn {
        width: 28px;
        height: 28px;
        margin-left: 10px;
        border-radius: 50%;
        font-family: 'JetBrains Mono', monospace;
        font-size: 12px;
        font-weight: 500;
        background: transparent;
        color: var(--orbit-hint, #666);
        border: 1.5px solid var(--orbit-border, #444);
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 0;
        transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1);
        flex-shrink: 0;
    }
    .help-btn:hover {
        background: var(--orbit-ink, #eee);
        color: var(--orbit-surface, #111);
        border-color: var(--orbit-ink, #eee);
    }
</style>
