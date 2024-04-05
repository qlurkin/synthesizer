use fundsp::hacker::*;
use fundsp::sound::*;
use funutd::*;

pub const NB_TRACKS: usize = 8;

pub struct Tone {
    octave: i32,
    semitone: i32,
}

#[allow(unused)]
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
            chorus_level: 0.2,
            delay_level: 0.2,
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
    pub tone: Tone,
    pub instrument: usize,
    pub velocity: u8,
}

pub struct Phrase {
    pub steps: Vec<Option<Step>>,
}

pub struct Chain {
    pub phrases: Vec<Option<usize>>,
}

pub struct Track {
    pub chains: Vec<Option<usize>>,
    pub event_id: Option<EventId>,
    pub mix_level: f64,
    pub snoop0: Snoop<f64>,
    pub snoop1: Snoop<f64>,
}

impl Track {
    fn new() -> Self {
        let (snoop0, _) = snoop(2048);
        let (snoop1, _) = snoop(2048);

        Self {
            chains: std::iter::repeat_with(|| None).take(256).collect(),
            event_id: None,
            mix_level: 1.0,
            snoop0,
            snoop1,
        }
    }

    pub fn get_unit(&mut self, instrument: &Instrument) -> Box<dyn AudioUnit64> {
        let (snoop0, snoop0_backend) = snoop(32768);
        let (snoop1, snoop1_backend) = snoop(32768);

        self.snoop0 = snoop0;
        self.snoop1 = snoop1;

        let mut net = Net64::wrap(instrument.get_unit());
        net = net
            >> (((multipass::<U2>() * self.mix_level) >> (snoop0_backend | snoop1_backend))
                | multipass::<U6>());
        Box::new(net)
    }
}

pub struct Tracker {
    pub tone: Tone,
    pub tracks: Vec<Track>,
    pub chains: Vec<Option<Chain>>,
    pub phrases: Vec<Option<Phrase>>,
    pub instruments: Vec<Option<Instrument>>,
    pub sequencer: Sequencer64,
    pub reverb_mix_level: Shared<f64>,
    pub chorus_mix_level: Shared<f64>,
    pub delay_mix_level: Shared<f64>,
    pub chorus_to_reverb_level: Shared<f64>,
    pub delay_to_reverb_level: Shared<f64>,
    reverb_room_size: f64,
    reverb_time: f64,
    reverb_diffusion: f64,
    reverb_modulation_speed: f64,
    reverb_filter_frequency: f64,
    chorus_separation: f64,
    chorus_variation: f64,
    chorus_mod_frequency: f64,
    pub snoop_reverb0: Snoop<f64>,
    pub snoop_reverb1: Snoop<f64>,
    pub snoop_chorus0: Snoop<f64>,
    pub snoop_chorus1: Snoop<f64>,
    pub snoop_delay0: Snoop<f64>,
    pub snoop_delay1: Snoop<f64>,
    delay_time: f64,
    delay_decay: f64,
    reverb: Slot64,
    chorus: Slot64,
    delay: Slot64,
}

