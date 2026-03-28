# Sequential Circuits Prophet-5: Complete Technical Reference
## For Software Recreation / DSP Implementation

---

## 1. Signal Flow / Architecture

Each of the Prophet-5's five voices is an independent analog synthesizer with the following signal path:

```
Keyboard/Z80 CPU
    |
    v
[DAC + Sample/Hold] --> Control Voltages
    |
    v
+--[OSC A (VCO)]--+     +--[OSC B (VCO)]--+
|  Saw / Pulse     |     |  Saw / Tri / Pulse |
+------------------+     +--------------------+
         |                         |
         v                         v
    +---------[MIXER]----------+
    | Osc A Level | Osc B Level | Noise Level |
    +------------------------------+
                  |
                  v
         [VCF (Low-Pass Filter)]  <-- Filter ADSR Envelope
         | 4-pole, 24dB/oct     |  <-- Keyboard Tracking
         | Resonance (Q)        |  <-- Poly Mod
         +----------------------+
                  |
                  v
         [VCA (Amplifier)]  <-- Amplifier ADSR Envelope
                  |
                  v
         Voice Output --> [Voice Summer] --> [Master Volume VCA] --> Output
```

### Global (shared across all 5 voices):
- 1x LFO (CEM3340-based, not keyboard-tracked)
- 1x Noise generator (white noise to mixer, pink noise to Wheel Mod)
- Pitch wheel
- Mod wheel
- Poly Mod routing (per-voice but same settings)
- Wheel Mod routing

### Digital Control System:
- Z80 CPU (Rev 1/2) or D780C NEC (later Rev 3)
- Dual 14-bit DAC system
  - Standard control: 7 MSBs = 128 steps of 83.3mV (~10.67V range)
  - Oscillator tuning: Full 14 bits = 16,384 steps of 651uV resolution (1/128 semitone precision)
- 23 Sample/Hold circuits on PCB 3 (Common Analog + Patch CVs)
- 15 Sample/Hold circuits on PCB 4 (Individual OSC and FILT CVs)
- 8253 Programmable Interval Timer for autotune

---

## 2. Oscillators (VCOs)

### Rev 1/2: SSM2030 (Solid State Music)
### Rev 3/3.3: CEM3340 (Curtis Electromusic Specialties)

Both are triangle-core VCOs with 1V/octave exponential frequency control.

### CEM3340 Specifications:

| Parameter | Value |
|-----------|-------|
| Topology | Triangle core |
| Frequency range | ~0.2 Hz to >20 kHz (10+ octave range) |
| Tracking | 1V/octave exponential |
| Max supply voltage | 24V between pins (typically +/-15V) |
| Timing capacitor | 1nF (polystyrene film or mica, temperature-stable) |
| Multiplier input resistor | 1.8K (standard) |
| Temperature compensation | On-chip (pins 1-2), generates temp-dependent voltage multiplied by incoming CV |

### Waveform Output Levels (at +15V supply):

| Waveform | Level |
|----------|-------|
| Ramp (Sawtooth) | 0 to ~10V (2/3 of V+) |
| Triangle | 0 to ~5V (1/3 of V+) |
| Pulse/Square | 0 to ~13.7V (V+ minus 1.3V), open emitter output with pull-down resistor |

### Waveforms Available Per Oscillator:

| Waveform | Osc A | Osc B |
|----------|-------|-------|
| Sawtooth | Yes | Yes |
| Pulse (with PWM) | Yes | Yes |
| Triangle | No | Yes |

**Waveforms are simultaneously selectable** -- you can combine saw + pulse on the same oscillator for richer timbres.

### Pulse Width Modulation:
- Manual pulse width control per oscillator
- Modulatable via LFO (Wheel Mod) and Poly Mod (Osc A only)
- Range: narrow pulse to 50% square wave

### Oscillator Sync:
- **Hard sync**: Osc A can be hard-synced to Osc B
- Hard sync resets the triangle core unconditionally when the master (Osc B) completes a cycle
- Implementation uses transistor-based triangle reset circuit (datasheet "Figure 5" method)
- Sync pulse amplitude at pin 6: 1V to 3V, negative-going pulses cause triangle core reversal
- **Soft sync** (pin 9): Reverses waveform when near peak, amplitude of input pulse sets the percentage either side of the peak at which reversal occurs

