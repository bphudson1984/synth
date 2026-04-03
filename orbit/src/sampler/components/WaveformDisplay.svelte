<script lang="ts">
    let {
        waveform = null,
        colour = '#E05555',
        name = '',
        startPct = 0,
        endPct = 1,
        onStartChange = (_v: number) => {},
        onEndChange = (_v: number) => {},
    }: {
        waveform: Float32Array | null;
        colour?: string;
        name?: string;
        startPct?: number;
        endPct?: number;
        onStartChange?: (value: number) => void;
        onEndChange?: (value: number) => void;
    } = $props();

    let canvas: HTMLCanvasElement;
    let container: HTMLDivElement;
    let dragging: 'start' | 'end' | null = $state(null);

    const HANDLE_W = 12; // px width of drag handle

    $effect(() => {
        if (!canvas) return;
        const ctx = canvas.getContext('2d');
        if (!ctx) return;
        const w = canvas.width = canvas.offsetWidth * 2;
        const h = canvas.height = canvas.offsetHeight * 2;
        ctx.clearRect(0, 0, w, h);

        if (!waveform || waveform.length === 0) {
            ctx.fillStyle = '#333';
            ctx.font = `${h * 0.25}px 'JetBrains Mono', monospace`;
            ctx.textAlign = 'center';
            ctx.fillText('DROP SAMPLE', w / 2, h / 2 + h * 0.08);
            return;
        }

        const sX = startPct * w;
        const eX = endPct * w;

        // Dim regions outside start/end
        ctx.fillStyle = 'rgba(0, 0, 0, 0.55)';
        ctx.fillRect(0, 0, sX, h);
        ctx.fillRect(eX, 0, w - eX, h);

        // Draw waveform
        const samplesPerPixel = waveform.length / w;
        ctx.strokeStyle = colour;
        ctx.lineWidth = 1;
        ctx.beginPath();
        const mid = h / 2;
        for (let x = 0; x < w; x++) {
            const s = Math.floor(x * samplesPerPixel);
            const e = Math.min(Math.floor((x + 1) * samplesPerPixel), waveform.length);
            let mn = 0, mx = 0;
            for (let i = s; i < e; i++) {
                const v = waveform[i];
                if (v < mn) mn = v;
                if (v > mx) mx = v;
            }
            ctx.moveTo(x, mid + mn * mid);
            ctx.lineTo(x, mid + mx * mid);
        }
        ctx.stroke();

        // Draw start handle
        ctx.fillStyle = '#4CAF50';
        ctx.fillRect(sX, 0, 4, h);
        // Draw end handle
        ctx.fillStyle = '#F44336';
        ctx.fillRect(eX - 4, 0, 4, h);

        // Draw name
        if (name) {
            ctx.fillStyle = colour;
            ctx.font = `${h * 0.16}px 'JetBrains Mono', monospace`;
            ctx.textAlign = 'left';
            ctx.fillText(name, sX + 10, h * 0.16 + 4);
        }
    });

    function getXPct(e: MouseEvent | Touch): number {
        if (!container) return 0;
        const rect = container.getBoundingClientRect();
        return Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
    }

    function onPointerDown(e: MouseEvent | TouchEvent) {
        if (!waveform || waveform.length === 0) return;
        const clientX = 'touches' in e ? e.touches[0].clientX : e.clientX;
        const rect = container.getBoundingClientRect();
        const xPct = (clientX - rect.left) / rect.width;

        // Determine which handle is closer
        const distToStart = Math.abs(xPct - startPct);
        const distToEnd = Math.abs(xPct - endPct);

        if (distToStart < distToEnd && distToStart < 0.08) {
            dragging = 'start';
        } else if (distToEnd < 0.08) {
            dragging = 'end';
        } else if (xPct < (startPct + endPct) / 2) {
            dragging = 'start';
        } else {
            dragging = 'end';
        }

        handleMove(e);
    }

    function handleMove(e: MouseEvent | TouchEvent) {
        if (!dragging) return;
        e.preventDefault();
        const point = 'touches' in e ? e.touches[0] : e;
        const pct = getXPct(point);

        if (dragging === 'start') {
            // Can't go past end - small margin
            const clamped = Math.min(pct, endPct - 0.01);
            onStartChange(Math.max(0, clamped));
        } else {
            // Can't go before start + small margin
            const clamped = Math.max(pct, startPct + 0.01);
            onEndChange(Math.min(1, clamped));
        }
    }

    function onPointerUp() {
        dragging = null;
    }
</script>

<svelte:window onmouseup={onPointerUp} onmousemove={dragging ? handleMove : undefined}
    ontouchend={onPointerUp} ontouchmove={dragging ? handleMove : undefined} />

<div class="waveform-container" bind:this={container}
    onmousedown={onPointerDown} ontouchstart={onPointerDown}>
    <canvas bind:this={canvas} class="waveform"></canvas>
</div>

<style>
    .waveform-container {
        position: relative;
        margin: 4px 16px;
        touch-action: none;
        cursor: ew-resize;
    }
    .waveform {
        width: 100%;
        height: 72px;
        background: var(--orbit-well, #1a1a1a);
        border-radius: 8px;
        display: block;
    }
</style>
