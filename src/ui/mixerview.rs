use crossterm::event::KeyCode;
use ratatui::{
    prelude::*,
    widgets::{block::Title, Block},
};

use super::{Message, State};
use anyhow::Result;

fn snoop_averager(snoop: &fundsp::hacker::Snoop<f64>, samples_nb: usize) -> f64 {
    let sum: f64 = (0..samples_nb).map(|i| snoop.at(i).abs()).sum();
    sum / samples_nb as f64
}

#[derive(PartialEq, Clone)]
pub enum MixerControl {
    Track(usize),
    Chorus,
    Delay,
    Reverb,
}

pub enum MixerMessage {
    Inc(MixerControl, f64),
    Up,
    Down,
    Left,
    Right,
}

pub struct MixerState {
    pub focused: MixerControl,
}

impl Default for MixerState {
    fn default() -> Self {
        Self {
            focused: MixerControl::Track(0),
        }
    }
}

fn is(state: &State, control: MixerControl) -> bool {
    state.mixer_state.focused == control
}

pub fn update_mixer(state: &mut State, msg: MixerMessage) -> Result<Vec<Message>> {
    match msg {
        MixerMessage::Up => Ok(vec![Message::MixerMessage(MixerMessage::Inc(
            state.mixer_state.focused.clone(),
            0.1,
        ))]),
        MixerMessage::Down => Ok(vec![Message::MixerMessage(MixerMessage::Inc(
            state.mixer_state.focused.clone(),
            -0.1,
        ))]),
        MixerMessage::Left => {
            match state.mixer_state.focused {
                MixerControl::Track(i) => {
                    if i > 0 {
                        state.mixer_state.focused = MixerControl::Track(i - 1);
                    }
                }
                MixerControl::Chorus => {
                    state.mixer_state.focused = MixerControl::Track(7);
                }
                MixerControl::Delay => {
                    state.mixer_state.focused = MixerControl::Chorus;
                }
                MixerControl::Reverb => {
                    state.mixer_state.focused = MixerControl::Delay;
                }
            }
            Ok(vec![])
        }
        MixerMessage::Right => {
            match state.mixer_state.focused {
                MixerControl::Track(i) => {
                    if i < 7 {
                        state.mixer_state.focused = MixerControl::Track(i + 1);
                    } else {
                        state.mixer_state.focused = MixerControl::Chorus;
                    }
                }
                MixerControl::Chorus => {
                    state.mixer_state.focused = MixerControl::Delay;
                }
                MixerControl::Delay => {
                    state.mixer_state.focused = MixerControl::Reverb;
                }
                _ => {}
            }
            Ok(vec![])
        }
        MixerMessage::Inc(control, value) => {
            match control {
                MixerControl::Track(i) => {
                    state.tracker.tracks[i].mix_level += value;
                }
                MixerControl::Chorus => {
                    state
                        .tracker
                        .chorus_mix_level
                        .set(state.tracker.chorus_mix_level.value() + value);
                }
                MixerControl::Delay => {
                    state
                        .tracker
                        .delay_mix_level
                        .set(state.tracker.delay_mix_level.value() + value);
                }
                MixerControl::Reverb => {
                    state
                        .tracker
                        .reverb_mix_level
                        .set(state.tracker.reverb_mix_level.value() + value);
                }
            }
            Ok(vec![])
        }
        _ => Ok(vec![]),
    }
}

