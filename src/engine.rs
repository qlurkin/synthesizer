use std::time::Instant;

const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

#[allow(unused)]
pub enum Waveform {
    Sine,
    Triangle,
    Saw,
    Square,
    Pulse { duty_cycle: f32 },
}

impl Waveform {
    pub fn tick(&self, frequency: f32, normalized_sample_index: f32, input: f32) -> f32 {
        let phase = normalized_sample_index * frequency + input;
        match self {
            Waveform::Sine => (phase * TWO_PI).sin(),
            Waveform::Triangle => 4.0 * (phase - (phase + 0.5).floor()).abs() - 1.0,
            Waveform::Saw => phase % 1.0,
            Waveform::Square => {
                if phase % 1.0 > 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
            Waveform::Pulse { duty_cycle } => {
                if phase % 1.0 > *duty_cycle {
                    1.0
                } else {
                    -1.0
                }
            }
        }
    }
}

#[allow(unused)]
pub enum FrequenceModifier {
    Factor(f32),
    Shift(f32),
    Fixed(f32),
    None,
}

impl FrequenceModifier {
    pub fn apply(&self, frequency: f32) -> f32 {
        match self {
            FrequenceModifier::None => frequency,
            FrequenceModifier::Shift(value) => frequency + value,
            FrequenceModifier::Factor(value) => frequency * value,
            FrequenceModifier::Fixed(value) => *value,
        }
    }
}

pub enum Gain {
    Const(f32),
    ADSREnvelope(ADSREnvelope),
}

impl Gain {
    pub fn level(&self, key_elapsed: f32, key_length: Option<f32>) -> f32 {
        match self {
            Gain::Const(v) => *v,
            Gain::ADSREnvelope(e) => e.level(key_elapsed, key_length),
        }
    }
}

pub struct Operator {
    pub waveform: Waveform,
    pub frequency_modifier: FrequenceModifier,
    pub gain: Gain,
}

impl Operator {
    pub fn output(
        &self,
        frequency: f32,
        key_elapsed: f32,
        key_length: Option<f32>,
        input: f32,
        normalized_sample_index: f32,
    ) -> f32 {
        self.gain.level(key_elapsed, key_length)
            * self.waveform.tick(
                self.frequency_modifier.apply(frequency),
                normalized_sample_index,
                input,
            )
    }

    pub fn done(&self, key_elapsed: f32, key_length: Option<f32>) -> bool {
        match &self.gain {
            Gain::Const(_) => {
                if let Some(key_length) = key_length {
                    key_elapsed > key_length
                } else {
                    false
                }
            }
            Gain::ADSREnvelope(env) => env.done(key_elapsed, key_length),
        }
    }
}

#[allow(unused)]
#[derive(Clone)]
pub enum Input {
    Operator(usize),
    Sum(Vec<usize>),
    None,
}

pub struct InstrumentOperator {
    pub operator: Operator,
    pub input: Input,
    pub last_sample: f32,
    pub called: bool,
}

impl InstrumentOperator {}

pub struct Instrument {
    pub operators: Vec<InstrumentOperator>,
}

impl Instrument {
    pub fn get_sample(
        &mut self,
        on_time: Instant,
        off_time: Option<Instant>,
        current_time: Instant,
        normalized_sample_index: f32,
        frequency: f32,
    ) -> (f32, bool) {
        self.operators.iter_mut().for_each(|operator| {
            operator.called = false;
            operator.last_sample = 0.0;
        });
        let key_elapsed = current_time.duration_since(on_time).as_secs_f32();
        let key_length = off_time.map(|off_time| off_time.duration_since(on_time).as_secs_f32());
        let sample = self.get_operator_output(
            0,
            frequency,
            key_elapsed,
            key_length,
            normalized_sample_index,
        );
        let done = self.operators[0].operator.done(key_elapsed, key_length);
        (sample, done)
    }

    pub fn get_operator_output(
        &mut self,
        index: usize,
        frequency: f32,
        key_elapsed: f32,
        key_length: Option<f32>,
        normalized_sample_index: f32,
    ) -> f32 {
        if self.operators[index].called {
            return self.operators[index].last_sample;
        }

        let input = match self.operators[index].input.clone() {
            Input::None => 0.0,
            Input::Operator(input_index) => self.get_operator_output(
                input_index,
                frequency,
                key_elapsed,
                key_length,
                normalized_sample_index,
            ),
            Input::Sum(ref indices) => indices
                .iter()
                .map(|input_index| {
                    self.get_operator_output(
                        *input_index,
                        frequency,
                        key_elapsed,
                        key_length,
                        normalized_sample_index,
                    )
                })
                .sum(),
        };
        let res = self.operators[index].operator.output(
            frequency,
            key_elapsed,
            key_length,
            input,
            normalized_sample_index,
        );

        self.operators[index].called = true;
        self.operators[index].last_sample = res;
        res
    }

    pub fn _done(&self, key_elapsed: f32, key_length: Option<f32>) -> bool {
        self.operators[0].operator.done(key_elapsed, key_length)
    }
}

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

pub struct ADSREnvelope {
    pub attack_time: f32,
    pub decay_time: f32,
    pub release_time: f32,
    pub sustained_level: f32,
    pub start_level: f32,
}

impl ADSREnvelope {
    fn level(&self, key_elapsed: f32, key_length: Option<f32>) -> f32 {
        if key_elapsed < self.attack_time {
            return self.start_level * key_elapsed / self.attack_time;
        }

        if key_elapsed < self.attack_time + self.decay_time {
            return self.start_level
                + (self.sustained_level - self.start_level)
                    * ((key_elapsed - self.attack_time) / self.decay_time);
        }
        if let Some(key_length) = key_length {
            let key_length = key_length.max(self.attack_time + self.decay_time);
            if key_elapsed > key_length {
                return (1.0 - (key_elapsed - key_length) / self.release_time)
                    * self.sustained_level;
            }
        }
        self.sustained_level
    }

    fn done(&self, key_elapsed: f32, key_length: Option<f32>) -> bool {
        if let Some(key_length) = key_length {
            if key_elapsed
                > (self.attack_time + self.decay_time + self.release_time)
                    .max(key_length + self.release_time)
            {
                return true;
            }
        }
        false
    }
}

pub struct Note {
    pub frequency: f32,
    pub on_time: Instant,
    pub off_time: Option<Instant>,
    pub instrument: usize,
    pub done: bool,
}

pub struct Engine {
    sample_rate: u32,
    sample_index: u32,
    notes: Vec<Note>,
    instruments: Vec<Instrument>,
}
impl Engine {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            notes: Vec::new(),
            sample_rate,
            sample_index: 0,
            instruments: Vec::new(),
        }
    }

    fn advance_sample(&mut self) {
        self.sample_index = (self.sample_index + 1) % self.sample_rate;
    }

    pub fn tick(&mut self) -> f32 {
        self.advance_sample();
        let normalized_sample_index = self.sample_index as f32 / self.sample_rate as f32;
        let current_time = Instant::now();
        self.notes.retain(|note| !note.done);
        let value = self
            .notes
            .iter_mut()
            .map(|note| {
                //note.tick(current_time, normalized_sample_index)
                let (sample, done) = self.instruments[note.instrument].get_sample(
                    note.on_time,
                    note.off_time,
                    current_time,
                    normalized_sample_index,
                    note.frequency,
                );
                note.done = done;
                sample
            })
            .sum();
        value
    }

    pub fn add_note(&mut self, note: Note) {
        self.notes.push(note);
    }

    pub fn add_instrument(&mut self, instrument: Instrument) {
        self.instruments.push(instrument);
    }
}
