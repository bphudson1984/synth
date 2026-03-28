<script lang="ts">
    import type { Writable } from 'svelte/store';

    export let label: string = '';
    export let store: Writable<boolean>;

    function toggle() {
        $store = !$store;
    }
</script>

<div class="switch-wrapper">
    <div class="led" class:on={$store}></div>
    <button class="switch-btn" class:pressed={$store} onclick={toggle} aria-label={label}>
    </button>
    <div class="label">{label}</div>
</div>

<style>
    .switch-wrapper {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 3px;
        width: 32px;
        user-select: none;
    }
    .led {
        width: 7px; height: 7px;
        border-radius: 50%;
        background: #3c1412;
        border: 0.5px solid #2a0e0c;
        transition: all 0.1s;
    }
    .led.on {
        background: #ff2d1e;
        box-shadow: 0 0 4px 2px rgba(255, 45, 30, 0.3), 0 0 8px 4px rgba(255, 45, 30, 0.1);
    }
    .switch-btn {
        width: 24px; height: 14px;
        background: #343430;
        border: 0.5px solid #41403a;
        border-radius: 2px;
        cursor: pointer;
        position: relative;
    }
    .switch-btn::after {
        content: '';
        position: absolute;
        top: 1px; left: 2px; right: 2px;
        height: 1px;
        background: rgba(255,255,255,0.08);
    }
    .switch-btn.pressed {
        background: #26262a;
        border-color: #333;
    }
    .switch-btn.pressed::after {
        background: rgba(0,0,0,0.15);
    }
    .label {
        font-size: 8px;
        color: #b9b7b2;
        white-space: nowrap;
    }
</style>
