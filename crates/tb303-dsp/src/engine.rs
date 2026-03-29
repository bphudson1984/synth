use crate::voice::AcidVoice;
use crate::sequencer::{AcidSequencer, AcidSeqEvent};

pub struct TB303 {
    pub voice: AcidVoice,
    pub sequencer: AcidSequencer,
    events: Vec<AcidSeqEvent>,
    pub master_volume: f32,
}

impl TB303 {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            voice: AcidVoice::new(sample_rate),
            sequencer: AcidSequencer::new(sample_rate),
            events: Vec::with_capacity(4),
            master_volume: 0.7,
        }
    }

    pub fn process(&mut self) -> f32 {
        self.events.clear();
        self.sequencer.process(&mut self.events);
        for event in &self.events {
            match event {
                AcidSeqEvent::NoteOn { note, accent, slide } => {
                    self.voice.note_on(*note, *accent, *slide);
                }
                AcidSeqEvent::NoteOff => { self.voice.note_off(); }
            }
        }
        self.voice.process() * self.master_volume
    }
}
