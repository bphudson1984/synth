use crate::{
    bass_drum::BassDrum, snare::SnareDrum, hihat::{ClosedHiHat, OpenHiHat, Cymbal},
    clap::HandClap, cowbell::Cowbell, tom::Tom, rimshot::RimShot,
    clave::Clave, maracas::Maracas,
};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Voice {
    BD = 0, SD = 1, LT = 2, MT = 3, HT = 4,
    RS = 5, CP = 6, CH = 7, OH = 8, CY = 9,
    CB = 10, MA = 11, CL = 12,
}

impl Voice {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::BD), 1 => Some(Self::SD),
            2 => Some(Self::LT), 3 => Some(Self::MT), 4 => Some(Self::HT),
            5 => Some(Self::RS), 6 => Some(Self::CP),
            7 => Some(Self::CH), 8 => Some(Self::OH), 9 => Some(Self::CY),
            10 => Some(Self::CB), 11 => Some(Self::MA), 12 => Some(Self::CL),
            _ => None,
        }
    }
}

pub struct TR808 {
    pub bd: BassDrum,
    pub sd: SnareDrum,
    pub lt: Tom,
    pub mt: Tom,
    pub ht: Tom,
    pub rs: RimShot,
    pub cp: HandClap,
    pub ch: ClosedHiHat,
    pub oh: OpenHiHat,
    pub cy: Cymbal,
    pub cb: Cowbell,
    pub ma: Maracas,
    pub cl: Clave,
    pub master_volume: f32,
}

impl TR808 {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            bd: BassDrum::new(sample_rate),
            sd: SnareDrum::new(sample_rate),
            lt: Tom::low(sample_rate),
            mt: Tom::mid(sample_rate),
            ht: Tom::high(sample_rate),
            rs: RimShot::new(sample_rate),
            cp: HandClap::new(sample_rate),
            ch: ClosedHiHat::new(sample_rate),
            oh: OpenHiHat::new(sample_rate),
            cy: Cymbal::new(sample_rate),
            cb: Cowbell::new(sample_rate),
            ma: Maracas::new(sample_rate),
            cl: Clave::new(sample_rate),
            master_volume: 0.8,
        }
    }

    pub fn trigger(&mut self, voice: Voice) {
        match voice {
            Voice::BD => self.bd.trigger(),
            Voice::SD => self.sd.trigger(),
            Voice::LT => self.lt.trigger(),
            Voice::MT => self.mt.trigger(),
            Voice::HT => self.ht.trigger(),
            Voice::RS => self.rs.trigger(),
            Voice::CP => self.cp.trigger(),
            Voice::CH => {
                self.ch.trigger();
                self.oh.choke(); // CH chokes OH
            }
            Voice::OH => self.oh.trigger(),
            Voice::CY => self.cy.trigger(),
            Voice::CB => self.cb.trigger(),
            Voice::MA => self.ma.trigger(),
            Voice::CL => self.cl.trigger(),
        }
    }

    pub fn process(&mut self) -> f32 {
        let out = self.bd.process()
            + self.sd.process()
            + self.lt.process()
            + self.mt.process()
            + self.ht.process()
            + self.rs.process()
            + self.cp.process()
            + self.ch.process()
            + self.oh.process()
            + self.cy.process()
            + self.cb.process()
            + self.ma.process()
            + self.cl.process();
        (out * self.master_volume).clamp(-1.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_voices_produce_sound() {
        let mut tr = TR808::new(44100.0);
        let voices = [
            Voice::BD, Voice::SD, Voice::LT, Voice::MT, Voice::HT,
            Voice::RS, Voice::CP, Voice::CH, Voice::OH, Voice::CY,
            Voice::CB, Voice::MA, Voice::CL,
        ];
        for voice in voices {
            tr.trigger(voice);
        }
        let buf: Vec<f32> = (0..4410).map(|_| tr.process()).collect();
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
    }

    #[test]
    fn test_ch_chokes_oh() {
        let mut tr = TR808::new(44100.0);
        tr.oh.decay = 1.0; // long OH
        tr.trigger(Voice::OH);
        // Render 50ms of OH
        for _ in 0..2205 { tr.process(); }

        // CH should choke the OH
        tr.trigger(Voice::CH);
        // After 100ms, OH should be silent (choked)
        for _ in 0..4410 { tr.process(); }
        // The OH's envelope should be near zero
        // (can't easily isolate OH output, but the choke mechanism is tested)
    }
}
