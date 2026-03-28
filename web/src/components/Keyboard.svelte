<script lang="ts">
    interface Props {
        onnoteon?: (e: CustomEvent<{ note: number; velocity: number }>) => void;
        onnoteoff?: (e: CustomEvent<{ note: number }>) => void;
    }
    let { onnoteon, onnoteoff }: Props = $props();

    const START_NOTE = 36; // C2
    const NUM_OCTAVES = 4;
    const TOTAL_KEYS = NUM_OCTAVES * 12 + 1; // C2 to C6

    const WHITE_W = 22;
    const WHITE_H = 80;
    const BLACK_W = 14;
    const BLACK_H = 50;

    let activeNotes = $state(new Set<number>());

    function isBlack(noteInOctave: number): boolean {
        return [1, 3, 6, 8, 10].includes(noteInOctave);
    }

    // Build key layout
    type Key = { note: number; x: number; black: boolean };
    let keys: Key[] = [];

    let whiteX = 0;
    for (let i = 0; i < TOTAL_KEYS; i++) {
        const note = START_NOTE + i;
        const noteInOctave = i % 12;
        if (!isBlack(noteInOctave)) {
            keys.push({ note, x: whiteX, black: false });
            whiteX += WHITE_W;
        }
    }
    const totalWidth = whiteX;

    // Add black keys
    whiteX = 0;
    for (let i = 0; i < TOTAL_KEYS; i++) {
        const note = START_NOTE + i;
        const noteInOctave = i % 12;
        if (!isBlack(noteInOctave)) {
            whiteX += WHITE_W;
        } else {
            keys.push({ note, x: whiteX - BLACK_W / 2 - 1, black: true });
        }
    }

    function onDown(note: number) {
        if (!activeNotes.has(note)) {
            activeNotes.add(note);
            onnoteon?.(new CustomEvent('noteon', { detail: { note, velocity: 100 } }));
        }
    }

    function onUp(note: number) {
        activeNotes.delete(note);
        onnoteoff?.(new CustomEvent('noteoff', { detail: { note } }));
    }

    // QWERTY mapping
    const QWERTY_LOWER: Record<string, number> = {
        z: 0, s: 1, x: 2, d: 3, c: 4, v: 5,
        g: 6, b: 7, h: 8, n: 9, j: 10, m: 11, ',': 12
    };
    const QWERTY_UPPER: Record<string, number> = {
        q: 12, '2': 13, w: 14, '3': 15, e: 16, r: 17,
        '5': 18, t: 19, '6': 20, y: 21, '7': 22, u: 23, i: 24
    };

    let octave = 4;

    function handleKeyDown(e: KeyboardEvent) {
        if (e.repeat) return;
        if (e.key === '=' || e.key === '+') { octave = Math.min(7, octave + 1); return; }
        if (e.key === '-') { octave = Math.max(1, octave - 1); return; }
        const k = e.key.toLowerCase();
        const offset = QWERTY_LOWER[k] ?? QWERTY_UPPER[k] ?? null;
        if (offset !== null) onDown(octave * 12 + offset);
    }

    function handleKeyUp(e: KeyboardEvent) {
        const k = e.key.toLowerCase();
        const offset = QWERTY_LOWER[k] ?? QWERTY_UPPER[k] ?? null;
        if (offset !== null) onUp(octave * 12 + offset);
    }
</script>

<svelte:window onkeydown={handleKeyDown} onkeyup={handleKeyUp} />

<div class="keyboard-container">
    <svg width={totalWidth} height={WHITE_H + 4} viewBox="0 0 {totalWidth} {WHITE_H + 4}">
        <!-- White keys -->
        {#each keys.filter(k => !k.black) as key}
            <rect
                x={key.x + 1} y={1}
                width={WHITE_W - 2} height={WHITE_H}
                rx="2"
                fill={activeNotes.has(key.note) ? '#c8c4b8' : '#f0ede5'}
                stroke="#888" stroke-width="0.5"
                onpointerdown={() => onDown(key.note)}
                onpointerup={() => onUp(key.note)}
                onpointerleave={() => onUp(key.note)}
                role="button"
                aria-label="Note {key.note}"
            />
        {/each}
        <!-- Black keys -->
        {#each keys.filter(k => k.black) as key}
            <rect
                x={key.x} y={1}
                width={BLACK_W} height={BLACK_H}
                rx="1"
                fill={activeNotes.has(key.note) ? '#444' : '#1a1a18'}
                stroke="#333" stroke-width="0.5"
                onpointerdown={() => onDown(key.note)}
                onpointerup={() => onUp(key.note)}
                onpointerleave={() => onUp(key.note)}
                role="button"
                aria-label="Note {key.note}"
            />
        {/each}
    </svg>
</div>

<style>
    .keyboard-container {
        padding: 8px 0;
        display: flex;
        justify-content: center;
    }
    svg { touch-action: none; }
    rect { cursor: pointer; }
</style>
