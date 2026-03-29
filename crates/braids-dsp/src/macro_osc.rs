use crate::models::*;
use crate::models::saw::SawModel;
use crate::models::square::SquareModel;
use crate::models::morph::MorphModel;
use crate::models::fold::FoldModel;
use crate::models::fm::FmModel;
use crate::models::fbfm::FbfmModel;
use crate::models::wavetable::WavetableModel;
use crate::models::vowel::VowelModel;
use crate::models::pluck::PluckModel;
use crate::models::bell::BellModel;
use crate::models::noise::NoiseModel;
use crate::models::swarm::SwarmModel;
use crate::models::cloud::CloudModel;

pub struct MacroOscillator {
    mode: OscMode,
    saw: SawModel,
    square: SquareModel,
    morph: MorphModel,
    fold: FoldModel,
    fm: FmModel,
    fbfm: FbfmModel,
    wavetable: WavetableModel,
    vowel: VowelModel,
    pluck: PluckModel,
    bell: BellModel,
    noise: NoiseModel,
    swarm: SwarmModel,
    cloud: CloudModel,
}

impl MacroOscillator {
    pub fn new(sr: f32) -> Self {
        Self {
            mode: OscMode::Saw,
            saw: SawModel::new(sr), square: SquareModel::new(sr),
            morph: MorphModel::new(sr), fold: FoldModel::new(sr),
            fm: FmModel::new(sr), fbfm: FbfmModel::new(sr),
            wavetable: WavetableModel::new(sr), vowel: VowelModel::new(sr),
            pluck: PluckModel::new(sr), bell: BellModel::new(sr),
            noise: NoiseModel::new(sr), swarm: SwarmModel::new(sr),
            cloud: CloudModel::new(sr),
        }
    }

    pub fn set_mode(&mut self, mode: OscMode) {
        if mode != self.mode {
            self.mode = mode;
        }
    }

    pub fn reset_active(&mut self) {
        match self.mode {
            OscMode::Saw => self.saw.reset(),
            OscMode::Square => self.square.reset(),
            OscMode::Morph => self.morph.reset(),
            OscMode::Fold => self.fold.reset(),
            OscMode::Fm => self.fm.reset(),
            OscMode::Fbfm => self.fbfm.reset(),
            OscMode::Wavetable => self.wavetable.reset(),
            OscMode::Vowel => self.vowel.reset(),
            OscMode::Pluck => self.pluck.reset(),
            OscMode::Bell => self.bell.reset(),
            OscMode::Noise => self.noise.reset(),
            OscMode::Swarm => self.swarm.reset(),
            OscMode::Cloud => self.cloud.reset(),
        }
    }

    pub fn render(&mut self, freq: f32, timbre: f32, color: f32, out: &mut [f32]) {
        match self.mode {
            OscMode::Saw => self.saw.render(freq, timbre, color, out),
            OscMode::Square => self.square.render(freq, timbre, color, out),
            OscMode::Morph => self.morph.render(freq, timbre, color, out),
            OscMode::Fold => self.fold.render(freq, timbre, color, out),
            OscMode::Fm => self.fm.render(freq, timbre, color, out),
            OscMode::Fbfm => self.fbfm.render(freq, timbre, color, out),
            OscMode::Wavetable => self.wavetable.render(freq, timbre, color, out),
            OscMode::Vowel => self.vowel.render(freq, timbre, color, out),
            OscMode::Pluck => self.pluck.render(freq, timbre, color, out),
            OscMode::Bell => self.bell.render(freq, timbre, color, out),
            OscMode::Noise => self.noise.render(freq, timbre, color, out),
            OscMode::Swarm => self.swarm.render(freq, timbre, color, out),
            OscMode::Cloud => self.cloud.render(freq, timbre, color, out),
        }
    }
}
