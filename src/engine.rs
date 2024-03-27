use crate::config::NB_TRACKS;
use fundsp::hacker::*;
use fundsp::sound::*;
use funutd::*;
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

#[allow(unused)]
pub enum Gain {
    Const(f32),
    AdsrEnvelope(AdsrEnvelope),
    Lfo(Lfo),
}

impl Gain {
    pub fn level(&self, key_elapsed: f32, key_length: Option<f32>) -> f32 {
        match self {
            Gain::Const(v) => *v,
            Gain::AdsrEnvelope(e) => e.level(key_elapsed, key_length),
            Gain::Lfo(lfo) => lfo.level(key_elapsed),
        }
    }

    pub fn done(&self, key_elapsed: f32, key_length: Option<f32>) -> bool {
        match self {
            Gain::Const(_) => {
                if let Some(key_length) = key_length {
                    key_elapsed > key_length
                } else {
                    false
                }
            }
            Gain::Lfo(_) => {
                if let Some(key_length) = key_length {
                    key_elapsed > key_length
                } else {
                    false
                }
            }
            Gain::AdsrEnvelope(env) => env.done(key_elapsed, key_length),
        }
    }
}

pub struct Lfo {
    pub waveform: Waveform,
    pub frequency: f32,
}

impl Lfo {
    pub fn level(&self, key_elapsed: f32) -> f32 {
        self.waveform.tick(self.frequency, key_elapsed, 0.0)
    }
}

pub struct Oscillator {
    pub waveform: Waveform,
    pub frequency_modifier: FrequenceModifier,
}

impl Oscillator {
    pub fn output(&self, frequency: f32, input: f32, normalized_sample_index: f32) -> f32 {
        self.waveform.tick(
            self.frequency_modifier.apply(frequency),
            normalized_sample_index,
            input,
        )
    }
}

#[allow(unused)]
pub enum Operation {
    Oscillator(usize, Box<Operation>),
    Sum(Vec<Operation>),
    Factor(Gain, Box<Operation>),
    None,
}

impl Operation {
    pub fn eval(
        &self,
        key_elapsed: f32,
        key_length: Option<f32>,
        eval_operator: &mut impl FnMut(usize, f32) -> f32,
    ) -> (f32, bool) {
        match self {
            Operation::None => (0.0, true),
            Operation::Oscillator(index, input) => {
                let (input, done) = input.eval(key_elapsed, key_length, eval_operator);
                let input = if done { 0.0 } else { input };
                (eval_operator(*index, input), false)
            }
            Operation::Sum(ref operations) => operations
                .iter()
                .map(|operation| operation.eval(key_elapsed, key_length, eval_operator))
                .reduce(|(acc_value, acc_done), (value, done)| {
                    (acc_value + value, acc_done && done)
                })
                .unwrap_or((0.0, true)),
            Operation::Factor(gain, operation) => {
                let (input, done) = operation.eval(key_elapsed, key_length, eval_operator);
                (
                    gain.level(key_elapsed, key_length) * input,
                    gain.done(key_elapsed, key_length) || done,
                )
            }
        }
    }
}

pub struct Instrument {
    pub oscillators: Vec<Oscillator>,
    pub algorithm: Operation,
}

impl Instrument {
    pub fn get_sample(
        &self,
        on_time: Instant,
        off_time: Option<Instant>,
        current_time: Instant,
        normalized_sample_index: f32,
        frequency: f32,
    ) -> (f32, bool) {
        let key_elapsed = current_time.duration_since(on_time).as_secs_f32();
        let key_length = off_time.map(|off_time| off_time.duration_since(on_time).as_secs_f32());

        let mut memory: Vec<Option<f32>> = vec![None; self.oscillators.len()];

        let mut eval = |index: usize, input: f32| {
            if let Some(value) = memory[index] {
                value
            } else {
                let value =
                    self.oscillators[index].output(frequency, input, normalized_sample_index);
                memory[index] = Some(value);
                value
            }
        };

        self.algorithm.eval(key_elapsed, key_length, &mut eval)
    }
}

pub struct AdsrEnvelope {
    pub attack_time: f32,
    pub decay_time: f32,
    pub release_time: f32,
    pub sustained_level: f32,
    pub start_level: f32,
}

impl AdsrEnvelope {
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

pub enum Message {
    Note { note: Note, track: u32 },
    Kill { track: u32, off_time: Instant },
}

pub struct Engine {
    sample_rate: u32,
    sample_index: u32,
    notes: Vec<Note>,
    instruments: Vec<Instrument>,
    tracks: [Option<Note>; NB_TRACKS as usize],
    unit: Box<dyn AudioUnit64>,
}
impl Engine {
    pub fn new(sample_rate: u32) -> Self {
        let mut rng = Rnd::new();
        Self {
            notes: Vec::new(),
            sample_rate,
            sample_index: 0,
            instruments: Vec::new(),
            tracks: Default::default(),
            unit: Box::new(bassdrum(0.2 + rng.f64() * 0.02, 180.0, 60.0) >> pan(0.0)),
        }
    }

    fn advance_sample(&mut self) {
        self.sample_index = (self.sample_index + 1) % self.sample_rate;
    }

    pub fn tick(&mut self) -> (f64, f64) {
        // self.advance_sample();
        // let normalized_sample_index = self.sample_index as f32 / self.sample_rate as f32;
        // let current_time = Instant::now();
        //
        // self.notes.retain(|note| !note.done);
        // self.tracks.iter_mut().for_each(|note| {
        //     if let Some(n) = note {
        //         if n.done {
        //             note.take();
        //         }
        //     }
        // });
        //
        // let decaying: f32 = self
        //     .notes
        //     .iter_mut()
        //     .map(|note| {
        //         //note.tick(current_time, normalized_sample_index)
        //         let (sample, done) = self.instruments[note.instrument].get_sample(
        //             note.on_time,
        //             note.off_time,
        //             current_time,
        //             normalized_sample_index,
        //             note.frequency,
        //         );
        //         note.done = done;
        //         sample
        //     })
        //     .sum();
        // let in_tracks: f32 = self
        //     .tracks
        //     .iter_mut()
        //     .map(|note| {
        //         if let Some(note) = note {
        //             let (sample, done) = self.instruments[note.instrument].get_sample(
        //                 note.on_time,
        //                 note.off_time,
        //                 current_time,
        //                 normalized_sample_index,
        //                 note.frequency,
        //             );
        //             note.done = done;
        //             sample
        //         } else {
        //             0.0
        //         }
        //     })
        //     .sum();
        // decaying + in_tracks
        self.unit.get_stereo()
    }

    pub fn handle_message(&mut self, message: Message) {
        match message {
            Message::Note { note, track } => self.add_note(note, track),
            Message::Kill { track, off_time } => self.kill_note(track, off_time),
        }
    }

    pub fn kill_note(&mut self, track: u32, off_time: Instant) {
        if let Some(mut old_note) = self.tracks[track as usize].take() {
            old_note.off_time = Some(off_time);
            self.notes.push(old_note);
        }
    }

    pub fn add_note(&mut self, note: Note, track: u32) {
        self.kill_note(track, note.on_time);
        self.tracks[track as usize] = Some(note);

        self.unit.reset();
    }

    pub fn add_instrument(&mut self, instrument: Instrument) {
        self.instruments.push(instrument);
    }
}
