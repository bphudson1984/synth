<script lang="ts">
    import { LEAD_COLOUR } from './constants';
    import {
        seqSteps, seqSelectedStep, seqCurrentStep, seqNumPages, seqCurrentPage,
        selectSeqStep, toggleSeqStepGate, addSeqPage, setSeqPage,
        removeStepGate, moveStep, type SeqStep,
    } from './stores/state';
    import { isPlaying } from '../shared/stores/transport';

    const PAGE_SIZE = 16;
    const NOTE_NAMES = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
    function noteName(n: number): string { return `${NOTE_NAMES[n % 12]}${Math.floor(n / 12) - 1}`; }

    $: steps = $seqSteps;
    $: sel = $seqSelectedStep;
    $: cur = $seqCurrentStep;
    $: playing = $isPlaying;
    $: numPages = $seqNumPages;
    $: page = $seqCurrentPage;

    let scrollEl: HTMLDivElement;

    // Auto-follow playhead
    let lastPlayPage = -1;
    $: if (playing && scrollEl) {
        const playPage = Math.floor(cur / PAGE_SIZE);
        if (playPage !== lastPlayPage) {
            lastPlayPage = playPage;
            setSeqPage(playPage);
            requestAnimationFrame(() => scrollToPage(playPage));
        }
    }
    $: if (!playing) { lastPlayPage = -1; }

    function scrollToPage(p: number) {
        if (!scrollEl) return;
        scrollEl.scrollTo({ left: p * scrollEl.clientWidth, behavior: 'smooth' });
    }

    function onScroll() {
        if (!scrollEl) return;
        const pw = scrollEl.clientWidth;
        if (pw === 0) return;
        const np = Math.round(scrollEl.scrollLeft / pw);
        if (np !== page && np >= 0 && np < numPages) setSeqPage(np);
    }

    function goToPage(p: number) { setSeqPage(p); scrollToPage(p); }

    // Mouse drag-to-scroll (for page swiping on desktop)
    let mouseDown = false;
    let mouseStartX = 0;
    let scrollStart = 0;
    let didDrag = false;

    function onMouseDown(e: MouseEvent) {
        if (!scrollEl) return;
        // Only start page-scroll drag if clicking on empty space, not on cells
        if ((e.target as HTMLElement).closest('.cell')) return;
        mouseDown = true;
        didDrag = false;
        mouseStartX = e.clientX;
        scrollStart = scrollEl.scrollLeft;
        scrollEl.style.scrollSnapType = 'none';
        scrollEl.style.cursor = 'grabbing';
    }
    function onMouseMove(e: MouseEvent) {
        if (!mouseDown || !scrollEl) return;
        const dx = e.clientX - mouseStartX;
        if (Math.abs(dx) > 5) didDrag = true;
        scrollEl.scrollLeft = scrollStart - dx;
    }
    function onMouseUp() {
        if (!mouseDown || !scrollEl) return;
        mouseDown = false;
        scrollEl.style.scrollSnapType = 'x mandatory';
        scrollEl.style.cursor = '';
        const pw = scrollEl.clientWidth;
        const tp = Math.round(scrollEl.scrollLeft / pw);
        scrollEl.scrollTo({ left: tp * pw, behavior: 'smooth' });
    }

    // --- Note drag (move/remove) ---
    let dragFrom: number | null = null;
    let dragGhost: { x: number; y: number; label: string } | null = null;
    let gridEl: HTMLDivElement[] = [];

    function onCellPointerDown(e: PointerEvent, globalIdx: number) {
        const step = steps[globalIdx];
        if (!step?.gate) return; // only drag active steps
        dragFrom = globalIdx;
        dragGhost = { x: e.clientX, y: e.clientY, label: step.label || noteName(step.notes[0]) };
    }

    function onGlobalPointerMove(e: PointerEvent) {
        if (dragGhost) {
            dragGhost = { ...dragGhost, x: e.clientX, y: e.clientY };
        }
    }

    function onGlobalPointerUp(e: PointerEvent) {
        if (dragFrom === null || !dragGhost) { dragFrom = null; dragGhost = null; return; }

        // Find which cell we dropped on
        const dropTarget = document.elementFromPoint(e.clientX, e.clientY);
        const cell = dropTarget?.closest('.cell') as HTMLElement | null;

        if (cell && cell.dataset.idx !== undefined) {
            const toIdx = parseInt(cell.dataset.idx);
            if (toIdx !== dragFrom) {
                moveStep(dragFrom, toIdx);
            }
        } else {
            // Dropped outside grid — remove the note
            removeStepGate(dragFrom);
        }

        dragFrom = null;
        dragGhost = null;
    }

    function handleStepClick(globalIdx: number) {
        if (didDrag) return;
        selectSeqStep(globalIdx);
    }
    function handleStepDblClick(globalIdx: number) {
        if (didDrag) return;
        toggleSeqStepGate(globalIdx);
    }
</script>

<svelte:window onpointermove={onGlobalPointerMove} onpointerup={onGlobalPointerUp} />

