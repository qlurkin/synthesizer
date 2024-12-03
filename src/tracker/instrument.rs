use fundsp::hacker::*;

use super::instrument_type::InstrumentType;

pub struct Instrument {
    ty: InstrumentType,
    dry_level: f32,
    reverb_level: f32,
    chorus_level: f32,
    delay_level: f32,
    pan: f32,
}

impl Instrument {
    pub fn new(ty: InstrumentType) -> Self {
        Self {
            ty,
            dry_level: 1.0,
            reverb_level: 1.0,
            chorus_level: 0.0,
            delay_level: 0.8,
            pan: 0.0,
        }
    }

    pub fn unit(&self, frequency: f32, velocity: f32) -> Box<dyn AudioUnit> {
        let net = Net::wrap(self.ty.unit(frequency, velocity));
        let net = net >> pan(self.pan);

        let net = net
            >> multisplit::<U2, U4>()
            >> ((self.dry_level * multipass::<U2>())
                | (self.chorus_level * multipass::<U2>())
                | (self.delay_level * multipass::<U2>())
                | (self.reverb_level * multipass::<U2>()));

        Box::new(net)
    }
}
