mod effects_view;
mod mixer_view;

use std::{collections::HashMap, time::Duration};

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
    effects_view::{render_effect, update_effect, EffectMessage, EffectState},
    mixer_view::{render_mixer, update_mixer, MixerMessage},
};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Option,
    Edit,
    Play,
    Quit,
}

pub struct State {
    pub tracker: Tracker,
    pub exit: bool,
    pub mixer_state: MixerState,
    pub effect_state: EffectState,
    pub keyboard: HashMap<Key, bool>,
}

impl State {
    pub fn new(tracker: Tracker) -> Self {
        Self {
            tracker,
            exit: false,
            mixer_state: MixerState::default(),
            effect_state: EffectState::default(),
            keyboard: HashMap::new(),
        }
    }
}

pub enum Message {
    Press(Key),
    Release(Key),
    Refresh,
    Play,
    Quit,
    MixerMessage(MixerMessage),
    EffectMessage(EffectMessage),
    Up,
    Down,
    Left,
    Right,
    EditUp,
    EditDown,
    EditLeft,
    EditRight,
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
        Message::Press(key) => {
            let key_state = state.keyboard.entry(key).or_insert(false);
            if !*key_state {
                *key_state = true;
            }
            Ok(vec![])
        }
        Message::Release(key) => {
            state.keyboard.insert(key, false);

            if let Some(down) = state.keyboard.get(&Key::Edit) {
                if *down {
                    return match key {
                        Key::Up => Ok(vec![Message::EditUp]),
                        Key::Down => Ok(vec![Message::EditDown]),
                        Key::Left => Ok(vec![Message::EditLeft]),
                        Key::Right => Ok(vec![Message::EditRight]),
                        _ => Ok(vec![]),
                    };
                }
            }

            match key {
                Key::Quit => Ok(vec![Message::Quit]),
                Key::Up => Ok(vec![Message::Up]),
                Key::Down => Ok(vec![Message::Down]),
                Key::Left => Ok(vec![Message::Left]),
                Key::Right => Ok(vec![Message::Right]),
                Key::Play => Ok(vec![Message::Play]),
                _ => Ok(vec![]),
            }
        }
        Message::EffectMessage(_) => update_effect(state, msg),
        Message::MixerMessage(_) => update_mixer(state, msg),
        Message::Up => update_mixer(state, msg),
        Message::Down => update_mixer(state, msg),
        Message::Left => update_mixer(state, msg),
        Message::Right => update_mixer(state, msg),
        Message::EditUp => update_mixer(state, msg),
        Message::EditDown => update_mixer(state, msg),
        Message::EditLeft => update_mixer(state, msg),
        Message::EditRight => update_mixer(state, msg),
        // _ => Ok(vec![]),
    }
}

pub fn handle_events() -> Result<Vec<Message>> {
    let timeout = Duration::from_secs_f32(1.0 / 60.0);
    if event::poll(timeout)? {
        Ok(match event::read()? {
            Event::Key(key_event) => handle_key_event(key_event),
            _ => vec![],
        })
    } else {
        Ok(vec![])
    }
}

fn handle_key_event(key_event: KeyEvent) -> Vec<Message> {
    let key = match key_event.code {
        KeyCode::Char(' ') => Some(Key::Play),
        KeyCode::Up => Some(Key::Up),
        KeyCode::Down => Some(Key::Down),
        KeyCode::Left => Some(Key::Left),
        KeyCode::Right => Some(Key::Right),
        KeyCode::Char('q') => Some(Key::Quit),
        KeyCode::Char('e') => Some(Key::Edit),
        KeyCode::Char('o') => Some(Key::Option),
        _ => None,
    };

    match key_event.kind {
        KeyEventKind::Press => {
            if let Some(key) = key {
                vec![Message::Press(key)]
            } else {
                vec![]
            }
        }
        KeyEventKind::Release => {
            if let Some(key) = key {
                vec![Message::Release(key)]
            } else {
                vec![]
            }
        }
        _ => vec![],
    }
}

fn render_app(state: &State, area: Rect, buf: &mut Buffer) {
    let title = Title::from(" TermTracker ".bold().red());
    let instructions = Title::from(Line::from(vec![
        " Play ".into(),
        "<Space>".blue().bold(),
        " Edit ".into(),
        "<E>".blue().bold(),
        " Option ".into(),
        "<O>".blue().bold(),
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
        .style(Style::default().cyan())
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
        Rect::new(inner_area.x + 36, inner_area.y + 10, 28, 16),
        buf,
        state,
    );
}
