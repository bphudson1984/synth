pub mod saw;
pub mod square;
pub mod morph;
pub mod fold;
pub mod fm;
pub mod fbfm;
pub mod wavetable;
pub mod vowel;
pub mod pluck;
pub mod bell;
pub mod noise;
pub mod swarm;
pub mod cloud;

pub trait OscModel {
    fn render(&mut self, freq_hz: f32, timbre: f32, color: f32, out: &mut [f32]);
    fn reset(&mut self);
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OscMode {
    Saw = 0, Square = 1, Morph = 2, Fold = 3,
    Fm = 4, Fbfm = 5, Wavetable = 6, Vowel = 7,
    Pluck = 8, Bell = 9, Noise = 10, Swarm = 11, Cloud = 12,
}

impl OscMode {
    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::Saw, 1 => Self::Square, 2 => Self::Morph, 3 => Self::Fold,
            4 => Self::Fm, 5 => Self::Fbfm, 6 => Self::Wavetable, 7 => Self::Vowel,
            8 => Self::Pluck, 9 => Self::Bell, 10 => Self::Noise, 11 => Self::Swarm,
            12 => Self::Cloud, _ => Self::Saw,
        }
    }
}
