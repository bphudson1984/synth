# ORBIT — Product requirements document

## 1. Product summary

ORBIT is a browser-based drum machine and step sequencer designed for mobile-first use. It allows users to program 16-step drum patterns across 8 voices, adjust per-voice parameters (level, decay, tone, pitch), and play back patterns in real time with synthesised or sampled drum sounds.

The interface is a single-screen app — no navigation, no modals, no pages. Everything is visible and reachable with one thumb.

---

## 2. Target platform

- **Primary:** Mobile web (iOS Safari, Android Chrome) — viewport 390×844 and equivalent
- **Secondary:** Desktop web (responsive, but mobile is the design target)
- **Technology:** React single-page app, Web Audio API via Tone.js for audio engine
- **Offline:** Should work fully offline once loaded (no server-side audio processing)

---

## 3. Screen layout (top to bottom)

Reference: sketch `1000051765.jpg`

The entire UI fits within a single mobile viewport with no scrolling. The vertical stack from top to bottom is:

```
┌─────────────────────────────┐
│  ① STEP SEQUENCER           │  2 rows × 8 steps = 16 steps
│  □□□□□□□□                   │  Tap to toggle steps on/off
│  □□□□□□□□                   │  for selected voice
├─────────────────────────────┤
│  ② TRANSPORT     [■] [▶]   │  Stop + Play buttons
│                  BPM: 120   │  Right-aligned with BPM readout
├─────────────────────────────┤
│                             │
│    ○    ○    ○              │
│       ◇ ◇                  │
│  ○   ◇   ◇   ○            │  ③ PAD CONSTELLATION
│       ◇ ◇                  │  8 voice pads in outer orbit
│    ○    ○    ○              │  4–5 param diamonds inner orbit
│                             │
├─────────────────────────────┤
│  ④ MULTI SLIDER             │  Full-width horizontal slider
│  ━━━━━━━━━━━●━━━━━━━━━━━━  │  Controls whatever is selected
│  LEVEL              75      │  Label + readout
└─────────────────────────────┘
```

### Layout rationale

- **Sequencer at top:** The pattern is the primary visual feedback — you see the beat structure immediately. Top placement also keeps it away from the thumb zone, preventing accidental edits while playing.
- **Transport between sequencer and pads:** Acts as a visual divider. Transport is used less frequently than pads/slider, so mid-screen placement is fine.
- **Pad constellation centre-lower:** Sits in the natural thumb reach zone for one-handed mobile use. This is the most frequently tapped area.
- **Slider anchored to bottom:** Always accessible without reaching. Full width maximises precision on a small screen.

---

## 4. Feature requirements

### 4.1 Voice pads

**FR-PAD-01:** The app shall display 8 drum voice pads arranged in a circular/constellation layout.

**FR-PAD-02:** Each pad shall be visually distinct via a unique colour (see design language).

**FR-PAD-03:** Tapping a pad shall select that voice. Only one voice can be selected at a time.

**FR-PAD-04:** When a voice is selected:
- The step sequencer updates to show that voice's 16-step pattern.
- The slider updates to show that voice's currently active parameter value.
- The slider fill colour changes to match the selected voice.

**FR-PAD-05:** During playback, pads shall visually pulse (scale + glow) when their voice is triggered by an active step on the current beat.

**FR-PAD-06:** Pad labels shall use 3-letter abbreviations: KCK, SNR, CHH, OHH, CLP, TOM, RIM, PRC.

### 4.2 Parameter diamonds

**FR-DIA-01:** 4 parameter diamonds shall be displayed in a smaller inner orbit within the pad circle: Level, Decay, Tone, Pitch.

**FR-DIA-02:** Tapping a diamond selects that parameter. Only one parameter can be active at a time.

**FR-DIA-03:** When a parameter is selected:
- The slider label updates to show the parameter name (e.g. `DECAY`).
- The slider value updates to show the current value of that parameter for the currently selected voice.

**FR-DIA-04:** Diamonds are visually monochrome (not voice-coloured) to distinguish them from pads.