pub fn render_mixer(area: Rect, buf: &mut Buffer, state: &State) {
    let title = Title::from(" Mixer ".bold());
    let block = Block::default().title(title.alignment(Alignment::Center));
    let inner = block.inner(area);
    block.render(area, buf);
    let tracker = &state.tracker;

    MixControl::new(
        tracker.tracks[0].mix_level,
        snoop_averager(&tracker.tracks[0].snoop0, 2048),
        snoop_averager(&tracker.tracks[0].snoop1, 2048),
        "T0".into(),
        is(state, MixerControl::Track(0)),
    )
    .render(Rect::new(inner.x + 1, inner.y + 2, 3, 8), buf);
    MixControl::new(
        tracker.tracks[1].mix_level,
        snoop_averager(&tracker.tracks[1].snoop0, 2048),
        snoop_averager(&tracker.tracks[1].snoop1, 2048),
        "T1".into(),
        is(state, MixerControl::Track(1)),
    )
    .render(Rect::new(inner.x + 4, inner.y + 2, 3, 8), buf);
    MixControl::new(
        tracker.tracks[2].mix_level,
        snoop_averager(&tracker.tracks[2].snoop0, 2048),
        snoop_averager(&tracker.tracks[2].snoop1, 2048),
        "T2".into(),
        is(state, MixerControl::Track(2)),
    )
    .render(Rect::new(inner.x + 7, inner.y + 2, 3, 8), buf);
    MixControl::new(
        tracker.tracks[3].mix_level,
        snoop_averager(&tracker.tracks[3].snoop0, 2048),
        snoop_averager(&tracker.tracks[3].snoop1, 2048),
        "T3".into(),
        is(state, MixerControl::Track(3)),
    )
    .render(Rect::new(inner.x + 10, inner.y + 2, 3, 8), buf);
    MixControl::new(
        tracker.tracks[4].mix_level,
        snoop_averager(&tracker.tracks[4].snoop0, 2048),
        snoop_averager(&tracker.tracks[4].snoop1, 2048),
        "T4".into(),
        is(state, MixerControl::Track(4)),
    )
    .render(Rect::new(inner.x + 13, inner.y + 2, 3, 8), buf);
    MixControl::new(
        tracker.tracks[5].mix_level,
        snoop_averager(&tracker.tracks[5].snoop0, 2048),
        snoop_averager(&tracker.tracks[5].snoop1, 2048),
        "T5".into(),
        is(state, MixerControl::Track(5)),
    )
    .render(Rect::new(inner.x + 16, inner.y + 2, 3, 8), buf);
    MixControl::new(
        tracker.tracks[6].mix_level,
        snoop_averager(&tracker.tracks[6].snoop0, 2048),
        snoop_averager(&tracker.tracks[6].snoop1, 2048),
        "T6".into(),
        is(state, MixerControl::Track(6)),
    )
    .render(Rect::new(inner.x + 19, inner.y + 2, 3, 8), buf);
    MixControl::new(
        tracker.tracks[7].mix_level,
        snoop_averager(&tracker.tracks[7].snoop0, 2048),
        snoop_averager(&tracker.tracks[7].snoop1, 2048),
        "T7".into(),
        is(state, MixerControl::Track(7)),
    )
    .render(Rect::new(inner.x + 22, inner.y + 2, 3, 8), buf);
    MixControl::new(
        tracker.chorus_mix_level.value(),
        snoop_averager(&tracker.snoop_chorus0, 2048),
        snoop_averager(&tracker.snoop_chorus1, 2048),
        "CH".into(),
        is(state, MixerControl::Chorus),
    )
    .render(Rect::new(inner.x + 25, inner.y + 2, 3, 8), buf);
    MixControl::new(
        tracker.delay_mix_level.value(),
        snoop_averager(&tracker.snoop_delay0, 2048),
        snoop_averager(&tracker.snoop_delay1, 2048),
        "DE".into(),
        is(state, MixerControl::Delay),
    )
    .render(Rect::new(inner.x + 28, inner.y + 2, 3, 8), buf);
    MixControl::new(
        tracker.reverb_mix_level.value(),
        snoop_averager(&tracker.snoop_reverb0, 2048),
        snoop_averager(&tracker.snoop_reverb1, 2048),
        "RE".into(),
        is(state, MixerControl::Reverb),
    )
    .render(Rect::new(inner.x + 31, inner.y + 2, 3, 8), buf);
}

struct MixControl {
    gain: f64,
    meter0: f64,
    meter1: f64,
    line_set: symbols::line::Set,
    label: String,
    focused: bool,
}

impl MixControl {
    fn new(gain: f64, meter0: f64, meter1: f64, label: String, focused: bool) -> Self {
        Self {
            gain,
            meter0,
            meter1,
            line_set: symbols::line::THICK,
            label,
            focused,
        }
    }
}

impl Widget for MixControl {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let level0 = ((area.height - 2) as f64 * self.meter0).floor() as u16;
        let level1 = ((area.height - 2) as f64 * self.meter1).floor() as u16;
        let bottom = area.bottom() - 2;
        let value = (255_f64 * self.gain).round() as u16;
        for i in area.y..(bottom - level0) {
            buf.get_mut(area.x, i)
                .set_symbol(self.line_set.vertical)
                .set_style(
                    Style::default()
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                );
        }
        for i in (bottom - level0)..bottom {
            buf.get_mut(area.x, i)
                .set_symbol(self.line_set.vertical)
                .set_style(Style::default().add_modifier(Modifier::BOLD));
        }
        for i in area.y..(bottom - level1) {
            buf.get_mut(area.x + 1, i)
                .set_symbol(self.line_set.vertical)
                .set_style(
                    Style::default()
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                );
        }
        for i in (bottom - level1)..bottom {
            buf.get_mut(area.x + 1, i)
                .set_symbol(self.line_set.vertical)
                .set_style(Style::default().add_modifier(Modifier::BOLD));
        }
        let mut line = Line::raw(format!("{:02x}", value).to_uppercase());

        if self.focused {
            line = line.style(Style::default().fg(Color::Black).bg(Color::White));
        }
        line.render(Rect::new(area.x, bottom, 2, 1), buf);
        Line::from(vec![self.label.gray()]).render(Rect::new(area.x, bottom + 1, 2, 1), buf);
    }
}