### Prophet-5 Rev 3 Implementation Notes:
- Omits the high-frequency tracking trimmer on pin 7
- Relies on internal autotune routine for HF compensation
- Pin 7 (HF tracking output) compensates for comparator switching delays above 5kHz
- Since oscillators can go flat as frequency increases (finite reversal time of triangle core), the autotune system compensates

### SSM2030 Differences (Rev 1/2):
- Same waveforms: sawtooth, triangle, pulse with PWM, soft sync
- 1V/octave scaling
- Required external tempco resistor soldered to pins and epoxied to chip top
- Less stable than CEM3340 (more drift, more "organic" character)
- Slightly different waveshaping characteristics

### Tuning System:
- Oscillator range: 9 octaves basic range
- Each oscillator CV combines: keyboard component (0-5V), frequency knob (0-4V), fine-tuning bias, poly-mod
- Osc B has fine-tune control (+/- ~7 semitones)
- Autotune tests C3 to C9, then extrapolates the curve for lower octaves (counting low-frequency pulses would take too long)

---

## 3. Filter (VCF)

### Rev 1/2: SSM2040 (Solid State Music)
### Rev 3/3.3: CEM3320 (Curtis Electromusic Specialties)

Both are 4-pole (24dB/octave) lowpass filters, but with different topologies and sonic character.

### SSM2040 (Rev 1/2):

| Parameter | Value |
|-----------|-------|
| Topology | 4 cascaded OTA stages (no integrated resonance VCA) |
| Slope | 24dB/octave (4-pole) |
| Frequency control range | 10,000:1 exponential (>13 octaves) |
| Resonance | External feedback path (designer-defined architecture) |
| Self-oscillation | Yes, with sufficient feedback gain |

**Key Nonlinearity -- The SSM2040 Sound:**
- Each filter stage uses an OTA (Operational Transconductance Amplifier) with a differential transistor pair input
- The differential pair implements a **tanh()** transfer function
- For small signals (below ~1Vpp at the input attenuating resistors), tanh approximates linearity
- When overdriven, produces **asymmetric soft clipping** -- this is the primary source of the SSM2040's "creamy" sound
- The gain cells cannot sink current much below ground, creating a characteristic asymmetric distortion
- Resonance feedback adds harmonics through this same soft-clipping mechanism
- **This is the single most important nonlinearity to model for authentic SSM2040 emulation**

**OTA Stage Structure:**
- Core: Q1, Q2, Q3, Q4 differential pair converts voltage to current, scaled by bias current
- Each stage = OTA + capacitor + Darlington follower = current-controlled integrator with negative feedback
- Transfer function of each stage: first-order lowpass
- 4 stages cascaded = 4th-order (24dB/oct) lowpass

### CEM3320 (Rev 3/3.3):

| Parameter | Value |
|-----------|-------|
| Topology | 4 OTA + buffer stages with integrated resonance VCA and exponential converter |
| Slope | 24dB/octave (lowpass configuration) |
| Frequency CV input | -6V to +6V max, 18mV/octave (60mV/decade) |
| Practical CV range | 0 to 200mV covers full audio bandwidth |
| Resonance input | Current-controlled, 100uA maximum |
| Self-oscillation | Yes, but oscillation amplitude is much less at lower frequencies than at high frequencies (passband droop) |

**CEM3320 Frequency Control:**
- Inverted control voltage (compared to intuitive direction)
- Suggested CV range: -25mV to +155mV for full audio sweep
- Matches VCO scale: 1V/octave (but in 18mV/oct at chip level, scaled externally)

**CEM3320 Resonance Behavior:**
- Integrated feedback VCA for voltage-controlled resonance (unlike SSM2040)
- At maximum resonance (self-oscillation), signal level drops significantly at lower frequencies
- Sequential Pro-One/Prophet-5 implementation uses buffered output (gain x3.4) to create more uniform resonance response across frequency spectrum
- Pro-One uses 200K series resistor with 0-15V control = ~75uA max (under the 100uA spec limit)

