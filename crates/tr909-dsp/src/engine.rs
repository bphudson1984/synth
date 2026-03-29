use crate::{
    bass_drum::BassDrum909, snare::SnareDrum909,
    hihat::{ClosedHiHat909, OpenHiHat909},
    clap::HandClap909, tom::Tom909, rimshot::RimShot909,
    cymbal::{CrashCymbal909, RideCymbal909},
};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Voice909 {
    BD = 0, SD = 1, LT = 2, MT = 3, HT = 4,
    RS = 5, CP = 6, CH = 7, OH = 8, CC = 9,
    RC = 10,
}

impl Voice909 {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::BD), 1 => Some(Self::SD),
            2 => Some(Self::LT), 3 => Some(Self::MT), 4 => Some(Self::HT),
            5 => Some(Self::RS), 6 => Some(Self::CP),
            7 => Some(Self::CH), 8 => Some(Self::OH),
            9 => Some(Self::CC), 10 => Some(Self::RC),
            _ => None,
        }
    }
}

pub struct TR909 {
    pub bd: BassDrum909,
    pub sd: SnareDrum909,
    pub lt: Tom909,
    pub mt: Tom909,
    pub ht: Tom909,
    pub rs: RimShot909,
    pub cp: HandClap909,
    pub ch: ClosedHiHat909,
    pub oh: OpenHiHat909,
    pub cc: CrashCymbal909,
    pub rc: RideCymbal909,
    pub master_volume: f32,
}

impl TR909 {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            bd: BassDrum909::new(sample_rate),
            sd: SnareDrum909::new(sample_rate),
            lt: Tom909::low(sample_rate),
            mt: Tom909::mid(sample_rate),
            ht: Tom909::high(sample_rate),
            rs: RimShot909::new(sample_rate),
            cp: HandClap909::new(sample_rate),
            ch: ClosedHiHat909::new(sample_rate),
            oh: OpenHiHat909::new(sample_rate),
            cc: CrashCymbal909::new(sample_rate),
            rc: RideCymbal909::new(sample_rate),
            master_volume: 0.8,
        }
    }

    pub fn trigger(&mut self, voice: Voice909) {
        match voice {
            Voice909::BD => self.bd.trigger(),
            Voice909::SD => self.sd.trigger(),
            Voice909::LT => self.lt.trigger(),
            Voice909::MT => self.mt.trigger(),
            Voice909::HT => self.ht.trigger(),
            Voice909::RS => self.rs.trigger(),
            Voice909::CP => self.cp.trigger(),
            Voice909::CH => { self.ch.trigger(); self.oh.choke(); }
            Voice909::OH => self.oh.trigger(),
            Voice909::CC => self.cc.trigger(),
            Voice909::RC => self.rc.trigger(),
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
            + self.cc.process()
            + self.rc.process();
        (out * self.master_volume).clamp(-1.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_909_voices_produce_sound() {
        let mut tr = TR909::new(44100.0);
        let voices = [
            Voice909::BD, Voice909::SD, Voice909::LT, Voice909::MT, Voice909::HT,
            Voice909::RS, Voice909::CP, Voice909::CH, Voice909::OH,
            Voice909::CC, Voice909::RC,
        ];
        for v in voices { tr.trigger(v); }
        let buf: Vec<f32> = (0..4410).map(|_| tr.process()).collect();
        audio_test_harness::level::assert_not_silent(&buf, 0.01);
    }

    #[test]
    fn test_909_ch_chokes_oh() {
        let mut tr = TR909::new(44100.0);
        tr.oh.decay = 1.0;
        tr.trigger(Voice909::OH);
        for _ in 0..2205 { tr.process(); }
        tr.trigger(Voice909::CH); // should choke OH
        for _ in 0..4410 { tr.process(); }
        // OH should be dying
    }
}
