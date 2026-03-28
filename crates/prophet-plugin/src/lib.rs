use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui, EguiState};
use prophet_dsp::synth::ProphetSynth;
use prophet_dsp::effects::chorus::StereoChorus;
use prophet_dsp::effects::delay::TapeDelay;
use prophet_dsp::effects::reverb::PlateReverb;
use std::sync::Arc;

mod gui;
pub mod presets;

pub struct ProphetPlugin {
    params: Arc<ProphetParams>,
    synth: ProphetSynth,
    chorus: StereoChorus,
    delay: TapeDelay,
    reverb: PlateReverb,
}

#[derive(Params)]
pub struct ProphetParams {
    #[persist = "editor-state-v2"]
    editor_state: Arc<EguiState>,

    #[id = "cutoff"]
    pub filter_cutoff: FloatParam,

    #[id = "reso"]
    pub filter_resonance: FloatParam,

    #[id = "volume"]
    pub volume: FloatParam,

    #[id = "osc_a_saw"]
    pub osc_a_saw: BoolParam,

    #[id = "osc_a_pulse"]
    pub osc_a_pulse: BoolParam,

    #[id = "osc_b_saw"]
    pub osc_b_saw: BoolParam,

    #[id = "osc_b_tri"]
    pub osc_b_tri: BoolParam,

    #[id = "osc_b_pulse"]
    pub osc_b_pulse: BoolParam,

    #[id = "osc_a_level"]
    pub osc_a_level: FloatParam,

    #[id = "osc_b_level"]
    pub osc_b_level: FloatParam,

    #[id = "noise_level"]
    pub noise_level: FloatParam,

    #[id = "filter_env_amt"]
    pub filter_env_amount: FloatParam,

    #[id = "f_attack"]
    pub filter_attack: FloatParam,

    #[id = "f_decay"]
    pub filter_decay: FloatParam,

    #[id = "f_sustain"]
    pub filter_sustain: FloatParam,

    #[id = "f_release"]
    pub filter_release: FloatParam,

    #[id = "a_attack"]
    pub amp_attack: FloatParam,

    #[id = "a_decay"]
    pub amp_decay: FloatParam,

    #[id = "a_sustain"]
    pub amp_sustain: FloatParam,

    #[id = "a_release"]
    pub amp_release: FloatParam,

    // Pulse width
    #[id = "osc_a_pw"]
    pub osc_a_pw: FloatParam,

    #[id = "osc_b_pw"]
    pub osc_b_pw: FloatParam,

    // Osc B pitch offset
    #[id = "osc_b_semi"]
    pub osc_b_semitones: FloatParam,

    #[id = "osc_b_fine"]
    pub osc_b_fine: FloatParam,

    // Filter drive
    #[id = "flt_drive"]
    pub filter_drive: FloatParam,

    // Hard sync
    #[id = "sync"]
    pub sync: BoolParam,

    // Poly Mod
    #[id = "pm_filt_env"]
    pub poly_mod_filt_env: FloatParam,

    #[id = "pm_osc_b"]
    pub poly_mod_osc_b: FloatParam,

    #[id = "pm_freq_a"]
    pub poly_mod_freq_a: BoolParam,

    #[id = "pm_pw_a"]
    pub poly_mod_pw_a: BoolParam,

    #[id = "pm_filter"]
    pub poly_mod_filter: BoolParam,

    // LFO
    #[id = "lfo_freq"]
    pub lfo_freq: FloatParam,

    #[id = "lfo_tri"]
    pub lfo_tri: BoolParam,

    #[id = "lfo_saw"]
    pub lfo_saw: BoolParam,

    #[id = "lfo_square"]
    pub lfo_square: BoolParam,

    // LFO initial amount — always-on modulation depth
    #[id = "lfo_amt"]
    pub lfo_initial_amount: FloatParam,

    // Wheel Mod
    #[id = "wm_mix"]
    pub wheel_mod_mix: FloatParam,

    #[id = "wm_freq_a"]
    pub wheel_mod_freq_a: BoolParam,

    #[id = "wm_freq_b"]
    pub wheel_mod_freq_b: BoolParam,

    #[id = "wm_pw_a"]
    pub wheel_mod_pw_a: BoolParam,

    #[id = "wm_pw_b"]
    pub wheel_mod_pw_b: BoolParam,

