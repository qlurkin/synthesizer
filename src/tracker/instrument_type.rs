use fundsp::hacker::*;

use super::{envelope::Envelope, waveform::Waveform};

pub enum InstrumentType {
    None,
    Simple {
        waveform: Waveform,
        envelope: Envelope,
    },
}

impl InstrumentType {
    pub fn unit(&self, frequency: f32, _velocity: f32) -> Box<dyn AudioUnit> {
        match self {
            InstrumentType::None => Box::new(zero()),
            InstrumentType::Simple {
                waveform,
                envelope: evlp,
            } => {
                let evlp = *evlp;
                Box::new(
                    dc(frequency)
                        >> (envelope(move |t| evlp.level(t)) * Net::wrap(waveform.unit())),
                )
            }
        }
    }
}
