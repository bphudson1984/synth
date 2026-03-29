<script lang="ts">
    import { VOICES, PARAMS } from './constants';
    import {
        selectedVoice, selectedParam, triggeredVoices, perPadEngine,
        sliderValue,
        selectVoice, selectParam, triggerPad, togglePadEngine, setSliderValue,
    } from './stores/state';
    import PadCircle from '../shared/components/PadCircle.svelte';
    import Slider from '../shared/components/Slider.svelte';
    import StepSequencer from './StepSequencer.svelte';
    import Transport from './Transport.svelte';

    $: selVoice = $selectedVoice;
    $: selParam = $selectedParam;
    $: triggered = $triggeredVoices;
    $: padEngines = $perPadEngine;
    $: sliderVal = $sliderValue;
    $: colour = VOICES[selVoice].colour;

    function handlePadClick(i: number) {
        selectVoice(i);
        triggerPad(i);
    }

    function badge(i: number): string | null {
        const eng = padEngines[i] ?? '808';
        return eng === '909' ? '9' : '8';
    }
</script>

<StepSequencer />
<Transport />
<PadCircle
    voices={VOICES.map(v => ({ id: v.id, label: v.label, colour: v.colour }))}
    params={[...PARAMS]}
    selectedVoice={selVoice}
    selectedParam={selParam}
    triggeredVoices={triggered}
    onPadClick={handlePadClick}
    onPadDblClick={togglePadEngine}
    onParamSelect={selectParam}
    {badge}
/>
<Slider
    label={selParam}
    value={sliderVal}
    {colour}
    onChange={setSliderValue}
/>
