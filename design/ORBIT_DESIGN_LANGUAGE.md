# ORBIT — Design language for drum machine UI

## Overview

ORBIT is a mobile-first drum machine / step sequencer UI. The design language is built around a central circular pad layout with parameter controls, a step sequencer, transport controls, and a single shared slider. The aesthetic is **minimalist, spacey, and luminous** — bright clean surfaces with colour that glows outward from active elements. Think "ISS control room", not "nightclub."

This document is the single source of truth for all visual and interaction design decisions. When building UI components, follow this spec exactly.

---

## Design principles

1. **Constellation, not grid.** Drum pads orbit a centre point in a circle. The circular layout is the UI's signature — it echoes orbital mechanics and gives each voice spatial identity. Never flatten this to a grid.

2. **Colour is identity.** Every drum voice owns a colour across the entire interface — its pad, its sequencer steps, its slider fill, its parameter readout. One glance tells you what you're editing.

3. **Glow, not shadow.** Active elements radiate outward via `box-shadow` halos. The UI breathes light rather than casting darkness. No drop shadows, no inset shadows, no dark gradients.

4. **One verb at a time.** Select a pad, then adjust. Select a parameter, then slide. The interface never asks for two decisions simultaneously. Mobile thumbs get one clear action.

---

## Colour system

### Voice palette

Each drum voice owns a single colour used everywhere that voice appears (pad, sequencer steps, slider fill, parameter readouts).

| Voice     | Hex       | Usage                          |
|-----------|-----------|--------------------------------|
| Kick      | `#378ADD` | Blue — anchor, foundation      |
| Snare     | `#D85A30` | Coral — punchy, forward        |
| Closed HH | `#1D9E75` | Deep teal — crisp, tight       |
| Open HH   | `#5DCAA5` | Light teal — airy, sustained   |
| Clap      | `#D4537E` | Pink — sharp, transient        |
| Tom       | `#EF9F27` | Amber — warm, resonant         |
| Rim       | `#7F77DD` | Purple — metallic, bright      |
| Perc      | `#639922` | Green — organic, textural      |

### Surface palette

Use CSS custom properties for all surfaces so the UI adapts to light/dark mode:

```css
--orbit-surface:    var(--color-background-primary);    /* card/panel bg */
--orbit-well:       var(--color-background-secondary);   /* recessed areas, tracks */
--orbit-well-bright: #262626;                            /* playhead highlight on recessed areas */
--orbit-canvas:     var(--color-background-tertiary);    /* page bg */
--orbit-ink:        var(--color-text-primary);           /* primary text, icons */
--orbit-label:      var(--color-text-secondary);         /* secondary labels */
--orbit-hint:       var(--color-text-tertiary);          /* hints, inactive text */
--orbit-border:     var(--color-border-tertiary);        /* 0.5px borders */
```

### Colour usage rules

- The slider fill colour ALWAYS matches the currently selected voice.
- Sequencer step "on" state uses the selected voice colour at full opacity.
- Sequencer step "off" state uses `var(--orbit-well)` with a `0.5px` border of `var(--orbit-border)`.
- The diamond parameter selectors are monochrome — `var(--orbit-well)` when idle, `var(--orbit-ink)` when active. They do NOT take the voice colour.
- Transport buttons are monochrome — outlined in `var(--orbit-ink)`.
- When playing, the play button inverts to a filled circle (ink bg, surface icon).

---

## Typography

**Single typeface: JetBrains Mono** throughout the entire UI. Load from Google Fonts or bundle.

```
@import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500&display=swap');
```

### Type scale

| Role               | Size  | Weight | Case       | Tracking       | Example              |
|--------------------|-------|--------|------------|----------------|----------------------|
| Display value      | 22px  | 500    | Normal     | -0.5px         | BPM readout: `120`   |
| Parameter value    | 16px  | 500    | Normal     | -0.5px         | Slider readout: `65` |
| Pad label          | 10px  | 400/500| Uppercase  | 0.5px          | `KCK`, `SNR`         |
| Section header     | 11px  | 400    | Uppercase  | 2px            | `SEQUENCE`, `VOICE`  |
| Body label         | 12px  | 400    | Normal     | 0              | Parameter names      |
| Readout            | 12px  | 500    | Normal     | 0              | Inline values        |

### Rules

