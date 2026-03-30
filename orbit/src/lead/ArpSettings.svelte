<script lang="ts">
    import { LEAD_COLOUR } from './constants';
    import {
        arpMode, arpDivision, arpOctaves, arpSettingsOpen,
        setArpMode, setArpDivision, setArpOctaves, toggleArpSettings,
    } from './stores/state';

    $: mode = $arpMode;
    $: div = $arpDivision;
    $: oct = $arpOctaves;
    $: open = $arpSettingsOpen;

    const modes = [
        { value: 'off', label: 'OFF' },
        { value: 'up', label: 'UP' },
        { value: 'down', label: 'DN' },
        { value: 'updown', label: 'U/D' },
        { value: 'random', label: 'RND' },
    ];
    const divisions = ['1/4', '1/8', '1/16', '1/32'];
</script>

<div class="arp-container" style="--arp-colour: {LEAD_COLOUR}">
    <button class="arp-toggle" class:active={mode !== 'off'} onclick={toggleArpSettings}>
        ARP {mode !== 'off' ? '(' + mode.toUpperCase() + ')' : ''}
    </button>

    {#if open}
        <div class="arp-drawer">
            <div class="arp-row">
                <span class="arp-label">MODE</span>
                <div class="arp-btns">
                    {#each modes as m}
                        <button
                            class="arp-btn"
                            class:active={mode === m.value}
                            onclick={() => setArpMode(m.value)}
                        >{m.label}</button>
                    {/each}
                </div>
            </div>
            <div class="arp-row">
                <span class="arp-label">DIV</span>
                <div class="arp-btns">
                    {#each divisions as d}
                        <button
                            class="arp-btn"
                            class:active={div === d}
                            onclick={() => setArpDivision(d)}
                        >{d}</button>
                    {/each}
                </div>
            </div>
            <div class="arp-row">
                <span class="arp-label">OCT</span>
                <div class="arp-btns">
                    {#each [1, 2, 3] as o}
                        <button
                            class="arp-btn"
                            class:active={oct === o}
                            onclick={() => setArpOctaves(o)}
                        >{o}</button>
                    {/each}
                </div>
            </div>
        </div>
    {/if}
</div>

<style>
    .arp-container { display: flex; flex-direction: column; align-items: center; }
    .arp-toggle {
        padding: 4px 14px;
        font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500;
        letter-spacing: 1px; background: transparent; color: var(--orbit-hint, #666);
        border: 1.5px solid var(--orbit-border, #444); border-radius: 12px;
        cursor: pointer; transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1);
        white-space: nowrap;
    }
    .arp-toggle.active { background: var(--arp-colour); color: #fff; border-color: var(--arp-colour); }
    .arp-drawer {
        width: 100%; max-width: 320px;
        margin-top: 8px; padding: 10px 12px;
        background: var(--orbit-well, #1a1a1a);
        border: 1px solid var(--orbit-border, #333);
        border-radius: 12px;
        display: flex; flex-direction: column; gap: 8px;
    }
    .arp-row { display: flex; align-items: center; gap: 8px; }
    .arp-label {
        font-size: 9px; font-weight: 500; letter-spacing: 1px; color: var(--orbit-hint, #666);
        min-width: 32px; text-align: right;
    }
    .arp-btns { display: flex; gap: 4px; flex: 1; }
    .arp-btn {
        flex: 1; padding: 4px 0;
        font-family: 'JetBrains Mono', monospace; font-size: 9px; font-weight: 500;
        letter-spacing: 0.3px; background: transparent; color: var(--orbit-hint, #666);
        border: 1px solid var(--orbit-border, #444); border-radius: 8px;
        cursor: pointer; transition: all 100ms;
    }
    .arp-btn.active { background: var(--arp-colour); color: #fff; border-color: var(--arp-colour); }
</style>
