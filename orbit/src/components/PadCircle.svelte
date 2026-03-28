<script lang="ts">
    import { VOICES, PARAMS } from '../constants';
    import {
        selectedVoice, selectedParam, triggeredVoices,
        selectVoice, selectParam, triggerPad
    } from '../stores/state';

    $: selVoice = $selectedVoice;
    $: selParam = $selectedParam;
    $: triggered = $triggeredVoices;

    const ORBIT_R = 120;  // pad orbit radius
    const DIAMOND_R = 48; // diamond orbit radius

    // Position 8 pads in a circle
    function padPos(index: number) {
        const angle = (index / 8) * Math.PI * 2 - Math.PI / 2; // start from top
        return {
            x: Math.cos(angle) * ORBIT_R,
            y: Math.sin(angle) * ORBIT_R,
        };
    }

    // Position 4 diamonds in inner circle
    function diamondPos(index: number) {
        const angle = (index / 4) * Math.PI * 2 - Math.PI / 2;
        return {
            x: Math.cos(angle) * DIAMOND_R,
            y: Math.sin(angle) * DIAMOND_R,
        };
    }

    function handlePadClick(index: number) {
        selectVoice(index);
        triggerPad(index);
    }
</script>

<div class="constellation">
    <div class="orbit-area">
        <!-- Voice pads -->
        {#each VOICES as voice, i}
            {@const pos = padPos(i)}
            {@const isSelected = selVoice === i}
            {@const isTriggered = triggered.has(i)}
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
                onclick={() => handlePadClick(i)}
                aria-label={voice.label}
            >
                <span class="pad-label" style="color: {isSelected || isTriggered ? '#fff' : voice.colour + '88'}">{voice.label}</span>
            </button>
        {/each}

        <!-- Parameter diamonds -->
        {#each PARAMS as param, i}
            {@const pos = diamondPos(i)}
            {@const isActive = selParam === param}
            <button
                class="diamond"
                class:active={isActive}
                style="
                    left: calc(50% + {pos.x}px - 14px);
                    top: calc(50% + {pos.y}px - 14px);
                "
                onclick={() => selectParam(param)}
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
        width: 300px;
        height: 300px;
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