- Only two weights: 400 (regular) and 500 (medium). Never use 600 or 700.
- Hierarchy comes from weight, case, and size — not font changes.
- Pad labels use 3-letter abbreviations: `KCK`, `SNR`, `CHH`, `OHH`, `CLP`, `TOM`, `RIM`, `PRC`.
- All section headers are uppercase with wide letter-spacing (2px).

---

## Component specifications

### Drum pads (circle)

Pads are arranged in a circle (the "orbit") around a central point.

```
Pad diameter:       52px
Orbit radius:       120px (centre of pad to centre of circle)
Touch target:       52px (meets 44px minimum)
Border radius:      50% (perfect circle)
```

#### States

| State     | Background                        | Border                              | Effect                                          | Text colour |
|-----------|-----------------------------------|-------------------------------------|-------------------------------------------------|-------------|
| Inactive  | Voice colour at 18% opacity       | 0.5px solid, voice colour at 40%    | None                                            | `var(--orbit-hint)` |
| Selected  | Voice colour at 100%              | None                                | `box-shadow: 0 0 16px {voice colour at 35%}`    | `#FFFFFF`   |
| Triggered | Voice colour at 100%              | None                                | `box-shadow: 0 0 20px {voice colour at 50%}`, `transform: scale(1.05)` | `#FFFFFF` |

- Triggered state fires on playback when the sequencer hits a step with that voice active. It pulses briefly (120ms) then returns to selected or inactive.
- Only one pad can be selected at a time. Selecting a pad immediately updates the slider, parameter diamonds, and sequencer view to show that voice's data.

### Parameter diamonds

Diamonds sit in a smaller orbit inside the pad circle. They control per-voice parameters (decay, tone, pitch, etc.).

```
Diamond size:       28px (side length before rotation)
Diamond orbit:      48px (centre to centre of circle)
Touch target:       44px (invisible hit area around the diamond)
Rotation:           45deg (square rotated to diamond)
```

#### States

| State   | Background            | Border                         |
|---------|-----------------------|--------------------------------|
| Idle    | `var(--orbit-well)`   | `0.5px solid var(--orbit-border)` |
| Active  | `var(--orbit-ink)`    | None                           |

- Diamonds are ALWAYS monochrome — they never inherit voice colour.
- Selecting a diamond routes the slider to control that parameter for the currently selected voice.
- Diamond labels appear as small text below each diamond (e.g. `decay`, `tone`, `pitch`). Use 10px JetBrains Mono, `var(--orbit-hint)`.

### Slider

A single horizontal slider shared across all controls. Its appearance changes based on what's selected.

```
Track height:       3px
Track background:   var(--orbit-well)
Track fill:         Current voice colour (when a pad is selected)
                    var(--orbit-ink) (when a parameter diamond is selected with no voice)
Track border-radius: 2px
Thumb diameter:     16px
Thumb background:   var(--orbit-surface)
Thumb border:       2px solid {fill colour}
Thumb border-radius: 50%
```

- The numeric readout sits to the right of the slider: 12px JetBrains Mono 500, `var(--orbit-ink)`.
- Above the slider, show the current control label: 11px JetBrains Mono 400 uppercase, `var(--orbit-hint)`. E.g. `LEVEL` or `DECAY`.
- Slider range: 0–100 integer values. Use `step="1"`.

### Transport controls

Circular outlined buttons for play/stop.

```
Button diameter:    44px
Border:             1.5px solid var(--orbit-ink)
Border radius:      50%
Background:         transparent (idle), var(--orbit-ink) (playing/active)
Icon colour:        var(--orbit-ink) (idle), var(--orbit-surface) (playing/active)
```

#### Play icon
- Triangle: `border-left: 10px solid`, `border-top: 7px solid transparent`, `border-bottom: 7px solid transparent`. Offset `margin-left: 3px` to optically centre.

#### Stop icon
- Square: `12px × 12px`, `border-radius: 1px`.

#### Playing state
- Play button fills solid: background becomes `var(--orbit-ink)`, icon becomes `var(--orbit-surface)`.

### Step sequencer

16 steps in a **2-row × 8-column grid**, positioned at the TOP of the screen. Steps 1–8 on the top row, 9–16 on the bottom row. Shows the pattern for the currently selected voice.

```
Grid:               2 rows × 8 columns
Step size:          Flexible — fill available width across 8 columns + gaps
Step aspect ratio:  1:1 (square)
Step border-radius: 4px
Step gap:           4px (horizontal and vertical)
Grid padding:       16px horizontal
```

#### States

