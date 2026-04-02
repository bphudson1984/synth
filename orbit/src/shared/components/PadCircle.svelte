<script lang="ts">
    interface Voice {
        id: string;
        label: string;
        colour: string;
    }

    let {
        voices,
        params,
        selectedVoice,
        selectedParam,
        triggeredVoices,
        onPadClick,
        onPadDown = undefined,
        onPadDblClick = undefined,
        onParamSelect,
        badge = undefined,
    }: {
        voices: Voice[];
        params: string[];
        selectedVoice: number;
        selectedParam: string;
        triggeredVoices: Set<number>;
        onPadClick: (index: number, durationMs: number) => void;
        onPadDown?: (index: number) => void;
        onPadDblClick?: (index: number) => void;
        onParamSelect: (param: string) => void;
        badge?: (index: number) => string | null;
    } = $props();

    const padDownTimes = new Map<number, number>();

    function handlePointerDown(e: PointerEvent, index: number) {
        (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
        padDownTimes.set(index, performance.now());
        onPadDown?.(index);
    }

    function handlePointerUp(e: PointerEvent, index: number) {
        const downTime = padDownTimes.get(index);
        padDownTimes.delete(index);
        const durationMs = downTime != null ? performance.now() - downTime : 0;
        onPadClick(index, durationMs);
    }

    const ORBIT_R = 120;
    const DIAMOND_R = 48;

    function padPos(index: number) {
        const angle = (index / voices.length) * Math.PI * 2 - Math.PI / 2;
        return {
            x: Math.cos(angle) * ORBIT_R,
            y: Math.sin(angle) * ORBIT_R,
        };
    }

    function diamondPos(index: number) {
        const angle = (index / params.length) * Math.PI * 2 - Math.PI / 2;
        return {
            x: Math.cos(angle) * DIAMOND_R,
            y: Math.sin(angle) * DIAMOND_R,
        };
    }

    function handlePadDblClick(e: MouseEvent, index: number) {
        if (!onPadDblClick) return;
        e.preventDefault();
        onPadDblClick(index);
    }
</script>

<div class="constellation">
    <div class="orbit-area">
        {#each voices as voice, i}
            {@const pos = padPos(i)}
            {@const isSelected = selectedVoice === i}
            {@const isTriggered = triggeredVoices.has(i)}
            {@const badgeText = badge?.(i) ?? null}
            <button
                class="pad"
                class:selected={isSelected}
                class:triggered={isTriggered}
                style="
                    left: calc(50% + {pos.x}px - 26px);
                    top: calc(50% + {pos.y}px - 26px);
                    --voice-color: {voice.colour};
                    background: {isSelected || isTriggered
                        ? voice.colour
                        : voice.colour + '2E'};
                    {isSelected || isTriggered ? '' : `border: 0.5px solid ${voice.colour}66;`}
                    {isSelected ? `box-shadow: 0 0 16px ${voice.colour}59;` : ''}
                    {isTriggered ? `box-shadow: 0 0 20px ${voice.colour}80; transform: scale(1.05);` : ''}
                "
                onpointerdown={(e) => handlePointerDown(e, i)}
                onpointerup={(e) => handlePointerUp(e, i)}
                ondblclick={(e) => handlePadDblClick(e, i)}
                aria-label={voice.label}
            >
                <span class="pad-label" style="color: {isSelected || isTriggered ? '#fff' : voice.colour + '88'}">{voice.label}</span>
                {#if badgeText}
                    <span class="engine-badge">{badgeText}</span>
                {/if}
            </button>
        {/each}

        {#each params as param, i}
            {@const pos = diamondPos(i)}
            {@const isActive = selectedParam === param}
            <button
                class="diamond"
                class:active={isActive}
                style="
                    left: calc(50% + {pos.x}px - 14px);
                    top: calc(50% + {pos.y}px - 14px);
                "
                onclick={() => onParamSelect(param)}
                aria-label={param}
            >
                <span class="diamond-label">{param.charAt(0).toUpperCase()}</span>
            </button>
            <span
                class="diamond-text"
                style="
                    left: calc(50% + {pos.x}px);
                    top: calc(50% + {pos.y}px + 22px);
                "
            >{param}</span>
        {/each}
    </div>
</div>

<style>
    .constellation {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        min-height: 280px;
    }
    .orbit-area {
        position: relative;
        width: 360px;
        height: 360px;
    }
    .pad {
        position: absolute;
        width: 52px; height: 52px;
        border-radius: 50%;
        border: none;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1);
    }
    .pad-label {
        font-size: 10px;
        font-weight: 500;
        letter-spacing: 0.5px;
        text-transform: uppercase;
        pointer-events: none;
    }
    .engine-badge {
        position: absolute;
        top: 2px;
        right: 6px;
        font-size: 8px;
        font-weight: 400;
        color: rgba(255,255,255,0.4);
        pointer-events: none;
    }
    .diamond {
        position: absolute;
        width: 28px; height: 28px;
        border-radius: 4px;
        transform: rotate(45deg);
        border: 0.5px solid var(--orbit-border, #444);
        background: var(--orbit-well, #1a1a1a);
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 100ms cubic-bezier(0.2, 0.8, 0.3, 1);
    }
    .diamond.active {
        background: var(--orbit-ink, #eee);
        border: none;
    }
    .diamond-label {
        transform: rotate(-45deg);
        font-size: 10px;
        font-weight: 500;
        color: var(--orbit-hint, #888);
        pointer-events: none;
    }
    .diamond.active .diamond-label { color: var(--orbit-surface, #111); }
    .diamond-text {
        position: absolute;
        font-size: 10px;
        color: var(--orbit-hint, #666);
        transform: translateX(-50%);
        pointer-events: none;
    }
</style>
