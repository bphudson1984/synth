<script lang="ts">
    let expandedSections = $state<Set<string>>(new Set(['getting-started']));

    function toggleSection(id: string) {
        const next = new Set(expandedSections);
        if (next.has(id)) next.delete(id);
        else next.add(id);
        expandedSections = next;
    }

    function isOpen(id: string): boolean {
        return expandedSections.has(id);
    }

    const { onBack }: { onBack?: () => void } = $props();
</script>

<div class="help-panel">

    {#if onBack}
        <button class="back-btn" onclick={onBack}>← BACK</button>
    {/if}

    <header class="help-header">
        <div class="help-brand">Hudsonic</div>
        <h1 class="help-title">ORBIT</h1>
        <p class="help-subtitle">User Manual</p>
    </header>

    <!-- TABLE OF CONTENTS -->
    <nav class="toc">
        <div class="toc-title">Contents</div>
        <ol class="toc-list">
            <li><button class="toc-link" onclick={() => { const el = document.getElementById('getting-started'); if (el) el.scrollIntoView({ behavior: 'smooth' }); if (!isOpen('getting-started')) toggleSection('getting-started'); }}>Getting Started</button></li>
            <li><button class="toc-link" onclick={() => { const el = document.getElementById('drum'); if (el) el.scrollIntoView({ behavior: 'smooth' }); if (!isOpen('drum')) toggleSection('drum'); }}>Drum Machine</button></li>
            <li><button class="toc-link" onclick={() => { const el = document.getElementById('pad'); if (el) el.scrollIntoView({ behavior: 'smooth' }); if (!isOpen('pad')) toggleSection('pad'); }}>Pad Synth</button></li>
            <li><button class="toc-link" onclick={() => { const el = document.getElementById('acid'); if (el) el.scrollIntoView({ behavior: 'smooth' }); if (!isOpen('acid')) toggleSection('acid'); }}>Acid Synth</button></li>
            <li><button class="toc-link" onclick={() => { const el = document.getElementById('lead'); if (el) el.scrollIntoView({ behavior: 'smooth' }); if (!isOpen('lead')) toggleSection('lead'); }}>Lead Synth</button></li>
            <li><button class="toc-link" onclick={() => { const el = document.getElementById('mix'); if (el) el.scrollIntoView({ behavior: 'smooth' }); if (!isOpen('mix')) toggleSection('mix'); }}>Mixer</button></li>
            <li><button class="toc-link" onclick={() => { const el = document.getElementById('fx'); if (el) el.scrollIntoView({ behavior: 'smooth' }); if (!isOpen('fx')) toggleSection('fx'); }}>FX Panel</button></li>
            <li><button class="toc-link" onclick={() => { const el = document.getElementById('glitch'); if (el) el.scrollIntoView({ behavior: 'smooth' }); if (!isOpen('glitch')) toggleSection('glitch'); }}>Glitch</button></li>
            <li><button class="toc-link" onclick={() => { const el = document.getElementById('sequencer'); if (el) el.scrollIntoView({ behavior: 'smooth' }); if (!isOpen('sequencer')) toggleSection('sequencer'); }}>Sequencer &amp; Recording</button></li>
            <li><button class="toc-link" onclick={() => { const el = document.getElementById('tips'); if (el) el.scrollIntoView({ behavior: 'smooth' }); if (!isOpen('tips')) toggleSection('tips'); }}>Tips &amp; Tricks</button></li>
            <li><button class="toc-link" onclick={() => { const el = document.getElementById('specs'); if (el) el.scrollIntoView({ behavior: 'smooth' }); if (!isOpen('specs')) toggleSection('specs'); }}>Technical Specs</button></li>
        </ol>
    </nav>
    <!-- GETTING STARTED -->
    <section id="getting-started" class="section">
        <button class="section-header" onclick={() => toggleSection('getting-started')}>
            <span class="section-number">01</span>
            <span class="section-title">Getting Started</span>
            <span class="chevron" class:open={isOpen('getting-started')}>▸</span>
        </button>
        {#if isOpen('getting-started')}
            <div class="section-body">
                <h3>What is ORBIT?</h3>
                <p>
                    ORBIT is a browser-based music production workstation that combines four
                    classic synthesiser and drum machine emulations into a single app. Everything
                    runs locally in your browser using WebAssembly — no server, no account, no
                    internet required after the initial load.
                </p>

                <h3>The Four Engines</h3>
                <div class="card-grid">
                    <div class="card" style="--card-accent: #378ADD">
                        <div class="card-label">DRUM</div>
                        <div class="card-desc">TR-808 / TR-909 drum machine with 8 voices and 16-step sequencer</div>
                    </div>
                    <div class="card" style="--card-accent: #E8944A">
                        <div class="card-label">PAD</div>
                        <div class="card-desc">Prophet-5 style polyphonic synth for chords and pads</div>
                    </div>
                    <div class="card" style="--card-accent: #5DBE6E">
                        <div class="card-label">ACID</div>
                        <div class="card-desc">TB-303 acid bass synth with accent, slide, and classic filter</div>
                    </div>
                    <div class="card" style="--card-accent: #B56ECC">
                        <div class="card-label">LEAD</div>
                        <div class="card-desc">Braids modal synth with 13 models — from saw to cloud granular</div>
                    </div>
                </div>

                <h3>First Launch</h3>
                <ol>
                    <li>Open ORBIT in a modern browser (Chrome, Firefox, or Safari).</li>
                    <li>Tap <strong>TAP TO START</strong> on the splash screen. This initialises all four audio engines — it may take a moment.</li>
                    <li>You arrive on the <strong>DRUM</strong> tab. Use the top navigation to switch between DRUM, PAD, ACID, LEAD, and MIX.</li>
                    <li>Hit <strong>Play ▶</strong> in any panel to start the transport. All engines share a single global clock.</li>
                </ol>

                <h3>Navigation</h3>
                <p>
                    The tab bar at the top of the screen lets you switch panels instantly. The five tabs are
                    <strong>DRUM</strong>, <strong>PAD</strong>, <strong>ACID</strong>, <strong>LEAD</strong>, and <strong>MIX</strong>.
                    A <strong>?</strong> button in the top-right opens this help page.
                </p>
                <p>
                    Playback continues across all engines even while you switch tabs, so you can
                    build up a multi-instrument arrangement in real time.
                </p>
            </div>
        {/if}
    </section>
    <!-- DRUM MACHINE -->
    <section id="drum" class="section">
        <button class="section-header" onclick={() => toggleSection('drum')}>
            <span class="section-number">02</span>
            <span class="section-title">Drum Machine</span>
            <span class="chevron" class:open={isOpen('drum')}>▸</span>
        </button>
        {#if isOpen('drum')}
            <div class="section-body">
                <p>
                    The Drum panel emulates both the <strong>Roland TR-808</strong> and <strong>TR-909</strong>
                    drum machines with 8 colour-coded voices and a 16-step pattern sequencer.
                </p>

                <h3>Voices</h3>
                <table>
                    <thead><tr><th>Pad</th><th>Voice</th><th>Colour</th></tr></thead>
                    <tbody>
                        <tr><td>KCK</td><td>Kick</td><td><span class="colour-swatch" style="background:#378ADD"></span> Blue</td></tr>
                        <tr><td>SNR</td><td>Snare</td><td><span class="colour-swatch" style="background:#D85A30"></span> Coral</td></tr>
                        <tr><td>CHH</td><td>Closed Hi-Hat</td><td><span class="colour-swatch" style="background:#1D9E75"></span> Teal</td></tr>
                        <tr><td>OHH</td><td>Open Hi-Hat</td><td><span class="colour-swatch" style="background:#5DCAA5"></span> Light Teal</td></tr>
                        <tr><td>CLP</td><td>Clap</td><td><span class="colour-swatch" style="background:#D4537E"></span> Pink</td></tr>
                        <tr><td>TOM</td><td>Tom</td><td><span class="colour-swatch" style="background:#EF9F27"></span> Amber</td></tr>
                        <tr><td>RIM</td><td>Rim Shot</td><td><span class="colour-swatch" style="background:#7F77DD"></span> Purple</td></tr>
                        <tr><td>PRC</td><td>Percussion</td><td><span class="colour-swatch" style="background:#639922"></span> Green</td></tr>
                    </tbody>
                </table>

                <h3>The Pad Circle</h3>
                <p>
                    Eight circular pads orbit the centre of the screen. <strong>Tap a pad</strong>
                    to select that voice and trigger it immediately. The selected voice determines
                    which row of the sequencer you are editing and which parameters the slider
                    controls.
                </p>
                <p>
                    <strong>Double-tap</strong> a pad to toggle that individual voice between the
                    808 and 909 engine. A small badge (<em>8</em> or <em>9</em>) appears on the pad
                    to show which engine is active.
                </p>

                <h3>Parameter Diamonds</h3>
                <p>
                    Four small diamond-shaped controls sit inside the pad orbit. Each one selects
                    a parameter for the horizontal slider:
                </p>
                <ul>
                    <li><strong>L — Level</strong>: Volume of the voice (0–100)</li>
                    <li><strong>D — Decay</strong>: How quickly the sound fades (0–100)</li>
                    <li><strong>T — Tone</strong>: Brightness / timbre (0–100)</li>
                    <li><strong>P — Pitch</strong>: Tuning of the voice (0–100)</li>
                </ul>

                <h3>Step Sequencer</h3>
                <p>
                    A 16-step grid (8&nbsp;×&nbsp;2 rows) sits above the pad circle. Tap any step
                    to toggle it on or off for the selected voice. During playback the current step
                    is highlighted with a white outline and glow.
                </p>

                <h3>808 / 909 Toggle</h3>
                <p>
                    Two buttons below the sequencer switch <strong>all</strong> voices between the
                    808 and 909 engines at once. Individual voices can still be overridden by
                    double-tapping their pad.
                </p>

                <h3>Presets</h3>
                <p>
                    A dropdown at the top offers <strong>20 built-in drum patterns</strong> spanning
                    house, techno, hip-hop, breakbeat, trap, drum &amp; bass, bossa nova, and more.
                    The <strong>RND</strong> button generates a random pattern with musically-weighted
                    voice densities.
                </p>

                <h4>Available Drum Presets</h4>
                <div class="preset-grid">
                    <span class="preset-tag">Four on the Floor</span>
                    <span class="preset-tag">Classic House</span>
                    <span class="preset-tag">Boom Bap</span>
                    <span class="preset-tag">Breakbeat</span>
                    <span class="preset-tag">Techno Basic</span>
                    <span class="preset-tag">Detroit Techno</span>
                    <span class="preset-tag">Acid House</span>
                    <span class="preset-tag">Minimal Techno</span>
                    <span class="preset-tag">Electro</span>
                    <span class="preset-tag">Reggaeton</span>
                    <span class="preset-tag">Trap</span>
                    <span class="preset-tag">UK Garage</span>
                    <span class="preset-tag">Disco</span>
                    <span class="preset-tag">Drum &amp; Bass</span>
                    <span class="preset-tag">Afrobeat</span>
                    <span class="preset-tag">Latin Percussion</span>
                    <span class="preset-tag">Industrial</span>
                    <span class="preset-tag">Lo-Fi Hip Hop</span>
                    <span class="preset-tag">Bossa Nova</span>
                    <span class="preset-tag">Trance</span>
                </div>

                <h3>Transport</h3>
                <p>
                    The play controls at the bottom include <strong>BPM −/+</strong> buttons
                    (range 60–200), a BPM display, and <strong>Stop / Play</strong> buttons.
                    The transport is shared across all panels — pressing play here also starts
                    the Pad, Acid, and Lead sequencers.
                </p>
            </div>
        {/if}
    </section>
    <!-- PAD SYNTH -->
    <section id="pad" class="section">
        <button class="section-header" onclick={() => toggleSection('pad')}>
            <span class="section-number">03</span>
            <span class="section-title">Pad Synth</span>
            <span class="chevron" class:open={isOpen('pad')}>▸</span>
        </button>
        {#if isOpen('pad')}
            <div class="section-body">
                <p>
                    The Pad panel is a <strong>Prophet-5 style polyphonic synthesiser</strong>
                    for playing chords and pads. Eight pads trigger chords from a C minor
                    harmonisation, and a powerful sequencer lets you record and arrange
                    chord progressions.
                </p>

                <h3>Chord Pads</h3>
                <table>
                    <thead><tr><th>Pad</th><th>Chord</th><th>Notes</th></tr></thead>
                    <tbody>
                        <tr><td>1</td><td>Cm</td><td>C3, Eb3, G3</td></tr>
                        <tr><td>2</td><td>Ddim</td><td>D3, F3, Ab3</td></tr>
                        <tr><td>3</td><td>Eb</td><td>Eb3, G3, Bb3</td></tr>
                        <tr><td>4</td><td>Fm</td><td>F3, Ab3, C4</td></tr>
                        <tr><td>5</td><td>Gm</td><td>G3, Bb3, D4</td></tr>
                        <tr><td>6</td><td>Ab</td><td>Ab3, C4, Eb4</td></tr>
                        <tr><td>7</td><td>Bb</td><td>Bb3, D4, F4</td></tr>
                        <tr><td>8</td><td>Cm'</td><td>C4, Eb4, G4</td></tr>
                    </tbody>
                </table>

                <h3>Parameters</h3>
                <table>
                    <thead><tr><th>Diamond</th><th>Parameter</th><th>Effect</th></tr></thead>
                    <tbody>
                        <tr><td>C</td><td>Cutoff</td><td>Filter frequency — low values sound dark, high values sound bright (200–12,000 Hz)</td></tr>
                        <tr><td>R</td><td>Resonance</td><td>Filter emphasis — high values create a sharp, whistling peak (0–3.0)</td></tr>
                        <tr><td>A</td><td>Attack</td><td>How slowly the sound fades in (5 ms – 2 s)</td></tr>
                        <tr><td>RE</td><td>Release</td><td>How slowly the sound fades out after releasing (10 ms – 3 s)</td></tr>
                    </tbody>
                </table>

                <h3>Arpeggiator</h3>
                <p>
                    Tap the <strong>ARP</strong> button to enable the arpeggiator. When active, holding
                    a chord pad causes the notes to cycle automatically. Open the ARP drawer to
                    configure:
                </p>
                <ul>
                    <li><strong>Mode</strong>: OFF, UP (ascending), DN (descending), U/D (ping-pong), RND (random)</li>
                    <li><strong>Division</strong>: 1/4, 1/8, 1/16, 1/32 note values</li>
                    <li><strong>Octaves</strong>: 1, 2, or 3 octave range</li>
                </ul>

                <h3>Note Sequencer</h3>
                <p>
                    The PAD panel includes a full note sequencer (see the
                    <strong>Sequencer &amp; Recording</strong> section for details).
                    Use it to program chord progressions that play back automatically.
                </p>

                <h3>Presets</h3>
                <p>14 built-in sounds organised by category:</p>
                <div class="preset-grid">
                    <span class="preset-tag">Warm Pad</span>
                    <span class="preset-tag">In The Air</span>
                    <span class="preset-tag">Blade Runner</span>
                    <span class="preset-tag">Human Nature</span>
                    <span class="preset-tag">Prophet Strings</span>
                    <span class="preset-tag">Prophet Brass</span>
                    <span class="preset-tag">Soft Brass</span>
                    <span class="preset-tag">Organ</span>
                    <span class="preset-tag">Take On Me</span>
                    <span class="preset-tag">Sync Lead</span>
                    <span class="preset-tag">Unison Lead</span>
                    <span class="preset-tag">Carpenter Horror</span>
                    <span class="preset-tag">Stranger Things</span>
                    <span class="preset-tag">Ghost Bell</span>
                </div>
            </div>
        {/if}
    </section>
    <!-- ACID SYNTH -->
    <section id="acid" class="section">
        <button class="section-header" onclick={() => toggleSection('acid')}>
            <span class="section-number">04</span>
            <span class="section-title">Acid Synth</span>
            <span class="chevron" class:open={isOpen('acid')}>▸</span>
        </button>
        {#if isOpen('acid')}
            <div class="section-body">
                <p>
                    The Acid panel emulates the classic <strong>Roland TB-303</strong> bass synthesiser,
                    famous for its squelchy filter and the birth of acid house music. It has a
                    built-in 16-step sequencer with per-step note, accent, and slide controls.
                </p>

                <h3>Waveform</h3>
                <p>
                    Two buttons toggle between <strong>SAW</strong> (sawtooth — bright, buzzy) and
                    <strong>SQR</strong> (square — hollow, nasal). The active waveform is highlighted.
                </p>

                <h3>Parameters</h3>
                <table>
                    <thead><tr><th>Diamond</th><th>Parameter</th><th>Effect</th></tr></thead>
                    <tbody>
                        <tr><td>C</td><td>Cutoff</td><td>Filter frequency (100–5,000 Hz)</td></tr>
                        <tr><td>R</td><td>Resonance</td><td>Filter peak / squelch (0–1.0)</td></tr>
                        <tr><td>E</td><td>Env Mod</td><td>How much the filter envelope sweeps the cutoff (0–1.0)</td></tr>
                        <tr><td>D</td><td>Distortion</td><td>Overdrive / saturation amount (0–1.0)</td></tr>
                    </tbody>
                </table>

                <h3>Step Sequencer</h3>
                <p>
                    The 16-step grid (8 × 2) shows the note name in each active step. Tap a step
                    to toggle its gate. When a step is selected, additional controls appear:
                </p>
                <ul>
                    <li><strong>ACC (Accent)</strong>: Boosts the volume and filter sweep of the step. Accented steps show a small coloured bar at the top of the cell.</li>
                    <li><strong>SLD (Slide)</strong>: Creates a portamento glide from the previous note into this one. Slide steps show an arrow marker (▸).</li>
                </ul>

                <h3>Note Pads &amp; Transposition</h3>
                <p>
                    Eight pads represent notes in a C minor scale. Tapping a pad
                    <strong>transposes the entire pattern</strong> by that interval — for example,
                    tapping "G" shifts everything up a fifth.
                </p>
                <p>
                    The <strong>OCT−</strong> and <strong>OCT+</strong> buttons shift the whole
                    pattern down or up by one octave (12 semitones). This is cumulative with
                    note-pad transposition.
                </p>

                <h3>Presets</h3>
                <p>23 acid bass presets across many styles:</p>
                <div class="preset-grid">
                    <span class="preset-tag">Acid Tracks</span>
                    <span class="preset-tag">Higher State</span>
                    <span class="preset-tag">Squelch Machine</span>
                    <span class="preset-tag">Bassline Basic</span>
                    <span class="preset-tag">Slide City</span>
                    <span class="preset-tag">Square Acid</span>
                    <span class="preset-tag">Voodoo Ray</span>
                    <span class="preset-tag">Mentasm Stab</span>
                    <span class="preset-tag">Windowlicker</span>
                    <span class="preset-tag">Minimal Pulse</span>
                    <span class="preset-tag">Techno Driver</span>
                    <span class="preset-tag">Dark Warehouse</span>
                    <span class="preset-tag">Hypnotic Roll</span>
                    <span class="preset-tag">Trance Gate</span>
                    <span class="preset-tag">Spiral</span>
                    <span class="preset-tag">Distorted Grind</span>
                    <span class="preset-tag">Buzz Saw</span>
                    <span class="preset-tag">Punisher</span>
                    <span class="preset-tag">Minor Blues</span>
                    <span class="preset-tag">Walking Bass</span>
                    <span class="preset-tag">Chromatic Crawl</span>
                    <span class="preset-tag">Funk Machine</span>
                    <span class="preset-tag">Bounce</span>
                </div>
            </div>
        {/if}
    </section>
    <!-- LEAD SYNTH -->
    <section id="lead" class="section">
        <button class="section-header" onclick={() => toggleSection('lead')}>
            <span class="section-number">05</span>
            <span class="section-title">Lead Synth</span>
            <span class="chevron" class:open={isOpen('lead')}>▸</span>
        </button>
        {#if isOpen('lead')}
            <div class="section-body">
                <p>
                    The Lead panel features the <strong>Braids modal synthesis engine</strong> with
                    13 different sound models. It can play single notes or chords, includes an
                    arpeggiator, latch mode, and a full note sequencer.
                </p>

                <h3>Synthesis Models</h3>
                <table>
                    <thead><tr><th>Model</th><th>Description</th></tr></thead>
                    <tbody>
                        <tr><td>SAW</td><td>Classic sawtooth oscillator — warm, buzzy</td></tr>
                        <tr><td>SQUARE</td><td>Square wave — hollow, reedy</td></tr>
                        <tr><td>MORPH</td><td>Morphing waveform — blends between shapes</td></tr>
                        <tr><td>FOLD</td><td>Wave-folder — adds harmonics through folding distortion</td></tr>
                        <tr><td>FM</td><td>Frequency modulation — metallic, bell-like</td></tr>
                        <tr><td>FBFM</td><td>Feedback FM — complex, evolving tones</td></tr>
                        <tr><td>WTBL</td><td>Wavetable — smoothly scans between stored waveforms</td></tr>
                        <tr><td>VOWL</td><td>Vowel formant — speech-like, human quality</td></tr>
                        <tr><td>PLUK</td><td>Plucked string — physical modelling of a plucked string</td></tr>
                        <tr><td>BELL</td><td>Bell — modal synthesis for ringing bell tones</td></tr>
                        <tr><td>NOIS</td><td>Noise — filtered noise for percussive textures</td></tr>
                        <tr><td>SWARM</td><td>Swarm — many detuned oscillators for thick chorus</td></tr>
                        <tr><td>CLOUD</td><td>Cloud — granular-like particle texture</td></tr>
                    </tbody>
                </table>

                <h3>Parameters</h3>
                <table>
                    <thead><tr><th>Diamond</th><th>Parameter</th><th>Effect</th></tr></thead>
                    <tbody>
                        <tr><td>T</td><td>Timbre</td><td>Primary tone shaping — exact effect depends on model (0–1.0)</td></tr>
                        <tr><td>C</td><td>Color</td><td>Secondary tone colour — model-dependent (0–1.0)</td></tr>
                        <tr><td>F</td><td>Cutoff</td><td>Low-pass filter frequency (100–12,000 Hz)</td></tr>
                        <tr><td>R</td><td>Release</td><td>How slowly the sound fades out (10 ms – 3 s)</td></tr>
                    </tbody>
                </table>

                <h3>Note / Chord Mode</h3>
                <p>
                    Toggle between <strong>NOTE</strong> and <strong>CHRD</strong> with the button
                    below the pad circle:
                </p>
                <ul>
                    <li><strong>NOTE mode</strong>: 8 pads play single notes from the C minor scale (C, D, Eb, F, G, Ab, Bb, C').</li>
                    <li><strong>CHORD mode</strong>: 8 pads play seventh chords — Cm7, Dm7b5, EbM7, Fm7, Gm7, AbM7, Bb7, Cm7'.</li>
                </ul>

                <h3>Latch Mode</h3>
                <p>
                    Tap <strong>LATCH</strong> to enable latching. When active, notes sustain
                    indefinitely until you play a different note. The button glows gold when on.
                    This is useful for hands-free pad sustain while you adjust other parameters.
                </p>

                <h3>Arpeggiator</h3>
                <p>
                    Works the same as the PAD arpeggiator — enable with the <strong>ARP</strong>
                    button and configure mode (OFF / UP / DN / U/D / RND), division (1/4 – 1/32),
                    and octaves (1–3) in the ARP drawer.
                </p>

                <h3>Presets</h3>
                <div class="preset-grid">
                    <span class="preset-tag">Wavetable Pad</span>
                    <span class="preset-tag">Melodic Lead</span>
                    <span class="preset-tag">FM Bells</span>
                    <span class="preset-tag">Pluck Pattern</span>
                    <span class="preset-tag">Swarm Chords</span>
                    <span class="preset-tag">Vowel Sequence</span>
                    <span class="preset-tag">Cloud Texture</span>
                    <span class="preset-tag">Fold Bass</span>
                </div>
            </div>
        {/if}
    </section>
    <!-- MIXER -->
    <section id="mix" class="section">
        <button class="section-header" onclick={() => toggleSection('mix')}>
            <span class="section-number">06</span>
            <span class="section-title">Mixer</span>
            <span class="chevron" class:open={isOpen('mix')}>▸</span>
        </button>
        {#if isOpen('mix')}
            <div class="section-body">
                <p>
                    The Mix panel gives you master control over all four engines with per-channel
                    volume, panning, mute, and solo.
                </p>

                <h3>Channels</h3>
                <table>
                    <thead><tr><th>Channel</th><th>Colour</th><th>Default Volume</th></tr></thead>
                    <tbody>
                        <tr><td>DRUM</td><td><span class="colour-swatch" style="background:#378ADD"></span> Blue</td><td>75</td></tr>
                        <tr><td>PAD</td><td><span class="colour-swatch" style="background:#E8944A"></span> Orange</td><td>75</td></tr>
                        <tr><td>ACID</td><td><span class="colour-swatch" style="background:#5DBE6E"></span> Green</td><td>75</td></tr>
                        <tr><td>LEAD</td><td><span class="colour-swatch" style="background:#B56ECC"></span> Purple</td><td>75</td></tr>
                    </tbody>
                </table>

                <h3>Controls per Channel</h3>
                <ul>
                    <li><strong>Pan Knob</strong>: Rotate to position the channel in the stereo field. Centre is labelled "C"; left values show "L" (e.g. L50); right values show "R" (e.g. R75).</li>
                    <li><strong>Volume Fader</strong>: Vertical slider from 0 to 100. A colour-matched gradient shows the current level.</li>
                    <li><strong>M (Mute)</strong>: Silences the channel. Turns red when active.</li>
                    <li><strong>S (Solo)</strong>: Isolates the channel — all non-soloed channels are silenced. Turns gold when active.</li>
                </ul>

                <h3>Mute / Solo Logic</h3>
                <p>
                    If <strong>any</strong> channel is soloed, only soloed channels produce sound.
                    A channel that is both muted and soloed will still play (solo wins). If
                    nothing is soloed, the mute buttons work independently.
                </p>
            </div>
        {/if}
    </section>

    <!-- FX PANEL -->
    <section id="fx" class="section">
        <button class="section-header" onclick={() => toggleSection('fx')}>
            <span class="section-number">07</span>
            <span class="section-title">FX Panel</span>
            <span class="chevron" class:open={isOpen('fx')}>▸</span>
        </button>
        {#if isOpen('fx')}
            <div class="section-body">
                <p>
                    The FX panel provides four studio-quality send effects that can be applied
                    to any combination of the four instrument channels. Each effect has its own
                    adjustable parameters and per-channel send levels.
                </p>

                <h3>Effects</h3>
                <p>
                    Four effect tabs sit at the top of the panel. Tap a tab to select that effect
                    and view its parameters and send levels:
                </p>
                <table>
                    <thead><tr><th>Tab</th><th>Effect</th><th>Description</th></tr></thead>
                    <tbody>
                        <tr><td>CHORUS</td><td>Chorus</td><td>Adds width and movement by layering modulated copies of the signal</td></tr>
                        <tr><td>DELAY</td><td>Delay</td><td>Repeating echo effect with adjustable time and feedback</td></tr>
                        <tr><td>REVERB</td><td>Reverb</td><td>Simulates room ambience and space</td></tr>
                        <tr><td>DISTORT</td><td>Distortion</td><td>Overdrive and saturation for grit and warmth</td></tr>
                    </tbody>
                </table>

                <h3>Effect Parameters</h3>
                <p>
                    Each effect has its own set of parameters controlled by horizontal sliders.
                    The current value is shown to the right of each slider.
                </p>

                <h4>Chorus</h4>
                <table>
                    <thead><tr><th>Parameter</th><th>Range</th><th>Effect</th></tr></thead>
                    <tbody>
                        <tr><td>RATE</td><td>0.1 – 5 Hz</td><td>Speed of the modulation — lower values produce a gentle sway, higher values create a vibrato</td></tr>
                        <tr><td>DEPTH</td><td>0 – 1</td><td>Intensity of the modulation — higher values produce a more pronounced chorus effect</td></tr>
                    </tbody>
                </table>

                <h4>Delay</h4>
                <table>
                    <thead><tr><th>Parameter</th><th>Range</th><th>Effect</th></tr></thead>
                    <tbody>
                        <tr><td>TIME</td><td>1 – 1000 ms</td><td>Delay time between repeats</td></tr>
                        <tr><td>FEEDBACK</td><td>0 – 0.95</td><td>How many repeats — higher values produce longer echo trails</td></tr>
                        <tr><td>TONE</td><td>0 – 1</td><td>Brightness of the echoes — lower values darken successive repeats</td></tr>
                    </tbody>
                </table>

                <h4>Reverb</h4>
                <table>
                    <thead><tr><th>Parameter</th><th>Range</th><th>Effect</th></tr></thead>
                    <tbody>
                        <tr><td>DECAY</td><td>0 – 0.99</td><td>Length of the reverb tail — higher values simulate larger spaces</td></tr>
                        <tr><td>DAMPING</td><td>0 – 1</td><td>High-frequency absorption — higher values make the reverb darker and more natural</td></tr>
                    </tbody>
                </table>

                <h4>Distortion</h4>
                <table>
                    <thead><tr><th>Parameter</th><th>Range</th><th>Effect</th></tr></thead>
                    <tbody>
                        <tr><td>DRIVE</td><td>0 – 1</td><td>Amount of overdrive / saturation</td></tr>
                        <tr><td>TONE</td><td>0 – 1</td><td>Brightness of the distorted signal</td></tr>
                        <tr><td>LEVEL</td><td>0 – 1</td><td>Output volume of the distortion effect</td></tr>
                    </tbody>
                </table>

                <h3>Sends</h3>
                <p>
                    Below the parameters, four vertical faders control how much of each instrument
                    channel is routed into the currently selected effect. Each fader ranges from
                    0 (no send) to 100 (full send):
                </p>
                <table>
                    <thead><tr><th>Label</th><th>Channel</th><th>Colour</th></tr></thead>
                    <tbody>
                        <tr><td>DR</td><td>Drum</td><td><span class="colour-swatch" style="background:#378ADD"></span> Blue</td></tr>
                        <tr><td>PD</td><td>Pad</td><td><span class="colour-swatch" style="background:#E8944A"></span> Orange</td></tr>
                        <tr><td>AC</td><td>Acid</td><td><span class="colour-swatch" style="background:#5DBE6E"></span> Green</td></tr>
                        <tr><td>LD</td><td>Lead</td><td><span class="colour-swatch" style="background:#B56ECC"></span> Purple</td></tr>
                    </tbody>
                </table>
                <p>
                    Send levels are independent per effect — for example, you can send the drum
                    channel heavily into reverb while keeping it dry on delay, and send the acid
                    channel into distortion without affecting the pad.
                </p>

                <h3>Workflow</h3>
                <ol>
                    <li>Select an effect tab (e.g. REVERB).</li>
                    <li>Adjust the effect parameters to taste (e.g. set DECAY to 0.6 for a medium room).</li>
                    <li>Raise the send fader for each channel you want to apply the effect to.</li>
                    <li>Switch tabs to configure additional effects and their sends.</li>
                </ol>
            </div>
        {/if}
    </section>

    <!-- GLITCH -->
    <section id="glitch" class="section">
        <button class="section-header" onclick={() => toggleSection('glitch')}>
            <span class="section-number">08</span>
            <span class="section-title">Glitch</span>
            <span class="chevron" class:open={isOpen('glitch')}>▸</span>
        </button>
        {#if isOpen('glitch')}
            <div class="section-body">
                <p>
                    The Glitch feature (labelled <strong>GLT</strong>) is a real-time beat-repeat
                    / stutter effect available on every panel. When activated, it rapidly re-triggers
                    a short segment of the current audio to create rhythmic glitch and stutter effects.
                </p>

                <h3>How It Works</h3>
                <p>
                    The GLT control appears in the transport bar at the bottom of each panel.
                    It consists of four circular buttons labelled <strong>8</strong>,
                    <strong>4</strong>, <strong>2</strong>, and <strong>1</strong>. Each number
                    represents a beat division:
                </p>
                <table>
                    <thead><tr><th>Button</th><th>Division</th><th>Effect</th></tr></thead>
                    <tbody>
                        <tr><td>8</td><td>1/8 note</td><td>Fastest stutter — rapid-fire retriggering</td></tr>
                        <tr><td>4</td><td>1/4 note</td><td>Fast stutter</td></tr>
                        <tr><td>2</td><td>1/2 note</td><td>Medium stutter</td></tr>
                        <tr><td>1</td><td>1 beat</td><td>Slowest stutter — longer repeated segment</td></tr>
                    </tbody>
                </table>

                <h3>Using the Glitch</h3>
                <ol>
                    <li><strong>Press and hold</strong> a GLT button to activate the stutter effect. The button glows to indicate it is active.</li>
                    <li><strong>Slide</strong> across buttons while holding to change the stutter size in real time for expressive fills.</li>
                    <li><strong>Release</strong> to stop the glitch — playback returns to normal immediately.</li>
                </ol>

                <h3>Per-Panel Glitch</h3>
                <p>
                    Each instrument panel has its own independent glitch control that only affects
                    that engine's audio:
                </p>
                <table>
                    <thead><tr><th>Panel</th><th>Glitch Colour</th><th>Affects</th></tr></thead>
                    <tbody>
                        <tr><td>DRUM</td><td><span class="colour-swatch" style="background:#378ADD"></span> Blue</td><td>Drum machine output only</td></tr>
                        <tr><td>PAD</td><td><span class="colour-swatch" style="background:#E8944A"></span> Orange</td><td>Pad synth output only</td></tr>
                        <tr><td>ACID</td><td><span class="colour-swatch" style="background:#5DBE6E"></span> Green</td><td>Acid synth output only</td></tr>
                        <tr><td>LEAD</td><td><span class="colour-swatch" style="background:#B56ECC"></span> Purple</td><td>Lead synth output only</td></tr>
                        <tr><td>MIX / FX</td><td>White</td><td>Master output — glitches all channels together</td></tr>
                    </tbody>
                </table>

                <h3>Tips</h3>
                <ul>
                    <li>Use the <strong>8</strong> division on drums for classic rapid-fire snare fills.</li>
                    <li>Try the <strong>1</strong> division on pads for a rhythmic gating effect.</li>
                    <li>Slide from <strong>1 → 8</strong> during a build-up for an accelerating stutter.</li>
                    <li>The MIX / FX glitch affects the master output, so it stutters all four channels simultaneously — great for dramatic breakdowns.</li>
                </ul>
            </div>
        {/if}
    </section>

    <!-- SEQUENCER & RECORDING -->
    <section id="sequencer" class="section">
        <button class="section-header" onclick={() => toggleSection('sequencer')}>
            <span class="section-number">09</span>
            <span class="section-title">Sequencer &amp; Recording</span>
            <span class="chevron" class:open={isOpen('sequencer')}>▸</span>
        </button>
        {#if isOpen('sequencer')}
            <div class="section-body">
                <p>
                    The PAD and LEAD panels share a powerful multi-page note sequencer. Up to
                    <strong>8 pages × 16 steps = 128 steps</strong> can be programmed per instrument.
                </p>

                <h3>Basic Operation</h3>
                <ol>
                    <li><strong>Tap</strong> a sequencer cell to select it.</li>
                    <li><strong>Double-tap</strong> a cell to toggle its gate (on/off).</li>
                    <li>Active cells show the chord or note name with a coloured background.</li>
                    <li>During playback the current step pulses with a white border and glow.</li>
                </ol>

                <h3>Recording</h3>
                <ol>
                    <li>Tap the <strong>REC</strong> button to enter recording mode (indicator turns green).</li>
                    <li>Start playback with the <strong>Play</strong> button.</li>
                    <li>Press pads in real time — each press is captured at the current step.</li>
                    <li>Hold a pad longer to extend the note's gate length.</li>
                    <li>Tap <strong>REC</strong> again to stop recording.</li>
                </ol>
                <p>
                    <em>Step entry</em>: When the transport is stopped, pressing a pad records at
                    the currently selected step and advances to the next.
                </p>

                <h3>Sequencer Settings (SEQ Drawer)</h3>
                <table>
                    <thead><tr><th>Control</th><th>Options</th><th>Effect</th></tr></thead>
                    <tbody>
                        <tr><td>DIR</td><td>FWD, REV, PING, RND</td><td>Direction of playback: forward, reverse, ping-pong, or random</td></tr>
                        <tr><td>RATE</td><td>1/4, 1/8, 1/16, 1/32</td><td>How fast steps advance relative to BPM</td></tr>
                        <tr><td>SWG</td><td>0–100</td><td>Swing amount — shifts alternate steps later for groove</td></tr>
                        <tr><td>←</td><td></td><td>Rotate entire pattern one step left</td></tr>
                        <tr><td>→</td><td></td><td>Rotate entire pattern one step right</td></tr>
                        <tr><td>RND</td><td></td><td>Randomise gate probabilities on active steps</td></tr>
                        <tr><td>CLR</td><td></td><td>Clear the entire sequence</td></tr>
                    </tbody>
                </table>

                <h3>Step Settings (STEP Drawer)</h3>
                <p>Select an active step to see these per-step controls:</p>
                <table>
                    <thead><tr><th>Control</th><th>Range</th><th>Effect</th></tr></thead>
                    <tbody>
                        <tr><td>VEL</td><td>0–100</td><td>Note loudness (maps to MIDI 0–127)</td></tr>
                        <tr><td>LEN</td><td>×1 – ×16</td><td>Gate duration — tap LEN then another step to stretch the note across multiple steps</td></tr>
                        <tr><td>PROB</td><td>0–100%</td><td>Chance the step plays — lower values add randomness. Steps below 80% show a dashed border.</td></tr>
                        <tr><td>RTCH</td><td>1×, 2×, 3×, 4×</td><td>Repeat the note 1–4 times in the gate period — great for rolls and fills</td></tr>
                        <tr><td>SKIP</td><td>On / Off</td><td>Skip this step during playback. Skipped steps appear faded with strikethrough text.</td></tr>
                    </tbody>
                </table>

                <h3>Multi-Page Sequencing</h3>
                <p>
                    Dot indicators at the top of the sequencer show the number of pages. Tap a
                    dot to jump to that page, or tap <strong>+</strong> to add a new page (up to
                    8). The sequencer auto-scrolls during playback to follow the current step.
                    You can also drag horizontally to scrub between pages.
                </p>

                <h3>Drag &amp; Reorder</h3>
                <p>
                    Click and hold a step, then drag it to a new position to rearrange your
                    sequence. The dragged step becomes faded and a ghost label shows where
                    it will land.
                </p>
            </div>
        {/if}
    </section>
    <!-- TIPS & TRICKS -->
    <section id="tips" class="section">
        <button class="section-header" onclick={() => toggleSection('tips')}>
            <span class="section-number">10</span>
            <span class="section-title">Tips &amp; Tricks</span>
            <span class="chevron" class:open={isOpen('tips')}>▸</span>
        </button>
        {#if isOpen('tips')}
            <div class="section-body">
                <h3>Building a Complete Track</h3>
                <ol>
                    <li>Set your tempo using the BPM −/+ buttons (60–200 BPM).</li>
                    <li>Start with a <strong>drum groove</strong> — load a preset or build one from scratch.</li>
                    <li>Switch to the <strong>PAD</strong> tab and record a chord progression using the sequencer.</li>
                    <li>Add an <strong>acid bassline</strong> on the ACID tab — tweak cutoff and resonance for that classic squelch.</li>
                    <li>Layer a <strong>lead melody</strong> on the LEAD tab — try different synthesis models for variety.</li>
                    <li>Open the <strong>MIX</strong> tab to balance volumes, pan channels, and use mute/solo to focus.</li>
                    <li>Press play and everything runs in sync!</li>
                </ol>

                <h3>Sound Design</h3>
                <ul>
                    <li><strong>Acid wobbles</strong>: Crank the resonance and envelope mod on the ACID panel, then sweep the cutoff for classic 303 squelch.</li>
                    <li><strong>Lush pads</strong>: On the PAD panel, use a slow attack (60+) and long release (70+) with moderate cutoff. Try the "Blade Runner" or "In The Air" presets as a starting point.</li>
                    <li><strong>Plucked leads</strong>: Set the LEAD model to PLUK with short release for staccato plucked-string sounds.</li>
                    <li><strong>Thick swarms</strong>: Use the SWARM model on the LEAD panel and adjust Timbre to control the number of detuned voices.</li>
                    <li><strong>808 vs 909</strong>: Double-tap individual drum pads to mix engines — try a 909 kick with 808 hi-hats.</li>
                </ul>

                <h3>Sequencer Techniques</h3>
                <ul>
                    <li><strong>Probability</strong>: Set some steps to 50–80% probability for organic, ever-changing patterns.</li>
                    <li><strong>Ratchets</strong>: Use 2× or 3× ratchet on hi-hats or snares for instant drum fills.</li>
                    <li><strong>Swing</strong>: Add 20–40% swing for a looser, groovier feel.</li>
                    <li><strong>Ping-pong direction</strong>: Set direction to PING for melodies that bounce back and forth.</li>
                    <li><strong>Random direction</strong>: Set direction to RND for generative, ever-evolving sequences.</li>
                    <li><strong>Step entry</strong>: Stop playback and use the pads to record one step at a time for precise programming.</li>
                </ul>

                <h3>Performance</h3>
                <ul>
                    <li><strong>Real-time tweaking</strong>: All parameter sliders respond instantly during playback — try sweeping the acid cutoff or pad filter live.</li>
                    <li><strong>Latch + Arp</strong>: Enable both latch and arpeggiator on the LEAD panel for hands-free evolving arpeggios.</li>
                    <li><strong>Mute/Solo</strong>: Use the MIX panel's mute and solo buttons to create breakdowns and build-ups.</li>
                    <li><strong>Transpose</strong>: On the ACID panel, tap different note pads to transpose the bassline to match your chord progression in real time.</li>
                </ul>
            </div>
        {/if}
    </section>

    <!-- TECHNICAL SPECS -->
    <section id="specs" class="section">
        <button class="section-header" onclick={() => toggleSection('specs')}>
            <span class="section-number">11</span>
            <span class="section-title">Technical Specs</span>
            <span class="chevron" class:open={isOpen('specs')}>▸</span>
        </button>
        {#if isOpen('specs')}
            <div class="section-body">
                <h3>Browser Requirements</h3>
                <ul>
                    <li>Chrome 66+, Firefox 76+, or Safari 14.1+</li>
                    <li>Web Audio API with AudioWorklet support</li>
                    <li>WebAssembly (WASM) support</li>
                </ul>

                <h3>Audio Engines</h3>
                <table>
                    <thead><tr><th>Engine</th><th>Emulation</th><th>Technology</th></tr></thead>
                    <tbody>
                        <tr><td>DRUM</td><td>Roland TR-808 / TR-909</td><td>Rust → WASM (tr808.wasm)</td></tr>
                        <tr><td>PAD</td><td>Sequential Prophet-5</td><td>Rust → WASM (prophet-dsp.wasm)</td></tr>
                        <tr><td>ACID</td><td>Roland TB-303</td><td>Rust → WASM (tb303.wasm)</td></tr>
                        <tr><td>LEAD</td><td>Mutable Instruments Braids</td><td>Rust → WASM (braids-dsp.wasm)</td></tr>
                    </tbody>
                </table>

                <h3>Default Settings</h3>
                <table>
                    <thead><tr><th>Setting</th><th>Default</th><th>Range</th></tr></thead>
                    <tbody>
                        <tr><td>BPM</td><td>120</td><td>60–200</td></tr>
                        <tr><td>Channel Volume</td><td>75</td><td>0–100</td></tr>
                        <tr><td>Pan</td><td>Centre (0)</td><td>L100 – R100</td></tr>
                        <tr><td>Drum Engine</td><td>808</td><td>808 / 909</td></tr>
                        <tr><td>Pad Preset</td><td>Warm Pad</td><td>14 presets</td></tr>
                        <tr><td>Acid Waveform</td><td>Sawtooth</td><td>SAW / SQR</td></tr>
                        <tr><td>Lead Model</td><td>SAW</td><td>13 models</td></tr>
                    </tbody>
                </table>

                <h3>How It Works</h3>
                <p>
                    ORBIT's audio engines are written in <strong>Rust</strong> and compiled to
                    <strong>WebAssembly</strong> for near-native performance in the browser. Each
                    engine runs inside an <strong>AudioWorklet</strong> on a dedicated real-time
                    audio thread, ensuring low-latency, glitch-free playback. The user interface
                    is built with <strong>Svelte 5</strong> and communicates with the WASM engines
                    via message ports.
                </p>
                <p>
                    No audio data is sent to a server — everything is synthesised locally on your
                    device. The app works fully offline once loaded.
                </p>

                <h3>Troubleshooting</h3>
                <ul>
                    <li><strong>No sound?</strong> Make sure your device volume is up and no channels are muted in the MIX panel. Some browsers require the initial "TAP TO START" interaction to enable audio.</li>
                    <li><strong>Engine won't load?</strong> Try a hard refresh (Ctrl+Shift+R / Cmd+Shift+R) to clear cached files.</li>
                    <li><strong>Crackling audio?</strong> Close other browser tabs to free up CPU. Chrome generally offers the best performance.</li>
                    <li><strong>Controls unresponsive?</strong> Ensure you've tapped "TAP TO START" on the splash screen first.</li>
                </ul>
            </div>
        {/if}
    </section>

    <footer class="help-footer">
        <p>ORBIT by Hudsonic — Browser-based music production</p>
    </footer>
</div>

<style>
    .help-panel {
        height: 100dvh;
        overflow-y: auto;
        padding: 24px 20px 48px;
        padding-top: calc(24px + env(safe-area-inset-top, 0px));
        padding-bottom: calc(48px + env(safe-area-inset-bottom, 0px));
        max-width: 640px;
        margin: 0 auto;
        -webkit-overflow-scrolling: touch;
    }

    .back-btn {
        display: inline-block;
        padding: 6px 16px;
        font-family: 'JetBrains Mono', monospace;
        font-size: 11px;
        font-weight: 500;
        letter-spacing: 2px;
        text-transform: uppercase;
        background: transparent;
        color: var(--orbit-ink, #eee);
        border: 1.5px solid var(--orbit-border, #333);
        border-radius: 12px;
        cursor: pointer;
        margin-bottom: 24px;
        transition: all 120ms cubic-bezier(0.2, 0.8, 0.3, 1);
    }
    .back-btn:hover {
        background: var(--orbit-ink, #eee);
        color: var(--orbit-surface, #111);
    }

    .help-header {
        text-align: center;
        margin-bottom: 32px;
    }
    .help-brand {
        font-size: 11px;
        font-weight: 300;
        letter-spacing: 6px;
        text-transform: uppercase;
        color: var(--orbit-hint, #666);
    }
    .help-title {
        font-size: 36px;
        font-weight: 500;
        letter-spacing: 10px;
        margin: 4px 0 8px;
        color: var(--orbit-ink, #eee);
    }
    .help-subtitle {
        font-size: 12px;
        font-weight: 400;
        letter-spacing: 3px;
        text-transform: uppercase;
        color: var(--orbit-label, #aaa);
        margin: 0;
    }

    /* Table of Contents */
    .toc {
        background: var(--orbit-well, #1a1a1a);
        border: 1px solid var(--orbit-border, #333);
        border-radius: 12px;
        padding: 16px 20px;
        margin-bottom: 24px;
    }
    .toc-title {
        font-size: 10px;
        font-weight: 500;
        letter-spacing: 2px;
        text-transform: uppercase;
        color: var(--orbit-hint, #666);
        margin-bottom: 12px;
    }
    .toc-list {
        margin: 0;
        padding-left: 20px;
        display: flex;
        flex-direction: column;
        gap: 6px;
    }
    .toc-link {
        font-size: 12px;
        color: var(--orbit-ink, #eee);
        text-decoration: none;
        transition: color 120ms;
        background: none;
        border: none;
        font-family: 'JetBrains Mono', monospace;
        cursor: pointer;
        padding: 0;
        text-align: left;
    }
    .toc-link:hover {
        color: #fff;
    }

    /* Sections */
    .section {
        margin-bottom: 8px;
        border: 1px solid var(--orbit-border, #333);
        border-radius: 12px;
        overflow: hidden;
        scroll-margin-top: 16px;
    }
    .section-header {
        width: 100%;
        display: flex;
        align-items: center;
        gap: 12px;
        padding: 14px 16px;
        background: var(--orbit-well, #1a1a1a);
        border: none;
        cursor: pointer;
        font-family: 'JetBrains Mono', monospace;
        transition: background 120ms;
    }
    .section-header:hover {
        background: var(--orbit-well-bright, #262626);
    }
    .section-number {
        font-size: 10px;
        font-weight: 500;
        color: var(--orbit-hint, #666);
        letter-spacing: 1px;
        min-width: 20px;
    }
    .section-title {
        font-size: 13px;
        font-weight: 500;
        letter-spacing: 1px;
        color: var(--orbit-ink, #eee);
        flex: 1;
        text-align: left;
    }
    .chevron {
        font-size: 12px;
        color: var(--orbit-hint, #666);
        transition: transform 200ms cubic-bezier(0.2, 0.8, 0.3, 1);
        display: inline-block;
    }
    .chevron.open {
        transform: rotate(90deg);
    }

    .section-body {
        padding: 16px 20px 20px;
        background: var(--orbit-surface, #111);
    }
    .section-body h3 {
        font-size: 12px;
        font-weight: 500;
        letter-spacing: 1.5px;
        text-transform: uppercase;
        color: var(--orbit-ink, #eee);
        margin: 20px 0 8px;
    }
    .section-body h3:first-child {
        margin-top: 0;
    }
    .section-body h4 {
        font-size: 11px;
        font-weight: 500;
        letter-spacing: 1px;
        color: var(--orbit-label, #aaa);
        margin: 16px 0 8px;
    }
    .section-body p {
        font-size: 12px;
        line-height: 1.7;
        color: var(--orbit-label, #aaa);
        margin: 0 0 10px;
    }
    .section-body ul, .section-body ol {
        font-size: 12px;
        line-height: 1.7;
        color: var(--orbit-label, #aaa);
        padding-left: 20px;
        margin: 0 0 10px;
    }
    .section-body li {
        margin-bottom: 4px;
    }
    .section-body strong {
        color: var(--orbit-ink, #eee);
        font-weight: 500;
    }
    .section-body em {
        color: var(--orbit-label, #aaa);
        font-style: italic;
    }

    /* Tables */
    table {
        width: 100%;
        border-collapse: collapse;
        margin: 8px 0 12px;
        font-size: 11px;
    }
    thead th {
        text-align: left;
        padding: 6px 8px;
        font-size: 10px;
        font-weight: 500;
        letter-spacing: 1px;
        text-transform: uppercase;
        color: var(--orbit-hint, #666);
        border-bottom: 1px solid var(--orbit-border, #333);
    }
    tbody td {
        padding: 6px 8px;
        color: var(--orbit-label, #aaa);
        border-bottom: 1px solid rgba(51, 51, 51, 0.5);
        vertical-align: top;
    }

    .colour-swatch {
        display: inline-block;
        width: 10px;
        height: 10px;
        border-radius: 50%;
        vertical-align: middle;
        margin-right: 4px;
    }

    /* Card grid */
    .card-grid {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 8px;
        margin: 12px 0;
    }
    .card {
        padding: 12px;
        border-radius: 8px;
        border: 1px solid color-mix(in srgb, var(--card-accent) 30%, transparent);
        background: color-mix(in srgb, var(--card-accent) 8%, transparent);
    }
    .card-label {
        font-size: 11px;
        font-weight: 500;
        letter-spacing: 2px;
        color: var(--card-accent);
        margin-bottom: 4px;
    }
    .card-desc {
        font-size: 11px;
        line-height: 1.5;
        color: var(--orbit-label, #aaa);
    }

    /* Preset grid */
    .preset-grid {
        display: flex;
        flex-wrap: wrap;
        gap: 6px;
        margin: 8px 0 12px;
    }
    .preset-tag {
        padding: 3px 10px;
        font-size: 10px;
        font-weight: 400;
        letter-spacing: 0.5px;
        color: var(--orbit-label, #aaa);
        background: var(--orbit-well, #1a1a1a);
        border: 1px solid var(--orbit-border, #333);
        border-radius: 10px;
    }

    /* Inline link */
    .inline-link {
        color: var(--orbit-ink, #eee);
        text-decoration: underline;
        text-underline-offset: 2px;
    }

    /* Footer */
    .help-footer {
        text-align: center;
        margin-top: 32px;
        padding-top: 16px;
        border-top: 1px solid var(--orbit-border, #333);
    }
    .help-footer p {
        font-size: 10px;
        letter-spacing: 1px;
        color: var(--orbit-hint, #666);
        margin: 0;
    }
</style>
