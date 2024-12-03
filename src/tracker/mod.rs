pub mod chain;
pub mod envelope;
pub mod instrument;
pub mod instrument_type;
pub mod phrase;
pub mod step;
pub mod tone;
pub mod track;
pub mod waveform;

use std::time::{Duration, Instant};

use chain::Chain;
use envelope::Envelope;
use fundsp::hacker::*;
use instrument::Instrument;
use instrument_type::InstrumentType;
use phrase::Phrase;
use tone::Tone;
use track::Track;
use waveform::Waveform;

pub const NB_TRACKS: usize = 8;

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
    last_update: Option<Instant>,
    pub play_time: Duration,
    pub bpm: f32,
    pub update_duration: Option<Duration>,
    pub tick_count: u32,
    pub step_count: u32,
    pub time_since_last_tick: Duration,
    pub remaining_ticks_for_next_step: u32,
    pub playing: bool,
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
        let delay_mix_level = shared(0.0);
        let chorus_to_reverb_level = shared(0.0);
        let delay_to_reverb_level = shared(0.0);
        let reverb_room_size = 5.0;
        let reverb_time = 1.0;
        let reverb_diffusion = 0.5;
        let reverb_modulation_speed = 1.0;
        let reverb_filter_frequency = 8000.0;
        let chorus_separation = 0.015;
        let chorus_variation = 0.005;
        let chorus_mod_frequency = 0.5;
        let delay_time = 1.0;
        let delay_decay = 3.0;

        // test
        // let room_size = shared(0.0);
        // let test_reverb = reverb2_stereo(var(&room_size)., 1.0, 1.0, 1.0, lowpole_hz(8000.0));

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
            last_update: None,
            play_time: Duration::from_secs_f32(0.0),
            time_since_last_tick: Duration::from_secs_f32(0.0),
            bpm: 128.0,
            update_duration: None,
            tick_count: 0,
            step_count: 0,
            remaining_ticks_for_next_step: 6,
            playing: false,
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

    fn step(&mut self) {
        self.step_count += 1;
        if self.step_count == self.song_step_count() as u32 {
            self.step_count = 0;
        }
        self.tracks.iter_mut().for_each(|track| {
            track.step();
        });
        self.play_note();
    }

    fn tick(&mut self) {
        self.remaining_ticks_for_next_step -= 1;
        if self.remaining_ticks_for_next_step == 0 {
            self.remaining_ticks_for_next_step = 6;
            self.step();
        }
    }

    pub fn update(&mut self) {
        // The length of a beat is specified by the bpm
        // beat are divided in 24 ticks
        // normally, a phrase's step is 6 ticks
        // so there is 4 steps in a beat

        let now = Instant::now();

        if !self.playing {
            self.last_update = Some(now);
            return;
        }

        let tps = self.bpm * 24.0 / 60.0;
        let tick_duration = 1.0 / tps;

        if let Some(last_update) = self.last_update {
            let update_duration = now.duration_since(last_update);
            self.update_duration = Some(update_duration);
            self.time_since_last_tick += update_duration;
            let ticks = (self.time_since_last_tick.as_secs_f32() * tps) as u32;
            self.tick_count += ticks;
            (0..ticks).for_each(|_| {
                self.tick();
            });
            self.time_since_last_tick -= ticks * Duration::from_secs_f32(tick_duration);
        }
        self.last_update = Some(now);

        let time_since_last_step =
            (self.tick_count % 6) as f32 * tick_duration + self.time_since_last_tick.as_secs_f32();
        let next_step_time = 6.0 * tick_duration - time_since_last_step;

        (0..self.tracks.len()).for_each(|i| {
            self.update_track(i, next_step_time);
        });
    }

    fn update_track(&mut self, track_id: usize, next_step_time: f32) {}

    pub fn play_note(&mut self) {
        let step_id = self.tracks[0].step_cursor;
        if self.get_phrase(0).steps[step_id].is_none() {
            return;
        }

        let tone = self.get_phrase(0).steps[step_id].as_ref().unwrap().tone;

        if let Some(ref instrument) = self.instruments[0] {
            self.tracks[0].sequencer.push_relative(
                0.0,
                0.2,
                Fade::Smooth,
                0.0,
                0.05,
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

    fn song_step_count(&self) -> usize {
        16
    }
}
