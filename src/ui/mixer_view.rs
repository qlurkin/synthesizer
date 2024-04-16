use fundsp::math::amp_db;
use ratatui::{
    prelude::*,
    widgets::{block::Title, Block, Borders},
};

use crate::math::{db_hex, inc_hex_db_amp};

use super::{Message, State};
use anyhow::Result;

fn snoop_maxer(snoop: &fundsp::hacker::Snoop<f64>, samples_nb: usize) -> f64 {
    if let Some(max) = (0..samples_nb)
        .map(|i| snoop.at(i).abs())
        .max_by(|a, b| a.total_cmp(b))
    {
        max
    } else {
        0.0
    }
}

#[derive(PartialEq, Clone)]
pub enum MixerControl {
    Track(usize),
    Chorus,
    Delay,
    Reverb,
}

pub enum MixerMessage {
    Inc(MixerControl, i16),
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

pub fn update_mixer(state: &mut State, msg: Message) -> Result<Vec<Message>> {
    match msg {
        Message::EditUp => Ok(vec![Message::MixerMessage(MixerMessage::Inc(
            state.mixer_state.focused.clone(),
            16,
        ))]),
        Message::EditDown => Ok(vec![Message::MixerMessage(MixerMessage::Inc(
            state.mixer_state.focused.clone(),
            -16,
        ))]),
        Message::EditLeft => Ok(vec![Message::MixerMessage(MixerMessage::Inc(
            state.mixer_state.focused.clone(),
            -1,
        ))]),
        Message::EditRight => Ok(vec![Message::MixerMessage(MixerMessage::Inc(
            state.mixer_state.focused.clone(),
            1,
        ))]),
        Message::Left => {
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
        Message::Right => {
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
        Message::MixerMessage(MixerMessage::Inc(control, value)) => {
            match control {
                MixerControl::Track(i) => {
                    state.tracker.tracks[i].mix_level =
                        inc_hex_db_amp(state.tracker.tracks[i].mix_level, value);
                }
                MixerControl::Chorus => {
                    state.tracker.chorus_mix_level.set(inc_hex_db_amp(
                        state.tracker.chorus_mix_level.value(),
                        value,
                    ));
                }
                MixerControl::Delay => {
                    state
                        .tracker
                        .delay_mix_level
                        .set(inc_hex_db_amp(state.tracker.delay_mix_level.value(), value));
                }
                MixerControl::Reverb => {
                    state.tracker.reverb_mix_level.set(inc_hex_db_amp(
                        state.tracker.reverb_mix_level.value(),
                        value,
                    ));
                }
            }
            Ok(vec![])
        }
        _ => Ok(vec![]),
    }
}

pub fn render_mixer(area: Rect, buf: &mut Buffer, state: &State) {
    let title = Title::from(" Mixer ".bold().red());
    let block = Block::default()
        .title(title.alignment(Alignment::Center))
        .borders(Borders::ALL)
        .border_set(symbols::border::THICK);
    let inner = block.inner(area);
    block.render(area, buf);
    let tracker = &state.tracker;

    for i in 0..8 {
        MixControl::new(
            tracker.tracks[i].mix_level,
            snoop_maxer(&tracker.tracks[i].snoop0, 2048),
            snoop_maxer(&tracker.tracks[i].snoop1, 2048),
            format!("T{}", i),
            is(state, MixerControl::Track(i)),
        )
        .render(Rect::new(inner.x + 1 + i as u16 * 3, inner.y, 2, 6), buf);
    }
    MixControl::new(
        tracker.chorus_mix_level.value(),
        snoop_maxer(&tracker.snoop_chorus0, 2048),
        snoop_maxer(&tracker.snoop_chorus1, 2048),
        "CH".into(),
        is(state, MixerControl::Chorus),
    )
    .render(Rect::new(inner.x + 25, inner.y, 2, 6), buf);
    MixControl::new(
        tracker.delay_mix_level.value(),
        snoop_maxer(&tracker.snoop_delay0, 2048),
        snoop_maxer(&tracker.snoop_delay1, 2048),
        "DE".into(),
        is(state, MixerControl::Delay),
    )
    .render(Rect::new(inner.x + 28, inner.y, 2, 6), buf);
    MixControl::new(
        tracker.reverb_mix_level.value(),
        snoop_maxer(&tracker.snoop_reverb0, 2048),
        snoop_maxer(&tracker.snoop_reverb1, 2048),
        "RE".into(),
        is(state, MixerControl::Reverb),
    )
    .render(Rect::new(inner.x + 31, inner.y, 2, 6), buf);
}

struct MixControl {
    gain: f64,
    meter0: f64,
    meter1: f64,
    label: String,
    focused: bool,
}

impl MixControl {
    fn new(gain: f64, meter0: f64, meter1: f64, label: String, focused: bool) -> Self {
        Self {
            gain,
            meter0,
            meter1,
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
        Meter::new(self.meter0).render(Rect::new(area.x, area.y, 1, area.height - 2), buf);
        Meter::new(self.meter1).render(Rect::new(area.x + 1, area.y, 1, area.height - 2), buf);

        let value = db_hex(amp_db(self.gain));

        let mut line = Line::raw(format!("{:02x}", value).to_uppercase());
        if self.focused {
            line = line.style(Style::default().fg(Color::Black).bg(Color::White));
        }
        line.render(Rect::new(area.x, area.bottom() - 2, 2, 1), buf);

        Line::from(vec![self.label.gray()]).render(Rect::new(area.x, area.bottom() - 1, 2, 1), buf);
    }
}

pub struct Meter {
    ratio: f64,
}

impl Meter {
    pub fn new(ratio: f64) -> Self {
        Self { ratio }
    }
}

impl Widget for Meter {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let bar_set = symbols::bar::NINE_LEVELS;
        let symbols = [
            bar_set.empty,
            bar_set.one_eighth,
            bar_set.one_quarter,
            bar_set.three_eighths,
            bar_set.half,
            bar_set.five_eighths,
            bar_set.three_quarters,
            bar_set.seven_eighths,
        ];

        let level = (area.height) as f64 * self.ratio;
        let full = level.floor();
        let partial = ((level - full) * 8.0).floor() as usize;
        let full = full as u16;
        for i in area.y..(area.bottom() - full - 1) {
            buf.get_mut(area.x, i)
                .set_symbol(bar_set.full)
                .set_style(Style::default().fg(Color::Black));
        }
        buf.get_mut(area.x, area.bottom() - full - 1)
            .set_symbol(symbols[partial])
            .set_style(Style::default().bg(Color::Black).fg(Color::Cyan));
        for i in (area.bottom() - full)..area.bottom() {
            buf.get_mut(area.x, i)
                .set_symbol(bar_set.full)
                .set_style(Style::default().fg(Color::Cyan));
        }
    }
}
