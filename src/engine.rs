use std::time::{Duration, Instant};

pub enum Waveform {
    Sine,
    Square,
    Saw,
    Triangle,
}

pub struct Oscillator {
    pub sample_rate: f32,
    pub waveform: Waveform,
    pub sample_index: f32,
    pub frequency: f32,
}

impl Oscillator {
    fn advance_sample(&mut self) {
        self.sample_index = (self.sample_index + 1.0) % self.sample_rate;
    }

    pub fn _set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    fn calculate_sine_output_from_freq(&self, freq: f32) -> f32 {
        let two_pi = 2.0 * std::f32::consts::PI;
        (self.sample_index * freq * two_pi / self.sample_rate).sin()
    }

    fn is_multiple_of_freq_above_nyquist(&self, multiple: f32) -> bool {
        self.frequency * multiple > self.sample_rate / 2.0
    }

    fn sine_wave(&mut self) -> f32 {
        self.advance_sample();
        self.calculate_sine_output_from_freq(self.frequency)
    }

    fn generative_waveform(&mut self, harmonic_index_increment: i32, gain_exponent: f32) -> f32 {
        self.advance_sample();
        let mut output = 0.0;
        let mut i = 1;
        while !self.is_multiple_of_freq_above_nyquist(i as f32) {
            let gain = 1.0 / (i as f32).powf(gain_exponent);
            output += gain * self.calculate_sine_output_from_freq(self.frequency * i as f32);
            i += harmonic_index_increment;
        }
        output
    }

    fn square_wave(&mut self) -> f32 {
        self.generative_waveform(2, 1.0)
    }

    fn saw_wave(&mut self) -> f32 {
        self.generative_waveform(1, 1.0)
    }

    fn triangle_wave(&mut self) -> f32 {
        self.generative_waveform(2, 2.0)
    }

    pub fn tick(&mut self) -> f32 {
        match self.waveform {
            Waveform::Sine => self.sine_wave(),
            Waveform::Square => self.square_wave(),
            Waveform::Saw => self.saw_wave(),
            Waveform::Triangle => self.triangle_wave(),
        }
    }
}

pub struct Envelope {
    pub attack_time: Duration,
    pub decay_time: Duration,
    pub release_time: Duration,
    pub sustained_level: f32,
    pub start_level: f32,
}

impl Envelope {
    fn level(&self, on_time: Instant, off_time: Option<Instant>, current_time: Instant) -> f32 {
        if let Some(off_time_value) = off_time {
            if current_time > off_time_value {
                return (1.0
                    - current_time.duration_since(off_time_value).as_secs_f32()
                        / self.release_time.as_secs_f32())
                    * self.sustained_level;
            }
        }

        let note_duration = current_time.duration_since(on_time).as_secs_f32();

        if note_duration < self.attack_time.as_secs_f32() {
            return self.start_level * note_duration / self.attack_time.as_secs_f32();
        }

        if note_duration < self.attack_time.as_secs_f32() + self.decay_time.as_secs_f32() {
            return self.sustained_level
                + (self.start_level - self.sustained_level)
                    * (1.0
                        - (note_duration - self.attack_time.as_secs_f32())
                            / self.decay_time.as_secs_f32());
        }
        self.sustained_level
    }

    fn done(&self, off_time: Option<Instant>, current_time: Instant) -> bool {
        if let Some(off_time_value) = off_time {
            if current_time > off_time_value + self.release_time {
                return true;
            }
        }
        false
    }
}

pub struct Note {
    pub envelope: Envelope,
    pub oscillators: Vec<(f32, Oscillator)>,
    pub on_time: Instant,
    pub off_time: Option<Instant>,
}

impl Note {
    fn tick(&mut self, current_time: Instant) -> f32 {
        let value: f32 = self
            .oscillators
            .iter_mut()
            .map(|(gain, oscillator)| *gain * oscillator.tick())
            .sum();
        value
            * self
                .envelope
                .level(self.on_time, self.off_time, current_time)
    }

    fn done(&self, current_time: Instant) -> bool {
        self.envelope.done(self.off_time, current_time)
    }
}

pub struct Engine {
    notes: Vec<Note>,
}
impl Engine {
    pub fn new() -> Self {
        Self { notes: Vec::new() }
    }

    pub fn tick(&mut self) -> f32 {
        let current_time = Instant::now();
        let mut w: usize = 0;
        self.notes.retain(|note| !note.done(current_time));
        let value = self
            .notes
            .iter_mut()
            .map(|note| note.tick(current_time))
            .sum();
        value
    }

    pub fn add_note(&mut self, note: Note) {
        self.notes.push(note);
    }
}
