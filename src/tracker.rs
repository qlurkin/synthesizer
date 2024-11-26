use fundsp::hacker::*;

use crate::imui::console::console_log;

pub const NB_TRACKS: usize = 8;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Tone {
    pub octave: i32,
    pub semitone: i32,
}

#[allow(unused)]
impl Tone {
    pub fn get_frequency(&self) -> f32 {
        let base_frequency = 440.0_f32; // Fréquence du La 4ème octave
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

pub enum InstrumentType {
    None,
    Simple {
        waveform: Waveform,
        envelope: Envelope,
    },
}

impl InstrumentType {
    pub fn unit(&self, frequency: f32, _velocity: f32) -> Box<dyn AudioUnit> {
        match self {
            InstrumentType::None => Box::new(zero()),
            InstrumentType::Simple { waveform, envelope } => {
                let envelope = *envelope;
                Box::new(
                    dc(frequency)
                        >> (fundsp::hacker::envelope(move |t| envelope.level(t))
                            * Net::wrap(waveform.unit())),
                )
            }
        }
    }
}

pub struct Instrument {
    ty: InstrumentType,
    dry_level: f32,
    reverb_level: f32,
    chorus_level: f32,
    delay_level: f32,
    pan: f32,
}

impl Instrument {
    pub fn new(ty: InstrumentType) -> Self {
        Self {
            ty,
            dry_level: 1.0,
            reverb_level: 1.0,
            chorus_level: 0.0,
            delay_level: 0.8,
            pan: 0.0,
        }
    }

    pub fn unit(&self, frequency: f32, velocity: f32) -> Box<dyn AudioUnit> {
        let net = Net::wrap(self.ty.unit(frequency, velocity));
        let net = net >> pan(self.pan);

        let net = net
            >> multisplit::<U2, U4>()
            >> ((self.dry_level * multipass::<U2>())
                | (self.chorus_level * multipass::<U2>())
                | (self.delay_level * multipass::<U2>())
                | (self.reverb_level * multipass::<U2>()));

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

impl Phrase {
    fn new() -> Self {
        Self {
            steps: std::iter::repeat_with(|| None).take(16).collect(),
        }
    }
}

pub struct Chain {
    pub phrases: Vec<Option<usize>>,
}

pub struct Track {
    pub chains: Vec<Option<usize>>,
    pub event_id: Option<EventId>,
    pub mix_level: Shared,
    pub snoop0: Snoop,
    pub snoop1: Snoop,
    pub sequencer: Sequencer,
    pub net: Net,
    pub chain_cursor: usize,
    pub phrase_cursor: usize,
    pub step_cursor: usize,
}

impl Track {
    fn new() -> Self {
        let (snoop0, snoop0_backend) = snoop(2048);
        let (snoop1, snoop1_backend) = snoop(2048);

        let mut sequencer = Sequencer::new(false, 8);

        let backend = sequencer.backend();

        let mix_level = shared(1.0);

        let mut net = Net::wrap(Box::new(backend));
        net = net
            >> (((multipass::<U2>() * (var(&mix_level) | var(&mix_level)))
                >> (snoop0_backend | snoop1_backend))
                | multipass::<U6>());

        Self {
            chains: std::iter::repeat_with(|| None).take(256).collect(),
            event_id: None,
            mix_level,
            snoop0,
            snoop1,
            sequencer,
            net,
            chain_cursor: 0,
            phrase_cursor: 0,
            step_cursor: 0,
        }
    }
}

pub struct Tracker {
    pub tone: Tone,
    pub tracks: Vec<Track>,
    pub chains: Vec<Option<Chain>>,
    pub phrases: Vec<Option<Phrase>>,
    pub instruments: Vec<Option<Instrument>>,
    pub reverb_mix_level: Shared,
    pub chorus_mix_level: Shared,
    pub delay_mix_level: Shared,
    pub chorus_to_reverb_level: Shared,
    pub delay_to_reverb_level: Shared,
    pub reverb_room_size: f32,
    pub reverb_time: f32,
    pub reverb_diffusion: f32,
    pub reverb_modulation_speed: f32,
    pub reverb_filter_frequency: f32,
    pub chorus_separation: f32,
    pub chorus_variation: f32,
    pub chorus_mod_frequency: f32,
    pub snoop_reverb0: Snoop,
    pub snoop_reverb1: Snoop,
    pub snoop_chorus0: Snoop,
    pub snoop_chorus1: Snoop,
    pub snoop_delay0: Snoop,
    pub snoop_delay1: Snoop,
    pub snoop_out0: Snoop,
    pub snoop_out1: Snoop,
    pub delay_time: f32,
    pub delay_decay: f32,
    reverb: Slot,
    chorus: Slot,
    delay: Slot,
}

impl Tracker {
    pub fn new(sample_rate: f64) -> (Self, BlockRateAdapter) {
        let mut net = Net::new(0, 2);
        let mut tracks: Vec<Track> = std::iter::repeat_with(Track::new).take(NB_TRACKS).collect();
        let track_ids: Vec<NodeId> = tracks
            .iter_mut()
            .map(|track| {
                let backend = track.net.backend();
                net.push(Box::new(backend))
            })
            .collect();
        let reverb_mix_level = shared(1.0);
        let chorus_mix_level = shared(1.0);
        let delay_mix_level = shared(1.0);
        let chorus_to_reverb_level = shared(0.0);
        let delay_to_reverb_level = shared(0.0);
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

        let (reverb, reverb_backend) = Slot::new(Box::new(multipass::<U2>()));
        let (chorus, chorus_backend) = Slot::new(Box::new(multipass::<U2>()));
        let (delay, delay_backend) = Slot::new(Box::new(multipass::<U2>()));

        let (snoop_reverb0, snoop_reverb0_backend) = snoop(2048);
        let (snoop_reverb1, snoop_reverb1_backend) = snoop(2048);
        let (snoop_chorus0, snoop_chorus0_backend) = snoop(2048);
        let (snoop_chorus1, snoop_chorus1_backend) = snoop(2048);
        let (snoop_delay0, snoop_delay0_backend) = snoop(2048);
        let (snoop_delay1, snoop_delay1_backend) = snoop(2048);
        let (snoop_out0, snoop_out0_backend) = snoop(2048);
        let (snoop_out1, snoop_out1_backend) = snoop(2048);

        let pre_chorus_mixer = net.push(Box::new(sumi::<U8, _, _>(|_| multipass::<U2>())));

        track_ids.iter().enumerate().for_each(|(i, id)| {
            net.connect(*id, 2, pre_chorus_mixer, i * 2);
            net.connect(*id, 3, pre_chorus_mixer, i * 2 + 1);
        });

        let chorus_id = net.push(Box::new(chorus_backend));

        net.pipe_all(pre_chorus_mixer, chorus_id);

        let chorus_spliter = net.push(Box::new(multisplit::<U2, U2>()));
        net.pipe_all(chorus_id, chorus_spliter);

        let pre_delay_mixer = net.push(Box::new(sumi::<U8, _, _>(|_| multipass::<U2>())));

        track_ids.iter().enumerate().for_each(|(i, id)| {
            net.connect(*id, 4, pre_delay_mixer, i * 2);
            net.connect(*id, 5, pre_delay_mixer, i * 2 + 1);
        });

        let delay_id = net.push(Box::new(delay_backend));

        net.pipe_all(pre_delay_mixer, delay_id);

        let delay_spliter = net.push(Box::new(multisplit::<U2, U2>()));
        net.pipe_all(delay_id, delay_spliter);

        let pre_reverb_mixer = net.push(Box::new(
            sumi::<U8, _, _>(|_| multipass::<U2>())
                + ((multipass::<U1>() * var(&chorus_to_reverb_level))
                    | (multipass::<U1>() * var(&chorus_to_reverb_level)))
                + ((multipass::<U1>() * var(&delay_to_reverb_level))
                    | (multipass::<U1>() * var(&delay_to_reverb_level))),
        ));

        track_ids.iter().enumerate().for_each(|(i, id)| {
            net.connect(*id, 6, pre_reverb_mixer, i * 2);
            net.connect(*id, 7, pre_reverb_mixer, i * 2 + 1);
        });
        net.connect(chorus_spliter, 2, pre_reverb_mixer, 16);
        net.connect(chorus_spliter, 3, pre_reverb_mixer, 17);
        net.connect(delay_spliter, 2, pre_reverb_mixer, 18);
        net.connect(delay_spliter, 3, pre_reverb_mixer, 19);

        let reverb_id = net.push(Box::new(reverb_backend));

        net.pipe_all(pre_reverb_mixer, reverb_id);

        let mixer = net.push(Box::new(
            (sumi::<U8, _, _>(|_| multipass::<U2>())
                + (((multipass::<U1>() * var(&chorus_mix_level)) >> snoop_chorus0_backend)
                    | ((multipass::<U1>() * var(&chorus_mix_level)) >> snoop_chorus1_backend))
                + (((multipass::<U1>() * var(&delay_mix_level)) >> snoop_delay0_backend)
                    | ((multipass::<U1>() * var(&delay_mix_level)) >> snoop_delay1_backend))
                + (((multipass::<U1>() * var(&reverb_mix_level)) >> snoop_reverb0_backend)
                    | ((multipass::<U1>() * var(&reverb_mix_level)) >> snoop_reverb1_backend)))
                >> (snoop_out0_backend | snoop_out1_backend),
        ));

        track_ids.iter().enumerate().for_each(|(i, id)| {
            net.connect(*id, 0, mixer, i * 2);
            net.connect(*id, 1, mixer, i * 2 + 1);
        });
        net.connect(chorus_spliter, 0, mixer, 16);
        net.connect(chorus_spliter, 1, mixer, 17);
        net.connect(delay_spliter, 0, mixer, 18);
        net.connect(delay_spliter, 1, mixer, 19);
        net.connect(reverb_id, 0, mixer, 20);
        net.connect(reverb_id, 1, mixer, 21);

        net.pipe_output(mixer);

        net.set_sample_rate(sample_rate);

        let backend = BlockRateAdapter::new(Box::new(net.backend()));

        let mut tracker = Self {
            tone: Tone {
                octave: 4,
                semitone: 0,
            },
            tracks,
            phrases: std::iter::repeat_with(|| None).take(256).collect(),
            chains: std::iter::repeat_with(|| None).take(256).collect(),
            instruments: Vec::new(),
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
            snoop_out0,
            snoop_out1,
        };

        tracker.rebuild_reverb();
        tracker.rebuild_chorus();
        tracker.rebuild_delay();

        tracker
            .instruments
            .push(Some(Instrument::new(InstrumentType::Simple {
                waveform: Waveform::Saw,
                envelope: Envelope::Ads {
                    attack: 0.01,
                    decay: 0.01,
                    sustain: 0.7,
                },
            })));

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
        let step_id = self.tracks[0].step_cursor;
        if self.get_phrase(0).steps[step_id].is_none() {
            return;
        }

        let tone = self.get_phrase(0).steps[step_id].as_ref().unwrap().tone;

        if let Some(ref instrument) = self.instruments[0] {
            self.tracks[0].sequencer.push_relative(
                0.0,
                1.0,
                Fade::Smooth,
                0.0,
                0.25,
                instrument.unit(tone.get_frequency(), 1.0),
            );
        }
    }

    pub fn get_phrase(&mut self, index: usize) -> &mut Phrase {
        if self.phrases[index].is_none() {
            self.phrases[index] = Some(Phrase::new());
        }
        self.phrases[index].as_mut().unwrap()
    }
}