| State      | Background              | Border                              |
|------------|-------------------------|-------------------------------------|
| Off        | `var(--orbit-well)`     | `0.5px solid var(--orbit-border)`   |
| On         | Current voice colour    | None                                |

#### Playhead (current step highlight)

The currently playing step receives a prominent highlight so the user can always see where the playhead is in the sequence. The highlight combines a **ring**, an optional **glow**, and a **bottom bar**.

| Sub-state          | Background                  | Ring (box-shadow outline)                  | Glow                                          | Bottom bar                           |
|--------------------|-----------------------------|--------------------------------------------|-----------------------------------------------|--------------------------------------|
| Playhead + Off     | `var(--orbit-well-bright)` (`#262626`) | `1.5px` ring via `box-shadow` in `var(--orbit-ink)`             | None                                          | `2px` bottom bar in `var(--orbit-ink)` |
| Playhead + On      | Current voice colour        | `0 0 0 1.5px var(--orbit-ink)`             | `0 0 10px` in voice colour (radiates outward) | `2px` bottom bar in `var(--orbit-ink)` |

- The ring and glow use `box-shadow` (never `outline`, which doesn't follow `border-radius`).
- Playhead advance is **instant** (0ms) — no transition on the ring or glow.
- The bottom bar is rendered via a `::after` pseudo-element, same as before.

- Tapping a step toggles it on/off for the selected voice.
- The sequencer always shows the pattern for whichever voice pad is currently selected.

### BPM display

```
Font:               22px JetBrains Mono 500
Colour:             var(--orbit-ink)
Letter-spacing:     -0.5px
```

- Positioned near the transport controls.
- Tap to edit (if interactive), or pair with +/- buttons.

---

## Layout — mobile viewport

Target viewport: **390 × 844px** (iPhone 14 / standard modern mobile).

The entire UI fits in a single viewport with **no scrolling** (`overflow: hidden` on body).

### Vertical stack (top to bottom, per sketch)

```
┌─────────────────────────────┐
│  ① STEP SEQUENCER           │  Top of screen
│  2 rows × 8 cols            │  ~100px
├─────────────────────────────┤
│  ② TRANSPORT + BPM          │  Right-aligned row
│                 [■][▶] 120  │  ~48px
├─────────────────────────────┤
│                             │
│    ○    ○    ○              │
│       ◇ ◇                  │
│  ○   ◇   ◇   ○            │  ③ PAD CONSTELLATION
│       ◇ ◇                  │  Hero area — takes remaining
│    ○    ○    ○              │  vertical space (flex: 1)
│                             │
├─────────────────────────────┤
│  LEVEL              75      │  ④ SLIDER AREA
│  ━━━━━━━━━●━━━━━━━━━━━━━━  │  Label + track + readout
│                             │  ~64px with padding
└─────────────────────────────┘
```

1. **Step sequencer (top)** — 2×8 grid. Top placement keeps it visible as primary feedback but out of the thumb zone to avoid accidental edits during play.
2. **Transport row** — Stop, Play buttons + BPM readout. Right-aligned. Sits between sequencer and pads as a visual divider.
3. **Pad constellation (centre, hero)** — Takes all remaining vertical space (`flex: 1`). The outer orbit of 8 voice pads and inner orbit of 4 parameter diamonds. Centre the orbits in this area both horizontally and vertically.
4. **Slider area (bottom)** — Full-width horizontal slider anchored to the bottom. Parameter label (left) + slider track + numeric readout (right). Always within thumb reach.

### Spacing

```
Page horizontal padding:   24px
Section vertical gap:      24px
Inner component gaps:      8px–12px
```

### Safe areas

Respect iOS safe area insets:
```css
padding-top: env(safe-area-inset-top);
padding-bottom: env(safe-area-inset-bottom);
```

---

## Motion and transitions

### Easing

All transitions use: `cubic-bezier(0.2, 0.8, 0.3, 1)` — a soft, slightly bouncy ease-out.

### Durations

| Transition          | Duration |
|---------------------|----------|
| Pad select          | 120ms    |
| Pad trigger pulse   | 120ms    |
| Slider thumb move   | 60ms     |
| Diamond select      | 100ms    |
| Step toggle         | 80ms     |
| Playhead advance    | 0ms (instant) |

### Glow animation

When a pad is triggered during playback, it briefly pulses:

```css
@keyframes pad-trigger {
  0%   { transform: scale(1);    box-shadow: 0 0 16px rgba(color, 0.35); }
  50%  { transform: scale(1.05); box-shadow: 0 0 20px rgba(color, 0.50); }
  100% { transform: scale(1);    box-shadow: 0 0 16px rgba(color, 0.35); }
}
```

Duration: 120ms, `ease-out`. Only applies to pads that are both selected AND triggered.

---

## Interaction model

### Selection flow

1. User taps a **pad** → pad becomes selected, slider shows `LEVEL`, sequencer shows that voice's pattern, slider fill takes voice colour.
2. User taps a **diamond** → diamond becomes active, slider label changes to that parameter name (e.g. `DECAY`), slider shows current value for the selected voice + parameter combo.
3. User drags **slider** → value updates in real time for the selected voice + selected parameter.
4. User taps a **sequencer step** → toggles that step on/off for the selected voice.

### Default state

On load:
- First pad (Kick) is selected.
- First diamond (Level) is active.
- Slider shows kick level.
- Sequencer shows kick pattern.
- Transport is stopped.

---

## Technical notes for implementation

### Framework

Build as a single-file React component (`.jsx`). Use Tailwind utility classes where possible, inline styles for voice-colour-dependent values that can't be expressed as static classes.

### State shape

```typescript
interface OrbitState {
  selectedVoice: number;          // 0–7 index into voices array
  selectedParam: string;          // 'level' | 'decay' | 'tone' | 'pitch'
  voiceParams: {
    [voiceIndex: number]: {
      level: number;              // 0–100
      decay: number;              // 0–100
      tone: number;               // 0–100
      pitch: number;              // 0–100
    };
  };
  patterns: {
    [voiceIndex: number]: boolean[]; // 16 booleans per voice
  };
  isPlaying: boolean;
  bpm: number;                    // 60–200
  currentStep: number;            // 0–15
}
```

### Voice definitions

```typescript
const VOICES = [
  { id: 'kick',    label: 'KCK', colour: '#378ADD' },
  { id: 'snare',   label: 'SNR', colour: '#D85A30' },
  { id: 'closedHH', label: 'CHH', colour: '#1D9E75' },
  { id: 'openHH',  label: 'OHH', colour: '#5DCAA5' },
  { id: 'clap',    label: 'CLP', colour: '#D4537E' },
  { id: 'tom',     label: 'TOM', colour: '#EF9F27' },
  { id: 'rim',     label: 'RIM', colour: '#7F77DD' },
  { id: 'perc',    label: 'PRC', colour: '#639922' },
];
```

### Audio

Use the **Web Audio API** with `Tone.js` for scheduling. Each voice maps to a synthesised drum sound or a loaded sample via `Tone.Player` or `Tone.MembraneSynth` / `Tone.NoiseSynth` / `Tone.MetalSynth`.

The sequencer loop should use `Tone.Sequence` or `Tone.Transport.scheduleRepeat` for sample-accurate timing. Do NOT use `setInterval` — it drifts.

---

## File structure suggestion

```
orbit/
├── DESIGN.md              ← design language (this file)
├── PRD.md                 ← product requirements document
├── src/
│   ├── Orbit.jsx          ← main component (full-screen layout)
│   ├── hooks/
│   │   └── useSequencer.ts ← Tone.js transport + scheduling
│   ├── components/
│   │   ├── StepSequencer.jsx ← 2×8 grid (top of screen)
│   │   ├── Transport.jsx     ← play/stop + BPM
│   │   ├── PadCircle.jsx     ← pad orbit layout
│   │   ├── Diamonds.jsx      ← parameter diamonds
│   │   └── Slider.jsx        ← shared multi slider (bottom)
│   └── constants.ts       ← VOICES, PARAMS, defaults
```

---

## CLAUDE.md integration

Add the following to your project's `CLAUDE.md` (or create one):

```markdown
## Design references

- Follow the design language in `DESIGN.md` for all visual decisions — colours, typography, spacing, component states, and motion.
- Follow the product requirements in `PRD.md` for feature behaviour, interaction flows, state shape, and acceptance criteria.
- The layout order (top to bottom) is: step sequencer → transport → pad constellation → slider. Do not reorder.
- All colours for voice-specific elements must use the exact hex values from the voice palette in DESIGN.md.
- Typography is JetBrains Mono only — no other fonts.
- Mobile-first: the entire UI must fit in 390×844 with no scrolling.
- Audio scheduling must use Tone.Transport — never setInterval.
```