    #[id = "wm_filter"]
    pub wheel_mod_filter: BoolParam,

    // Glide
    #[id = "glide_rate"]
    pub glide_rate: FloatParam,

    #[id = "glide_on"]
    pub glide_on: BoolParam,

    // Unison
    #[id = "unison"]
    pub unison: BoolParam,

    // Drift
    #[id = "drift"]
    pub drift: FloatParam,

    // Effects — Chorus
    #[id = "ch_rate"]
    pub chorus_rate: FloatParam,
    #[id = "ch_depth"]
    pub chorus_depth: FloatParam,
    #[id = "ch_mix"]
    pub chorus_mix: FloatParam,

    // Effects — Delay
    #[id = "dl_time"]
    pub delay_time: FloatParam,
    #[id = "dl_fb"]
    pub delay_feedback: FloatParam,
    #[id = "dl_tone"]
    pub delay_tone: FloatParam,
    #[id = "dl_mix"]
    pub delay_mix: FloatParam,

    // Effects — Reverb
    #[id = "rv_decay"]
    pub reverb_decay: FloatParam,
    #[id = "rv_damp"]
    pub reverb_damping: FloatParam,
    #[id = "rv_mix"]
    pub reverb_mix: FloatParam,
}

impl Default for ProphetPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(ProphetParams::default()),
            synth: ProphetSynth::new(44100.0),
            chorus: StereoChorus::new(44100.0),
            delay: TapeDelay::new(44100.0),
            reverb: PlateReverb::new(44100.0),
        }
    }
}

impl Default for ProphetParams {
    fn default() -> Self {
        let env_time_range = FloatRange::Skewed {
            min: 0.001,
            max: 10.0,
            factor: FloatRange::skew_factor(-2.0),
        };

        Self {
            editor_state: EguiState::from_size(1600, 440),

            filter_cutoff: FloatParam::new(
                "Filter Cutoff",
                10000.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 20000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(5.0))
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(0))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),

            filter_resonance: FloatParam::new(
                "Resonance",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(5.0))
            .with_step_size(0.01),

            volume: FloatParam::new(
                "Volume",
                -6.0,
                FloatRange::Linear { min: -60.0, max: 0.0 },
            )
            .with_smoother(SmoothingStyle::Linear(3.0))
            .with_unit(" dB"),

            osc_a_saw: BoolParam::new("Osc A Saw", true),
            osc_a_pulse: BoolParam::new("Osc A Pulse", false),
            osc_b_saw: BoolParam::new("Osc B Saw", false),
            osc_b_tri: BoolParam::new("Osc B Triangle", false),
            osc_b_pulse: BoolParam::new("Osc B Pulse", false),

