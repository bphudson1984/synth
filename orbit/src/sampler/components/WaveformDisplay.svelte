<script lang="ts">
    let { waveform = null, colour = '#E05555', name = '' }: {
        waveform: Float32Array | null;
        colour?: string;
        name?: string;
    } = $props();

    let canvas: HTMLCanvasElement;

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

        // Draw waveform as peak pairs
        const samplesPerPixel = waveform.length / w;
        ctx.strokeStyle = colour;
        ctx.lineWidth = 1;
        ctx.beginPath();
        const mid = h / 2;
        for (let x = 0; x < w; x++) {
            const start = Math.floor(x * samplesPerPixel);
            const end = Math.min(Math.floor((x + 1) * samplesPerPixel), waveform.length);
            let min = 0, max = 0;
            for (let i = start; i < end; i++) {
                const v = waveform[i];
                if (v < min) min = v;
                if (v > max) max = v;
            }
            ctx.moveTo(x, mid + min * mid);
            ctx.lineTo(x, mid + max * mid);
        }
        ctx.stroke();

        // Draw name
        if (name) {
            ctx.fillStyle = colour;
            ctx.font = `${h * 0.18}px 'JetBrains Mono', monospace`;
            ctx.textAlign = 'left';
            ctx.fillText(name, 8, h * 0.18 + 4);
        }
    });
</script>

<canvas bind:this={canvas} class="waveform"></canvas>

<style>
    .waveform {
        width: 100%;
        height: 64px;
        background: var(--orbit-well, #1a1a1a);
        border-radius: 8px;
        margin: 4px 16px;
        display: block;
    }
</style>
