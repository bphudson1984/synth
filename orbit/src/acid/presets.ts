export interface AcidStep {
    note: number; gate: boolean; accent: boolean; slide: boolean;
}
export interface AcidPreset {
    name: string;
    steps: AcidStep[];
    cutoff: number; resonance: number; envMod: number; decay: number;
    accent: number; dist: number; waveform: 'saw' | 'square';
}

function s(note: number, gate = true, accent = false, slide = false): AcidStep {
    return { note, gate, accent, slide };
}
const R: AcidStep = { note: 48, gate: false, accent: false, slide: false };

export const PRESETS: AcidPreset[] = [
    {
        name: 'Acid Tracks',
        steps: [
            s(47, true, true), s(47), s(36, true, true), s(48, true, true),
            s(36), s(51, true, false, true), s(51), s(48, true, true),
            s(47, true, true), s(47), s(36, true, true), s(48, true, true),
            s(36), s(51, true, false, true), s(51), s(48, true, true),
        ],
        cutoff: 25, resonance: 75, envMod: 80, decay: 20, accent: 70, dist: 35, waveform: 'saw',
    },
    {
        name: 'Higher State',
        steps: [
            s(48, true, true), s(51, true, false, true), s(48), s(55, true, true),
            s(48, true, false, true), s(51), s(48, true, true), R,
            s(48, true, true), s(51, true, false, true), s(48), s(55, true, true),
            s(48, true, false, true), s(51), s(48, true, true), s(53, true, false, true),
        ],
        cutoff: 20, resonance: 80, envMod: 85, decay: 15, accent: 75, dist: 40, waveform: 'saw',
    },
    {
        name: 'Squelch Machine',
        steps: [
            s(36, true, true), R, s(36), s(36, true, true),
            R, s(48, true, true, true), R, s(36),
            s(36, true, true), s(36), R, s(36, true, true),
            s(48, true, false, true), R, s(36), s(36, true, true),
        ],
        cutoff: 15, resonance: 85, envMod: 90, decay: 10, accent: 80, dist: 55, waveform: 'saw',
    },
    {
        name: 'Bassline Basic',
        steps: [
            s(48, true, true), s(48), s(48), s(51),
            s(48), s(48, true, true), R, s(53),
            s(48, true, true), s(48), s(48), s(51),
            s(48), s(48, true, true), s(55, true, false, true), s(53),
        ],
        cutoff: 35, resonance: 60, envMod: 65, decay: 30, accent: 50, dist: 20, waveform: 'saw',
    },
    {
        name: 'Slide City',
        steps: [
            s(48, true, false, true), s(51, true, false, true), s(55, true, false, true), s(48),
            s(53, true, true, true), s(48, true, false, true), s(51, true, false, true), s(55),
            s(48, true, false, true), s(51, true, false, true), s(55, true, false, true), s(48),
            s(53, true, true, true), s(55, true, false, true), s(51, true, false, true), s(48),
        ],
        cutoff: 30, resonance: 70, envMod: 75, decay: 25, accent: 60, dist: 30, waveform: 'saw',
    },
    {
        name: 'Square Acid',
        steps: [
            s(48, true, true), s(48), R, s(60, true, true),
            s(48), s(48, true, false, true), s(51, true, true), R,
            s(48, true, true), R, s(48), s(60, true, true, true),
            R, s(48), s(51, true, true), s(48, true, false, true),
        ],
        cutoff: 20, resonance: 75, envMod: 85, decay: 15, accent: 70, dist: 45, waveform: 'square',
    },
    // --- Classic acid house ---
    {
        name: 'Voodoo Ray',
        steps: [
            s(48, true, true), R, s(48), s(55, true, false, true),
            s(48, true, true), s(48), R, s(51, true, true, true),
            s(48), R, s(48, true, true), s(55, true, false, true),
            s(48), s(48, true, true), R, s(53, true, false, true),
        ],
        cutoff: 22, resonance: 78, envMod: 82, decay: 18, accent: 72, dist: 38, waveform: 'saw',
    },
    {
        name: 'Mentasm Stab',
        steps: [
            s(36, true, true), s(36, true, true), R, s(36),
            s(48, true, true, true), R, s(36, true, true), R,
            s(36, true, true), s(36), R, s(36, true, true),
            s(48, true, true, true), R, R, s(36),
        ],
        cutoff: 18, resonance: 88, envMod: 92, decay: 8, accent: 85, dist: 60, waveform: 'saw',
    },
    {
        name: 'Windowlicker',
        steps: [
            s(48, true, false, true), s(51, true, false, true), s(53, true, true, true), s(48),
            R, s(55, true, false, true), s(53, true, false, true), s(48, true, true),
            s(48, true, false, true), s(51, true, false, true), s(53, true, true, true), s(48),
            s(56, true, false, true), s(55, true, false, true), s(53, true, false, true), s(51),
        ],
        cutoff: 28, resonance: 72, envMod: 78, decay: 22, accent: 65, dist: 32, waveform: 'saw',
    },
    // --- Minimal & techno ---
    {
        name: 'Minimal Pulse',
        steps: [
            s(36, true, true), s(36), s(36), s(36),
            s(36, true, true), s(36), s(36), s(36),
            s(36, true, true), s(36), s(36), s(36),
            s(36, true, true), s(36), s(36), s(48, true, true, true),
        ],
        cutoff: 20, resonance: 70, envMod: 88, decay: 12, accent: 60, dist: 25, waveform: 'saw',
    },
    {
        name: 'Techno Driver',
        steps: [
            s(36, true, true), R, s(36), s(36, true, true),
            s(36), R, s(48, true, true), R,
            s(36, true, true), R, s(36), s(36, true, true),
            R, s(36), s(48, true, true, true), s(36),
        ],
        cutoff: 25, resonance: 65, envMod: 75, decay: 20, accent: 68, dist: 40, waveform: 'saw',
    },
    {
        name: 'Dark Warehouse',
        steps: [
            s(36, true, true), s(36), R, s(39, true, false, true),
            s(36), s(36, true, true), R, R,
            s(36, true, true), s(36), R, s(39, true, false, true),
            s(36), R, s(36, true, true), s(36),
        ],
        cutoff: 12, resonance: 82, envMod: 70, decay: 25, accent: 75, dist: 50, waveform: 'square',
    },
    // --- Hypnotic / trance ---
    {
        name: 'Hypnotic Roll',
        steps: [
            s(48), s(48, true, true), s(48), s(48, true, true),
            s(48), s(48, true, true), s(48), s(48, true, true),
            s(48), s(48, true, true), s(48), s(48, true, true),
            s(48), s(48, true, true), s(48), s(48, true, true),
        ],
        cutoff: 18, resonance: 90, envMod: 95, decay: 8, accent: 85, dist: 35, waveform: 'saw',
    },
    {
        name: 'Trance Gate',
        steps: [
            s(48, true, true), R, s(48), R,
            s(48, true, true), R, s(48), R,
            s(48, true, true), R, s(48), R,
            s(48, true, true), s(55, true, false, true), s(48), R,
        ],
        cutoff: 30, resonance: 68, envMod: 80, decay: 15, accent: 62, dist: 28, waveform: 'saw',
    },
    {
        name: 'Spiral',
        steps: [
            s(48, true, false, true), s(50, true, false, true), s(51, true, false, true), s(53, true, false, true),
            s(55, true, true, true), s(53, true, false, true), s(51, true, false, true), s(50, true, false, true),
            s(48, true, false, true), s(50, true, false, true), s(51, true, false, true), s(53, true, false, true),
            s(55, true, true, true), s(58, true, true, true), s(55, true, false, true), s(51, true, false, true),
        ],
        cutoff: 25, resonance: 75, envMod: 70, decay: 20, accent: 55, dist: 22, waveform: 'saw',
    },
    // --- Aggressive / industrial ---
    {
        name: 'Distorted Grind',
        steps: [
            s(36, true, true), s(36, true, true), R, s(36, true, true),
            R, s(36, true, true), s(36, true, true), R,
            s(36, true, true), R, s(36, true, true), s(36, true, true),
            R, s(48, true, true, true), R, s(36, true, true),
        ],
        cutoff: 10, resonance: 92, envMod: 95, decay: 5, accent: 90, dist: 75, waveform: 'saw',
    },
    {
        name: 'Buzz Saw',
        steps: [
            s(36, true, true), s(36), s(36, true, true), s(36),
            s(36, true, true), s(36), s(36, true, true), s(36),
            s(36, true, true), s(36), s(36, true, true), s(36),
            s(36, true, true), s(48, true, true, true), s(36, true, true), s(36),
        ],
        cutoff: 15, resonance: 85, envMod: 90, decay: 6, accent: 88, dist: 70, waveform: 'square',
    },
    {
        name: 'Punisher',
        steps: [
            s(36, true, true), R, s(36, true, true), R,
            s(48, true, true, true), s(36, true, true), R, s(36, true, true),
            R, s(36, true, true), R, s(48, true, true, true),
            s(36, true, true), R, s(36, true, true), s(36),
        ],
        cutoff: 8, resonance: 95, envMod: 98, decay: 5, accent: 92, dist: 80, waveform: 'saw',
    },
    // --- Melodic / musical ---
    {
        name: 'Minor Blues',
        steps: [
            s(48, true, true), s(51), s(53), s(54, true, false, true),
            s(55, true, true), R, s(51), s(48, true, false, true),
            s(48, true, true), s(51), s(53), s(54, true, false, true),
            s(55, true, true), s(58, true, false, true), s(55), s(53),
        ],
        cutoff: 40, resonance: 55, envMod: 60, decay: 35, accent: 50, dist: 18, waveform: 'saw',
    },
    {
        name: 'Walking Bass',
        steps: [
            s(48, true, true), R, s(50), s(51, true, false, true),
            s(53, true, true), R, s(51), s(50, true, false, true),
            s(48, true, true), R, s(46, true, false, true), s(48),
            s(51, true, true), s(53, true, false, true), s(55), s(53, true, false, true),
        ],
        cutoff: 45, resonance: 45, envMod: 55, decay: 40, accent: 45, dist: 12, waveform: 'saw',
    },
    {
        name: 'Chromatic Crawl',
        steps: [
            s(48, true, false, true), s(49, true, false, true), s(50, true, false, true), s(51, true, true, true),
            s(50, true, false, true), s(49, true, false, true), s(48, true, false, true), s(47, true, false, true),
            s(46, true, false, true), s(47, true, false, true), s(48, true, false, true), s(49, true, true, true),
            s(48, true, false, true), s(47, true, false, true), s(46, true, false, true), s(48, true, true),
        ],
        cutoff: 32, resonance: 68, envMod: 72, decay: 22, accent: 58, dist: 28, waveform: 'saw',
    },
    // --- Funky / groovy ---
    {
        name: 'Funk Machine',
        steps: [
            s(48, true, true), R, s(48), s(60, true, true),
            R, s(48), s(48, true, true), R,
            s(53, true, true), R, s(48), s(60, true, true, true),
            s(48), R, s(48, true, true), R,
        ],
        cutoff: 35, resonance: 62, envMod: 70, decay: 20, accent: 65, dist: 30, waveform: 'saw',
    },
    {
        name: 'Bounce',
        steps: [
            s(48, true, true), R, R, s(48),
            s(60, true, true, true), R, s(48, true, true), R,
            R, s(48), s(55, true, true, true), R,
            s(48, true, true), R, R, s(53, true, false, true),
        ],
        cutoff: 28, resonance: 72, envMod: 78, decay: 18, accent: 70, dist: 35, waveform: 'saw',
    },
    {
        name: 'Offbeat Wobble',
        steps: [
            R, s(48, true, true), R, s(48),
            R, s(48, true, true), R, s(51, true, false, true),
            R, s(48, true, true), R, s(48),
            R, s(48, true, true), R, s(53, true, true, true),
        ],
        cutoff: 22, resonance: 78, envMod: 82, decay: 15, accent: 72, dist: 38, waveform: 'saw',
    },
    // --- Deep / atmospheric ---
    {
        name: 'Deep Sub',
        steps: [
            s(36, true, true), s(36), s(36), R,
            s(36), s(36, true, true), s(36), s(36),
            R, s(36), s(36, true, true), s(36),
            s(36), R, s(36), s(36, true, true),
        ],
        cutoff: 15, resonance: 50, envMod: 45, decay: 45, accent: 40, dist: 10, waveform: 'saw',
    },
    {
        name: 'Dub Acid',
        steps: [
            s(36, true, true), R, R, s(48, true, false, true),
            R, R, s(36, true, true), R,
            R, s(48, true, true, true), R, R,
            s(36), R, R, s(43, true, false, true),
        ],
        cutoff: 20, resonance: 75, envMod: 65, decay: 35, accent: 55, dist: 25, waveform: 'saw',
    },
];