**Prophet-5 Rev 3 Filter Implementation:**
- CEM3320 capacitors were at **half the value** of typical setups, shifting everything one octave higher
- This means the filter tracks differently than a "standard" CEM3320 circuit
- Filter CV sums: individual S/H + common analog sum + poly-mod, all with adjustable gain

### Key Sonic Differences (SSM2040 vs CEM3320):

| Characteristic | SSM2040 (Rev 1/2) | CEM3320 (Rev 3) |
|---------------|-------------------|------------------|
| Overall character | Warm, creamy, "organic" | Cleaner, brighter, slightly "colder" |
| Resonance | Softer, more musical self-oscillation | Tighter, more precise resonance |
| Overdrive | Asymmetric soft clipping, adds harmonics | Less distortion, more transparent |
| Low end | Fuller bass response | Slightly thinner |
| Noise floor | Higher | Lower noise, improved resonance level |
| Stability | Less consistent | More consistent and predictable |

---

## 4. Envelope Generators (ADSR)

### Rev 1/2: SSM2050 (Solid State Music)
### Rev 3/3.3: CEM3310 (Curtis Electromusic Specialties)

Two envelope generators per voice: one for VCF, one for VCA.

### CEM3310 Specifications:

| Parameter | Value |
|-----------|-------|
| Type | ADSR (Attack, Decay, Sustain, Release) |
| Time control range | 50,000:1 minimum |
| Attack time | ~0.5ms to ~10s (exponential voltage control) |
| Decay time | ~0.5ms to ~10s (exponential voltage control) |
| Release time | ~0.5ms to ~10s (exponential voltage control) |
| Sustain | 0% to 100% of peak voltage Vp (linearly controlled) |
| Curve shape | **True RC envelope shape** (exponential charge/discharge) |
| Control feedthrough | 90uV max (exceptionally low) |
| Gate/Trigger | Independent Gate and Trigger inputs |

### Envelope Curve Shapes:

**Attack phase:**
- Exponential charge curve (RC charging toward a voltage higher than the peak)
- The attack charges toward approximately 1.5x the peak voltage, then switches to decay when it crosses the peak threshold
- This creates the characteristic "overshoot" feeling of analog ADSR envelopes
- The curve is: `V(t) = Vpeak * (1 - e^(-t/tau_attack)) * overshoot_factor`

**Decay phase:**
- Exponential discharge from peak toward sustain level
- True RC discharge: `V(t) = (Vpeak - Vsustain) * e^(-t/tau_decay) + Vsustain`

**Sustain phase:**
- Constant level, linearly controllable from 0V to Vp

**Release phase:**
- Exponential discharge from current level toward 0V
- `V(t) = Vcurrent * e^(-t/tau_release)`

### Triggering Behavior:
- **Gate input**: Holds the envelope in sustain phase while gate is high; triggers release when gate goes low
- **Trigger input**: Separate from gate -- allows retriggering the attack phase without releasing
- In the Prophet-5, each key press generates both a gate and a trigger
- **Multi-trigger mode** (default): Each new key press retriggers envelopes from their current level
- The attack phase starts from whatever voltage the envelope is currently at (not necessarily 0V)

### SSM2050 vs CEM3310:
- Both implement the same exponential RC curve shapes
- Both have similar functional behavior
- SSM2050 output level is approximately **half the amplitude** of CEM3310 ("one is twice as loud as the other")
- SSM2050 has slightly different timing characteristics contributing to Rev 1/2's "feel"

---

## 5. VCA (Voltage Controlled Amplifier)

### Rev 1/2: SSM2020 (Dual Lin/Log VCA)
### Rev 3/3.3: CA3280 (RCA/Intersil Dual OTA)

**Note: The Prophet-5 did NOT use the CEM3360.** It used CA3280 dual OTAs for its VCA functions.

### VCA Implementation:

| Parameter | Rev 1/2 (SSM2020) | Rev 3 (CA3280) |
|-----------|-------------------|----------------|
| Chip count | 21x SSM2020 | 14x CA3280 |
| Type | Dual linear/logarithmic VCA | Dual Operational Transconductance Amplifier |
| Response modes | Linear and logarithmic | Linear (with PNP current bending) |