            osc_a_level: FloatParam::new(
                "Osc A Level",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            osc_b_level: FloatParam::new(
                "Osc B Level",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            noise_level: FloatParam::new(
                "Noise Level",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),

            filter_env_amount: FloatParam::new(
                "Filter Env Amount",
                5000.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 20000.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_unit(" Hz"),

            filter_attack: FloatParam::new("Filter Attack", 0.01, env_time_range)
                .with_unit(" s"),
            filter_decay: FloatParam::new("Filter Decay", 0.3, env_time_range)
                .with_unit(" s"),
            filter_sustain: FloatParam::new(
                "Filter Sustain",
                0.2,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            filter_release: FloatParam::new("Filter Release", 0.3, env_time_range)
                .with_unit(" s"),

            amp_attack: FloatParam::new("Amp Attack", 0.005, env_time_range)
                .with_unit(" s"),
            amp_decay: FloatParam::new("Amp Decay", 0.3, env_time_range)
                .with_unit(" s"),
            amp_sustain: FloatParam::new(
                "Amp Sustain",
                0.8,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            amp_release: FloatParam::new("Amp Release", 0.3, env_time_range)
                .with_unit(" s"),

            osc_a_pw: FloatParam::new(
                "Osc A Pulse Width",
                0.5,
                FloatRange::Linear { min: 0.01, max: 0.99 },
            ),
            osc_b_pw: FloatParam::new(
                "Osc B Pulse Width",
                0.5,
                FloatRange::Linear { min: 0.01, max: 0.99 },
            ),

            osc_b_semitones: FloatParam::new(
                "Osc B Semitones",
                0.0,
                FloatRange::Linear { min: -24.0, max: 24.0 },
            )
            .with_step_size(1.0),
            osc_b_fine: FloatParam::new(
                "Osc B Fine",
                0.0,
                FloatRange::Linear { min: -100.0, max: 100.0 },
            )
            .with_unit(" cents"),
            filter_drive: FloatParam::new(
                "Filter Drive",
                1.0,
                FloatRange::Linear { min: 0.5, max: 5.0 },
            ),

            sync: BoolParam::new("Sync", false),

            poly_mod_filt_env: FloatParam::new(
                "PolyMod Filt Env",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            poly_mod_osc_b: FloatParam::new(
                "PolyMod Osc B",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            poly_mod_freq_a: BoolParam::new("PolyMod → Freq A", false),
            poly_mod_pw_a: BoolParam::new("PolyMod → PW A", false),
            poly_mod_filter: BoolParam::new("PolyMod → Filter", false),

            lfo_freq: FloatParam::new(
                "LFO Freq",
                5.0,
                FloatRange::Skewed { min: 0.1, max: 20.0, factor: FloatRange::skew_factor(-1.0) },
            )
            .with_unit(" Hz"),
            lfo_tri: BoolParam::new("LFO Triangle", true),
            lfo_saw: BoolParam::new("LFO Saw", false),
            lfo_square: BoolParam::new("LFO Square", false),

            lfo_initial_amount: FloatParam::new(
                "LFO Amount",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),

            wheel_mod_mix: FloatParam::new(
                "WheelMod Mix",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            wheel_mod_freq_a: BoolParam::new("WM → Freq A", false),
            wheel_mod_freq_b: BoolParam::new("WM → Freq B", false),
            wheel_mod_pw_a: BoolParam::new("WM → PW A", false),
            wheel_mod_pw_b: BoolParam::new("WM → PW B", false),
            wheel_mod_filter: BoolParam::new("WM → Filter", false),

            glide_rate: FloatParam::new(
                "Glide Rate",
                0.1,
                FloatRange::Skewed { min: 0.001, max: 2.0, factor: FloatRange::skew_factor(-2.0) },
            )
            .with_unit(" s"),
            glide_on: BoolParam::new("Glide", false),

            unison: BoolParam::new("Unison", false),

            drift: FloatParam::new(
                "Drift",
                1.0,
                FloatRange::Linear { min: 0.0, max: 10.0 },
            )
            .with_unit(" Hz"),

            // Chorus
            chorus_rate: FloatParam::new("Chorus Rate", 0.8,
                FloatRange::Skewed { min: 0.1, max: 5.0, factor: FloatRange::skew_factor(-1.0) })
                .with_unit(" Hz"),
            chorus_depth: FloatParam::new("Chorus Depth", 0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 }),
            chorus_mix: FloatParam::new("Chorus Mix", 0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 }),

            // Delay
            delay_time: FloatParam::new("Delay Time", 375.0,
                FloatRange::Skewed { min: 1.0, max: 2000.0, factor: FloatRange::skew_factor(-1.5) })
                .with_unit(" ms"),
            delay_feedback: FloatParam::new("Delay Feedback", 0.4,
                FloatRange::Linear { min: 0.0, max: 0.95 }),
            delay_tone: FloatParam::new("Delay Tone", 0.6,
                FloatRange::Linear { min: 0.0, max: 1.0 }),
            delay_mix: FloatParam::new("Delay Mix", 0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 }),

            // Reverb
            reverb_decay: FloatParam::new("Reverb Decay", 0.7,
                FloatRange::Linear { min: 0.0, max: 0.99 }),
            reverb_damping: FloatParam::new("Reverb Damp", 0.7,
                FloatRange::Linear { min: 0.0, max: 1.0 }),
            reverb_mix: FloatParam::new("Reverb Mix", 0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 }),
        }
    }
}

impl Plugin for ProphetPlugin {
    const NAME: &'static str = "Prophet-5";
    const VENDOR: &'static str = "Synth Project";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: None,
            main_output_channels: NonZeroU32::new(2), // Stereo
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        let sr = buffer_config.sample_rate;
        self.synth = ProphetSynth::new(sr);
        self.chorus = StereoChorus::new(sr);
        self.delay = TapeDelay::new(sr);
        self.reverb = PlateReverb::new(sr);
        true
    }

    fn reset(&mut self) {
        // Re-init synth at current sample rate
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let params = self.params.clone();
        create_egui_editor(
            self.params.editor_state.clone(),
            (),
            |ctx, _state| {
                // Set up dark theme on first open
                let mut style = (*ctx.style()).clone();
                style.visuals.dark_mode = true;
                style.visuals.panel_fill = egui::Color32::from_rgb(25, 25, 25);
                ctx.set_style(style);
            },
            move |egui_ctx, setter, _state| {
                gui::panel::draw_panel(egui_ctx, setter, &params);
            },
        )
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let mut next_event = context.next_event();

        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            // Read params (smoothed per-sample)
            let cutoff = self.params.filter_cutoff.smoothed.next();
            let resonance = self.params.filter_resonance.smoothed.next();
            let volume_db = self.params.volume.smoothed.next();
            let osc_a_level = self.params.osc_a_level.smoothed.next();
            let osc_b_level = self.params.osc_b_level.smoothed.next();
            let noise_level = self.params.noise_level.smoothed.next();
            let filter_env_amount = self.params.filter_env_amount.smoothed.next();
            let f_attack = self.params.filter_attack.smoothed.next();
            let f_decay = self.params.filter_decay.smoothed.next();
            let f_sustain = self.params.filter_sustain.smoothed.next();
            let f_release = self.params.filter_release.smoothed.next();
            let a_attack = self.params.amp_attack.smoothed.next();
            let a_decay = self.params.amp_decay.smoothed.next();
            let a_sustain = self.params.amp_sustain.smoothed.next();
            let a_release = self.params.amp_release.smoothed.next();

            // Apply params to all voices
            let osc_a_saw = self.params.osc_a_saw.value();
            let osc_a_pulse = self.params.osc_a_pulse.value();
            let osc_b_saw = self.params.osc_b_saw.value();
            let osc_b_tri = self.params.osc_b_tri.value();
            let osc_b_pulse = self.params.osc_b_pulse.value();
            let osc_a_pw = self.params.osc_a_pw.smoothed.next();
            let osc_b_pw = self.params.osc_b_pw.smoothed.next();
            let sync = self.params.sync.value();
            let pm_filt_env = self.params.poly_mod_filt_env.smoothed.next();
            let pm_osc_b = self.params.poly_mod_osc_b.smoothed.next();
            let pm_freq_a = self.params.poly_mod_freq_a.value();
            let pm_pw_a = self.params.poly_mod_pw_a.value();
            let pm_filter = self.params.poly_mod_filter.value();

            let osc_b_semi = self.params.osc_b_semitones.smoothed.next();
            let osc_b_fine_val = self.params.osc_b_fine.smoothed.next();
            let flt_drive = self.params.filter_drive.smoothed.next();

            self.synth.for_each_voice(|v| {
                v.osc_a.set_saw(osc_a_saw);
                v.osc_a.set_pulse(osc_a_pulse);
                v.osc_b.set_saw(osc_b_saw);
                v.osc_b.set_tri(osc_b_tri);
                v.osc_b.set_pulse(osc_b_pulse);
                // PW set on voice, not directly on oscillator (voice handles modulation)
                v.osc_a_pw = osc_a_pw;
                v.osc_b_pw = osc_b_pw;
                v.osc_a_level = osc_a_level;
                v.osc_b_level = osc_b_level;
                v.noise_level = noise_level;
                v.filter_cutoff = cutoff;
                v.filter_env_amount = filter_env_amount;
                v.filter.set_resonance(resonance * 4.0);
                v.filter_drive = flt_drive;
                v.filter_env.set_attack(f_attack);
                v.filter_env.set_decay(f_decay);
                v.filter_env.set_sustain(f_sustain);
                v.filter_env.set_release(f_release);
                v.amp_env.set_attack(a_attack);
                v.amp_env.set_decay(a_decay);
                v.amp_env.set_sustain(a_sustain);
                v.amp_env.set_release(a_release);
                v.sync_enabled = sync;
                v.osc_b_semitones = osc_b_semi;
                v.osc_b_fine = osc_b_fine_val;
                v.poly_mod_filt_env_amt = pm_filt_env;
                v.poly_mod_osc_b_amt = pm_osc_b;
                v.poly_mod_dest_freq_a = pm_freq_a;
                v.poly_mod_dest_pw_a = pm_pw_a;
                v.poly_mod_dest_filter = pm_filter;
                v.set_glide_enabled(self.params.glide_on.value());
                v.set_glide_rate(self.params.glide_rate.smoothed.next());
                v.set_drift_amount(self.params.drift.smoothed.next());
            });

            // LFO params
            self.synth.lfo.set_frequency(self.params.lfo_freq.smoothed.next());
            self.synth.lfo.set_triangle(self.params.lfo_tri.value());
            self.synth.lfo.set_sawtooth(self.params.lfo_saw.value());
            self.synth.lfo.set_square(self.params.lfo_square.value());

            // Wheel Mod config
            self.synth.wheel_mod_source_mix = self.params.wheel_mod_mix.smoothed.next();
            self.synth.lfo_initial_amount = self.params.lfo_initial_amount.smoothed.next();
            self.synth.wheel_mod_dest_freq_a = self.params.wheel_mod_freq_a.value();
            self.synth.wheel_mod_dest_freq_b = self.params.wheel_mod_freq_b.value();
            self.synth.wheel_mod_dest_pw_a = self.params.wheel_mod_pw_a.value();
            self.synth.wheel_mod_dest_pw_b = self.params.wheel_mod_pw_b.value();
            self.synth.wheel_mod_dest_filter = self.params.wheel_mod_filter.value();

            // Unison
            self.synth.unison = self.params.unison.value();

            // Drain MIDI events up to this sample
            while let Some(event) = next_event {
                if event.timing() > sample_id as u32 {
                    break;
                }

                match event {
                    NoteEvent::NoteOn { note, velocity, .. } => {
                        self.synth.note_on(note, (velocity * 127.0) as u8);
                    }
                    NoteEvent::NoteOff { note, .. } => {
                        self.synth.note_off(note);
                    }
                    NoteEvent::MidiCC { cc, value, .. } => {
                        if cc == 1 { // Mod wheel
                            self.synth.mod_wheel = value;
                        }
                    }
                    NoteEvent::MidiPitchBend { value, .. } => {
                        // value is 0.0-1.0 in nih-plug, map to -1..+1
                        self.synth.pitch_bend = value * 2.0 - 1.0;
                    }
                    _ => (),
                }

                next_event = context.next_event();
            }

            // Generate audio
            let dry = self.synth.process();

            // Effect parameters
            self.chorus.rate = self.params.chorus_rate.smoothed.next();
            self.chorus.depth = self.params.chorus_depth.smoothed.next();
            self.chorus.mix = self.params.chorus_mix.smoothed.next();
            self.delay.time_ms = self.params.delay_time.smoothed.next();
            self.delay.feedback = self.params.delay_feedback.smoothed.next();
            self.delay.tone = self.params.delay_tone.smoothed.next();
            self.delay.mix = self.params.delay_mix.smoothed.next();
            self.reverb.decay = self.params.reverb_decay.smoothed.next();
            self.reverb.damping = self.params.reverb_damping.smoothed.next();
            self.reverb.mix = self.params.reverb_mix.smoothed.next();

            // Effects chain: mono → chorus (stereo) → delay → reverb
            let (ch_l, ch_r) = self.chorus.process(dry);
            let (dl_l, dl_r) = self.delay.process(ch_l, ch_r);
            let (rv_l, rv_r) = self.reverb.process_stereo(dl_l, dl_r);

            let gain = util::db_to_gain_fast(volume_db);

            // Write stereo output
            let mut ch_iter = channel_samples.into_iter();
            if let Some(l) = ch_iter.next() { *l = rv_l * gain; }
            if let Some(r) = ch_iter.next() { *r = rv_r * gain; }
        }

        ProcessStatus::KeepAlive
    }
}

impl ClapPlugin for ProphetPlugin {
    const CLAP_ID: &'static str = "com.synth-project.prophet-5";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Prophet-5 synthesizer recreation");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Instrument,
        ClapFeature::Synthesizer,
        ClapFeature::Mono,
    ];
}

impl Vst3Plugin for ProphetPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"Prophet5SynthPl\0";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Synth,
    ];
}

nih_export_clap!(ProphetPlugin);
nih_export_vst3!(ProphetPlugin);
