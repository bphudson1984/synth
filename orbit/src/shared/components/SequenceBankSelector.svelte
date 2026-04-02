<script lang="ts">
    let {
        currentIndex = 0,
        count = 1,
        maxCount = 8,
        colour = '#eee',
        chainActive = false,
        randomActive = false,
        onSelect = (_i: number) => {},
        onAdd = () => {},
        onDuplicate = () => {},
        onDelete = () => {},
        onToggleChain = () => {},
        onToggleRandom = () => {},
    }: {
        currentIndex: number;
        count: number;
        maxCount: number;
        colour: string;
        chainActive: boolean;
        randomActive: boolean;
        onSelect: (index: number) => void;
        onAdd: () => void;
        onDuplicate: () => void;
        onDelete: () => void;
        onToggleChain: () => void;
        onToggleRandom: () => void;
    } = $props();
</script>

<div class="seq-bank" style="--bank-colour: {colour}">
    <div class="bank-slots">
        {#each Array(count) as _, i}
            <button
                class="slot-btn"
                class:active={i === currentIndex}
                onclick={() => onSelect(i)}
            >{i + 1}</button>
        {/each}
    </div>
    <button class="action-btn" onclick={onAdd} disabled={count >= maxCount}>+</button>
    <button class="action-btn" onclick={onDuplicate} disabled={count >= maxCount}>DUP</button>
    <button class="action-btn del" onclick={onDelete} disabled={count < 2}>DEL</button>
    <button class="action-btn" class:chain-active={chainActive} onclick={onToggleChain} disabled={count < 2}>CHAIN</button>
    <button class="action-btn" class:chain-active={randomActive} onclick={onToggleRandom} disabled={count < 2}>RND</button>
</div>

<style>
    .seq-bank {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 6px;
        padding: 4px 12px;
    }
    .bank-slots {
        display: flex;
        gap: 4px;
    }
    .slot-btn {
        min-width: 24px;
        height: 24px;
        padding: 0 4px;
        border-radius: 6px;
        border: 1.5px solid var(--orbit-border, #444);
        background: transparent;
        color: var(--orbit-hint, #666);
        font-family: 'JetBrains Mono', monospace;
        font-size: 10px;
        font-weight: 600;
        cursor: pointer;
        transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1);
    }
    .slot-btn.active {
        background: var(--bank-colour);
        color: #111;
        border-color: var(--bank-colour);
    }
    .action-btn {
        height: 24px;
        padding: 0 8px;
        border-radius: 6px;
        border: 1.5px solid var(--orbit-border, #444);
        background: transparent;
        color: var(--orbit-hint, #666);
        font-family: 'JetBrains Mono', monospace;
        font-size: 10px;
        font-weight: 500;
        letter-spacing: 0.5px;
        cursor: pointer;
        transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1);
    }
    .action-btn:active {
        background: var(--bank-colour);
        color: #111;
        border-color: var(--bank-colour);
    }
    .action-btn:disabled {
        opacity: 0.3;
        cursor: default;
    }
    .action-btn.del:not(:disabled):active {
        background: #ee3333;
        color: #fff;
        border-color: #ee3333;
    }
    .action-btn.chain-active {
        background: var(--bank-colour);
        color: #111;
        border-color: var(--bank-colour);
    }
</style>
