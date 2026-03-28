# Iconic Synth Sound Recreation Research
## Prophet-5 Style Patch Breakdowns

This document compiles extensive research on recreating famous synthesizer sounds,
translated into Prophet-5 architecture parameters where possible. The Prophet-5 has:
- OSC A: Saw, Pulse (with PW control)
- OSC B: Saw, Triangle, Pulse (with PW control)
- Mixer: Osc A level, Osc B level, Noise level
- Filter: 4-pole 24dB/oct lowpass, cutoff (Hz), resonance (0-1), env amount (Hz)
- Filter ADSR envelope (seconds)
- Amp ADSR envelope (seconds)
- Poly Mod: Filter Env -> Osc A freq / PW A / Filter; Osc B -> Osc A freq / PW A / Filter
- LFO: Triangle/Saw/Square waveforms, frequency (Hz)
- Wheel Mod: LFO/Noise mix -> Osc A freq / Osc B freq / PW A / PW B / Filter
- Sync (Osc A synced to Osc B)
- Glide, Unison, Drift

---

## 1980s ICONIC SYNTH SOUNDS

---

### 1. Depeche Mode -- "Enjoy the Silence" (1990) -- Main Synth Pad

**Original gear:** E-mu Emulator II sampler (choir/string floppy disk samples),
Minimoog (bass), ARP 2600 (bass). The main pad is a *sampled* choir/string sound,
not a synthesized patch.

**Prophet-5 Recreation Strategy:** Since the original is a sample-based choir pad,
we approximate with a warm, breathy, slow-evolving pad using PWM for movement:

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Pulse, PW 0.35 | Hollow, vocal-like quality |
| Osc B | Saw | Adds string-like harmonics |
| Osc A Level | 0.7 | |
| Osc B Level | 0.5 | Blended underneath |
| Noise Level | 0.03 | Slight breath/air |
| Filter Cutoff | 1800 Hz | Warm, not too bright |
| Filter Resonance | 0.08 | Gentle emphasis |
| Filter Env Amount | 1200 Hz | Subtle opening |
| Filter A/D/S/R | 0.6 / 1.8 / 0.65 / 2.0 s | Slow attack, sustained |
| Amp A/D/S/R | 0.7 / 0.5 / 1.0 / 2.5 s | Slow swell, long release |
| LFO Freq | 1.0 Hz | Slow modulation |
| LFO Shape | Triangle | |
| Wheel Mod | PW A | PWM for movement |
| Drift | 2.5 | Adds organic warmth |

**Key character:** Slow attack, sustained, warm and breathy. The original choir
texture is impossible to fully replicate with subtractive synthesis, but a PWM pad
with slow filter modulation gets in the right emotional territory.

