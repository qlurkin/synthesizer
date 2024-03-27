use std::{sync::mpsc::Sender, time::Instant};

use crate::config::NB_TRACKS;
use crate::engine::{Message, Note};
use fundsp::hacker::*;
use fundsp::sound::*;
use funutd::*;

pub struct Tone {
    pub octave: i32,
    pub semitone: i32,
}

impl Tone {
    pub fn get_frequency(&self) -> f32 {
        let base_frequency = 440.0; // Fréquence du La 4ème octave
        let semitone_ratio = 2.0_f32.powf(1.0 / 12.0); // Ratio entre deux demi-tons successifs

        let semitone_difference = (self.octave - 4) * 12 + self.semitone; // Nombre total de demi-tons par rapport au La 4ème octave
        base_frequency * semitone_ratio.powi(semitone_difference)
    }

    pub fn up(&self, n: u32) -> Self {
        let mut octave = self.octave;
        let mut semitone = self.semitone;
        for _ in 0..n {
            if semitone == 2 {
                octave += 1;
            }
            semitone = (semitone + 9 + 1).rem_euclid(12) - 9;
        }
        Self { semitone, octave }
    }

    pub fn down(&self, n: u32) -> Self {
        let mut octave = self.octave;
        let mut semitone = self.semitone;
        for _ in 0..n {
            if semitone == -9 {
                octave -= 1;
            }
            semitone = (semitone + 9 - 1).rem_euclid(12) - 9;
        }
        Self { semitone, octave }
    }

    pub fn get_string(&self) -> String {
        let semitone_str = match self.semitone {
            0 => "A-",
            1 => "A#",
            2 => "B-",
            -9 => "C-",
            -8 => "C#",
            -7 => "D-",
            -6 => "D#",
            -5 => "E-",
            -4 => "F-",
            -3 => "F#",
            -2 => "G-",
            _ => "G#",
        };
        format!("{}{}", semitone_str, self.octave)
    }
}

pub struct Step {
    pub tone: Tone,
}

pub struct Phrase {
    pub steps: [Option<Step>; 16],
}

pub struct Chain {
    pub phrases: [Option<Phrase>; 16],
}

pub struct Tracker {
    pub tone: Tone,
    pub frontend: fundsp::sequencer::Sequencer64,
    pub song: Vec<[Option<Chain>; NB_TRACKS as usize]>,
}

impl Tracker {
    pub fn new(frontend: fundsp::sequencer::Sequencer64) -> Self {
        Self {
            frontend,
            tone: Tone {
                octave: 4,
                semitone: 0,
            },
            song: Vec::with_capacity(16),
        }
    }

    pub fn semi_tone_up(&mut self) {
        self.tone = self.tone.up(1);
    }

    pub fn semi_tone_down(&mut self) {
        self.tone = self.tone.down(1);
    }

    pub fn play_note(&mut self) {
        let on_time = Instant::now();
        let note = Note {
            frequency: self.tone.get_frequency(),
            on_time,
            off_time: Some(on_time),
            instrument: 0,
            done: false,
        };
        // self.tx.send(Message::Note { note, track: 0 }).unwrap();
        let mut rng = Rnd::new();
        self.frontend.push_relative(
            0.0,
            1.0,
            Fade::Smooth,
            0.0,
            0.25,
            Box::new(bassdrum(0.2 + rng.f64() * 0.02, 180.0, 60.0) >> pan(0.0)),
        );
    }
}