### CA3280 (Rev 3) Circuit Details:
- Control current derives from CVs via a **2N4250 PNP transistor** voltage-to-current converter
- The OTA's transconductance adjusts through bias current (Iabc)
- The PNP transistor (base to ground) arrangement can introduce **CV curve bending** -- a subtle nonlinearity
- All VCA circuits in the Prophet-5 use this same OTA + 2N4250 topology

### SSM2020 (Rev 1/2):
- Dual VCA with switchable linear or logarithmic response
- Slightly different response characteristics than CA3280
- Part of the overall "warmer" character of Rev 1/2 units

### VCA Signal Chain:
1. Filter output enters first VCA cell (acts as on/off switch for filter type selection in Rev 4)
2. Second VCA cell controlled by Amplifier ADSR envelope
3. Voice output sent to voice summing bus

---

## 6. LFO Section

### Implementation:
The Prophet-5 uses a **single CEM3340** configured as an LFO (not keyboard-tracked) shared across all 5 voices.

### Specifications:

| Parameter | Value |
|-----------|-------|
| Waveforms | Sawtooth, Triangle, Square |
| Frequency range | ~0.1 Hz to ~20 Hz (sub-audio) |
| Pulse width | Fixed at 50% (square wave) |
| Scope | Global -- one LFO modulates all voices simultaneously |

### Waveform Characteristics:
- **Triangle**: Bipolar -- positive for half cycle, negative for other half. Ideal for vibrato that goes equally sharp and flat around center pitch
- **Sawtooth**: Unipolar positive only (ramp up)
- **Square**: Unipolar positive only

### LFO CV Routing:
The LFO triangle output receives **level-shifting for symmetrical bipolar modulation** when used for frequency modulation. This ensures vibrato is centered around the played pitch rather than only bending in one direction.

---

## 7. Poly Mod and Wheel Mod Sections

### Poly Mod (Per-Voice Modulation):

This is the Prophet-5's most powerful sound design feature -- it operates **independently per voice**, creating different modulation in each voice.

**Sources (amount knobs for each):**
1. Filter Envelope Generator output
2. Oscillator B output (triangle waveform specifically)

**Destinations (on/off switches):**
1. Oscillator A Frequency (FM synthesis)
2. Oscillator A Pulse Width
3. Filter Cutoff Frequency

**Implementation Details:**
- Poly Mod is embedded within each voice's signal path
- Since Osc B runs at audio rate, routing Osc B -> Osc A frequency creates **audio-rate FM** (analog FM synthesis)
- Since each voice's Osc B can be at slightly different frequencies (due to analog drift), the FM effect differs per voice -- this is key to the Prophet-5's "alive" quality
- Filter envelope -> Osc A frequency creates pitch sweeps per note
- Filter envelope -> Filter cutoff adds additional envelope modulation on top of the filter's own ADSR

**Signal Flow:**
```
Filter EG output --[Amount knob]-->
                                     +--> Osc A Freq (switch)
Osc B Triangle --[Amount knob]----> +--> Osc A PW (switch)
                                     +--> Filter Cutoff (switch)
```

### Wheel Mod (Performance Modulation):

Controlled by the Mod Wheel for real-time performance expression.

**Sources (mixed via CV-controlled mixer):**
1. LFO (triangle, sawtooth, or square -- selected by LFO waveform switch)
2. Pink Noise (from noise generator)

The mod wheel acts as a **master attenuator** -- at 0, no modulation passes through regardless of source settings.

**Destinations (on/off switches):**
1. Oscillator A Frequency
2. Oscillator B Frequency
3. Oscillator A Pulse Width
4. Oscillator B Pulse Width
5. Filter Cutoff Frequency

**Implementation:**
- Sources are mixed before the mod wheel attenuator
- The mixed signal passes through the mod wheel amount control
- Then routes to selected destinations via switches
- Wheel Mod is **global** (mono-mod) -- same modulation applied to all voices simultaneously
- This is fundamentally different from Poly Mod which varies per voice

### Pitch Wheel:
- Spring-loaded, returns to center
- Default range: +/- 1 semitone
- Programmable per patch: 1 to 12 semitones (up to 1 octave)

