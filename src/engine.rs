use std::time::{Duration, Instant};

const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

pub trait Oscillator: Send {
    fn tick(&self, normalized_sample_index: f32) -> f32;
}

pub struct ConstOscillator {
    pub value: f32,
}

impl ConstOscillator {
    #[allow(unused)]
    pub fn new(value: f32) -> Box<Self> {
        Box::new(Self { value })
    }
}

impl Oscillator for ConstOscillator {
    fn tick(&self, _normalized_sample_index: f32) -> f32 {
        self.value
    }
}

pub struct SineOscillator {
    pub frequency: f32,
    pub lfo: Option<Box<dyn Oscillator>>,
}

impl SineOscillator {
    #[allow(unused)]
    pub fn new(frequency: f32) -> Box<Self> {
        Box::new(Self {
            frequency,
            lfo: None,
        })
    }

    #[allow(unused)]
    pub fn new_fm(frequency: f32, lfo: Box<dyn Oscillator>) -> Box<Self> {
        Box::new(Self {
            frequency,
            lfo: Some(lfo),
        })
    }
}

impl Oscillator for SineOscillator {
    fn tick(&self, normalized_sample_index: f32) -> f32 {
        let lfo_term = if let Some(ref lfo_osc) = self.lfo {
            lfo_osc.tick(normalized_sample_index)
        } else {
            0.0
        };
        (normalized_sample_index * self.frequency * TWO_PI + lfo_term).sin()
    }
}

pub struct PulseOscillator {
    pub frequency: f32,
    pub duty_cycle: f32,
    pub lfo: Option<Box<dyn Oscillator>>,
}

impl PulseOscillator {
    #[allow(unused)]
    pub fn new(frequency: f32, duty_cycle: f32) -> Box<Self> {
        Box::new(Self {
            frequency,
            duty_cycle,
            lfo: None,
        })
    }

    #[allow(unused)]
    pub fn new_fm(frequency: f32, duty_cycle: f32, lfo: Box<dyn Oscillator>) -> Box<Self> {
        Box::new(Self {
            frequency,
            duty_cycle,
            lfo: Some(lfo),
        })
    }
}

impl Oscillator for PulseOscillator {
    fn tick(&self, normalized_sample_index: f32) -> f32 {
        let lfo_term = if let Some(ref lfo_osc) = self.lfo {
            lfo_osc.tick(normalized_sample_index)
        } else {
            0.0
        };
        if (normalized_sample_index * self.frequency + lfo_term) % 1.0 > self.duty_cycle {
            1.0
        } else {
            -1.0
        }
    }
}

pub struct SquareOscillator {
    pub frequency: f32,
    pub lfo: Option<Box<dyn Oscillator>>,
}

impl SquareOscillator {
    #[allow(unused)]
    pub fn new(frequency: f32) -> Box<Self> {
        Box::new(Self {
            frequency,
            lfo: None,
        })
    }

    #[allow(unused)]
    pub fn new_fm(frequency: f32, lfo: Box<dyn Oscillator>) -> Box<Self> {
        Box::new(Self {
            frequency,
            lfo: Some(lfo),
        })
    }
}

impl Oscillator for SquareOscillator {
    fn tick(&self, normalized_sample_index: f32) -> f32 {
        let lfo_term = if let Some(ref lfo_osc) = self.lfo {
            lfo_osc.tick(normalized_sample_index)
        } else {
            0.0
        };
        if (normalized_sample_index * self.frequency + lfo_term) % 1.0 > 0.5 {
            1.0
        } else {
            -1.0
        }
    }
}

pub struct SawOscillator {
    pub frequency: f32,
    pub lfo: Option<Box<dyn Oscillator>>,
}

impl SawOscillator {
    #[allow(unused)]
    pub fn new(frequency: f32) -> Box<Self> {
        Box::new(Self {
            frequency,
            lfo: None,
        })
    }

    #[allow(unused)]
    pub fn new_fm(frequency: f32, lfo: Box<dyn Oscillator>) -> Box<Self> {
        Box::new(Self {
            frequency,
            lfo: Some(lfo),
        })
    }
}

impl Oscillator for SawOscillator {
    fn tick(&self, normalized_sample_index: f32) -> f32 {
        let lfo_term = if let Some(ref lfo_osc) = self.lfo {
            lfo_osc.tick(normalized_sample_index)
        } else {
            0.0
        };
        (normalized_sample_index * self.frequency + lfo_term) % 1.0
    }
}

pub struct TriangleOscillator {
    pub frequency: f32,
    pub lfo: Option<Box<dyn Oscillator>>,
}

impl TriangleOscillator {
    #[allow(unused)]
    pub fn new(frequency: f32) -> Box<Self> {
        Box::new(Self {
            frequency,
            lfo: None,
        })
    }

