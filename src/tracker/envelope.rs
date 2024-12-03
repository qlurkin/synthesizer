use fundsp::hacker::*;

#[derive(Clone, Copy)]
pub enum Envelope {
    Ads {
        attack: f64,
        decay: f64,
        sustain: f64,
    },
    None,
}

impl Envelope {
    pub fn level(&self, time: f64) -> f64 {
        match self {
            Envelope::Ads {
                attack,
                decay,
                sustain,
            } => {
                if time < *attack {
                    lerp(0.0_f64, 1.0_f64, time / attack)
                } else {
                    let decay_time = time - attack;
                    if decay_time < *decay {
                        lerp(1.0_f64, *sustain, decay_time / decay)
                    } else {
                        *sustain
                    }
                }
            }
            Envelope::None => 1.0,
        }
    }
}
