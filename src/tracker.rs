use fundsp::hacker::*;
use fundsp::sound::*;
use funutd::*;

pub const NB_TRACKS: usize = 8;

pub struct Tone {
    octave: i32,
    semitone: i32,
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

pub struct Instrument {
    unit: Box<dyn AudioUnit64>,
    dry_level: f64,
    reverb_level: f64,
    chorus_level: f64,
    delay_level: f64,
    pan: f64,
}

impl Instrument {
    pub fn new(unit: Box<dyn AudioUnit64>) -> Self {
        Self {
            unit,
            dry_level: 1.0,
            reverb_level: 0.2,
            chorus_level: 0.1,
            delay_level: 0.1,
            pan: 0.0,
        }
    }

    pub fn get_unit(&self) -> Box<dyn AudioUnit64> {
        let net = Net64::wrap(self.unit.clone());
        let net = net >> pan(self.pan);

        let net = net
            >> multisplit::<U2, U4>()
            >> ((self.dry_level * multipass::<U2>())
                | (self.reverb_level * multipass::<U2>())
                | (self.chorus_level * multipass::<U2>())
                | (self.delay_level * multipass::<U2>()));

        Box::new(net)
    }
}

pub struct Step {
    tone: Tone,
    instrument: usize,
    velocity: u8,
}

pub struct Phrase {
    steps: Vec<Option<Step>>,
}

pub struct Chain {
    phrases: Vec<Option<usize>>,
}

pub struct Track {
    chains: Vec<Option<usize>>,
    event_id: Option<EventId>,
    mix_level: Shared<f64>,
}

impl Track {
    fn new() -> Self {
        Self {
            chains: std::iter::repeat_with(|| None).take(256).collect(),
            event_id: None,
            mix_level: shared(0.5),
        }
    }
}

pub struct Tracker {
    pub tone: Tone,
    tracks: Vec<Track>,
    chains: Vec<Option<Chain>>,
    phrases: Vec<Option<Phrase>>,
    instruments: Vec<Option<Instrument>>,
    sequencer: Sequencer64,
    net: Net64,
}

impl Tracker {
    pub fn new(sample_rate: f64) -> (Self, BlockRateAdapter64) {
        let mut sequencer = Sequencer64::new(false, 8);
        let sequencer_backend = sequencer.backend();
        println!("outputs {}", sequencer_backend.outputs());

        let mut net = Net64::wrap(Box::new(sequencer_backend));

        println!("outputs {}", net.outputs());

        net = net
            >> (multipass::<U2>()
                | reverb2_stereo(10.0, 2.0, 0.5, 1.0, lowpole_hz(8000.0))
                | chorus(0, 0.015, 0.005, 0.5)
                | chorus(0, 0.015, 0.005, 0.5)
                | feedback(delay(1.0) * db_amp(-3.0))
                | feedback(delay(1.0) * db_amp(-3.0)))
            >> multijoin::<U2, U4>();

        net.set_sample_rate(sample_rate);

        let backend = BlockRateAdapter64::new(Box::new(net.backend()));

        let mut tracker = Self {
            tone: Tone {
                octave: 4,
                semitone: 0,
            },
            tracks: std::iter::repeat_with(Track::new).take(NB_TRACKS).collect(),
            phrases: std::iter::repeat_with(|| None).take(256).collect(),
            chains: std::iter::repeat_with(|| None).take(256).collect(),
            instruments: Vec::new(),
            sequencer,
            net,
        };

        let mut rng = Rnd::new();
        tracker
            .instruments
            .push(Some(Instrument::new(Box::new(bassdrum(
                0.2 + rng.f64() * 0.02,
                180.0,
                60.0,
            )))));

        (tracker, backend)
    }

    pub fn semi_tone_up(&mut self) {
        self.tone = self.tone.up(1);
    }

    pub fn semi_tone_down(&mut self) {
        self.tone = self.tone.down(1);
    }

    pub fn play_note(&mut self) {
        if let Some(ref instrument) = self.instruments[0] {
            self.sequencer
                .push_relative(0.0, 1.0, Fade::Smooth, 0.0, 0.25, instrument.get_unit());
        }
    }
}