**FR-DIA-05:** The default selected parameter on app load is Level.

### 4.3 Multi slider

**FR-SLD-01:** A single full-width horizontal slider shall be displayed at the bottom of the screen.

**FR-SLD-02:** The slider controls whichever parameter is currently selected (via diamonds) for whichever voice is currently selected (via pads).

**FR-SLD-03:** The slider fill colour shall match the currently selected voice colour.

**FR-SLD-04:** The slider shall display:
- A label above/left showing the active parameter name (uppercase, e.g. `LEVEL`)
- A numeric readout to the right showing the current integer value (0–100)

**FR-SLD-05:** Slider changes shall take effect in real time (no confirm step).

**FR-SLD-06:** The slider range is 0–100 with integer steps.

### 4.4 Step sequencer

**FR-SEQ-01:** The step sequencer shall display 16 steps in a 2-row × 8-column grid layout.

**FR-SEQ-02:** The sequencer shows the pattern for the currently selected voice only.

**FR-SEQ-03:** Tapping a step toggles it on (voice colour) or off (neutral).

**FR-SEQ-04:** During playback, the current step position shall be visually indicated (playhead marker — bottom border highlight or equivalent).

**FR-SEQ-05:** Each voice has an independent 16-step pattern. Switching voices switches the displayed pattern.

**FR-SEQ-06:** Steps read left-to-right, top row first: steps 1–8 top row, steps 9–16 bottom row.

### 4.5 Transport controls

**FR-TRN-01:** Two transport buttons: Play and Stop.

**FR-TRN-02:** Play starts sequencer playback from the current step position. If stopped, playback starts from step 1.

**FR-TRN-03:** Stop halts playback and resets the playhead to step 1.

**FR-TRN-04:** During playback:
- The play button visually inverts (filled state) to indicate playing.
- The playhead advances through steps at the rate defined by BPM.
- All 8 voices play simultaneously — the sequencer checks all voice patterns on each step and triggers sounds for any voice that has that step active.

**FR-TRN-05:** BPM shall be displayed alongside transport controls.

**FR-TRN-06:** BPM shall be adjustable. Minimum implementation: tap the BPM display to enter edit mode, or provide +/− buttons. Range: 60–200 BPM.

### 4.6 Audio engine

**FR-AUD-01:** Each voice shall produce a distinct synthesised drum sound using Web Audio API / Tone.js.

**FR-AUD-02:** Suggested sound mapping:
| Voice | Synthesis approach |
|-------|-------------------|
| Kick  | `Tone.MembraneSynth` — low-frequency sine with pitch envelope |
| Snare | `Tone.NoiseSynth` + `Tone.MembraneSynth` — noise burst + body |
| Closed HH | `Tone.MetalSynth` — short decay, high frequency |
| Open HH | `Tone.MetalSynth` — longer decay |
| Clap | `Tone.NoiseSynth` — filtered noise burst, short |
| Tom | `Tone.MembraneSynth` — mid-frequency |
| Rim | `Tone.MetalSynth` or `Tone.MembraneSynth` — short, bright |
| Perc | `Tone.NoiseSynth` — band-passed noise, very short |

**FR-AUD-03:** The Level parameter (0–100) shall control the output gain of each voice (mapped to 0.0–1.0 or equivalent dB range).

**FR-AUD-04:** The Decay parameter (0–100) shall control the amplitude envelope release time.

**FR-AUD-05:** The Tone parameter (0–100) shall control a filter cutoff frequency (low-pass) applied to the voice output.

**FR-AUD-06:** The Pitch parameter (0–100) shall control the base frequency/pitch offset of the voice.

**FR-AUD-07:** Sequencer timing MUST use `Tone.Transport` or equivalent sample-accurate scheduling. `setInterval`/`setTimeout` are not acceptable — they drift.

**FR-AUD-08:** Audio context must be started on a user gesture (tap/click) to comply with browser autoplay policies. Show a "tap to start" overlay if needed on first load.

