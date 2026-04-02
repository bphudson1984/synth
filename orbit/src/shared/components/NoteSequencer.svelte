<script lang="ts">
    import type { NoteSequencerStore, SeqStep } from '../stores/noteSequencer';
    import { isPlaying, isRecording, toggleRecord } from '../stores/transport';

    import { writable } from 'svelte/store';

    export let colour: string;
    export let seq: NoteSequencerStore;

    const PAGE_SIZE = 16;
    const NOTE_NAMES = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
    function noteName(n: number): string { return `${NOTE_NAMES[n % 12]}${Math.floor(n / 12) - 1}`; }

    // Destructure stores from the seq instance
    $: ({ seqSteps: stepsStore, seqSelectedStep: selStore, seqCurrentStep: curStore,
          seqNumPages: numPagesStore, seqCurrentPage: pageStore } = seq);
    $: steps = $stepsStore;
    $: sel = $selStore;
    $: cur = $curStore;
    $: playing = $isPlaying;
    $: recording = $isRecording;
    $: numPages = $numPagesStore;
    $: page = $pageStore;

    let scrollEl: HTMLDivElement;

    let lastPlayPage = -1;
    $: if (playing && scrollEl) {
        const playPage = Math.floor(cur / PAGE_SIZE);
        if (playPage !== lastPlayPage) {
            lastPlayPage = playPage;
            seq.setSeqPage(playPage);
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
        if (np !== page && np >= 0 && np < numPages) seq.setSeqPage(np);
    }
    function goToPage(p: number) { seq.setSeqPage(p); scrollToPage(p); }

    let mouseDown = false;
    let mouseStartX = 0;
    let scrollStart = 0;
    let didDrag = false;
    function onMouseDown(e: MouseEvent) {
        if (!scrollEl || (e.target as HTMLElement).closest('.cell')) return;
        mouseDown = true; didDrag = false; mouseStartX = e.clientX; scrollStart = scrollEl.scrollLeft;
        scrollEl.style.scrollSnapType = 'none'; scrollEl.style.cursor = 'grabbing';
    }
    function onMouseMove(e: MouseEvent) {
        if (!mouseDown || !scrollEl) return;
        if (Math.abs(e.clientX - mouseStartX) > 5) didDrag = true;
        scrollEl.scrollLeft = scrollStart - (e.clientX - mouseStartX);
    }
    function onMouseUp() {
        if (!mouseDown || !scrollEl) return;
        mouseDown = false; scrollEl.style.scrollSnapType = 'x mandatory'; scrollEl.style.cursor = '';
        const tp = Math.round(scrollEl.scrollLeft / scrollEl.clientWidth);
        scrollEl.scrollTo({ left: tp * scrollEl.clientWidth, behavior: 'smooth' });
    }

    let dragFrom: number | null = null;
    const dragGhostStore = writable<{ x: number; y: number; label: string } | null>(null);
    $: dragGhost = $dragGhostStore;

    function onCellPointerDown(e: PointerEvent, globalIdx: number) {
        const step = steps[globalIdx];
        if (!step?.gate) return;
        dragFrom = globalIdx;
        dragGhostStore.set({ x: e.clientX, y: e.clientY, label: step.label || noteName(step.notes[0]) });
    }
    function onGlobalPointerMove(e: PointerEvent) {
        if (dragGhost) dragGhostStore.set({ ...dragGhost, x: e.clientX, y: e.clientY });
    }
    function onGlobalPointerUp(e: PointerEvent) {
        if (dragFrom === null || !dragGhost) { dragFrom = null; dragGhostStore.set(null); return; }
        const cell = document.elementFromPoint(e.clientX, e.clientY)?.closest('.cell') as HTMLElement | null;
        if (cell && cell.dataset.idx !== undefined) {
            const toIdx = parseInt(cell.dataset.idx);
            if (toIdx !== dragFrom) seq.moveStep(dragFrom, toIdx);
        } else { seq.removeStepGate(dragFrom); }
        dragFrom = null; dragGhostStore.set(null);
    }

    function handleStepClick(globalIdx: number) { if (!didDrag) seq.selectSeqStep(globalIdx); }
    function handleStepDblClick(globalIdx: number) { if (!didDrag) seq.toggleSeqStepGate(globalIdx); }
</script>

<svelte:window onpointermove={onGlobalPointerMove} onpointerup={onGlobalPointerUp} />

{#if dragGhost}
    <div class="drag-ghost" style="left: {dragGhost.x}px; top: {dragGhost.y}px; background: {colour}">
        {dragGhost.label}
    </div>
{/if}

<div class="note-seq" style="--seq-colour: {colour}">
    <div class="seq-header">
        <div class="page-dots">
            {#each Array(numPages) as _, i}
                <button class="dot" class:active={i === page} onclick={() => goToPage(i)}></button>
            {/each}
        </div>
        <button class="add-page-btn" onclick={seq.addSeqPage} disabled={numPages >= 8}>+</button>
    </div>
    <div class="scroller" bind:this={scrollEl} onscroll={onScroll}
        onmousedown={onMouseDown} onmousemove={onMouseMove} onmouseup={onMouseUp} onmouseleave={onMouseUp}>
        {#each Array(numPages) as _, p}
            <div class="page">
                <div class="grid">
                    {#each Array(PAGE_SIZE) as _, i}
                        {@const globalIdx = p * PAGE_SIZE + i}
                        {@const step = steps[globalIdx]}
                        <button class="cell" class:active={step?.gate} class:selected={sel === globalIdx}
                            class:playhead={playing && cur === globalIdx} class:drag-source={dragFrom === globalIdx}
                            class:skipped={step?.skip && step?.gate} class:low-prob={step?.gate && step?.probability < 80}
                            data-idx={globalIdx}
                            onclick={() => handleStepClick(globalIdx)} ondblclick={() => handleStepDblClick(globalIdx)}
                            onpointerdown={(e) => onCellPointerDown(e, globalIdx)}>
                            {#if step?.gate}
                                <span class="note-label">{step.label || noteName(step.notes[0])}</span>
                                {#if step.gatePct > 100}<span class="len-badge">{Math.round(step.gatePct / 100)}</span>{/if}
                                {#if step.ratchet > 1}<span class="ratchet-dots">{'•'.repeat(step.ratchet)}</span>{/if}
                            {/if}
                        </button>
                    {/each}
                </div>
            </div>
        {/each}
    </div>
    <div class="seq-footer">
        <button class="rec-btn" class:active={recording} onclick={toggleRecord}>
            <span class="rec-dot"></span>
            Record
        </button>
    </div>
</div>

<style>
    .note-seq { padding: 0 12px; }
    .seq-header { display: flex; justify-content: center; align-items: center; gap: 8px; padding: 0 0 4px; }
    .page-dots { display: flex; gap: 6px; }
    .dot { width: 8px; height: 8px; border-radius: 50%; background: var(--orbit-border, #333); border: none; cursor: pointer; padding: 0; transition: background 200ms; }
    .dot.active { background: var(--seq-colour); }
    .add-page-btn { width: 20px; height: 20px; border-radius: 50%; border: 1px solid var(--orbit-border, #444); background: transparent; color: var(--orbit-hint, #666); font-size: 14px; cursor: pointer; display: flex; align-items: center; justify-content: center; }
    .add-page-btn:active { background: var(--seq-colour); color: #fff; border-color: var(--seq-colour); }
    .add-page-btn:disabled { opacity: 0.3; cursor: default; }
    .seq-footer { display: flex; justify-content: center; padding: 6px 0 2px; }
    .rec-btn { display: flex; align-items: center; gap: 5px; border-radius: 14px; border: 1px solid var(--orbit-border, #444); background: transparent; color: var(--orbit-hint, #777); cursor: pointer; padding: 4px 12px; font-size: 10px; font-family: inherit; font-weight: 500; letter-spacing: 0.5px; transition: all 200ms; }
    .rec-btn.active { border-color: #ee3333; color: #ee3333; }
    .rec-dot { width: 8px; height: 8px; border-radius: 50%; background: #662222; transition: background 200ms, box-shadow 200ms; }
    .rec-btn.active .rec-dot { background: #ee3333; box-shadow: 0 0 6px #ee3333; }
    .scroller { display: flex; overflow-x: auto; scroll-snap-type: x mandatory; scrollbar-width: none; -webkit-overflow-scrolling: touch; }
    .scroller::-webkit-scrollbar { display: none; }
    .page { flex: 0 0 100%; scroll-snap-align: start; box-sizing: border-box; }
    .grid { display: grid; grid-template-columns: repeat(8, 1fr); gap: 4px; }
    .cell { aspect-ratio: 1.2; border-radius: 4px; border: 0.5px solid var(--orbit-border, #333); background: var(--orbit-well, #1a1a1a); cursor: pointer; position: relative; display: flex; align-items: center; justify-content: center; transition: background 80ms; padding: 0; touch-action: none; }
    .cell:active { transform: scale(0.95); }
    .cell.active { background: color-mix(in srgb, var(--seq-colour) 30%, var(--orbit-well)); border-color: var(--seq-colour); }
    .cell.selected { box-shadow: 0 0 0 1.5px var(--seq-colour); }
    .cell.playhead { box-shadow: 0 0 0 1.5px var(--orbit-ink, #eee); }
    .cell.playhead.selected { box-shadow: 0 0 0 1.5px var(--orbit-ink), 0 0 8px var(--seq-colour); }
    .cell.playhead:not(.active) { background: var(--orbit-well-bright, #262626); }
    .cell.playhead::after { content: ''; position: absolute; bottom: 0; left: 0; right: 0; height: 2px; background: var(--orbit-ink); }
    .cell.drag-source { opacity: 0.3; }
    .cell.skipped { opacity: 0.35; text-decoration: line-through; }
    .cell.low-prob { border-style: dashed; }
    .note-label { font-size: 9px; font-weight: 500; color: var(--orbit-ink); pointer-events: none; letter-spacing: 0.3px; }
    .len-badge { position: absolute; top: 1px; right: 3px; font-size: 7px; font-weight: 400; color: var(--seq-colour); pointer-events: none; opacity: 0.7; }
    .ratchet-dots { position: absolute; bottom: 1px; font-size: 6px; color: var(--seq-colour); pointer-events: none; line-height: 1; }
    .drag-ghost { position: fixed; pointer-events: none; z-index: 1000; transform: translate(-50%, -50%); padding: 4px 8px; color: #fff; font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500; border-radius: 6px; box-shadow: 0 0 12px rgba(0,0,0,0.5); }
</style>