---

## 8. Mixer Section

The mixer combines oscillator and noise signals before the filter.

### Controls:
1. **Oscillator A Level** (knob)
2. **Oscillator B Level** (knob)
3. **Noise Level** (knob) -- white noise

### Implementation Details:
- Waveform selection uses **CD4016 analog switches** controlled by computer latches
- Selected waveforms from each oscillator are mixed with the level knobs
- **White noise** is mixed to the summing node via a resistor (R1 in Rev 4)
- Noise level is **set in one place for all voices** (global setting, not per-voice)
- The combined signal is buffered (Rev 4: U6-D with R181 = 15K in feedback path) before entering the filter

### Rev 4 Mixer Architecture:
- VCO 1 and 2 levels controlled by SSI2164 VCA cells (U4-A and U4-B)
- Outputs merged at summing node
- Noise mixed via R1
- Buffered mix feeds both filter types simultaneously:
  - SSI2140 (SSM-type): input attenuated by 23.2K (R76)
  - CEM3320: input attenuated by 169K (R82)

---

## 9. Voice Architecture

### 5-Voice Polyphony:
- 5 completely independent analog voice circuits
- Each voice has its own: 2x VCO, mixer, VCF, 2x EG, VCA
- Voices are summed together via 30K resistors into a voice summing buffer

### Voice Allocation Algorithm:
- **Primary mode: Last-note priority with sequential assignment**
- Voice 1 assigned to first key, Voice 2 to second, etc.
- After initial 5 assignments, the system uses **last-note priority** (voice stealing)
- When all voices are in use, the oldest note is stolen for the new note
- The system attempts to **reassign the same voice** to a note if that voice is available
- This is NOT a simple cyclic/round-robin allocation

### Key Priority Modes (Rev 3.3 firmware V10.6):
- Last-note priority (default)
- Low-note priority available in later firmware revisions

### Keyboard:
- 61 keys (C to C, 5 octaves)
- No velocity sensitivity (original; Rev 4 added velocity/aftertouch)
- N-key rollover via diode matrix (any number of simultaneous keys)
- Keyboard scanning: CPU activates matrix rows sequentially, reads column data

### Glide (Portamento):
- **Only works in monophonic/unison mode** (original hardware)
- Implementation: OTA with current mirror (Q309) and capacitive integration
- Exponential glide characteristic (constant time regardless of interval)
- Rev 4 added polyphonic glide

---

## 10. Analog Nonlinearities and "Happy Accidents"

These are the characteristics that define the Prophet-5 sound and are critical for authentic software emulation:

### Voice-to-Voice Variation:
- **Each voice has slightly different characteristics** due to component tolerances
- VCO pitch drift varies per voice (more pronounced in Rev 1/2)
- Filter cutoff frequency varies slightly between voices
- Envelope timing varies between voices
- VCA response varies between voices
- **This per-voice variation is the primary source of the "big," "wide," "alive" sound**

### Oscillator Drift and Instability:
- Analog VCOs drift over time due to temperature changes
- Rev 1/2 (SSM2030): More drift, required frequent retuning
- Rev 3 (CEM3340): Much more stable, but still drifts
- Autotune compensates but is not perfect -- residual errors add character
- **The autotune only tests C3-C9** and extrapolates, so lowest octaves have more error

### DAC Quantization:
- Standard 7-bit CV: 128 steps = some staircase stepping on swept parameters
- Oscillator tuning uses 14-bit: 16,384 steps = smooth enough for pitch
- **Parameter CVs (filter, envelope times, etc.) are quantized to 128 steps** -- this is audible on slow filter sweeps as subtle stepping

### Filter Nonlinearities:
- **SSM2040**: tanh() soft clipping in each OTA stage, asymmetric distortion when overdriven
- **CEM3320**: Passband droop at resonance (self-oscillation weaker at low frequencies)
- Both filters add subtle harmonic content even at moderate resonance settings

### VCA Nonlinearities:
- CA3280 with 2N4250 PNP arrangement introduces subtle CV curve bending
- Not a perfectly linear or exponential response -- somewhere between