{#if dragGhost}
    <div class="drag-ghost" style="left: {dragGhost.x}px; top: {dragGhost.y}px">
        {dragGhost.label}
    </div>
{/if}

<div class="lead-seq" style="--lead-colour: {LEAD_COLOUR}">
    <div class="seq-header">
        <div class="page-dots">
            {#each Array(numPages) as _, i}
                <button class="dot" class:active={i === page} onclick={() => goToPage(i)}></button>
            {/each}
        </div>
        <button class="add-page-btn" onclick={addSeqPage} disabled={numPages >= 8}>+</button>
    </div>
    <div
        class="scroller"
        bind:this={scrollEl}
        onscroll={onScroll}
        onmousedown={onMouseDown}
        onmousemove={onMouseMove}
        onmouseup={onMouseUp}
        onmouseleave={onMouseUp}
    >
        {#each Array(numPages) as _, p}
            <div class="page">
                <div class="grid">
                    {#each Array(PAGE_SIZE) as _, i}
                        {@const globalIdx = p * PAGE_SIZE + i}
                        {@const step = steps[globalIdx]}
                        <button
                            class="cell"
                            class:active={step?.gate}
                            class:selected={sel === globalIdx}
                            class:playhead={playing && cur === globalIdx}
                            class:drag-source={dragFrom === globalIdx}
                            class:skipped={step?.skip && step?.gate}
                            class:low-prob={step?.gate && step?.probability < 80}
                            data-idx={globalIdx}
                            onclick={() => handleStepClick(globalIdx)}
                            ondblclick={() => handleStepDblClick(globalIdx)}
                            onpointerdown={(e) => onCellPointerDown(e, globalIdx)}
                        >
                            {#if step?.gate}
                                <span class="note-label">{step.label || noteName(step.notes[0])}</span>
                                {#if step.ratchet > 1}
                                    <span class="ratchet-dots">{'•'.repeat(step.ratchet)}</span>
                                {/if}
                            {/if}
                        </button>
                    {/each}
                </div>
            </div>
        {/each}
    </div>
</div>

<style>
    .lead-seq { padding: 0 12px; }
    .seq-header {
        display: flex; justify-content: center; align-items: center; gap: 8px;
        padding: 0 0 4px;
    }
    .page-dots { display: flex; gap: 6px; }
    .dot {
        width: 8px; height: 8px; border-radius: 50%;
        background: var(--orbit-border, #333); border: none;
        cursor: pointer; padding: 0; transition: background 200ms;
    }
    .dot.active { background: var(--lead-colour); }
    .add-page-btn {
        width: 20px; height: 20px; border-radius: 50%;
        border: 1px solid var(--orbit-border, #444);
        background: transparent; color: var(--orbit-hint, #666);
        font-size: 14px; cursor: pointer;
        display: flex; align-items: center; justify-content: center;
    }
    .add-page-btn:active { background: var(--lead-colour); color: #fff; border-color: var(--lead-colour); }
    .add-page-btn:disabled { opacity: 0.3; cursor: default; }
    .scroller {
        display: flex;
        overflow-x: auto;
        scroll-snap-type: x mandatory;
        scrollbar-width: none;
        -webkit-overflow-scrolling: touch;
    }
    .scroller::-webkit-scrollbar { display: none; }
    .page {
        flex: 0 0 100%;
        scroll-snap-align: start;
        box-sizing: border-box;
    }
    .grid { display: grid; grid-template-columns: repeat(8, 1fr); gap: 4px; }
    .cell {
        aspect-ratio: 1.2; border-radius: 4px;
        border: 0.5px solid var(--orbit-border, #333);
        background: var(--orbit-well, #1a1a1a);
        cursor: pointer; position: relative;
        display: flex; align-items: center; justify-content: center;
        transition: background 80ms; padding: 0;
        touch-action: none;
    }
    .cell:active { transform: scale(0.95); }
    .cell.active { background: color-mix(in srgb, var(--lead-colour) 30%, var(--orbit-well)); border-color: var(--lead-colour); }
    .cell.selected { box-shadow: 0 0 0 1.5px var(--lead-colour); }
    .cell.playhead { box-shadow: 0 0 0 1.5px var(--orbit-ink, #eee); }
    .cell.playhead.selected { box-shadow: 0 0 0 1.5px var(--orbit-ink), 0 0 8px var(--lead-colour); }
    .cell.playhead:not(.active) { background: var(--orbit-well-bright, #262626); }
    .cell.playhead::after { content: ''; position: absolute; bottom: 0; left: 0; right: 0; height: 2px; background: var(--orbit-ink); }
    .cell.drag-source { opacity: 0.3; }
    .cell.skipped { opacity: 0.35; text-decoration: line-through; }
    .cell.low-prob { border-style: dashed; }
    .note-label { font-size: 9px; font-weight: 500; color: var(--orbit-ink); pointer-events: none; letter-spacing: 0.3px; }
    .ratchet-dots { position: absolute; bottom: 1px; font-size: 6px; color: var(--lead-colour); pointer-events: none; line-height: 1; }
    .drag-ghost {
        position: fixed;
        pointer-events: none;
        z-index: 1000;
        transform: translate(-50%, -50%);
        padding: 4px 8px;
        background: var(--lead-colour);
        color: #fff;
        font-family: 'JetBrains Mono', monospace;
        font-size: 10px;
        font-weight: 500;
        border-radius: 6px;
        box-shadow: 0 0 12px rgba(181, 110, 204, 0.5);
    }
</style>