    #[allow(unused)]
    pub fn new_fm(frequency: f32, lfo: Box<dyn Oscillator>) -> Box<Self> {
        Box::new(Self {
            frequency,
            lfo: Some(lfo),
        })
    }
}

impl Oscillator for TriangleOscillator {
    fn tick(&self, normalized_sample_index: f32) -> f32 {
        let lfo_term = if let Some(ref lfo_osc) = self.lfo {
            lfo_osc.tick(normalized_sample_index)
        } else {
            0.0
        };

        let phase = normalized_sample_index * self.frequency + lfo_term;

        4.0 * (phase - (phase + 0.5).floor()).abs() - 1.0
    }
}

pub struct SumOscillator {
    pub oscillators: Vec<Box<dyn Oscillator>>,
}

impl SumOscillator {
    #[allow(unused)]
    pub fn new(oscillators: Vec<Box<dyn Oscillator>>) -> Box<Self> {
        Box::new(Self { oscillators })
    }
}

impl Oscillator for SumOscillator {
    fn tick(&self, normalized_sample_index: f32) -> f32 {
        self.oscillators
            .iter()
            .map(|oscillator| oscillator.tick(normalized_sample_index))
            .sum()
    }
}

pub struct ProdOscillator {
    pub oscillators: Vec<Box<dyn Oscillator>>,
}

impl ProdOscillator {
    #[allow(unused)]
    pub fn new(oscillators: Vec<Box<dyn Oscillator>>) -> Box<Self> {
        Box::new(Self { oscillators })
    }

    #[allow(unused)]
    pub fn by_const(factor: f32, oscillator: Box<dyn Oscillator>) -> Box<Self> {
        Box::new(Self {
            oscillators: vec![ConstOscillator::new(factor), oscillator],
        })
    }
}

impl Oscillator for ProdOscillator {
    fn tick(&self, normalized_sample_index: f32) -> f32 {
        self.oscillators
            .iter()
            .map(|oscillator| oscillator.tick(normalized_sample_index))
            .product()
    }
}

pub struct Envelope {
    pub attack_time: f32,
    pub decay_time: f32,
    pub release_time: f32,
    pub sustained_level: f32,
    pub start_level: f32,
}

impl Envelope {
    fn level(&self, on_time: Instant, off_time: Option<Instant>, current_time: Instant) -> f32 {
        if let Some(off_time_value) = off_time {
            if current_time > off_time_value {
                return (1.0
                    - current_time.duration_since(off_time_value).as_secs_f32()
                        / self.release_time)
                    * self.sustained_level;
            }
        }

        let note_duration = current_time.duration_since(on_time).as_secs_f32();

        if note_duration < self.attack_time {
            return self.start_level * note_duration / self.attack_time;
        }

        if note_duration < self.attack_time + self.decay_time {
            return self.start_level
                + (self.sustained_level - self.start_level)
                    * ((note_duration - self.attack_time) / self.decay_time);
        }
        self.sustained_level
    }

    fn done(&self, off_time: Option<Instant>, current_time: Instant) -> bool {
        if let Some(off_time_value) = off_time {
            if current_time > off_time_value + Duration::from_secs_f32(self.release_time) {
                return true;
            }
        }
        false
    }
}

pub struct Note {
    pub envelope: Envelope,
    pub oscillator: Box<dyn Oscillator>,
    pub on_time: Instant,
    pub off_time: Option<Instant>,
}

impl Note {
    fn tick(&mut self, current_time: Instant, normalized_sample_index: f32) -> f32 {
        self.oscillator.tick(normalized_sample_index)
            * self
                .envelope
                .level(self.on_time, self.off_time, current_time)
    }

    fn done(&self, current_time: Instant) -> bool {
        self.envelope.done(self.off_time, current_time)
    }
}

pub struct Engine {
    sample_rate: u32,
    sample_index: u32,
    notes: Vec<Note>,
}
impl Engine {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            notes: Vec::new(),
            sample_rate,
            sample_index: 0,
        }
    }

    fn advance_sample(&mut self) {
        self.sample_index = (self.sample_index + 1) % self.sample_rate;
    }

    pub fn tick(&mut self) -> f32 {
        self.advance_sample();
        let normalized_sample_index = self.sample_index as f32 / self.sample_rate as f32;
        let current_time = Instant::now();
        self.notes.retain(|note| !note.done(current_time));
        let value = self
            .notes
            .iter_mut()
            .map(|note| note.tick(current_time, normalized_sample_index))
            .sum();
        value
    }

    pub fn add_note(&mut self, note: Note) {
        self.notes.push(note);
    }
}