impl Tracker {
    pub fn new(sample_rate: f64) -> (Self, BlockRateAdapter64) {
        let mut sequencer = Sequencer64::new(false, 8);
        let sequencer_backend = sequencer.backend();
        let reverb_mix_level = shared(1.0);
        let chorus_mix_level = shared(0.0);
        let delay_mix_level = shared(0.0);
        let chorus_to_reverb_level = shared(1.0);
        let delay_to_reverb_level = shared(1.0);
        let reverb_room_size = 10.0;
        let reverb_time = 2.0;
        let reverb_diffusion = 0.5;
        let reverb_modulation_speed = 1.0;
        let reverb_filter_frequency = 8000.0;
        let chorus_separation = 0.015;
        let chorus_variation = 0.005;
        let chorus_mod_frequency = 0.5;
        let delay_time = 1.0;
        let delay_decay = 3.0;

        let (reverb, reverb_backend) = Slot64::new(Box::new(multipass::<U2>()));
        let (chorus, chorus_backend) = Slot64::new(Box::new(multipass::<U2>()));
        let (delay, delay_backend) = Slot64::new(Box::new(multipass::<U2>()));

        let (snoop_reverb0, snoop_reverb0_backend) = snoop(2048);
        let (snoop_reverb1, snoop_reverb1_backend) = snoop(2048);
        let (snoop_chorus0, snoop_chorus0_backend) = snoop(2048);
        let (snoop_chorus1, snoop_chorus1_backend) = snoop(2048);
        let (snoop_delay0, snoop_delay0_backend) = snoop(2048);
        let (snoop_delay1, snoop_delay1_backend) = snoop(2048);

        let mut net = Net64::new(0, 2);

        let sequencer_id = net.push(Box::new(sequencer_backend));
        let reverb_id = net.push(Box::new(reverb_backend));
        let chorus_id = net.push(Box::new(chorus_backend));
        let delay_id = net.push(Box::new(delay_backend));

        let mixer = net.push(Box::new(
            multipass::<U2>()
                + (((multipass::<U1>() * var(&reverb_mix_level)) >> snoop_reverb0_backend)
                    | ((multipass::<U1>() * var(&reverb_mix_level)) >> snoop_reverb1_backend))
                + (((multipass::<U1>() * var(&chorus_mix_level)) >> snoop_chorus0_backend)
                    | ((multipass::<U1>() * var(&chorus_mix_level)) >> snoop_chorus1_backend))
                + (((multipass::<U1>() * var(&delay_mix_level)) >> snoop_delay0_backend)
                    | ((multipass::<U1>() * var(&delay_mix_level)) >> snoop_delay1_backend)),
        ));

        let reverb_inputs_mixer = net.push(Box::new(
            multipass::<U2>()
                + ((multipass::<U1>() * var(&chorus_to_reverb_level))
                    | (multipass::<U1>() * var(&chorus_to_reverb_level)))
                + ((multipass::<U1>() * var(&delay_to_reverb_level))
                    | (multipass::<U1>() * var(&delay_to_reverb_level))),
        ));

        let chorus_spliter = net.push(Box::new(multisplit::<U2, U2>()));
        let delay_spliter = net.push(Box::new(multisplit::<U2, U2>()));

        net.pipe(chorus_id, chorus_spliter);
        net.pipe(delay_id, delay_spliter);
        net.pipe(reverb_inputs_mixer, reverb_id);

        net.connect(sequencer_id, 0, mixer, 0);
        net.connect(sequencer_id, 1, mixer, 1);

        net.connect(sequencer_id, 2, reverb_inputs_mixer, 0);
        net.connect(sequencer_id, 3, reverb_inputs_mixer, 1);

        net.connect(sequencer_id, 4, chorus_id, 0);
        net.connect(sequencer_id, 5, chorus_id, 1);

        net.connect(chorus_spliter, 0, mixer, 4);
        net.connect(chorus_spliter, 1, mixer, 5);
        net.connect(chorus_spliter, 2, reverb_inputs_mixer, 2);
        net.connect(chorus_spliter, 3, reverb_inputs_mixer, 3);

        net.connect(sequencer_id, 6, delay_id, 0);
        net.connect(sequencer_id, 7, delay_id, 1);

        net.connect(delay_spliter, 0, mixer, 6);
        net.connect(delay_spliter, 1, mixer, 7);
        net.connect(delay_spliter, 2, reverb_inputs_mixer, 4);
        net.connect(delay_spliter, 3, reverb_inputs_mixer, 5);

        net.connect(reverb_id, 0, mixer, 2);
        net.connect(reverb_id, 1, mixer, 3);

        net.pipe_output(mixer);

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
            reverb_mix_level,
            chorus_mix_level,
            delay_mix_level,
            chorus_to_reverb_level,
            delay_to_reverb_level,
            reverb_room_size,
            reverb_time,
            reverb_diffusion,
            reverb_modulation_speed,
            reverb_filter_frequency,
            chorus_separation,
            chorus_variation,
            chorus_mod_frequency,
            delay_time,
            delay_decay,
            reverb,
            chorus,
            delay,
            snoop_delay0,
            snoop_delay1,
            snoop_reverb0,
            snoop_reverb1,
            snoop_chorus0,
            snoop_chorus1,
        };

        tracker.rebuild_reverb();
        tracker.rebuild_chorus();
        tracker.rebuild_delay();

        let mut rng = Rnd::new();
        tracker
            .instruments
            .push(Some(Instrument::new(Box::new(bassdrum(
                0.2 + rng.f64() * 0.02,
                180.0,
                60.0,
            )))));

        tracker.tracks[0].mix_level = 1.0;

        (tracker, backend)
    }

    pub fn rebuild_reverb(&mut self) {
        self.reverb.set(
            Fade::Smooth,
            0.1,
            Box::new(reverb2_stereo(
                self.reverb_room_size,
                self.reverb_time,
                self.reverb_diffusion,
                self.reverb_modulation_speed,
                lowpole_hz(self.reverb_filter_frequency),
            )),
        );
    }

    pub fn rebuild_chorus(&mut self) {
        self.chorus.set(
            Fade::Smooth,
            0.1,
            Box::new(
                chorus(
                    0,
                    self.chorus_separation,
                    self.chorus_variation,
                    self.chorus_mod_frequency,
                ) | chorus(
                    0,
                    self.chorus_separation,
                    self.chorus_variation,
                    self.chorus_mod_frequency,
                ),
            ),
        );
    }

    pub fn rebuild_delay(&mut self) {
        self.delay.set(
            Fade::Smooth,
            0.1,
            Box::new(
                feedback(delay(self.delay_time) * db_amp(-self.delay_decay))
                    | feedback(delay(self.delay_time) * db_amp(-self.delay_decay)),
            ),
        );
    }

    pub fn _semi_tone_up(&mut self) {
        self.tone = self.tone.up(1);
    }

    pub fn _semi_tone_down(&mut self) {
        self.tone = self.tone.down(1);
    }

    pub fn play_note(&mut self) {
        if let Some(ref instrument) = self.instruments[0] {
            self.sequencer.push_relative(
                0.0,
                1.0,
                Fade::Smooth,
                0.0,
                0.25,
                self.tracks[0].get_unit(instrument),
            );
        }
    }
}
