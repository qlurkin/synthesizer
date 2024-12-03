use fundsp::hacker::*;

pub enum Waveform {
    Sine,
    Saw,
    Triangle,
    Square,
    Pulse { duty_cycle: f32 },
}

impl Waveform {
    pub fn unit(&self) -> Box<dyn AudioUnit> {
        match self {
            Waveform::Sine => Box::new(sine()),
            Waveform::Saw => Box::new(saw()),
            Waveform::Triangle => Box::new(triangle()),
            Waveform::Square => Box::new(square()),
            Waveform::Pulse { duty_cycle } => {
                Box::new((multipass::<U1>() | dc(*duty_cycle)) >> pulse())
            }
        }
    }
}
