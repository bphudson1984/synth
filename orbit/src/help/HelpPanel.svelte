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
            <li><button class="toc-link" onclick={() => { const el = document.getElementById('bass'); if (el) el.scrollIntoView({ behavior: 'smooth' }); if (!isOpen('bass')) toggleSection('bass'); }}>Bass Synth</button></li>
            <li><button class="toc-link" onclick={() => { const el = document.getElementById('sampler'); if (el) el.scrollIntoView({ behavior: 'smooth' }); if (!isOpen('sampler')) toggleSection('sampler'); }}>Sampler</button></li>
            <li><button class="toc-link" onclick={() => { const el = document.getElementById('mix'); if (el) el.scrollIntoView({ behavior: 'smooth' }); if (!isOpen('mix')) toggleSection('mix'); }}>Mixer</button></li>
            <li><button class="toc-link" onclick={() => { const el = document.getElementById('fx'); if (el) el.scrollIntoView({ behavior: 'smooth' }); if (!isOpen('fx')) toggleSection('fx'); }}>FX Rack</button></li>
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
                    ORBIT is a browser-based music production workstation that combines six
                    classic synthesiser, drum machine, and sampler emulations into a single app
                    with a shared FX rack. Everything runs locally in your browser using
                    WebAssembly — no server, no account, no internet required after the initial load.
                </p>

                <h3>The Six Engines</h3>
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
                    <div class="card" style="--card-accent: #D4A843">
                        <div class="card-label">BASS</div>
                        <div class="card-desc">Korg Volca Bass with 3 VCOs, diode bridge filter, and LFO</div>
                    </div>
                    <div class="card" style="--card-accent: #E05555">
                        <div class="card-label">SAMPLE</div>
                        <div class="card-desc">SP-404MK2 inspired sampler with 16 pads, vocoder, and bit crush</div>
                    </div>
                </div>

                <h3>First Launch</h3>
                <ol>
                    <li>Open ORBIT in a modern browser (Chrome, Firefox, or Safari).</li>
                    <li>Tap <strong>TAP TO START</strong> on the splash screen. This initialises all six audio engines — it may take a moment.</li>
                    <li>You arrive on the <strong>DRUM</strong> tab. Use the top navigation to switch between DRUM, PAD, ACID, LEAD, BASS, SAMPLE, MIX, and FX.</li>
                    <li>Hit <strong>Play ▶</strong> in any panel to start the transport. All engines share a single global clock.</li>
                </ol>

                <h3>Navigation</h3>
                <p>
                    The tab bar at the top of the screen lets you switch panels instantly. The tabs are
                    <strong>DRUM</strong>, <strong>PAD</strong>, <strong>ACID</strong>, <strong>LEAD</strong>,
                    <strong>BASS</strong>, <strong>SAMPLE</strong>, <strong>MIX</strong>, and <strong>FX</strong>.
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
                    the Pad, Acid, Lead, Bass, and Sampler sequencers.
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
    <!-- BASS SYNTH -->
    <section id="bass" class="section">
        <button class="section-header" onclick={() => toggleSection('bass')}>
            <span class="section-number">06</span>
            <span class="section-title">Bass Synth</span>
            <span class="chevron" class:open={isOpen('bass')}>▸</span>
        </button>
        {#if isOpen('bass')}
            <div class="section-body">
                <p>
                    The Bass panel is a <strong>Korg Volca Bass</strong> inspired analogue-modelling
                    synthesiser with three independent VCOs, a diode bridge lowpass filter, and
                    a free-running LFO. It excels at deep subs, acid lines, and evolving bass textures.
                </p>

                <h3>3-VCO Architecture</h3>
                <p>
                    Each of the three voltage-controlled oscillators can be set independently:
                </p>
                <table>
                    <thead><tr><th>Control</th><th>Per VCO</th><th>Description</th></tr></thead>
                    <tbody>
                        <tr><td>WAVE</td><td>✓</td><td>SAW (sawtooth) or SQR (square 50% duty) — both PolyBLEP antialiased</td></tr>
                        <tr><td>PITCH</td><td>✓</td><td>Semitone offset from base note (−12 to +12)</td></tr>
                        <tr><td>ACTIVE</td><td>✓</td><td>Enable or disable the oscillator in the mix</td></tr>
                    </tbody>
                </table>
                <p>
                    VCO 2 and VCO 3 default to slight detune (±0.1 semitones) for a warm unison
                    effect. Disable individual oscillators to thin the sound, or stack all three
                    with pitch offsets for power chords and octave layers.
                </p>

                <h3>Group Mode</h3>
                <ul>
                    <li><strong>ALL (Unison)</strong>: All three VCOs play the same note — thick, monophonic bass.</li>
                    <li><strong>POLY (Paraphonic)</strong>: Each VCO plays a different note via round-robin — up to 3 simultaneous pitches sharing one filter.</li>
                </ul>

                <h3>Filter</h3>
                <p>
                    A <strong>diode bridge lowpass filter</strong> inspired by the Korg MiniKorg 700S.
                    It is a 2-pole (12 dB/oct) design with nonlinear saturation — louder signals
                    naturally push the cutoff lower, creating a breathing, organic quality.
                </p>
                <table>
                    <thead><tr><th>Parameter</th><th>Range</th><th>Description</th></tr></thead>
                    <tbody>
                        <tr><td>CUTOFF</td><td>20–12,000 Hz</td><td>Filter frequency</td></tr>
                        <tr><td>PEAK</td><td>0–1.0</td><td>Resonance — above 0.95 the filter self-oscillates</td></tr>
                        <tr><td>EG INT</td><td>0–1.0</td><td>How much the envelope sweeps the cutoff</td></tr>
                    </tbody>
                </table>

                <h3>Envelope</h3>
                <table>
                    <thead><tr><th>Parameter</th><th>Range</th><th>Description</th></tr></thead>
                    <tbody>
                        <tr><td>ATTACK</td><td>0.5 ms – 10 s</td><td>Time from gate-on to peak</td></tr>
                        <tr><td>DEC/REL</td><td>10 ms – 10 s</td><td>Shared decay and release time</td></tr>
                        <tr><td>SUSTAIN</td><td>ON / OFF</td><td>ON = hold at peak (ASR), OFF = decay to zero (AD)</td></tr>
                        <tr><td>EG→VCA</td><td>ON / OFF</td><td>Route envelope to amplitude — disable for drone-like sustain</td></tr>
                    </tbody>
                </table>

                <h3>LFO</h3>
                <table>
                    <thead><tr><th>Parameter</th><th>Range</th><th>Description</th></tr></thead>
                    <tbody>
                        <tr><td>RATE</td><td>0.1–30 Hz</td><td>Oscillation speed</td></tr>
                        <tr><td>INT</td><td>0–1.0</td><td>Modulation depth</td></tr>
                        <tr><td>WAVE</td><td>TRI / SQR</td><td>Triangle or square LFO shape</td></tr>
                        <tr><td>→PITCH</td><td>ON / OFF</td><td>Modulate VCO pitch (vibrato)</td></tr>
                        <tr><td>→CUTOFF</td><td>ON / OFF</td><td>Modulate filter cutoff (wah)</td></tr>
                        <tr><td>→AMP</td><td>ON / OFF</td><td>Modulate volume (tremolo)</td></tr>
                    </tbody>
                </table>

                <h3>Legato &amp; Glide</h3>
                <p>
                    When a new note starts while the previous gate is still open, the VCOs
                    <strong>glide smoothly</strong> to the new pitch instead of retriggering the
                    envelope. This is perfect for expressive acid slides and smooth bass runs.
                </p>

                <h3>Note Pads</h3>
                <p>
                    Eight pads in the <strong>C natural minor scale</strong> (C, D, Eb, F, G, Ab, Bb, C')
                    starting at MIDI note 36 (C2). Tap to trigger notes live or record into the sequencer.
                </p>

                <h3>Presets</h3>
                <p>
                    <strong>10 sound presets</strong> set all synth parameters, and <strong>16 pattern
                    presets</strong> load ready-made bass sequences:
                </p>
                <h4>Sound Presets</h4>
                <div class="preset-grid">
                    <span class="preset-tag">Warm Unison</span>
                    <span class="preset-tag">Sub Weight</span>
                    <span class="preset-tag">Acid Bass</span>
                    <span class="preset-tag">Deep Throb</span>
                    <span class="preset-tag">Growl</span>
                    <span class="preset-tag">Pulse Pad</span>
                    <span class="preset-tag">Squelch Box</span>
                    <span class="preset-tag">Fifth Stack</span>
                    <span class="preset-tag">Haunted</span>
                    <span class="preset-tag">Motorik</span>
                </div>
                <h4>Pattern Presets</h4>
                <div class="preset-grid">
                    <span class="preset-tag">House Foundation</span>
                    <span class="preset-tag">Sub Weight</span>
                    <span class="preset-tag">Dub Pressure</span>
                    <span class="preset-tag">Techno Pulse</span>
                    <span class="preset-tag">Motorik</span>
                    <span class="preset-tag">Syncopated Funk</span>
                    <span class="preset-tag">Acid Line</span>
                    <span class="preset-tag">Squelch</span>
                    <span class="preset-tag">Minor Walk</span>
                    <span class="preset-tag">Octave Jump</span>
                    <span class="preset-tag">Chromatic Descent</span>
                    <span class="preset-tag">Unison Throb</span>
                    <span class="preset-tag">Fifth Groove</span>
                    <span class="preset-tag">Haunted</span>
                    <span class="preset-tag">Growl Machine</span>
                    <span class="preset-tag">Off Beat</span>
                </div>
            </div>
        {/if}
    </section>
    <!-- SAMPLER -->
    <section id="sampler" class="section">
        <button class="section-header" onclick={() => toggleSection('sampler')}>
            <span class="section-number">07</span>
            <span class="section-title">Sampler</span>
            <span class="chevron" class:open={isOpen('sampler')}>▸</span>
        </button>
        {#if isOpen('sampler')}
            <div class="section-body">
                <p>
                    The Sample panel is an <strong>SP-404MK2 inspired sampler</strong> with 16 trigger
                    pads, 32-voice polyphony, per-pad DSP effects (vocoder, bit crush), interactive
                    waveform slicing, and full sequencer integration.
                </p>

                <h3>Loading Samples</h3>
                <ul>
                    <li><strong>LOAD button</strong>: Opens a file picker — select any audio file (MP3, WAV, etc.) to load onto the selected pad.</li>
                    <li><strong>Drag &amp; drop</strong>: Drop an audio file directly onto a pad to load it.</li>
                    <li><strong>STOCK KIT</strong>: Loads a built-in kit of 12 drum and melodic samples (kicks, snares, hi-hats, claps, toms, and Casio keys) onto pads 1–12.</li>
                    <li><strong>REC button</strong>: Record from your microphone directly to the selected pad.</li>
                </ul>
                <p>
                    Samples are automatically saved to your browser's local storage and restored
                    when you reopen ORBIT.
                </p>

                <h3>The 16 Pads</h3>
                <p>
                    Sixteen pads are arranged in a circle, mapped to MIDI notes 36–51. Tap a pad
                    to select it and trigger playback. Loaded pads glow red; empty pads appear grey.
                    The pad label and sample name are shown above the waveform display.
                </p>

                <h3>Play Modes</h3>
                <table>
                    <thead><tr><th>Mode</th><th>Behaviour</th></tr></thead>
                    <tbody>
                        <tr><td>ONE-SHOT</td><td>Plays sample to the end regardless of gate length (default)</td></tr>
                        <tr><td>GATE</td><td>Plays while held — releases on note-off with configurable release fade</td></tr>
                        <tr><td>LOOP</td><td>Loops continuously between the start and end points until stopped</td></tr>
                    </tbody>
                </table>

                <h3>Choke Groups</h3>
                <p>
                    Assign pads to one of <strong>4 choke groups</strong> (or OFF). When a pad in a
                    choke group triggers, all other voices in the same group are released — perfect
                    for open/closed hi-hat pairs or one-at-a-time kicks.
                </p>

                <h3>Per-Pad Parameters</h3>
                <p>Tap <strong>SETTINGS</strong> to reveal parameter controls for the selected pad:</p>

                <h4>Level</h4>
                <table>
                    <thead><tr><th>Parameter</th><th>Range</th><th>Description</th></tr></thead>
                    <tbody>
                        <tr><td>VOLUME</td><td>0–1.0</td><td>Output level</td></tr>
                        <tr><td>PAN</td><td>L100 – R100</td><td>Stereo position</td></tr>
                        <tr><td>BIT CRUSH</td><td>1–16 bits</td><td>Reduces audio fidelity — 16 = clean, lower values add lo-fi crunch</td></tr>
                    </tbody>
                </table>

                <h4>Pitch</h4>
                <table>
                    <thead><tr><th>Parameter</th><th>Range</th><th>Description</th></tr></thead>
                    <tbody>
                        <tr><td>PITCH</td><td>−24 to +24 semitones</td><td>Transpose the sample up or down</td></tr>
                    </tbody>
                </table>

                <h4>Envelope</h4>
                <table>
                    <thead><tr><th>Parameter</th><th>Range</th><th>Description</th></tr></thead>
                    <tbody>
                        <tr><td>ATTACK</td><td>0–2 s</td><td>Fade-in time</td></tr>
                        <tr><td>RELEASE</td><td>0–2 s</td><td>Fade-out time (GATE and LOOP modes)</td></tr>
                    </tbody>
                </table>

                <h3>Waveform Slicing</h3>
                <p>
                    The waveform display shows the loaded sample with draggable
                    <strong style="color:#4CAF50">green (START)</strong> and
                    <strong style="color:#F44336">red (END)</strong> handles. Drag to select which
                    portion of the sample plays. Regions outside the slice are dimmed. Looping,
                    reverse, and pitch shifting all respect the slice boundaries.
                </p>

                <h3>Vocoder</h3>
                <p>
                    Each pad has a built-in <strong>channel vocoder</strong> that analyses the spectral
                    envelope of the sample and applies it to an internal carrier oscillator — making
                    the synth "speak" with the sample's character.
                </p>
                <table>
                    <thead><tr><th>Parameter</th><th>Range</th><th>Description</th></tr></thead>
                    <tbody>
                        <tr><td>ENABLE</td><td>ON / OFF</td><td>Activate vocoder processing</td></tr>
                        <tr><td>ROOT</td><td>MIDI 24–84</td><td>Reference pitch the sample was recorded at</td></tr>
                        <tr><td>CARRIER</td><td>SAW / SQR / NOISE</td><td>Carrier oscillator waveform</td></tr>
                        <tr><td>BANDS</td><td>4–16</td><td>Number of analysis bands (more = higher fidelity)</td></tr>
                        <tr><td>FORMANT</td><td>−12 to +12</td><td>Shift spectral envelope up or down (semitones)</td></tr>
                        <tr><td>MIX</td><td>0–1.0</td><td>Dry/wet blend — 0 = original sample, 1 = full vocoder</td></tr>
                    </tbody>
                </table>

                <h3>Reverse &amp; Other Controls</h3>
                <ul>
                    <li><strong>REVERSE</strong>: Toggle to play the sample backwards.</li>
                    <li><strong>CHOKE</strong>: Select choke group (OFF, 1, 2, 3, or 4).</li>
                    <li><strong>MODE</strong>: Choose ONE-SHOT, GATE, or LOOP playback.</li>
                </ul>

                <h3>32-Voice Polyphony</h3>
                <p>
                    Up to 32 simultaneous voices can play at once across all 16 pads. When the
                    voice pool is full, the oldest voice is stolen. Each voice has independent
                    pitch, envelope, vocoder, and panning — so layering multiple pads produces
                    rich, full arrangements.
                </p>
            </div>
        {/if}
    </section>
    <!-- MIXER -->
    <section id="mix" class="section">
        <button class="section-header" onclick={() => toggleSection('mix')}>
            <span class="section-number">08</span>
            <span class="section-title">Mixer</span>
            <span class="chevron" class:open={isOpen('mix')}>▸</span>
        </button>
        {#if isOpen('mix')}
            <div class="section-body">
                <p>
                    The Mix panel gives you master control over all six engines with per-channel
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
                        <tr><td>BASS</td><td><span class="colour-swatch" style="background:#D4A843"></span> Gold</td><td>75</td></tr>
                        <tr><td>SAMPLE</td><td><span class="colour-swatch" style="background:#E05555"></span> Red</td><td>75</td></tr>
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

    <!-- FX RACK -->
    <section id="fx" class="section">
        <button class="section-header" onclick={() => toggleSection('fx')}>
            <span class="section-number">09</span>
            <span class="section-title">FX Rack</span>
            <span class="chevron" class:open={isOpen('fx')}>▸</span>
        </button>
        {#if isOpen('fx')}
            <div class="section-body">
                <p>
                    The FX panel is a <strong>shared effects rack</strong> with four independent effect
                    processors. Every engine can send audio to each effect at an independent level,
                    so you can apply different amounts of reverb, delay, chorus, and distortion to
                    each instrument.
                </p>

                <h3>Effects</h3>
                <table>
                    <thead><tr><th>Effect</th><th>Parameters</th><th>Description</th></tr></thead>
                    <tbody>
                        <tr><td>CHORUS</td><td>RATE (0.1–5 Hz), DEPTH (0–1)</td><td>Stereo chorus for width and movement</td></tr>
                        <tr><td>DELAY</td><td>TIME (1–1000 ms), FEEDBACK (0–0.95), TONE (0–1)</td><td>Tempo-synced echo with tone-filtered repeats</td></tr>
                        <tr><td>REVERB</td><td>DECAY (0–0.99), DAMPING (0–1)</td><td>Algorithmic reverb for room and space</td></tr>
                        <tr><td>DISTORTION</td><td>DRIVE (0–1), TONE (0–1), LEVEL (0–1)</td><td>Waveshaping overdrive with tone control</td></tr>
                    </tbody>
                </table>

                <h3>Send Levels</h3>
                <p>
                    Each effect tab shows six send faders — one per engine. Raise a fader to send
                    more of that engine's audio into the effect:
                </p>
                <table>
                    <thead><tr><th>Send</th><th>Colour</th><th>Engine</th></tr></thead>
                    <tbody>
                        <tr><td>DR</td><td><span class="colour-swatch" style="background:#378ADD"></span> Blue</td><td>Drum</td></tr>
                        <tr><td>BS</td><td><span class="colour-swatch" style="background:#D4A843"></span> Gold</td><td>Bass</td></tr>
                        <tr><td>PD</td><td><span class="colour-swatch" style="background:#E8944A"></span> Orange</td><td>Pad</td></tr>
                        <tr><td>AC</td><td><span class="colour-swatch" style="background:#5DBE6E"></span> Green</td><td>Acid</td></tr>
                        <tr><td>LD</td><td><span class="colour-swatch" style="background:#B56ECC"></span> Purple</td><td>Lead</td></tr>
                        <tr><td>SM</td><td><span class="colour-swatch" style="background:#E05555"></span> Red</td><td>Sampler</td></tr>
                    </tbody>
                </table>
                <p>
                    All send levels default to 0 (dry). Raise the faders to taste — a little reverb
                    on the pad synth and some delay on the lead is a great starting point.
                </p>
            </div>
        {/if}
    </section>

    <!-- SEQUENCER & RECORDING -->
    <section id="sequencer" class="section">
        <button class="section-header" onclick={() => toggleSection('sequencer')}>
            <span class="section-number">10</span>
            <span class="section-title">Sequencer &amp; Recording</span>
            <span class="chevron" class:open={isOpen('sequencer')}>▸</span>
        </button>
        {#if isOpen('sequencer')}
            <div class="section-body">
                <p>
                    The <strong>PAD</strong>, <strong>LEAD</strong>, <strong>BASS</strong>, and
                    <strong>SAMPLE</strong> panels share a powerful multi-page note sequencer. Up to
                    <strong>8 pages × 16 steps = 128 steps</strong> can be programmed per instrument.
                    Each engine also supports <strong>sequence banks</strong> with up to 8 variations
                    that can be chained or randomised.
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

                <h3>Sequence Banks &amp; Chaining</h3>
                <p>
                    Each engine can store up to <strong>8 sequence variations</strong> in a bank.
                    The bank selector appears below the sequencer grid with numbered buttons
                    for each slot. Controls include:
                </p>
                <ul>
                    <li><strong>+</strong>: Create a new empty sequence (up to 8 max).</li>
                    <li><strong>DUP</strong>: Duplicate the current sequence into a new slot — useful for creating variations.</li>
                    <li><strong>DEL</strong>: Delete the current sequence (at least 1 must remain).</li>
                    <li><strong>CHAIN</strong>: When active, the sequencer automatically advances to the next sequence when the current one ends (1→2→3→…→1). Mutually exclusive with RND.</li>
                    <li><strong>RND</strong>: When active, a random sequence is selected at the end of each loop (never the same one twice in a row). Mutually exclusive with CHAIN.</li>
                </ul>
                <p>
                    Combine chaining with multi-page sequences to build long, evolving song structures
                    — or use random mode for generative, never-repeating performances.
                </p>

                <h3>Glitch Mode</h3>
                <p>
                    Every panel includes a <strong>spring-loaded glitch slider</strong> in the play
                    controls area. Press and hold one of the four buttons to stutter-repeat the
                    last few steps in real time:
                </p>
                <ul>
                    <li><strong>8</strong>: Repeats the last 1/8 of a step — fastest stutter</li>
                    <li><strong>4</strong>: Repeats the last 1/4 of a step</li>
                    <li><strong>2</strong>: Repeats the last 1/2 of a step</li>
                    <li><strong>1</strong>: Repeats the last full step — slowest stutter</li>
                </ul>
                <p>
                    The glitch is <strong>spring-loaded</strong> — release your finger and it snaps
                    back to normal playback instantly. Drag between buttons while holding to change
                    the repeat size on the fly. The MIX panel also has a global glitch that affects
                    all engines at once.
                </p>

                <h3>Synth Settings &amp; Quick Slots</h3>
                <p>
                    The <strong>PAD</strong>, <strong>LEAD</strong>, <strong>ACID</strong>, and
                    <strong>BASS</strong> panels each have a <strong>SETTINGS</strong> button that
                    opens a deep-edit panel with full access to every synth parameter — oscillators,
                    filters, envelopes, LFOs, modulation routing, and more.
                </p>
                <p>
                    Each settings panel also offers <strong>8 assignable quick slots</strong>. Tap a
                    numbered button (1–8) next to any slider parameter to assign it to that slot.
                    Once assigned, select the slot and use the quick-access slider at the bottom of
                    the panel to adjust the parameter in real time during playback — perfect for
                    live performance tweaking without opening the full settings panel.
                </p>
            </div>
        {/if}
    </section>
    <!-- TIPS & TRICKS -->
    <section id="tips" class="section">
        <button class="section-header" onclick={() => toggleSection('tips')}>
            <span class="section-number">11</span>
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
                    <li>Add a <strong>bass foundation</strong> on the BASS tab — use all 3 VCOs detuned for thickness.</li>
                    <li>Drop in <strong>samples</strong> on the SAMPLE tab — load a stock kit or your own sounds.</li>
                    <li>Open the <strong>MIX</strong> tab to balance volumes, pan channels, and use mute/solo to focus.</li>
                    <li>Add <strong>FX</strong> — send some reverb to the pad, delay to the lead, and distortion to the bass.</li>
                    <li>Press play and everything runs in sync!</li>
                </ol>

                <h3>Sound Design</h3>
                <ul>
                    <li><strong>Acid wobbles</strong>: Crank the resonance and envelope mod on the ACID panel, then sweep the cutoff for classic 303 squelch.</li>
                    <li><strong>Lush pads</strong>: On the PAD panel, use a slow attack (60+) and long release (70+) with moderate cutoff. Try the "Blade Runner" or "In The Air" presets as a starting point.</li>
                    <li><strong>Plucked leads</strong>: Set the LEAD model to PLUK with short release for staccato plucked-string sounds.</li>
                    <li><strong>Thick swarms</strong>: Use the SWARM model on the LEAD panel and adjust Timbre to control the number of detuned voices.</li>
                    <li><strong>808 vs 909</strong>: Double-tap individual drum pads to mix engines — try a 909 kick with 808 hi-hats.</li>
                    <li><strong>Deep sub bass</strong>: On the BASS panel, try the "Sub Weight" preset — one square-wave VCO with low cutoff for pure sub.</li>
                    <li><strong>Paraphonic bass chords</strong>: Switch BASS group mode to POLY and detune VCOs to different semitones for bass chords through one filter.</li>
                    <li><strong>Lo-fi samples</strong>: On the SAMPLE panel, reduce BIT CRUSH to 4–8 bits for vintage digital crunch. Combine with pitch shifting for lo-fi textures.</li>
                    <li><strong>Vocal samples</strong>: Enable the vocoder on a pad, load a vocal sample, and play pitched notes through it for robotic vocal effects.</li>
                </ul>

                <h3>Sequencer Techniques</h3>
                <ul>
                    <li><strong>Probability</strong>: Set some steps to 50–80% probability for organic, ever-changing patterns.</li>
                    <li><strong>Ratchets</strong>: Use 2× or 3× ratchet on hi-hats or snares for instant drum fills.</li>
                    <li><strong>Swing</strong>: Add 20–40% swing for a looser, groovier feel.</li>
                    <li><strong>Ping-pong direction</strong>: Set direction to PING for melodies that bounce back and forth.</li>
                    <li><strong>Random direction</strong>: Set direction to RND for generative, ever-evolving sequences.</li>
                    <li><strong>Step entry</strong>: Stop playback and use the pads to record one step at a time for precise programming.</li>
                    <li><strong>Sequence chaining</strong>: Create 2–4 variations of a pattern in the bank, then enable CHAIN for an evolving song structure.</li>
                    <li><strong>Random sequences</strong>: Enable RND on the sequence bank for generative, non-repeating arrangements.</li>
                    <li><strong>Glitch fills</strong>: Hold the glitch button (8 or 4) right before a drop for instant build-up tension, then release for the payoff.</li>
                </ul>

                <h3>FX Tips</h3>
                <ul>
                    <li><strong>Reverb on pads</strong>: A medium reverb send on the PAD channel adds depth and space to chord stabs.</li>
                    <li><strong>Delay on lead</strong>: Synced delay on the LEAD channel creates rhythmic echoes that fill out the mix.</li>
                    <li><strong>Distortion on bass</strong>: A touch of distortion on the BASS or ACID channel adds presence and grit.</li>
                    <li><strong>Chorus on everything</strong>: Subtle chorus sends across multiple channels widen the stereo image.</li>
                </ul>

                <h3>Performance</h3>
                <ul>
                    <li><strong>Real-time tweaking</strong>: All parameter sliders respond instantly during playback — try sweeping the acid cutoff or pad filter live.</li>
                    <li><strong>Latch + Arp</strong>: Enable both latch and arpeggiator on the LEAD panel for hands-free evolving arpeggios.</li>
                    <li><strong>Mute/Solo</strong>: Use the MIX panel's mute and solo buttons to create breakdowns and build-ups.</li>
                    <li><strong>Transpose</strong>: On the ACID panel, tap different note pads to transpose the bassline to match your chord progression in real time.</li>
                    <li><strong>Quick slots</strong>: Assign your most-used parameters to quick slots for one-slider live control without opening settings.</li>
                    <li><strong>Global glitch</strong>: Use the MIX panel's glitch slider to stutter all engines at once for dramatic performance effects.</li>
                </ul>
            </div>
        {/if}
    </section>

    <!-- TECHNICAL SPECS -->
    <section id="specs" class="section">
        <button class="section-header" onclick={() => toggleSection('specs')}>
            <span class="section-number">12</span>
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
                        <tr><td>BASS</td><td>Korg Volca Bass</td><td>Rust → WASM (volca-bass.wasm)</td></tr>
                        <tr><td>SAMPLE</td><td>Roland SP-404MK2 style</td><td>Rust → WASM (sampler.wasm)</td></tr>
                        <tr><td>FX</td><td>Shared effects rack</td><td>Rust → WASM (fx-rack.wasm)</td></tr>
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
                        <tr><td>Bass Preset</td><td>Warm Unison</td><td>10 sound + 16 pattern presets</td></tr>
                        <tr><td>Sampler</td><td>Empty (load via STOCK KIT)</td><td>16 pads</td></tr>
                        <tr><td>FX Sends</td><td>All at 0 (dry)</td><td>0–100 per engine per effect</td></tr>
                    </tbody>
                </table>

                <h3>How It Works</h3>
                <p>
                    ORBIT's audio engines are written in <strong>Rust</strong> and compiled to
                    <strong>WebAssembly</strong> for near-native performance in the browser. Each
                    engine runs inside an <strong>AudioWorklet</strong> on a dedicated real-time
                    audio thread, ensuring low-latency, glitch-free playback. A shared FX rack
                    processes all engine outputs through four independent effect buses. The user
                    interface is built with <strong>Svelte 5</strong> and communicates with the
                    WASM engines via message ports.
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
