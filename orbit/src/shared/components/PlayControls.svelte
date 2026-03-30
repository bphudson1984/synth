<script lang="ts">
    import { isPlaying, togglePlay, bpm, MIN_BPM, MAX_BPM } from '../stores/transport';

    $: playing = $isPlaying;
    $: currentBpm = $bpm;

    function adjustBpm(delta: number) {
        bpm.update(b => Math.max(MIN_BPM, Math.min(MAX_BPM, b + delta)));
    }
</script>

<div class="play-controls">
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
    .play-controls {
        display: flex;
        justify-content: flex-end;
        align-items: center;
        gap: 12px;
        padding: 4px 24px 0;
    }
    .bpm-section { display: flex; align-items: center; gap: 8px; }
    .bpm-value {
        font-size: 22px; font-weight: 500;
        color: var(--orbit-ink, #eee);
        letter-spacing: -0.5px; min-width: 42px; text-align: center;
    }
    .bpm-btn {
        width: 28px; height: 28px; border-radius: 50%;
        border: 1px solid var(--orbit-border, #444);
        background: transparent; color: var(--orbit-ink, #eee);
        font-size: 16px; cursor: pointer;
        display: flex; align-items: center; justify-content: center;
    }
    .transport-buttons { display: flex; gap: 8px; }
    .transport-btn {
        width: 40px; height: 40px; border-radius: 50%;
        border: 1.5px solid var(--orbit-ink, #eee);
        background: transparent; cursor: pointer;
        display: flex; align-items: center; justify-content: center;
        transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1);
    }
    .transport-btn.active { background: var(--orbit-ink, #eee); }
    .stop-icon { width: 11px; height: 11px; background: var(--orbit-ink, #eee); border-radius: 1px; }
    .transport-btn.active .stop-icon { background: var(--orbit-surface, #111); }
    .play-icon {
        width: 0; height: 0;
        border-left: 9px solid var(--orbit-ink, #eee);
        border-top: 6px solid transparent; border-bottom: 6px solid transparent;
        margin-left: 2px;
    }
    .transport-btn.active .play-icon { border-left-color: var(--orbit-surface, #111); }
</style>