Sources:
- [Reverb - Synth Sounds of Enjoy the Silence](https://reverb.com/news/the-synth-sounds-of-depeche-modes-enjoy-the-silence)
- [Gearspace - Creating Enjoy the Silence synths](https://gearspace.com/board/electronic-music-instruments-and-electronic-music-production/1023992-creating-enjoy-silence-synths.html)
- [KVR - Enjoy the Silence synth](https://www.kvraudio.com/forum/viewtopic.php?t=213839)

---

### 2. Depeche Mode -- "Personal Jesus" (1989) -- Synth Hook

**Original gear:** E-mu Emulator series, ARP 2600, various samplers.
The synth bass/hook uses filter self-oscillation with saw + square oscillators.

**Prophet-5 Recreation:**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | Core tone |
| Osc B | Pulse, PW 0.5 (square) | Added body |
| Osc A Level | 0.8 | |
| Osc B Level | 0.6 | |
| Noise Level | 0.0 | |
| Filter Cutoff | 350 Hz | Dark starting point |
| Filter Resonance | 0.65 | Near self-oscillation for the "sweep" |
| Filter Env Amount | 5000 Hz | Big sweep for the hook |
| Filter A/D/S/R | 0.001 / 0.25 / 0.1 / 0.05 s | Fast attack, quick decay |
| Amp A/D/S/R | 0.001 / 0.3 / 0.7 / 0.08 s | Percussive |
| Drift | 1.0 | |

**Key character:** Percussive, aggressive filter sweep with near-self-oscillation
resonance. Short decay creates the punchy, biting character.

Sources:
- [GearNews - How to sound like Depeche Mode](https://www.gearnews.com/how-to-sound-like-depeche-mode-1987-1990/)
- [MusicRadar - Depeche Mode synths](https://www.musicradar.com/how-to/depeche-mode-synths)

---

### 3. New Order -- "Blue Monday" (1983) -- Bass Synth

**Original gear:** Moog Source. Two slightly detuned oscillators with fast-decaying
LP filter envelope.

**Prophet-5 Recreation (from Syntorial preset recipe):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | Core saw bass |
| Osc B | Pulse, PW 1.0 (square) | Added punch |
| Osc A Level | 0.7 | |
| Osc B Level | 0.3 | |
| Noise Level | 0.0 | |
| Filter Cutoff | ~55% range (~2200 Hz) | Mid-range starting point |
| Filter Resonance | 0.0 | No resonance peak |
| Filter Env Amount | 25% range (~1250 Hz) | Moderate sweep |
| Filter A/D/S/R | 0.0 / 0.3 / 0.0 / 10.0 s | Fast attack, 300ms decay, no sustain |
| Amp A/D/S/R | 0.0 / 0.25 / 0.2 / 0.16 s | Quick, punchy |
| Unison | true | For thickness (4 voices, ~11 cent detune) |
| Drift | 1.0 | |

**Key character:** Punchy, plucked bass with fast filter decay. The detuned unison
gives it thickness. Original used a 24dB ladder filter (matches Prophet-5).

Sources:
- [Syntorial - Blue Monday 88 Bass Preset](https://www.syntorial.com/preset-recipe/new-order-blue-monday-88-bass/)
- [MusicRadar - Blue Monday Best Synth Sound](https://www.musicradar.com/news/new-order-blue-monday-best-synth-sound)
- [Gearspace - Blue Monday Moog Source](https://gearspace.com/board/electronic-music-instruments-and-electronic-music-production/1051686-quot-blue-monday-quot-moog-source-synth-bass.html)
- [Vintage Synth Forums - Blue Monday Prophet 5](https://forum.vintagesynth.com/viewtopic.php?f=1&t=58892)

---

### 3b. New Order -- "Blue Monday" -- Lead Synth

**Original gear:** Oberheim OB-Xa for leads, Emulator for choir samples.

**Prophet-5 Recreation:**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Pulse, PW 0.45 | Bright, hollow lead |
| Osc B | Saw | Adds harmonics |
| Osc A Level | 0.8 | |
| Osc B Level | 0.5 | |
| Filter Cutoff | 3500 Hz | Open, bright |
| Filter Resonance | 0.15 | Slight emphasis |
| Filter Env Amount | 2000 Hz | |
| Filter A/D/S/R | 0.01 / 0.5 / 0.4 / 0.3 s | |
| Amp A/D/S/R | 0.01 / 0.3 / 0.85 / 0.3 s | |
| Drift | 1.5 | |

---

### 4. Tears for Fears -- "Everybody Wants to Rule the World" (1985) -- Shimmer Pad

**Original gear:** Sequential Prophet T-8 (upgraded Prophet-5), PPG Wave, DX7.
The two-chord motif was the Prophet T-8. This is one of the closest to actual
Prophet-5 architecture since the T-8 IS a Prophet-5 variant.

**Prophet-5 Recreation (from Reverb Machine detailed breakdown):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Pulse, PW 0.31 | Narrow pulse, bright/thin character |
| Osc B | Pulse, PW 0.83 | Nearly-saw width, wider harmonic content |
| Osc A Level | 0.7 | |
| Osc B Level | 0.7 | |
| Noise Level | 0.0 | |
| Filter Cutoff | 482 Hz | Research-verified exact value |
| Filter Resonance | ~0.1 | Very low |
| Filter Env Amount | 3750 Hz | Significant sweep ("3.75" in source) |
| Filter A/D/S/R | 0.01 / 3.27 / 0.15 / 0.3 s | Long 3.27s decay, low sustain |
| Amp A/D/S/R | 0.01 / 0.5 / 0.9 / 0.4 s | |
| LFO Freq | 6.82 Hz | Fast LFO for PWM shimmer |
| LFO Shape | Triangle | |
| Wheel Mod | PW A (mod depth 0.172) | PWM creates shimmer |
| Drift | 1.5 | |

**Key character:** The asymmetric pulse widths (31% vs 83%) create a complex,
shimmering harmonic blend. The long 3.27s filter decay gives it that slow,
blooming quality. PWM via LFO at 6.82 Hz adds the characteristic shimmer.

Sources:
- [Reverb Machine - Tears for Fears EWTRTW Synths](https://reverbmachine.com/blog/tears-for-fears-everybody-wants-to-rule-the-world-synths/)
- [Reverb - Synth Sounds of EWTRTW](https://reverb.com/news/the-synth-sounds-of-tears-for-fears-everybody-wants-to-rule-the-world)
- [MusicTech - Best TFF Remake Tutorial](https://musictech.com/news/music/how-to-recreate-tears-for-fears-everybody-wants-to-rule-the-world-synths/)

---

### 5. Tears for Fears -- "Shout" (1984) -- Big Synth Stabs

**Original gear:** Fairlight CMI (the "SARARR/ARR1" sample), Prophet-5/T-8 (bass
in unison mode). The iconic stab is actually a Fairlight sample, not a synth patch.

**Prophet-5 Recreation -- Lead/Stab (from Syntorial recipe):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Pulse, PW 0.8 | Near-saw for airy character |
| Osc B | Pulse, PW 0.8 | Matched, +1 cent detune |
| Osc A Level | 0.5 | |
| Osc B Level | 0.5 | |
| Noise Level | 0.3 (as ratio) | Adds breathy air quality |
| Filter Cutoff | 80% (~8000 Hz) | Open, bright |
| Filter Resonance | 0.4 | Moderate resonance peak |
| Filter Env Amount | 0 Hz | No filter sweep |
| Amp A/D/S/R | 0.01 / 0.3 / 1.0 / 0.13 s | Quick stab |
| Drift | 1.5 | |

**Prophet-5 Recreation -- Bass:**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | Fat unison bass |
| Osc B | Saw | |
| Osc A Level | 0.8 | |
| Osc B Level | 0.8 | |
| Filter Cutoff | 400 Hz | |
| Filter Resonance | 0.15 | |
| Filter Env Amount | 3000 Hz | |
| Filter A/D/S/R | 0.001 / 0.3 / 0.4 / 0.1 s | |
| Amp A/D/S/R | 0.001 / 0.3 / 0.8 / 0.15 s | |
| Unison | true | Prophet-5 unison mode as used on original |

Sources:
- [Reverb Machine - TFF Shout Synths](https://reverbmachine.com/blog/tears-for-fears-shout-synths/)
- [Syntorial - Shout Lead Preset](https://www.syntorial.com/preset-recipe/tears-for-fears-shout-lead/)
- [Devin Gademan - Shout Synth Hook](https://www.devingademan.com/writings/tears-for-fears-shout-synth-hook-sararr-arr1-using-the-roland-u-220-syn-vox-1-preset/)

---

### 6. A-ha -- "Take On Me" (1985) -- Synth Riff

**Original gear:** Yamaha DX7 (main melody) layered with Roland Juno-60 (body)
and PPG Wave 2.2 (bell shimmer layer).

**Prophet-5 Recreation:**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Pulse, PW 0.4 | Bright, punchy |
| Osc B | Saw | Adds warmth and body |
| Osc A Level | 0.8 | Slightly louder (higher octave character) |
| Osc B Level | 0.5 | |
| Filter Cutoff | 5000 Hz | Bright, needs to cut through |
| Filter Resonance | 0.15 | Slight bite |
| Filter Env Amount | 2000 Hz | |
| Filter A/D/S/R | 0.001 / 0.3 / 0.2 / 0.1 s | Fast pluck |
| Amp A/D/S/R | 0.001 / 0.25 / 0.0 / 0.1 s | Fast attack, zero sustain, staccato |
| Drift | 1.0 | |

**Key character:** Bouncy, staccato. Zero sustain on amp envelope is critical for
the percussive, bouncing feel. The original DX7 FM character cannot be fully
replicated but pulse+saw gets the energy right.

Sources:
- [Syntorial - Take On Me Lead](https://www.syntorial.com/preset-recipe/a-ha-take-on-me-lead/)
- [MusicRadar - Greatest Synth Sounds: Take On Me](https://www.musicradar.com/how-to/the-40-greatest-synth-sounds-of-all-time-no-8-a-ha-take-on-me)
- [Splice - Recreating Take On Me with Pigments](https://splice.com/blog/recreating-a-ha-take-on-me-pigments/)
- [Muzines - Sound Diving: Take On Me](https://www.muzines.co.uk/blog/sound-diving-2-take-on-me/40)

---

### 7. Eurythmics -- "Sweet Dreams" (1983) -- Main Sequence

**Original gear:** Roland SH-101 (bass sequence), Oberheim OB-X (pad). The sequence
is a simple bass line with PWM and chorus.

**Prophet-5 Recreation (from Syntorial recipe):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Pulse, PW 0.5 (square) | Core square wave tone |
| Osc B | (off) | Single oscillator patch |
| Osc A Level | 1.0 | |
| Osc B Level | 0.0 | |
| Filter Cutoff | ~8000 Hz (80%) | Mostly open |
| Filter Resonance | 0.0 | |
| Filter Env Amount | 0 Hz | No filter envelope |
| Amp A/D/S/R | 0.0 / 0.45 / 0.0 / 0.4 s | Plucked, no sustain |
| LFO Freq | ~2.5 Hz (1/8 note at 126 BPM) | Tempo-synced PWM |
| LFO Shape | Triangle | |
| Wheel Mod | PW A, amount 20% | PWM for smeared tone |
| Drift | 1.0 | |

**Alternative richer approach (forum consensus):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Pulse, PW 0.45 | |
| Osc B | Saw | Detuned ~8% from Osc A |
| Osc A Level | 0.6 | |
| Osc B Level | 0.6 | Equal blend |
| Filter Cutoff | Open | |
| Amp A/D/S/R | 0.03 / 0.45 / 0.0 / 0.4 s | Slight attack softening |
| LFO | Triangle, slow, to PW A | |

**Key character:** PWM creates the "smeared" tone. The sound is panned L/R with
slight variations. Chorus effect is essential (not available on Prophet-5 hardware
but implied by the detuned two-osc approach).

Sources:
- [Syntorial - Sweet Dreams Bass](https://www.syntorial.com/preset-recipe/eurythmics-sweet-dreams-are-made-of-this-bass/)
- [MusicRadar - Get the Sound of Sweet Dreams](https://www.musicradar.com/tuition/tech/get-the-sound-of-eurythmics-sweet-dreams-565782)
- [MusicRadar - Greatest Synth Sounds: Sweet Dreams](https://www.musicradar.com/news/the-40-greatest-synth-sounds-of-all-time-no-7-eurythmics-sweet-dreams-are-made-of-this)

---

### 8. Soft Cell -- "Tainted Love" (1981) -- Synth

**Original gear:** Synclavier II (main keyboard/piano sounds), Korg Synthe-bass,
Serge Modular, Delta Labs DL4 delay. The iconic "bink bink" is a Synare drum
snare through fast delay with feedback.

**Prophet-5 Recreation (main synth line):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Pulse, PW 0.5 (square) | Clean, electronic character |
| Osc B | Pulse, PW 0.5 (square) | Slightly detuned for width |
| Osc A Level | 0.7 | |
| Osc B Level | 0.7 | |
| Filter Cutoff | 4000 Hz | Bright, open |
| Filter Resonance | 0.1 | |
| Filter Env Amount | 1000 Hz | Subtle |
| Filter A/D/S/R | 0.001 / 0.4 / 0.5 / 0.2 s | |
| Amp A/D/S/R | 0.001 / 0.3 / 0.8 / 0.15 s | |
| Drift | 1.0 | |

**Key character:** Clean, precise, slightly cold digital character. The Synclavier
was a digital instrument so the Prophet-5 recreation should stay clean with low
drift. Square waves give the electronic purity.

Sources:
- [MusicRadar - Greatest Synth Sounds: Tainted Love](https://www.musicradar.com/news/the-40-greatest-synth-sounds-of-all-time-no-26-soft-cell-tainted-love)
- [Sound On Sound - Classic Tracks: Tainted Love](https://www.soundonsound.com/techniques/classic-tracks-soft-cell-tainted-love)
- [Mix Online - Classic Tracks: Tainted Love](https://www.mixonline.com/recording/classic-tracks/classic-tracks-tainted-love)

---

### 9. OMD -- "Enola Gay" (1980) -- Bright Lead

**Original gear:** Korg Micro-Preset. Sharp, piercing quality from high resonance
and wide-open filter.

**Prophet-5 Recreation:**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Pulse, PW 0.5 (square) | Clean, piercing |
| Osc B | (off or very low) | Minimal, thin sound |
| Osc A Level | 1.0 | |
| Osc B Level | 0.1 | Sub-oscillator depth |
| Filter Cutoff | 6000 Hz | Bright, open |
| Filter Resonance | 0.45 | High -- key to the piercing quality |
| Filter Env Amount | 2000 Hz | |
| Filter A/D/S/R | 0.001 / 0.4 / 0.2 / 0.15 s | Body after attack |
| Amp A/D/S/R | 0.01 / 0.4 / 0.2 / 0.15 s | 10% attack removes sharpness |
| Drift | 0.5 | Clean, precise |

**Key character:** Bright, piercing, high resonance. The Korg Micro-Preset was a
very simple instrument. Keep the patch minimal. High resonance emphasizes upper
harmonics for that cutting lead quality.

Sources:
- [MusicRadar - Greatest Synth Sounds: Enola Gay](https://www.musicradar.com/news/the-40-greatest-synth-sounds-of-all-time-no-39-omd-enola-gay)
- [Syntorial - OMD Enola Gay Lead](https://www.syntorial.com/preset-recipe/omd-enola-gay-lead/)
- [GearNews - How to Sound Like OMD](https://www.gearnews.com/orchestral-manoeuvres-in-the-dark-how-to-sound-like-omd/)

---

### 10. Gary Numan -- "Cars" (1979) -- Pulsing Synth and Lead

**Original gear:** Moog Polymoog 280a using "Vox Humana" preset, MXR Phase 90
phaser, plate reverb.

**Prophet-5 Recreation -- Lead (from Syntorial recipe):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | Core sawtooth tone |
| Osc B | (off) | Single oscillator |
| Osc A Level | 1.0 | Full volume |
| Osc B Level | 0.0 | |
| Filter Cutoff | 40% (~1600 Hz) | Band-pass character |
| Filter Resonance | 0.3 | Moderate |
| Filter Key Track | 100% | Filter follows keyboard |
| Amp A/D/S/R | 0.14 / 0.3 / 1.0 / 0.65 s | 140ms attack, full sustain |
| LFO Freq | tempo-synced 1/8 note | Pitch modulation |
| LFO Shape | Triangle | |
| Wheel Mod | Freq A, depth 1 semitone | Vibrato |
| Drift | 1.5 | |

**Prophet-5 Recreation -- Percussive "Cars" Hit (from Syntorial):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | |
| Noise Level | high (~0.8) | White noise burst |
| Filter Cutoff | 70% (~5600 Hz) | |
| Filter Resonance | 0.5 | |
| Amp A/D/S/R | 0.0 / 0.3 / 0.0 / 0.95 s | Fast transient, long release |

**Key character:** The Polymoog Vox Humana is a complex multi-oscillator preset
with formant-like filtering. The Prophet-5 gets close with moderate filter cutoff,
resonance, and key tracking. The MXR phaser is critical to the sound -- external
processing needed. The 140ms attack softens the transient slightly.

Sources:
- [Syntorial - Cars Lead](https://www.syntorial.com/preset-recipe/gary-numan-cars-lead/)
- [Syntorial - Cars Perc](https://www.syntorial.com/preset-recipe/gary-numan-cars-perc/)
- [MusicRadar - How to Emulate Cars](https://www.musicradar.com/artists/i-do-not-think-i-have-a-talent-for-pop-music-im-much-better-if-i-slow-it-down-a-bit-make-it-a-bit-menacing-add-a-hint-of-mystery-or-oppression-im-in-my-element-at-that-point-how-to-emulate-the-sound-of-gary-numans-synth-pop-classic-cars)
- [Almalibre Studios - Redesigning Cars Lead](https://almalibrestudios.com/redesigning-the-lead-from-gary-numans-cars/)

---

### 11. Peter Gabriel -- "Sledgehammer" (1986) -- Synth Layers

**Original gear:** Prophet-5 (confirmed), NED Synclavier, E-mu Emulator II
(Shakuhachi flute sample).

**Prophet-5 Recreation -- Vibrato Organ Synth:**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | Rich harmonic content |
| Osc B | Pulse, PW 0.5 | Square for organ quality |
| Osc A Level | 0.6 | |
| Osc B Level | 0.5 | |
| Filter Cutoff | 5000 Hz | Open, bright organ |
| Filter Resonance | 0.0 | No resonance |
| Filter Env Amount | 0 Hz | No filter sweep |
| Amp A/D/S/R | 0.001 / 0.05 / 1.0 / 0.03 s | Instant on/off, organ-like |
| LFO Freq | 6.0 Hz | Fast vibrato |
| LFO Shape | Triangle | |
| Wheel Mod | Freq A + Freq B | Pitch vibrato on both oscs |
| Drift | 0.5 | |

**Key character:** Farfisa-like organ with heavy vibrato. The Prophet-5 was actually
used on the original recording. Fast LFO vibrato is the defining characteristic.

Sources:
- [PresetPatch - Sledgehammer](https://www.presetpatch.com/synth/arturia-polybrute/resources/475-peter-gabriel---sledgehammer)
- [Arturia Forum - Sledgehammer Patch](https://forum.arturia.com/t/sledgehammer-patch/5403)

---

### 12. Kate Bush -- "Running Up That Hill" (1985) -- Driving Synth

**Original gear:** Fairlight CMI (sampled cello "Cello 2" patch), LinnDrum,
Quantec reverb.

The main riff and chords both use the Fairlight's stock Cello 2 sample with
portamento/pitch bend programmed in. Kate Bush resampled the cello note with
pitchbend back into the Fairlight.

**Prophet-5 Recreation (approximating the cello/string character):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | String-like harmonics |
| Osc B | Saw | Slightly detuned for thickness |
| Osc A Level | 0.8 | |
| Osc B Level | 0.6 | |
| Filter Cutoff | 2500 Hz | Mid-range, cello-like |
| Filter Resonance | 0.1 | |
| Filter Env Amount | 1500 Hz | |
| Filter A/D/S/R | 0.05 / 0.4 / 0.5 / 0.3 s | Quick but not instant attack |
| Amp A/D/S/R | 0.03 / 0.3 / 0.8 / 0.3 s | |
| Glide | On, rate 0.08 | Critical: portamento for pitch scoop |
| LFO Freq | 1.5 Hz | Subtle movement |
| LFO Shape | Triangle | |
| Wheel Mod | PW A | |
| Drift | 1.5 | |

**Key character:** The pitch scoop/portamento is essential to the character. The
Fairlight's sampled cello has a quality that subtractive synthesis cannot fully
replicate, but dual detuned saws with glide captures the driving, insistent energy.
External reverb is critical (large hall/plate).

Sources:
- [Reverb Machine - Kate Bush RUTH Synths](https://reverbmachine.com/blog/kate-bush-running-up-that-hill-synth-sounds/)
- [MusicRadar - Kate Bush Synth and Drum Machine](https://www.musicradar.com/news/kate-bush-running-up-that-hill-synth-drum-machine)
- [GearNews - How to Sound Like Kate Bush](https://www.gearnews.com/how-to-sound-like-kate-bush-running-up-that-hill-a-deal-with-god/)

---

### 13. Yazoo -- "Don't Go" (1982) -- Vince Clarke Lead

**Original gear:** ARP 2600 (lead), Sequential Circuits Pro-One (most other parts).
Despite common belief, "Don't Go" used the ARP 2600, not a Prophet-5. "Only You"
was done entirely on the Pro-One.

**Prophet-5 Recreation (from Syntorial recipe):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | Low volume, sub-octave character |
| Osc B | Saw | +2 octaves, dominant |
| Osc A Level | 0.2 | Subtle low end |
| Osc B Level | 0.8 | Bright, prominent |
| Filter Cutoff | 60% (~3200 Hz) | Mid-range |
| Filter Resonance | 0.0 | |
| Filter Env Amount | 55% (~2750 Hz) | Big filter sweep |
| Filter A/D/S/R | 0.02 / 0.15 / 0.0 / 0.1 s | Fast attack, 150ms decay |
| Amp A/D/S/R | 0.01 / 0.3 / 1.0 / 0.015 s | Quick on, sustained, snap off |
| Drift | 1.0 | |

**Key character:** The "brassy, squelchy" quality comes from fast filter envelope
attack and short decay modulating the cutoff. The high-octave oscillator dominates
giving it a bright, cutting character.

Sources:
- [Syntorial - Yazoo Don't Go Lead](https://www.syntorial.com/preset-recipe/yazoo-dont-go-lead/)
- [Synthtopia - Yazoo Don't Go Pro-One](https://www.synthtopia.com/content/2014/07/27/yazoo-dont-go-re-created-w-sequential-circuits-pro-one-synth/)
- [Muzines - How They Do Only You](https://www.muzines.co.uk/articles/how-they-do-only-you/4589)

---

### 13b. Yazoo -- "Only You" (1982) -- Vince Clarke Patches

**Original gear:** Sequential Circuits Pro-One exclusively. Original patch settings
were published in "One Two Testing" magazine, Issue 1, 1982.

**Prophet-5 Recreation (string pad):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Pulse, PW 0.4 | String-like |
| Osc B | Saw | Adds richness |
| Osc A Level | 0.65 | |
| Osc B Level | 0.65 | |
| Filter Cutoff | 2500 Hz | Warm |
| Filter Resonance | 0.05 | |
| Filter Env Amount | 800 Hz | Subtle |
| Filter A/D/S/R | 0.3 / 0.8 / 0.65 / 0.8 s | Slow, evolving |
| Amp A/D/S/R | 0.4 / 0.5 / 0.9 / 0.7 s | Slow attack, sustained |
| LFO Freq | 1.5 Hz | |
| Wheel Mod | PW A | Gentle PWM |
| Drift | 2.0 | |

---

### 14. Japan -- "Ghosts" (1981) -- Richard Barbieri Textures

**Original gear:** Prophet-5 (main drone + big string sound), Oberheim OBX
(stabby chord), Roland System 700 (S&H intro sound). This is one of the
most iconic actual Prophet-5 recordings.

**Prophet-5 Recreation -- Main Drone Pad:**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | Rich drone base |
| Osc B | Pulse, PW 0.35 | Adds vocal quality |
| Osc A Level | 0.7 | |
| Osc B Level | 0.6 | |
| Noise Level | 0.03 | Subtle air |
| Filter Cutoff | 1200 Hz | Dark, atmospheric |
| Filter Resonance | 0.15 | |
| Filter Env Amount | 800 Hz | Gentle movement |
| Filter A/D/S/R | 1.0 / 3.0 / 0.5 / 2.5 s | Very slow, evolving |
| Amp A/D/S/R | 1.5 / 1.0 / 1.0 / 3.0 s | Slow fade in, long release |
| LFO Freq | 0.3 Hz | Very slow modulation |
| LFO Shape | Triangle | |
| Wheel Mod | Filter + PW A | Slow filter + PWM movement |
| Poly Mod | Osc B -> Filter, amount 0.15 | Subtle FM-like movement |
| Drift | 3.5 | Heavy drift for organic quality |

**Prophet-5 Recreation -- String Pad:**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Pulse, PW 0.4 | |
| Osc B | Saw | |
| Osc A Level | 0.7 | |
| Osc B Level | 0.7 | |
| Filter Cutoff | 3000 Hz | Brighter than drone |
| Filter Resonance | 0.08 | |
| Filter Env Amount | 1500 Hz | |
| Filter A/D/S/R | 0.5 / 1.5 / 0.6 / 1.5 s | |
| Amp A/D/S/R | 0.5 / 0.5 / 1.0 / 2.0 s | |
| Wheel Mod | PW A + PW B | |
| Drift | 2.5 | |

**Key character:** Barbieri's approach was to try to create "world music instruments
and acoustic instruments programmed on the Prophet." Deep programming with evolving
filters, LFOs, and pitch modulation. Heavy drift is essential for the organic,
breathing quality.

Sources:
- [MusicRadar - 5 Prophet-5 Powered Tracks](https://www.musicradar.com/news/5-tracks-producers-need-to-hear-featuring-the-prophet-5-synth)
- [Sound On Sound - Classic Tracks: Japan Ghosts](https://www.soundonsound.com/techniques/classic-tracks-japan-ghosts)
- [KVR - Japan Ghosts Synths](https://www.kvraudio.com/forum/viewtopic.php?t=218483)
- [Life In Tokyo - Prophet 5 Family Tree](https://www.lifeintokyo.net/familytree_prophetfive.html)

---

## FILM / TV SCORES

---

### 15. Stranger Things Theme (2016) -- Arpeggiated Synth

**Original gear:** Oberheim SEM Two Voice (main arp), Prophet-5 (bass), Yamaha
CS-60. Kyle Dixon & Michael Stein of S U R V I V E.

**Prophet-5 Recreation -- Main Arp (from Syntorial recipe):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | Core tone |
| Osc B | (off) | |
| Osc A Level | 1.0 | |
| Noise Level | 0.2 | Adds grit |
| Filter Cutoff | 80% (~6400 Hz) | Mostly open |
| Filter Resonance | 0.0 | |
| Filter Env Amount | ~500 Hz (10%) | Subtle |
| Filter A/D/S/R | 0.0 / 0.05 / 0.0 / 0.1 s | Ultra-fast 50ms decay |
| Amp A/D/S/R | 0.0 / 0.3 / 0.0 / 0.02 s | Plucked, 300ms decay |
| Poly Mod | Osc B -> Freq A (FM), amount 20% | Metallic FM character |
| Drift | 1.5 | |

Note: The FM via Poly Mod is key to getting the slightly metallic, unsettling
quality. Osc B at +3 octaves (if pitch control available) modulates Osc A pitch.

**Prophet-5 Recreation -- Pluck Layer (from Syntorial recipe):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Pulse, PW 0.5, +9 cents | |
| Osc B | Pulse, PW 0.5, -10 cents | Detuned for width |
| Sub Osc | Pulse, PW 1.0, -1 octave, level 0.4 | Low end weight |
| Filter Cutoff | 50% (~4000 Hz) | |
| Amp A/D/S/R | 0.0 / 0.3 / 1.0 / 0.015 s | |
| LFO Freq | 1/16 note | |
| LFO Shape | Saw | Rising mod shape |
| LFO -> Filter Cutoff | 30% | Creates pulsing filter |
| Mod Wheel -> Cutoff | 50% | Performance control |

**Key character:** C major 7th arpeggio in E minor at 84 BPM. The filter envelope
modulation depth increasing over time is the most distinctive aspect. Key tracking
makes lower arp notes darker than higher ones.

Sources:
- [Syntorial - Stranger Things Arp](https://www.syntorial.com/preset-recipe/survive-stranger-things-theme-arp/)
- [Syntorial - Stranger Things Pluck](https://www.syntorial.com/preset-recipe/survive-stranger-things-theme-pluck/)
- [Soundfly - Rebuild the Spooky Arp](https://flypaper.soundfly.com/produce/rebuild-the-spooky-arpeggiated-synth-stranger-things/)
- [Splice - Recreating Stranger Things Theme](https://splice.com/blog/recreating-stranger-things-theme/)

---

### 16. Blade Runner (1982) -- Main Pad/Brass Theme

**Original gear:** Yamaha CS-80 (the definitive sound). Voted #1 greatest synth
sound of all time by MusicRadar. The CS-80 has poly aftertouch, which is
critical to Vangelis's expressive playing.

**Prophet-5 Recreation -- Brass Lead (from Syntorial recipe):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Pulse, PW 0.5 | |
| Osc B | Pulse, PW 0.5, +1 oct, -6 cents | Octave up, detuned |
| Osc A Level | 0.6 | |
| Osc B Level | 0.4 | |
| Filter Cutoff | 75% (~6000 Hz) | |
| Filter Resonance | 0.15 | |
| Filter Env Amount | ~750 Hz (15%) | Subtle |
| Filter A/D/S/R | 0.65 / 10.0 / 0.0 / 10.0 s | 650ms attack, 10s decay |
| Amp A/D/S/R | 0.25 / 1.9 / 0.4 / 0.75 s | |
| LFO Freq | ~2.5 Hz (1/8 note equiv) | |
| LFO Shape | Triangle | |
| Wheel Mod | PW A + PW B, amount 80% | Heavy PWM for movement |
| Drift | 3.0 | |

**Prophet-5 Recreation -- Dark Pad:**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | Two detuned saws |
| Osc B | Saw | Detuned opposite direction |
| Osc A Level | 0.7 | |
| Osc B Level | 0.7 | |
| Noise Level | 0.05 | Subtle breath |
| Filter Cutoff | 800 Hz | Dark |
| Filter Resonance | 0.18 | |
| Filter Env Amount | 2000 Hz | |
| Filter A/D/S/R | 1.0 / 3.0 / 0.5 / 2.5 s | Very slow, cinematic |
| Amp A/D/S/R | 0.8 / 1.0 / 1.0 / 3.0 s | |
| LFO Freq | 0.2 Hz | Ultra-slow |
| LFO Shape | Triangle | |
| Wheel Mod | Filter | Slow filter sweep |
| Drift | 3.0 | Heavy for organic movement |

**Key character:** The CS-80 has two layers (Voice I + Voice II), each with
independent filters and envelopes, plus ring modulation. The poly aftertouch
opening the filter while playing is a huge part of the sound. Use mod wheel
mapped to filter cutoff to simulate this. External plate reverb with big
size/decay is essential.

Sources:
- [Syntorial - Blade Runner Brass](https://www.syntorial.com/preset-recipe/vangelis-blade-runner-brass/)
- [Reverb Machine - Vangelis Blade Runner Synths](https://reverbmachine.com/blog/vangelis-blade-runner-synth-sounds/)
- [MusicRadar - #1 Greatest Synth Sound](https://www.musicradar.com/news/blade-runner-best-synth-sound)
- [Synthtopia - Free Blade Runner Blues CS-80V Preset](https://www.synthtopia.com/content/2017/10/19/free-arturia-cs-80v-preset-nails-iconic-blade-runner-blues-sound/)

---

### 17. John Carpenter -- "Halloween" (1978) -- Pulsing Lead

**Original gear:** Moog Modular IIIP (original film), Prophet-5 and Prophet-10
(Halloween II). The famous piano melody is in 5/4 time and is acoustic piano,
not synth. The synth drones and stingers underneath are the Moog modular.

**Prophet-5 Recreation -- Stabbing Lead (Halloween II style):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | |
| Osc B | Saw | Stacked, slight detune |
| Osc A Level | 0.8 | |
| Osc B Level | 0.8 | |
| Filter Cutoff | 3000 Hz | Mid-bright |
| Filter Resonance | 0.2 | |
| Filter Env Amount | 4000 Hz | Big sweep |
| Filter A/D/S/R | 0.001 / 0.5 / 0.3 / 0.2 s | |
| Amp A/D/S/R | 0.001 / 0.3 / 0.85 / 0.2 s | |
| Unison | true | All voices stacked for massive lead |
| Glide | On, rate 0.06 | Portamento for menace |
| Drift | 2.5 | |

**Prophet-5 Recreation -- Dark Drone/Stinger:**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | |
| Osc B | Triangle | Sub bass weight |
| Osc A Level | 0.8 | |
| Osc B Level | 0.3 | |
| Noise Level | 0.08 | |
| Filter Cutoff | 250 Hz | Very dark |
| Filter Resonance | 0.6 | Near-self-oscillation |
| Filter Env Amount | 1500 Hz | |
| Filter A/D/S/R | 2.0 / 3.0 / 0.3 / 2.0 s | Ultra-slow |
| Amp A/D/S/R | 1.5 / 1.0 / 0.8 / 2.5 s | |
| Poly Mod | Osc B -> Freq A + Filter | Unsettling movement |
| Poly Mod Osc B amount | 0.2 | |
| LFO Freq | 0.15 Hz | Glacially slow |
| Drift | 4.0 | Maximum instability |

**Key character:** The Prophet-5 was literally used on Halloween II. Carpenter's
approach involves very slow, menacing filter sweeps, high resonance, and unstable
oscillators. The 5/4 piano pattern is the iconic element; the synths provide
atmosphere and dread.

Sources:
- [Dubspot - Legendary Synth Sounds of John Carpenter](https://blog.dubspot.com/john-carpenter-synth-sounds)
- [Reverb - Synth Sounds of John Carpenter](https://reverb.com/news/the-synth-sounds-of-john-carpenter)
- [Far Out Magazine - Create Halloween Synth Sounds](https://faroutmagazine.co.uk/create-synth-sounds-john-carpenter-film-halloween/)
- [Sound On Sound - John Carpenter](https://www.soundonsound.com/people/john-carpenter)

---

### 18. John Carpenter -- "Escape from New York" (1981) -- Bass and Lead

**Original gear:** Prophet-10, Prophet-5, ARP Avatar (x2), ARP Quadra, ARP
Sequencer, Eventide H949 Harmonizer. Alan Howarth co-composed.

**Prophet-5 Recreation -- Bass:**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | |
| Osc B | Saw | |
| Osc A Level | 0.8 | |
| Osc B Level | 0.8 | |
| Filter Cutoff | 300 Hz | Dark, heavy |
| Filter Resonance | 0.25 | |
| Filter Env Amount | 3000 Hz | Punchy sweep |
| Filter A/D/S/R | 0.001 / 0.3 / 0.2 / 0.1 s | |
| Amp A/D/S/R | 0.001 / 0.4 / 0.7 / 0.1 s | |
| Unison | true | Massive bass |
| Drift | 2.0 | |

**Prophet-5 Recreation -- Lead:**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | |
| Osc B | Pulse, PW 0.45 | |
| Osc A Level | 0.8 | |
| Osc B Level | 0.6 | |
| Filter Cutoff | 2000 Hz | |
| Filter Resonance | 0.2 | |
| Filter Env Amount | 4000 Hz | |
| Filter A/D/S/R | 0.001 / 0.6 / 0.4 / 0.3 s | |
| Amp A/D/S/R | 0.001 / 0.4 / 0.85 / 0.3 s | |
| Glide | On, rate 0.08 | |
| Drift | 2.0 | |

**Key character:** The Prophet-5/10 were literally the instruments used. The
Carpenter/Howarth sound relies on sequenced patterns, portamento, and the raw
analog character of the Prophets. The Eventide harmonizer adds thickness.

Sources:
- [Synthtopia - Secrets of Escape from New York](https://www.synthtopia.com/content/2019/04/20/secrets-of-the-escape-from-new-york-soundtrack/)
- [Gearspace - Escape from New York Synths](https://gearspace.com/board/electronic-music-instruments-and-electronic-music-production/1222080-escape-new-york-synths-soundtrack.html)
- [Equipboard - John Carpenter](https://equipboard.com/pros/john-carpenter)

---

### 19. Terminator Theme (1984) -- Synth Pad

**Original gear:** Oberheim OB-Xa (haunting low D drone), SCI Prophet-10
(metallic heartbeat pulse, percussion). Brad Fiedel synced these by hand.

**Prophet-5 Recreation -- Haunting Pad (OB-Xa style):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | |
| Osc B | Pulse, PW 0.4 | Mixed texture |
| Osc A Level | 0.7 | |
| Osc B Level | 0.6 | |
| Noise Level | 0.03 | Subtle atmosphere |
| Filter Cutoff | 600 Hz | Dark, ominous |
| Filter Resonance | 0.2 | |
| Filter Env Amount | 1500 Hz | |
| Filter A/D/S/R | 1.5 / 3.0 / 0.4 / 2.0 s | Slow, evolving |
| Amp A/D/S/R | 1.0 / 1.0 / 0.9 / 2.5 s | Sustained drone |
| LFO Freq | 0.2 Hz | Very slow |
| LFO Shape | Triangle | |
| Wheel Mod | Filter | Slow filter sweep |
| Drift | 3.0 | |

**Prophet-5 Recreation -- Metallic Pulse (Prophet-10 style):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Pulse, PW 0.5 | Clean metallic |
| Osc B | Triangle | Sub-harmonic weight |
| Osc A Level | 0.8 | |
| Osc B Level | 0.3 | |
| Filter Cutoff | 1500 Hz | |
| Filter Resonance | 0.4 | Metallic ring |
| Filter Env Amount | 5000 Hz | |
| Filter A/D/S/R | 0.001 / 0.15 / 0.0 / 0.05 s | Ultra-fast, percussive |
| Amp A/D/S/R | 0.001 / 0.2 / 0.0 / 0.1 s | Short hit |
| Poly Mod | Osc B -> Freq A | Metallic FM character |
| Poly Mod Osc B amount | 0.3 | |
| Drift | 1.5 | |

**Key character:** The frying-pan-hit metallic percussion is not a synth -- Fiedel
literally struck a cast iron pan. The Prophet-10's heartbeat pulse and the OB-Xa's
sustained low drone are the synth elements. Fiedel synced the two machines by hand.

Sources:
- [Synthtopia - Recreating Terminator Theme](https://www.synthtopia.com/content/2025/03/13/recreating-the-terminator-theme-on-vintage-modern-synths/)
- [Gearspace - Terminator 1 Synths](https://gearspace.com/board/electronic-music-instruments-and-electronic-music-production/392143-terminator-1-synths.html)
- [MusicRadar - Brad Fiedel Terminator Interview](https://www.musicradar.com/artists/i-went-and-got-this-big-cast-iron-frying-pan-and-a-hammer-and-i-held-it-up-facing-this-funky-old-mic-and-just-went-bam-terminator-composer-brad-fiedel-on-the-making-of-his-iconic-synth-fuelled-sci-fi-soundtrack)

---

### 20. Twin Peaks (1990) -- Dreamy Synth

**Original gear:** Yamaha DX-7, Roland D-50, Roland MKS-70 (JX-10 rack), Roland
D-550. The pads were a JX-10 layered with D-50. Angelo Badalamenti composed.

**Prophet-5 Recreation -- Laura Palmer's Theme Pad:**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | String-like base |
| Osc B | Saw | Detuned for dreaminess |
| Osc A Level | 0.65 | |
| Osc B Level | 0.65 | |
| Noise Level | 0.02 | Hint of air |
| Filter Cutoff | 1200 Hz | Dark, mysterious |
| Filter Resonance | 0.08 | |
| Filter Env Amount | 800 Hz | Gentle opening |
| Filter A/D/S/R | 0.8 / 2.0 / 0.5 / 2.0 s | Slow bloom |
| Amp A/D/S/R | 1.0 / 0.5 / 1.0 / 3.0 s | Slow attack, long release |
| LFO Freq | 0.5 Hz | Very slow |
| LFO Shape | Triangle | |
| Wheel Mod | PW A (set Osc A to Pulse PW 0.4) | Dreamy movement |
| Drift | 3.0 | Vintage, warm instability |

**Alternate approach using reverse saw:**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | Step up one octave |
| Osc B | Saw (reverse polarity via phase) | Cuts mid-range |
| Filter Cutoff | Start low, automate up | 24 dB/oct slope |
| LFO | Sine, slow, to Osc B detune | Creates vintage dreamy effect |

**Key character:** The Twin Peaks pad is warm, dark, slow-moving. Everything should
feel like it is underwater or behind glass. Heavy reverb with high-end rolled off.
The LFO creating slow detuning between oscillators is key to the "dreamy" quality.

Sources:
- [Soundfly - Recreate Twin Peaks Synth Sound](https://flypaper.soundfly.com/produce/how-to-recreate-the-synth-sound-of-twin-peaks-laura-palmers-theme/)
- [Synthtopia - Synth Sounds of Twin Peaks](https://www.synthtopia.com/content/2017/05/21/the-synth-sounds-of-twin-peaks/)
- [Reverb - Synth Sounds of Twin Peaks](https://reverb.com/news/video-the-synth-sounds-of-twin-peaks)
- [MATRIXSYNTH - Twin Peaks Pad Breakdown](https://www.matrixsynth.com/2021/03/the-perfect-pad-twin-peaks-breakdown.html)

---

## MODERN / CONTEMPORARY

---

### 21. Tame Impala -- Analog Synth Sounds (Currents, 2015)

**Original gear:** Roland Juno-106 (primary), Roland JV-1080, Korg MS-20,
Moog Sub Phatty. Kevin Parker's signature uses the Juno-106 chorus heavily.

**Prophet-5 Recreation -- "Nangs" Filter Wah (Juno-106 style):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | Classic Juno tone |
| Osc B | (off) | Single oscillator like the Juno |
| Osc A Level | 1.0 | |
| Filter Cutoff | 50% (~4000 Hz) | Starting point |
| Filter Resonance | ~0.1 | Low |
| Filter Env Amount | ~1000 Hz | |
| Amp A/D/S/R | 0.01 / 0.5 / 0.8 / 0.3 s | |
| LFO Freq | ~3.8 Hz (1/16 at 98 BPM) | Tempo-synced |
| LFO Shape | Saw | Rising mod shape for "wah" |
| Wheel Mod | Filter | LFO modulates filter cutoff |
| Drift | 2.0 | |

**Prophet-5 Recreation -- "Let It Happen" Sequenced Bass:**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | |
| Osc B | Pulse, PW 0.45 | |
| Osc A Level | 0.7 | |
| Osc B Level | 0.5 | |
| Filter Cutoff | 800 Hz | |
| Filter Resonance | 0.3 | |
| Filter Env Amount | 3000 Hz | |
| Filter A/D/S/R | 0.001 / 0.2 / 0.15 / 0.1 s | Plucked |
| Amp A/D/S/R | 0.001 / 0.3 / 0.6 / 0.1 s | |
| Drift | 1.5 | |

**Key character:** Tame Impala's sound relies heavily on the Juno-106 chorus and
Roland character. On the Prophet-5, use drift and slight detuning to approximate
the chorus. The filter "wah-wah" on Nangs is a saw-wave LFO modulating the filter.

Sources:
- [Reverb Machine - Tame Impala Synth Sounds (3 parts)](https://reverbmachine.com/blog/tame-impala-synth-sounds/)
- [Happy Mag - Tame Impala Currents Synth Sounds](https://happymag.tv/learn-how-to-play-the-synth-sounds-of-tame-impalas-currents-thanks-to-a-real-life-synth-wizard/)

---

### 22. MGMT -- "Kids" (2007) -- Synth Riff

**Original gear:** Moog Little Phatty (lead), Korg Delta (chords). The lead has
a distinctive "honky" resonant quality.

**Prophet-5 Recreation -- Lead (from Reverb Machine, using Prophet-5 V):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Pulse, PW 0.5 (square) | Single square wave |
| Osc B | (off) | Minimal |
| Osc A Level | 1.0 | |
| Filter Cutoff | 1687 Hz | Research-verified exact value |
| Filter Resonance | 0.57 | Very high -- "honky" character |
| Filter Env Amount | 0 Hz | No filter envelope |
| Amp A/D/S/R | 0.01 / 0.3 / 0.85 / 0.2 s | |
| LFO Freq | 5.65 Hz | Wide pitch vibrato |
| LFO Shape | Triangle | |
| Wheel Mod | Freq A, wide depth | LFO 2 vibrato (not mod-wheel controlled) |
| Drift | 1.0 | |

**Prophet-5 Recreation -- Chord/Organ Pad:**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Pulse, PW 0.5 (square) | Organ-like |
| Osc B | Saw | Combined for richness |
| Osc A Level | 0.6 | |
| Osc B Level | 0.6 | |
| Filter Cutoff | 4000 Hz | Open |
| Filter Resonance | 0.1 | |
| LFO Freq | 5.65 Hz | Deep, fast vibrato |
| LFO Shape | Triangle | |
| Wheel Mod | Freq A + Freq B | Vibrato on both oscillators |
| Drift | 1.5 | |

**Key character:** The high resonance at 1687 Hz with a square wave is the secret
to the "honky" mid-range-focused lead. The wide vibrato via LFO is also essential.
No filter envelope -- the tone is static with just the resonance peak shaping it.

Sources:
- [Reverb Machine - MGMT Kids Synth Sounds](https://reverbmachine.com/blog/mgmt-kids-synth-sounds/)
- [MusicRadar - How to Make MGMT Kids Synth](https://www.musicradar.com/tuition/tech/how-to-make-an-mgmt-kids-style-synth-sound-453001)
- [Reverb - Recreating MGMT Oracular Spectacular](https://reverb.com/news/recreating-mgmts-oracular-spectacular-synths-with-software-instruments)

---

### 23. M83 -- "Midnight City" (2011) -- Synth Lead

**Original gear:** NOT a traditional synth patch! Anthony Gonzalez created this
by singing the melody at the top of his voice and heavily distorting the vocal
recording. The Yamaha CS-80 and Roland JX-3P were used elsewhere on the album.

**Prophet-5 Recreation (synthetic approximation):**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | Stable pitch |
| Osc B | Saw | With pitch bend (swept upward) |
| Osc A Level | 0.7 | |
| Osc B Level | 0.5 | |
| Sync | true | Creates harmonic sweep |
| Filter Cutoff | 4000 Hz | Bright |
| Filter Resonance | 0.2 | |
| Filter Env Amount | 3000 Hz | |
| Filter A/D/S/R | 0.1 / 1.0 / 0.5 / 0.5 s | Medium sweep |
| Amp A/D/S/R | 0.05 / 0.5 / 0.85 / 0.5 s | |
| Poly Mod | Filter Env -> Freq A, amount 0.4 | Pitch sweep on attack |
| LFO Freq | 0.3 Hz | Slow undulation |
| Wheel Mod | Filter | |
| Drift | 2.0 | |

**Key character:** The original is literally a distorted voice, which gives it an
inherently vocal, human quality impossible to perfectly replicate with synthesis.
Using oscillator sync with poly mod sweep and a bright, resonant filter gets in
the ballpark. Heavy reverb and stereo delay are essential. Sidechain compression
for the pulsing effect.

Sources:
- [KVR - What is Midnight City Synth](https://www.kvraudio.com/forum/viewtopic.php?t=341622)
- [Happy Mag - Engineering M83's Hurry Up We're Dreaming](https://happymag.tv/engineering-the-sound-m83s-hurry-up-were-dreaming/)

---

### 24. Stranger Things Score -- Kyle Dixon & Michael Stein Textures

**Original gear:** Predominantly Juno-60, Juno-106, Prophet-5, SH-101, ARP 2600,
various modular. S U R V I V E use a mix of vintage analog.

**Prophet-5 Recreation -- Evolving Dark Texture:**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Pulse, PW 0.4 | |
| Osc B | Triangle | Sub for FM modulation |
| Osc A Level | 0.8 | |
| Osc B Level | 0.2 | Low in mix, high in mod |
| Noise Level | 0.05 | |
| Filter Cutoff | 400 Hz | Dark |
| Filter Resonance | 0.4 | |
| Filter Env Amount | 2000 Hz | |
| Filter A/D/S/R | 1.0 / 2.0 / 0.3 / 1.5 s | Slow |
| Amp A/D/S/R | 0.8 / 1.0 / 0.8 / 2.0 s | |
| Poly Mod | Osc B -> Freq A + Filter | Evolving FM textures |
| Poly Mod Osc B amount | 0.25 | |
| LFO Freq | 0.1 Hz | Ultra-slow |
| LFO Shape | Triangle | |
| Wheel Mod | Filter | |
| Drift | 4.0 | Maximum instability |

**Key character:** Dark, unsettling, evolving textures. Heavy use of filter
automation, slow LFOs, and FM via Poly Mod for metallic, unstable timbres.
High drift adds organic unpredictability.

Sources:
- [Syntorial - Stranger Things Synths](https://www.syntorial.com/tutorials/stranger-things-synths/)
- [FACT Magazine - Stranger Things Synths](https://www.factmag.com/2017/10/28/stranger-things-synths-plugins-techniques/)

---

### 25. Daft Punk -- "Giorgio by Moroder" (2013) -- Analog Section

**Original gear:** Modcan modular synthesizer (custom system), Moog modular,
various vintage analog. The arpeggio was captured via MIDI through multiple
different synths layered.

**Prophet-5 Recreation -- Analog Arpeggio Layer:**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | Classic analog saw |
| Osc B | Saw | Slightly detuned |
| Osc A Level | 0.8 | |
| Osc B Level | 0.6 | |
| Filter Cutoff | 2000 Hz | |
| Filter Resonance | 0.15 | |
| Filter Env Amount | 3000 Hz | |
| Filter A/D/S/R | 0.001 / 0.2 / 0.1 / 0.1 s | Plucked arp character |
| Amp A/D/S/R | 0.001 / 0.25 / 0.15 / 0.1 s | Short, percussive |
| Drift | 2.0 | |

**Prophet-5 Recreation -- Bass Sweep:**

| Parameter | Value | Notes |
|-----------|-------|-------|
| Osc A | Saw | |
| Osc B | (off) | |
| Osc A Level | 1.0 | |
| Filter Cutoff | 200 Hz | Starts very dark |
| Filter Resonance | 0.3 | |
| Filter Env Amount | 8000 Hz | Massive sweep |
| Filter A/D/S/R | 0.001 / 1.0 / 0.0 / 0.5 s | Long filter decay |
| Amp A/D/S/R | 0.001 / 0.5 / 0.6 / 0.2 s | |
| Drift | 1.5 | |

**Key character:** The track's analog section celebrates raw, vintage analog
synthesis. The arpeggio was layered through multiple synths for richness.
The filter manipulation -- especially pitch-dropping saw waves while adjusting
filter frequency to create "tom-like" tonal shifts -- is a key technique.

Sources:
- [Complex - Modcan Synth Used on Giorgio](https://www.complex.com/music/2014/01/daft-punk-modcan-synth)
- [KVR - Giorgio by Moroder Outro Sound](https://www.kvraudio.com/forum/viewtopic.php?t=563744)
- [Synth Ctrl - How to Sound Like Daft Punk](https://synthctrl.com/blogs/blog/how-to-sound-like-daft-punk)

---

## SUMMARY: Prophet-5 Parameter Translation Guide

When translating from other synth architectures to Prophet-5:

### Filter Cutoff Translation
- Percentage-based values: 0% = ~20 Hz, 50% = ~2000-4000 Hz, 100% = ~16000 Hz
- The Prophet-5's CEM3320 filter has a 24dB/oct slope matching most classic analogs
- "Fully open" = 10000-16000 Hz range

### Envelope Translation
- "Fast attack" = 0.001-0.005 s
- "Medium attack" = 0.05-0.2 s
- "Slow attack" = 0.3-1.5 s
- "Short decay" = 0.1-0.3 s
- "Medium decay" = 0.3-1.0 s
- "Long decay" = 1.0-10.0 s

### Character Mapping
- **Chorus effect** (not on Prophet-5): Use drift (2.0+) and slight PW differences
- **Unison/detune** from other synths: Use Prophet-5 Unison mode or Osc B detuning
- **FM synthesis** (DX7 etc): Use Poly Mod (Osc B -> Freq A) for cross-modulation
- **Sample-based sounds** (Fairlight, Emulator): Cannot truly replicate; use Poly Mod
  FM or PWM for timbral complexity as an approximation
- **Poly aftertouch** (CS-80): Map mod wheel to filter cutoff for manual control
- **Ring modulation**: Approximate with Poly Mod at high amounts
- **Multi-layer** sounds: Prophet-5 has 5 voices, no layering; focus on the most
  characterful single layer

### Original Prophet-5 Songs in This List
These songs actually used a Prophet-5 (or direct variant like Prophet T-8/10):
- Japan "Ghosts" -- Prophet-5 confirmed (drone + strings)
- Tears for Fears "EWTRTW" -- Prophet T-8 (upgraded Prophet-5)
- Tears for Fears "Shout" -- Prophet-5 bass in unison
- Peter Gabriel "Sledgehammer" -- Prophet-5 confirmed
- John Carpenter "Halloween II" -- Prophet-5 and Prophet-10
- John Carpenter "Escape from New York" -- Prophet-5 and Prophet-10
- Terminator -- Prophet-10 (same architecture as Prophet-5)
- Stranger Things -- Prophet-5 used by S U R V I V E