### Sample/Hold Droop:
- S/H circuits droop over time (capacitor leakage)
- This creates very subtle pitch and parameter drift during sustained notes
- More pronounced in older units with degraded capacitors

### Waveform Imperfections:
- Sawtooth has finite reset time (not infinitely fast flyback)
- At high frequencies, the sawtooth becomes less ideal
- Triangle has slightly rounded peaks
- Pulse width is not perfectly stable

### The "Vintage Knob" Concept (Rev 4):
Dave Smith identified these key parameters for emulating vintage character:
- **Oscillator pitch instability** (random drift per voice)
- **Filter cutoff variation** (per voice)
- **Envelope timing variation** (per voice)
- Ranges from perfectly stable (Rev 4 setting) to extremely temperamental (Rev 1 setting)

---

## 11. Rev 1/2 vs Rev 3/3.3 Key Differences

### Chip Comparison:

| Function | Rev 1/2 | Rev 3/3.3 |
|----------|---------|-----------|
| VCO | SSM2030 | CEM3340 |
| VCF | SSM2040 | CEM3320 |
| EG | SSM2050 | CEM3310 |
| VCA | SSM2020 (21x) | CA3280 (14x) |
| CPU | Z80 | D780C (NEC) / Z80 |
| Pots | 100KB (linear) | 10KB |
| Patch memory | 40 (Rev1) / 40 (Rev2, cassette backup) | 120 |

### Sound Character:

**Rev 1/2 (SSM):**
- Warmer, softer, more "3D" and "organic"
- More voice-to-voice variation (less matched components)
- SSM2040 filter: creamier resonance, musical overdrive, fuller bass
- SSM2030 VCOs: more drift, wider detuning between voices
- SSM2050 envelopes: half the output amplitude of CEM3310
- Overall: "silky," "smooth," reminiscent of 1970s analog warmth
- Rev 1 was most unstable (only 182 units, hand-assembled)
- Rev 2 is generally considered the "best sounding" version

**Rev 3/3.3 (CEM):**
- Cleaner, brighter, more "in your face," slightly more aggressive
- More consistent voice-to-voice matching
- CEM3320 filter: tighter resonance, less coloration, lower noise
- CEM3340 VCOs: more stable, better tracking
- CEM3310 envelopes: double the output level of SSM2050
- Overall: more "flat" sounding but more precise and reliable
- Nearly 6,000 units produced (most common version)
- Some describe it as "slightly cold and featureless by comparison"
- Rev 3.3 added MIDI

### Reliability:
- Rev 1: Very unreliable (most upgraded to Rev 2 in the field)
- Rev 2: Moderate reliability, SSM chips now extremely scarce
- Rev 3: Most reliable, CEM chips more available (but getting scarce)

---

## 12. Unison/Detune Mode

### Unison Mode:
When activated, all 5 voices play the **same note** (monophonic but 5 oscillators thick).

**Voice Allocation in Unison:**
- A single UNISON CV derived from the **lowest note played** becomes the main pitch control for all voices
- All five GATE signals occur simultaneously
- The keyboard becomes monophonic

### Unison Detune:

**Detune Spread Algorithm:**
The Oscillator B Fine control adds variable detuning across voices:

| Voice | Detune Amount |
|-------|--------------|
| Voice 1 | No detune (reference pitch) |
| Voice 2 | +1x detune value (both Osc A and B) |
| Voice 3 | -1x detune value (both Osc A and B) |
| Voice 4 | +2x detune value (both Osc A and B) |
| Voice 5 | -2x detune value (both Osc A and B) |

- Detune is **symmetric** around voice 1, so perceived pitch center doesn't shift
- The detune amount is continuously variable via the Osc B Fine knob
- Both oscillators within each voice are detuned by the same amount
- If a program already has non-zero Osc B Fine, unison detune will sound different (detune is additive)

### Unison States (Rev 3.3 firmware):
The Unison switch cycles through: **Off -> Normal Unison -> Uni1 -> Off**

### Monophonic Behavior:
- In unison mode, **legato playing** does not retrigger envelopes (single trigger)
- Staccato playing retrigs envelopes
- Glide (portamento) is available in unison mode

---