---

## 5. State management

### 5.1 App state shape

```typescript
interface OrbitState {
  // Selection
  selectedVoice: number;            // 0–7
  selectedParam: 'level' | 'decay' | 'tone' | 'pitch';

  // Per-voice parameters (8 voices × 4 params)
  voiceParams: Record<number, {
    level: number;                  // 0–100, default 75
    decay: number;                  // 0–100, default 50
    tone: number;                   // 0–100, default 50
    pitch: number;                  // 0–100, default 50
  }>;

  // Patterns (8 voices × 16 steps)
  patterns: Record<number, boolean[]>;  // each is boolean[16]

  // Transport
  isPlaying: boolean;
  bpm: number;                      // 60–200, default 120
  currentStep: number;              // 0–15
}
```

### 5.2 Default state

On app load:
- Voice 0 (Kick) is selected
- Parameter "level" is selected
- All voices initialised with default params: level 75, decay 50, tone 50, pitch 50
- All patterns empty (all steps off)
- Transport stopped, BPM 120, currentStep 0

---

## 6. Interaction flows

### 6.1 Programming a beat

1. User taps a voice pad (e.g. KCK) → pad highlights, sequencer shows kick pattern
2. User taps steps in the sequencer to toggle them on → steps fill with kick colour
3. User taps another voice pad (e.g. SNR) → sequencer switches to snare pattern
4. User programs snare steps
5. User taps Play → all programmed voices play back simultaneously

### 6.2 Adjusting a parameter

1. User taps a voice pad (e.g. CHH)
2. User taps a parameter diamond (e.g. Decay)
3. Slider label changes to `DECAY`, slider shows current decay value for closed hat
4. User drags slider → decay value updates in real time, affecting audio immediately

### 6.3 Adjusting level (shortcut)

1. User taps a voice pad → Level is the default parameter, slider immediately shows level
2. User drags slider → level changes
3. No need to tap the Level diamond first (it's pre-selected)

---

## 7. Non-functional requirements

**NFR-01: Performance.** UI must render at 60fps during playback. No jank on slider drag or step toggling while sequencer is running.

**NFR-02: Touch targets.** All tappable elements must meet a 44×44px minimum touch target.

**NFR-03: Single viewport.** The entire UI must fit within a 390×844 viewport with no scrolling.

**NFR-04: Offline capable.** After initial load, the app must function without network connectivity.

**NFR-05: Audio latency.** Perceived latency between a step trigger and audio output must be <20ms.

**NFR-06: No scrolling.** `overflow: hidden` on the body. The UI is a fixed instrument panel.

---

## 8. Future considerations (out of scope for v1)

These are NOT part of the initial build but are worth keeping the architecture open for:

- **Pattern save/load** — persist patterns to localStorage or cloud
- **Swing/shuffle** — offset even-numbered steps for groove
- **Pattern length** — variable step count (8, 16, 32, 64)
- **Sample upload** — replace synth voices with user samples
- **Kit presets** — switchable sets of voice parameters
- **Visual EQ / waveform** — real-time audio visualisation in the pad circle centre
- **MIDI output** — send patterns to external hardware/DAW
- **Pattern chaining** — link multiple 16-step patterns into longer sequences

---

## 9. Acceptance criteria

The app is considered complete for v1 when:

1. All 8 voice pads are displayed in a circular layout and are individually selectable
2. 4 parameter diamonds are displayed and are individually selectable
3. The slider controls the selected parameter for the selected voice in real time
4. The 16-step sequencer (2×8 grid) displays and edits patterns per voice
5. Transport play/stop works with audible drum synthesis at the displayed BPM
6. All 8 voices sound distinct and are audible simultaneously during playback
7. The Level, Decay, Tone, and Pitch parameters audibly affect each voice
8. The playhead visually advances through steps during playback
9. Pads pulse when their voice triggers during playback
10. The entire UI fits on a 390×844 mobile viewport with no scrolling
11. The visual design matches the ORBIT design language specification
