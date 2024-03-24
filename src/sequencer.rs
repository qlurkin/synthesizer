use std::{sync::mpsc::Sender, time::Instant};

use crate::engine::Note;

pub struct Sequencer {
    pub frequency: f32,
    pub tx: Sender<Note>,
}
impl Sequencer {
    pub fn semi_tone_up(&mut self) {
        self.frequency *= 2.0_f32.powf(1.0 / 12.0);
        // self.tx.send(self.frequency).unwrap();
    }

    pub fn semi_tone_down(&mut self) {
        self.frequency /= 2.0_f32.powf(1.0 / 12.0);
        // self.tx.send(self.frequency).unwrap();
    }

    pub fn play_note(&mut self) {
        let on_time = Instant::now();
        let note = Note {
            frequency: self.frequency,
            on_time,
            off_time: Some(on_time),
            instrument: 0,
            done: false,
        };
        self.tx.send(note).unwrap();
    }
}