## Appendix A: Complete Per-Voice Signal Path (Component Level, Rev 3)

```
KEYBOARD CV (14-bit DAC -> S/H)
  + OSC A FREQ knob (7-bit DAC -> S/H)
  + FINE TUNE bias
  + POLY MOD (Filter EG * amount + Osc B tri * amount) [if Freq A switch on]
  + WHEEL MOD (LFO/Noise * mod wheel) [if Freq A switch on]
  + PITCH WHEEL CV
  + GLIDE CV
  + LFO CV (triangle, level-shifted bipolar)
  |
  v
OSC A [CEM3340]
  -> Sawtooth output (0-10V)
  -> Pulse output (0-13.7V, PWM via PW knob + Poly Mod + Wheel Mod)
  -> [CD4016 analog switch selects active waveforms]
  -> OSC A LEVEL knob
  |
  v
           MIXER SUMMING NODE
  ^                              ^
  |                              |
OSC B [CEM3340]              WHITE NOISE
  -> Saw / Tri / Pulse           -> NOISE LEVEL knob
  -> OSC B LEVEL knob
  |
  v
VCF [CEM3320]
  Cutoff CV = CUTOFF FREQ knob
            + FILTER EG * EG AMOUNT knob
            + KEYBOARD TRACKING (0%, 50%, 100%)
            + POLY MOD [if Filter switch on]
            + WHEEL MOD [if Filter switch on]
  Resonance CV = RESONANCE knob
  |
  v
FILTER EG [CEM3310]  (A/D/S/R knobs, 7-bit DAC)
  |
  v (controls VCF cutoff AND available as Poly Mod source)

VCA [CA3280 OTA]
  Control CV = AMP EG output
  |
  v
AMP EG [CEM3310]  (A/D/S/R knobs, 7-bit DAC)
  |
  v
VOICE OUTPUT -> 30K summing resistor -> MASTER SUMMER
```

---

## Appendix B: Key Voltage Ranges for DSP Implementation

| Signal | Range | Notes |
|--------|-------|-------|
| Keyboard CV | 0 to 5V | 5 octaves, 1V/oct |
| Oscillator freq CV total | ~0 to 9V | ~9 octave range |
| DAC standard output | 0 to ~10.67V | 128 steps of 83.3mV |
| DAC fine (14-bit) | 0 to ~10.67V | 16,384 steps of 651uV |
| Filter cutoff CV | -6V to +6V max at chip | 18mV/oct at CEM3320 |
| Filter resonance | 0 to 100uA (current) | Via external resistor network |
| Envelope output | 0V to Vp | RC exponential curves |
| LFO output | Bipolar (triangle), unipolar (saw/square) | Sub-audio rates |
| VCA control | Via 2N4250 V-to-I converter | Exponential-ish response |

---

## Appendix C: Implementation Notes for Software Recreation

### Critical Modeling Priorities (in order of sonic importance):

1. **Oscillator waveforms with bandlimited aliasing control** -- use PolyBLEP or similar for saw/pulse
2. **Filter nonlinearity** -- model the tanh() saturation in each OTA stage for SSM2040; model passband droop for CEM3320
3. **Per-voice variation** -- random offsets to pitch, filter cutoff, envelope timing, VCA gain per voice
4. **Oscillator drift** -- slow random modulation of each oscillator's pitch (different rate/amount per voice)
5. **Envelope curve shape** -- true RC exponential (not linear segments), with attack overshoot
6. **Poly Mod at audio rate** -- Osc B -> Osc A freq must work at full audio rate for FM effects
7. **Simultaneous waveform selection** -- saw + pulse combined on same oscillator
8. **Hard sync** -- proper triangle core reset behavior
9. **DAC quantization** -- 7-bit stepping on parameters (subtle but audible on slow sweeps)
10. **Sample/Hold droop** -- very subtle pitch drift on sustained notes

### Filter DSP Approaches:
- **SSM2040**: Model as 4 cascaded 1-pole lowpass filters, each with tanh() waveshaping on the input. Resonance as external feedback from output to input. The key is that the tanh() happens INSIDE each stage, not just at the input.
- **CEM3320**: Similar 4-pole cascade but with integrated resonance VCA. Less nonlinear -- can use a cleaner model with resonance compensation for passband droop at low frequencies.

