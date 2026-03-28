<script lang="ts">
    import type { Writable } from 'svelte/store';

    export let label: string = '';
    export let store: Writable<number>;
    export let min: number = 0;
    export let max: number = 1;
    export let size: number = 44;
    export let silver: boolean = false;

    // 7 o'clock (min) to 5 o'clock (max), 300° sweep clockwise
    const MIN_ANGLE = 210;
    const MAX_ANGLE = 510;
    const SWEEP = MAX_ANGLE - MIN_ANGLE;

    let dragging = false;

    $: normalized = (($store - min) / (max - min));
    $: angle = MIN_ANGLE + normalized * SWEEP;

    function onPointerDown(e: PointerEvent) {
        dragging = true;
        (e.target as Element).setPointerCapture(e.pointerId);
    }

    function onPointerMove(e: PointerEvent) {
        if (!dragging) return;
        const sensitivity = e.shiftKey ? 0.001 : 0.004;
        const delta = -e.movementY * sensitivity;
        const newNorm = Math.max(0, Math.min(1, normalized + delta));
        $store = min + newNorm * (max - min);
    }

    function onPointerUp() {
        dragging = false;
    }

    function onDblClick() {
        // Reset to center/default
        $store = min + (max - min) * 0.5;
    }

    $: r = size / 2;
    $: bodyR = r * 0.82;
    $: capR = bodyR * 0.65;

    function arcPath(cx: number, cy: number, radius: number, startDeg: number, endDeg: number): string {
        const s = (startDeg - 90) * Math.PI / 180;
        const e = (endDeg - 90) * Math.PI / 180;
        const x1 = cx + radius * Math.cos(s);
        const y1 = cy + radius * Math.sin(s);
        const x2 = cx + radius * Math.cos(e);
        const y2 = cy + radius * Math.sin(e);
        const large = endDeg - startDeg > 180 ? 1 : 0;
        return `M ${x1} ${y1} A ${radius} ${radius} 0 ${large} 1 ${x2} ${y2}`;
    }

    $: indicatorRad = ((angle - 90) * Math.PI) / 180;
</script>

<div class="knob-wrapper" style="width: {size + 8}px">
    <div class="label">{label}</div>
    <svg
        width={size} height={size}
        viewBox="0 0 {size} {size}"
        onpointerdown={onPointerDown}
        onpointermove={onPointerMove}
        onpointerup={onPointerUp}
        ondblclick={onDblClick}
        role="slider"
        aria-label={label}
        aria-valuenow={$store}
    >
        <!-- Track arc -->
        <path d={arcPath(r, r, r - 2, MIN_ANGLE, MAX_ANGLE)}
            fill="none" stroke="#2a2a28" stroke-width="2" stroke-linecap="round" />

        <!-- Value arc -->
        {#if normalized > 0.01}
            <path d={arcPath(r, r, r - 2, MIN_ANGLE, angle)}
                fill="none" stroke={silver ? "#b0ada5" : "#8c8880"} stroke-width="2" stroke-linecap="round" />
        {/if}

        {#if !silver}
            <!-- Black knob body -->
            <circle cx={r} cy={r} r={bodyR} fill="#2a2a28" />
            <circle cx={r} cy={r} r={bodyR} fill="none" stroke="#3c3a36" stroke-width="0.5" />
            <!-- Silver cap -->
            <circle cx={r} cy={r} r={capR} fill="url(#capGrad)" />
            <circle cx={r} cy={r} r={capR} fill="none" stroke="#4a4842" stroke-width="0.4" />
            <!-- Indicator on cap -->
            <line
                x1={r + Math.cos(indicatorRad) * capR * 0.25}
                y1={r + Math.sin(indicatorRad) * capR * 0.25}
                x2={r + Math.cos(indicatorRad) * capR * 0.88}
                y2={r + Math.sin(indicatorRad) * capR * 0.88}
                stroke="white" stroke-width="2" stroke-linecap="round" />
        {:else}
            <!-- Silver knob -->
            <circle cx={r} cy={r} r={bodyR} fill="url(#silverGrad)" />
            <circle cx={r} cy={r} r={bodyR} fill="none" stroke="#5a5850" stroke-width="0.4" />
            <line
                x1={r + Math.cos(indicatorRad) * bodyR * 0.25}
                y1={r + Math.sin(indicatorRad) * bodyR * 0.25}
                x2={r + Math.cos(indicatorRad) * bodyR * 0.88}
                y2={r + Math.sin(indicatorRad) * bodyR * 0.88}
                stroke="#32302d" stroke-width="2" stroke-linecap="round" />
        {/if}

        <defs>
            <radialGradient id="capGrad" cx="40%" cy="35%">
                <stop offset="0%" stop-color="#b8b6b0" />
                <stop offset="100%" stop-color="#646260" />
            </radialGradient>
            <radialGradient id="silverGrad" cx="40%" cy="35%">
                <stop offset="0%" stop-color="#d2d0c8" />
                <stop offset="100%" stop-color="#828078" />
            </radialGradient>
        </defs>
    </svg>
</div>

<style>
    .knob-wrapper {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 2px;
        user-select: none;
    }
    .label {
        font-size: 8.5px;
        color: #c8c5bc;
        text-transform: uppercase;
        letter-spacing: 0.5px;
        white-space: nowrap;
    }
    svg {
        cursor: grab;
        touch-action: none;
    }
    svg:active { cursor: grabbing; }
</style>
