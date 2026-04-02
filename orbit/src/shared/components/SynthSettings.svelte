<script lang="ts">
    import type { SettingsSection, SettingsParam } from '../types/settings';

    let {
        sections,
        colour,
        values,
        onParamChange,
    }: {
        sections: SettingsSection[];
        colour: string;
        values: Record<number, number>;
        onParamChange: (id: number, value: number) => void;
    } = $props();

    let activeTab = $state(0);

    function sliderPct(param: SettingsParam): number {
        const raw = values[param.id] ?? param.default;
        return ((raw - param.min) / (param.max - param.min)) * 100;
    }

    function handleSlider(param: SettingsParam, e: Event) {
        const pct = Number((e.target as HTMLInputElement).value);
        const value = param.min + (pct / 100) * (param.max - param.min);
        onParamChange(param.id, value);
    }

    function handleToggle(param: SettingsParam) {
        const current = values[param.id] ?? param.default;
        onParamChange(param.id, current > 0.5 ? 0 : 1);
    }

    function handleSelect(param: SettingsParam, value: number) {
        onParamChange(param.id, value);
    }

    function formatValue(param: SettingsParam): string {
        const raw = values[param.id] ?? param.default;
        if (param.max >= 100) return Math.round(raw).toString();
        if (param.max - param.min > 10) return raw.toFixed(1);
        return raw.toFixed(2);
    }
</script>

<div class="settings">
    <div class="tabs">
        {#each sections as section, i}
            <button
                class="tab"
                class:active={activeTab === i}
                onclick={() => activeTab = i}
                style="--c: {colour}"
            >{section.label}</button>
        {/each}
    </div>
    <div class="section">
        {#each sections[activeTab].params as param}
            <div class="param-row">
                {#if param.type === 'toggle'}
                    <button
                        class="toggle-btn"
                        class:on={(values[param.id] ?? param.default) > 0.5}
                        onclick={() => handleToggle(param)}
                        style="--c: {colour}"
                    >{param.name}</button>
                {:else if param.type === 'select'}
                    <span class="param-label">{param.name}</span>
                    <div class="select-group">
                        {#each param.options ?? [] as opt}
                            <button
                                class="select-btn"
                                class:active={Math.round(values[param.id] ?? param.default) === opt.value}
                                onclick={() => handleSelect(param, opt.value)}
                                style="--c: {colour}"
                            >{opt.label}</button>
                        {/each}
                    </div>
                {:else}
                    <span class="param-label">{param.name}</span>
                    <input
                        type="range"
                        min="0"
                        max="100"
                        step="1"
                        value={sliderPct(param)}
                        oninput={(e) => handleSlider(param, e)}
                        class="param-slider"
                        style="--c: {colour}; --fill-pct: {sliderPct(param)}%"
                    />
                    <span class="param-value" style="color: {colour}">{formatValue(param)}</span>
                {/if}
            </div>
        {/each}
    </div>
</div>

<style>
    .settings {
        flex: 1;
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }
    .tabs {
        display: flex;
        gap: 4px;
        padding: 8px 16px;
        overflow-x: auto;
        scrollbar-width: none;
        flex-shrink: 0;
    }
    .tabs::-webkit-scrollbar { display: none; }
    .tab {
        padding: 4px 10px;
        font-family: 'JetBrains Mono', monospace;
        font-size: 9px;
        font-weight: 500;
        letter-spacing: 0.5px;
        white-space: nowrap;
        background: transparent;
        color: var(--orbit-hint, #666);
        border: 1px solid var(--orbit-border, #444);
        border-radius: 8px;
        cursor: pointer;
        transition: all 100ms;
    }
    .tab.active {
        background: var(--c);
        color: #fff;
        border-color: var(--c);
    }
    .section {
        flex: 1;
        overflow-y: auto;
        padding: 8px 24px 16px;
        display: flex;
        flex-direction: column;
        gap: 14px;
    }
    .param-row {
        display: flex;
        align-items: center;
        gap: 12px;
    }
    .param-label {
        font-size: 10px;
        font-weight: 500;
        letter-spacing: 0.5px;
        color: var(--orbit-label, #aaa);
        min-width: 60px;
        flex-shrink: 0;
    }
    .param-slider {
        flex: 1;
        height: 3px;
        -webkit-appearance: none;
        appearance: none;
        background: linear-gradient(
            to right,
            var(--c) 0%, var(--c) var(--fill-pct),
            var(--orbit-well, #1a1a1a) var(--fill-pct)
        );
        border-radius: 2px;
        outline: none;
        cursor: pointer;
    }
    .param-slider::-webkit-slider-thumb {
        -webkit-appearance: none;
        width: 14px; height: 14px;
        border-radius: 50%;
        background: var(--orbit-surface, #111);
        border: 2px solid var(--c);
        cursor: pointer;
    }
    .param-slider::-moz-range-thumb {
        width: 14px; height: 14px;
        border-radius: 50%;
        background: var(--orbit-surface, #111);
        border: 2px solid var(--c);
        cursor: pointer;
    }
    .param-value {
        font-size: 10px;
        font-weight: 500;
        min-width: 36px;
        text-align: right;
    }
    .toggle-btn {
        padding: 5px 14px;
        font-family: 'JetBrains Mono', monospace;
        font-size: 10px;
        font-weight: 500;
        letter-spacing: 0.5px;
        background: transparent;
        color: var(--orbit-hint, #666);
        border: 1.5px solid var(--orbit-border, #444);
        border-radius: 10px;
        cursor: pointer;
        transition: all 100ms;
    }
    .toggle-btn.on {
        background: var(--c);
        color: #fff;
        border-color: var(--c);
    }
    .select-group {
        display: flex;
        gap: 4px;
    }
    .select-btn {
        padding: 4px 10px;
        font-family: 'JetBrains Mono', monospace;
        font-size: 9px;
        font-weight: 500;
        letter-spacing: 0.3px;
        background: transparent;
        color: var(--orbit-hint, #666);
        border: 1px solid var(--orbit-border, #444);
        border-radius: 8px;
        cursor: pointer;
        transition: all 100ms;
    }
    .select-btn.active {
        background: var(--c);
        color: #fff;
        border-color: var(--c);
    }
</style>