### Envelope DSP:
- Use true exponential segments, NOT linear
- Attack charges toward ~1.5x peak, switches to decay at peak crossing
- Decay/Release discharge exponentially toward target (sustain level or 0)
- Retrigger starts from current value, not from zero
- Time range: ~0.5ms to ~10s per segment (50,000:1 ratio)

---

## Sources

- [Electric Druid - CEM3340 VCO Designs](https://electricdruid.net/cem3340-vco-voltage-controlled-oscillator-designs/)
- [Electric Druid - CEM3320 Filter Designs](https://electricdruid.net/cem3320-filter-designs/)
- [Prophet-5 Service Manual (Archive.org)](https://archive.org/stream/synthmanual-prophet-5-service-manual/prophet-5servicemanual_djvu.txt)
- [Sequential Circuits Prophet-5 - Synth DIY Wiki](https://sdiy.info/wiki/Sequential_Circuits_Prophet-5)
- [Prophet-5 Wikipedia](https://en.wikipedia.org/wiki/Prophet-5)
- [Equipboard - Sequential Prophet Guide](https://equipboard.com/posts/sequential-prophet-5-guide)
- [SCI Prophet 5 Technical Assistance](https://www.sounddoctorin.com/synthtec/sci/p5.htm)
- [CEM & SSM Chips in Synthesizers - Rosen Sound](https://rosensound.com/pages/cem-ssm-chips-in-synthesizers)
- [Prophet-5 Rev 4 Circuit Discussion - Mod Wiggler](https://modwiggler.com/forum/viewtopic.php?t=275117)
- [Sequential Prophet-5 Rev 4 Info - maffez](https://maffez.com/?page_id=3556)
- [Prophet-5 User's Guide 1.3 (Sequential)](https://sequential.com/wp-content/uploads/2021/02/Prophet-5-Users-Guide-1.3.pdf)
- [ManualsLib - Prophet-5 Key Priority/Voice Assignment](https://www.manualslib.com/manual/2004417/Sequential-Prophet-5.html?page=56)
- [Prophet 5 Rev 3.3 V10.6 Firmware Instructions (Tauntek)](http://www.tauntek.com/V10pt6instrs.pdf)
- [SSM2040 vs CEM3320 Discussion (synth-diy)](https://synth-diy.org/pipermail/synth-diy/2023-November/173036.html)
- [CEM3310 Datasheet](https://www.bustedgear.com/images/datasheets/CEM3310.pdf)
- [SSM2030 Datasheet (KA Electronics)](https://www.ka-electronics.com/images/SSM/SSM2030.pdf)
- [KRM Music - Prophet-5 DAC Issues](https://www.keithrobertmurray.com/articles/prophet-5-dac.html)
- [Prophet-5 Revisions (Analog Synth Museum)](http://analogsynthmuseum.free.fr/prophet5revision.htm)
- [SSM2040 Filter Analysis - Ryan Williams](http://www.sdiy.org/destrukto/notes/ssm2040_analysis.pdf)
- [Sequential Prophet-5 & Prophet-10 Review (Sound on Sound)](https://www.soundonsound.com/reviews/sequential-prophet-5-prophet-10)
- [Prophet-5 Owner's Manual (Synthfool)](https://synthfool.com/docs/SequentialCircuits/Prophet_Series/Sequential%20Circuits%20Prophet%205%20Owners%20Manual.pdf)
- [Arturia Prophet-V Manual](https://downloads.arturia.com/products/prophet-v/manual/ProphetV_Manual_2_6_0_EN.pdf)
- [CEM3320 Datasheet (Electric Druid)](https://electricdruid.net/wp-content/uploads/2017/06/CEM3320-VCF.pdf)
- [AS3310 ADSR (Electric Druid)](https://electricdruid.net/product/as3310-vcadsr/)
- [Prophet-5 Chips & Circuits (Analog Synth Museum)](http://analogsynthmuseum.free.fr/PROPHET5CHIPS)
- [Sequencer.de - Prophet 5](https://www.sequencer.de/syns/sequentialcircuits/Prophet5.html)
