<script lang="ts">
    import { fxOrder, chorusRate, chorusDepth, chorusMix, delayTime, delayFeedback, delayTone, delayMix, reverbDecay, reverbDamping, reverbMix } from '../stores/params';
    import Knob from './Knob.svelte';

    // Effect definitions: id matches the WASM effect index
    const EFFECTS = [
        { id: 0, name: 'CHORUS' },
        { id: 1, name: 'DELAY' },
        { id: 2, name: 'REVERB' },
    ];

    let dragIdx: number | null = null;
    let overIdx: number | null = null;

    function onDragStart(e: DragEvent, idx: number) {
        dragIdx = idx;
        if (e.dataTransfer) {
            e.dataTransfer.effectAllowed = 'move';
            e.dataTransfer.setData('text/plain', String(idx));
        }
    }

    function onDragOver(e: DragEvent, idx: number) {
        e.preventDefault();
        if (e.dataTransfer) e.dataTransfer.dropEffect = 'move';
        overIdx = idx;
    }

    function onDragLeave() {
        overIdx = null;
    }

    function onDrop(e: DragEvent, targetIdx: number) {
        e.preventDefault();
        overIdx = null;
        if (dragIdx === null || dragIdx === targetIdx) return;

        fxOrder.update(order => {
            const newOrder = [...order];
            [newOrder[dragIdx!], newOrder[targetIdx]] = [newOrder[targetIdx], newOrder[dragIdx!]];
            return newOrder;
        });
        dragIdx = null;
    }

    function onDragEnd() {
        dragIdx = null;
        overIdx = null;
    }

    // Map effect id to its slot position for rendering
    $: orderedEffects = $fxOrder.map((effectId, slotIdx) => ({
        slotIdx,
        effect: EFFECTS[effectId],
    }));
</script>

{#each orderedEffects as { slotIdx, effect } (effect.id)}
    <div
        class="fx-slot"
        class:dragging={dragIdx === slotIdx}
        class:drag-over={overIdx === slotIdx && dragIdx !== slotIdx}
        draggable="true"
        role="listitem"
        ondragstart={(e) => onDragStart(e, slotIdx)}
        ondragover={(e) => onDragOver(e, slotIdx)}
        ondragleave={onDragLeave}
        ondrop={(e) => onDrop(e, slotIdx)}
        ondragend={onDragEnd}
    >
        <section>
            <h3>
                <span class="drag-handle" aria-label="Drag to reorder">⠿</span>
                {effect.name}
            </h3>
            {#if effect.id === 0}
                <div class="row">
                    <Knob label="RATE" store={chorusRate} min={0.1} max={5} />
                    <Knob label="DEPTH" store={chorusDepth} />
                    <Knob label="MIX" store={chorusMix} />
                </div>
            {:else if effect.id === 1}
                <div class="row">
                    <Knob label="TIME" store={delayTime} min={1} max={2000} />
                    <Knob label="FDBK" store={delayFeedback} min={0} max={0.95} />
                    <Knob label="TONE" store={delayTone} />
                    <Knob label="MIX" store={delayMix} />
                </div>
            {:else if effect.id === 2}
                <div class="row">
                    <Knob label="DECAY" store={reverbDecay} min={0} max={0.99} />
                    <Knob label="DAMP" store={reverbDamping} />
                    <Knob label="MIX" store={reverbMix} />
                </div>
            {/if}
        </section>
        {#if slotIdx < 2}
            <div class="divider"></div>
        {/if}
    </div>
{/each}

<style>
    .fx-slot {
        display: flex;
        align-items: flex-start;
    }
    .fx-slot > section {
        padding: 0 4px;
        border-radius: 4px;
        border: 1px solid transparent;
        transition: border-color 0.15s, background 0.15s, opacity 0.15s;
        cursor: grab;
    }
    .fx-slot > section:active {
        cursor: grabbing;
    }
    .fx-slot.dragging > section {
        opacity: 0.4;
    }
    .fx-slot.drag-over > section {
        border-color: #6a6050;
        background: rgba(106, 96, 80, 0.15);
    }

    section h3 {
        font-size: 9px;
        color: #c3c0b6;
        text-align: center;
        letter-spacing: 0.5px;
        margin-bottom: 6px;
        white-space: nowrap;
    }

    .drag-handle {
        cursor: grab;
        opacity: 0.5;
        font-size: 11px;
        vertical-align: middle;
        margin-right: 2px;
        user-select: none;
    }
    .drag-handle:hover {
        opacity: 1;
    }

    .row {
        display: flex;
        align-items: flex-start;
        gap: 2px;
        margin-bottom: 2px;
    }

    .divider {
        width: 1px;
        align-self: stretch;
        background: #302e2a;
        margin: 0 4px;
    }
</style>
