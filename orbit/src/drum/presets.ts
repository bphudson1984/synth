// Drum pattern presets
// Voice indices: 0=kick, 1=snare, 2=closedHH, 3=openHH, 4=clap, 5=tom, 6=rim, 7=perc
// Each pattern is 8 arrays of 16 booleans

export interface DrumPreset {
    name: string;
    pattern: boolean[][];
}

// Helper: convert a string like "x...x...x...x..." to boolean array
function p(s: string): boolean[] {
    return s.split('').map(c => c === 'x');
}

export const PRESETS: DrumPreset[] = [
    {
        name: 'Four on the Floor',
        pattern: [
            p('x...x...x...x...'), // kick
            p('....x.......x...'), // snare
            p('..x...x...x...x.'), // chh
            p('................'), // ohh
            p('................'), // clap
            p('................'), // tom
            p('................'), // rim
            p('................'), // perc
        ],
    },
    {
        name: 'Classic House',
        pattern: [
            p('x...x...x...x...'), // kick
            p('....x.......x...'), // snare
            p('x.x.x.x.x.x.x.x.'), // chh
            p('..............x.'), // ohh
            p('....x.......x...'), // clap
            p('................'), // tom
            p('................'), // rim
            p('.x....x..x....x.'), // perc
        ],
    },
    {
        name: 'Boom Bap',
        pattern: [
            p('x..x......x.....'), // kick
            p('....x.......x...'), // snare
            p('x.x.x.x.x.x.x.x.'), // chh
            p('................'), // ohh
            p('................'), // clap
            p('................'), // tom
            p('................'), // rim
            p('................'), // perc
        ],
    },
    {
        name: 'Breakbeat',
        pattern: [
            p('x.........x.....'), // kick
            p('....x..x....x...'), // snare
            p('x.x.x.x.x.x.x.x.'), // chh
            p('......x.........'), // ohh
            p('................'), // clap
            p('................'), // tom
            p('................'), // rim
            p('..x...........x.'), // perc
        ],
    },
    {
        name: 'Techno Basic',
        pattern: [
            p('x...x...x...x...'), // kick
            p('........x.......'), // snare
            p('.x.x.x.x.x.x.x.'), // chh
            p('x...............'), // ohh
            p('....x.......x...'), // clap
            p('................'), // tom
            p('................'), // rim
            p('................'), // perc
        ],
    },
    {
        name: 'Detroit Techno',
        pattern: [
            p('x...x...x...x...'), // kick
            p('................'), // snare
            p('..x...x...x...x.'), // chh
            p('x.......x.......'), // ohh
            p('....x.......x...'), // clap
            p('................'), // tom
            p('x.....x.........'), // rim
            p('................'), // perc
        ],
    },
    {
        name: 'Acid House',
        pattern: [
            p('x...x...x...x...'), // kick
            p('................'), // snare
            p('x.x.x.x.x.x.x.x.'), // chh
            p('....x.......x...'), // ohh
            p('....x.......x...'), // clap
            p('................'), // tom
            p('..........x.....'), // rim
            p('x.............x.'), // perc
        ],
    },
    {
        name: 'Minimal Techno',
        pattern: [
            p('x...x...x...x...'), // kick
            p('................'), // snare
            p('....x.......x...'), // chh
            p('................'), // ohh
            p('........x.......'), // clap
            p('................'), // tom
            p('x...........x...'), // rim
            p('................'), // perc
        ],
    },
    {
        name: 'Electro',
        pattern: [
            p('x..x..x.x..x..x.'), // kick
            p('....x.......x...'), // snare
            p('x.x.x.x.x.x.x.x.'), // chh
            p('................'), // ohh
            p('....x.......x..x'), // clap
            p('..........x.x...'), // tom
            p('................'), // rim
            p('................'), // perc
        ],
    },
    {
        name: 'Reggaeton',
        pattern: [
            p('x...x..x....x...'), // kick
            p('....x.......x...'), // snare
            p('x.x.x.x.x.x.x.x.'), // chh
            p('................'), // ohh
            p('...x...x...x...x'), // clap
            p('................'), // tom
            p('................'), // rim
            p('..x.......x.....'), // perc
        ],
    },
    {
        name: 'Trap',
        pattern: [
            p('x..........x....'), // kick
            p('....x.......x...'), // snare
            p('x.xxx.x.x.xxx.x.'), // chh
            p('..........x.....'), // ohh
            p('....x.......x...'), // clap
            p('................'), // tom
            p('................'), // rim
            p('................'), // perc
        ],
    },
    {
        name: 'UK Garage',
        pattern: [
            p('x.....x...x.....'), // kick
            p('....x.......x...'), // snare
            p('x.x.x.x.x.x.x.x.'), // chh
            p('................'), // ohh
            p('....x.......x...'), // clap
            p('................'), // tom
            p('.x..........x...'), // rim
            p('..x...x.........'), // perc
        ],
    },
    {
        name: 'Disco',
        pattern: [
            p('x...x...x...x...'), // kick
            p('....x.......x...'), // snare
            p('..x...x...x...x.'), // chh
            p('x...x...x...x...'), // ohh
            p('................'), // clap
            p('................'), // tom
            p('................'), // rim
            p('.x.x.x.x.x.x.x.'), // perc
        ],
    },
    {
        name: 'Drum & Bass',
        pattern: [
            p('x.....x.........'), // kick
            p('....x...x...x...'), // snare
            p('x.x.x.x.x.x.x.x.'), // chh
            p('......x.........'), // ohh
            p('................'), // clap
            p('................'), // tom
            p('................'), // rim
            p('.x..........x...'), // perc
        ],
    },
    {
        name: 'Afrobeat',
        pattern: [
            p('x...x.....x.x...'), // kick
            p('........x.......'), // snare
            p('x.x.x.x.x.x.x.x.'), // chh
            p('....x.......x...'), // ohh
            p('................'), // clap
            p('.x..........x...'), // tom
            p('x.....x.x.....x.'), // rim
            p('..x.......x.....'), // perc
        ],
    },
    {
        name: 'Latin Percussion',
        pattern: [
            p('x.......x.......'), // kick
            p('................'), // snare
            p('x..x..x..x..x..x'), // chh
            p('................'), // ohh
            p('...x..x.........'), // clap
            p('x.....x...x.....'), // tom
            p('..x.x...x.x.x...'), // rim
            p('.x...x.x...x.x.'), // perc
        ],
    },
    {
        name: 'Industrial',
        pattern: [
            p('x.x...x.x.x...x.'), // kick
            p('....x..x....x..x'), // snare
            p('x.x.x.x.x.x.x.x.'), // chh
            p('..x.......x.....'), // ohh
            p('....x.......x...'), // clap
            p('................'), // tom
            p('x...x...x...x...'), // rim
            p('................'), // perc
        ],
    },
    {
        name: 'Lo-Fi Hip Hop',
        pattern: [
            p('x......x..x.....'), // kick
            p('....x.........x.'), // snare
            p('x.x...x.x.x...x.'), // chh
            p('..........x.....'), // ohh
            p('................'), // clap
            p('................'), // tom
            p('..x.............'), // rim
            p('............x...'), // perc
        ],
    },
    {
        name: 'Bossa Nova',
        pattern: [
            p('x.....x.....x...'), // kick
            p('................'), // snare
            p('x.x.x.x.x.x.x.x.'), // chh
            p('................'), // ohh
            p('................'), // clap
            p('................'), // tom
            p('x..x..x..x..x..x'), // rim
            p('...x.x......x.x.'), // perc
        ],
    },
    {
        name: 'Trance',
        pattern: [
            p('x...x...x...x...'), // kick
            p('................'), // snare
            p('.x.x.x.x.x.x.x.'), // chh
            p('....x.......x...'), // ohh
            p('....x.......x...'), // clap
            p('................'), // tom
            p('................'), // rim
            p('x.......x.......'), // perc
        ],
    },
];
