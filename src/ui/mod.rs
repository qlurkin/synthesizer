mod mixerview;

use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, Borders,
    },
};

use crate::{tracker::Tracker, ui::mixerview::MixerView};

pub struct State {
    pub tracker: Tracker,
    pub exit: bool,
}

impl State {
    pub fn new(tracker: Tracker) -> Self {
        Self {
            tracker,
            exit: false,
        }
    }
}

pub enum Message {
    Refresh,
    Play,
    Quit,
}

pub fn render(state: &State, frame: &mut Frame) {
    render_app(state, frame.size(), frame.buffer_mut());
}

pub fn update(state: &mut State, msg: Message) -> Result<Vec<Message>> {
    match msg {
        Message::Refresh => {
            state.tracker.tracks[0].snoop0.update();
            state.tracker.tracks[0].snoop1.update();
            state.tracker.snoop_chorus0.update();
            state.tracker.snoop_chorus1.update();
            state.tracker.snoop_delay0.update();
            state.tracker.snoop_delay1.update();
            state.tracker.snoop_reverb0.update();
            state.tracker.snoop_reverb1.update();

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
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(inner_area);

    let mixer = MixerView::new();

    mixer.render(layout[0], buf, &state.tracker);
}
