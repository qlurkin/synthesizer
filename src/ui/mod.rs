mod effects_view;
mod mixer_view;

use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Axis, Block, Borders, Chart, Dataset, GraphType,
    },
};

use crate::{tracker::Tracker, ui::mixer_view::MixerState};

use self::{
    effects_view::{render_effect, EffectState},
    mixer_view::{render_mixer, update_mixer, MixerMessage},
};

pub struct State {
    pub tracker: Tracker,
    pub exit: bool,
    pub mixer_state: MixerState,
    pub effect_state: EffectState,
}

impl State {
    pub fn new(tracker: Tracker) -> Self {
        Self {
            tracker,
            exit: false,
            mixer_state: MixerState::default(),
            effect_state: EffectState::default(),
        }
    }
}

pub enum Message {
    Refresh,
    Play,
    Quit,
    MixerMessage(MixerMessage),
    Up,
    Down,
    Left,
    Right,
}

pub fn render(state: &State, frame: &mut Frame) {
    render_app(state, frame.size(), frame.buffer_mut());
}

pub fn update(state: &mut State, msg: Message) -> Result<Vec<Message>> {
    match msg {
        Message::Refresh => {
            for i in 0..8 {
                state.tracker.tracks[i].snoop0.update();
                state.tracker.tracks[i].snoop1.update();
            }
            state.tracker.snoop_chorus0.update();
            state.tracker.snoop_chorus1.update();
            state.tracker.snoop_delay0.update();
            state.tracker.snoop_delay1.update();
            state.tracker.snoop_reverb0.update();
            state.tracker.snoop_reverb1.update();
            state.tracker.snoop_out0.update();
            state.tracker.snoop_out1.update();

            handle_events()
        }
        Message::Play => {
            state.tracker.play_note();
            Ok(Vec::new())
        }
        Message::Quit => {
            state.exit = true;
            Ok(Vec::new())
        }
        Message::MixerMessage(mixer_message) => update_mixer(state, mixer_message),
        Message::Up => Ok(vec![Message::MixerMessage(MixerMessage::Up)]),
        Message::Down => Ok(vec![Message::MixerMessage(MixerMessage::Down)]),
        Message::Left => Ok(vec![Message::MixerMessage(MixerMessage::Left)]),
        Message::Right => Ok(vec![Message::MixerMessage(MixerMessage::Right)]),
    }
}

pub fn handle_events() -> Result<Vec<Message>> {
    let timeout = Duration::from_secs_f32(1.0 / 60.0);
    if event::poll(timeout)? {
        Ok(match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                handle_key_event(key_event)
            }
            _ => vec![],
        })
    } else {
        Ok(vec![])
    }
}

fn handle_key_event(key_event: KeyEvent) -> Vec<Message> {
    match key_event.code {
        KeyCode::Char('q') => vec![Message::Quit],
        KeyCode::Char(' ') => vec![Message::Play],
        KeyCode::Up => vec![Message::Up],
        KeyCode::Down => vec![Message::Down],
        KeyCode::Left => vec![Message::Left],
        KeyCode::Right => vec![Message::Right],
        _ => vec![],
    }
}

fn render_app(state: &State, area: Rect, buf: &mut Buffer) {
    let title = Title::from(" KentaW Tracker ".bold());
    let instructions = Title::from(Line::from(vec![
        " Play ".into(),
        "<Space>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ]));
    let block = Block::default()
        .title(title.alignment(Alignment::Center))
        .title(
            instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .borders(Borders::ALL)
        .border_set(symbols::border::THICK);

    let inner_area = block.inner(area);
    block.render(area, buf);

    let graph_area = Rect::new(inner_area.x, inner_area.y, inner_area.width, 10);

    let points = 2048;
    let points0: Vec<(f64, f64)> = (0..points)
        .map(|i| {
            let y = state.tracker.snoop_out0.at(i);
            ((points - i) as f64, y)
        })
        .collect();

    let datasets = vec![Dataset::default()
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().magenta())
        .data(points0.as_slice())];

    let x_axis = Axis::default().bounds([0.0, 2048.0]);

    let y_axis = Axis::default().bounds([-1.0, 1.0]);

    Chart::new(datasets)
        .x_axis(x_axis)
        .y_axis(y_axis)
        .render(graph_area, buf);

    render_mixer(
        Rect::new(inner_area.x, inner_area.y + 10, 36, 8),
        buf,
        state,
    );
    render_effect(
        Rect::new(inner_area.x + 36, inner_area.y + 10, 36, 8),
        buf,
        state,
    );
}
