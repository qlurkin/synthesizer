use anyhow::{anyhow, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph,
    },
};

use crate::sequencer::Sequencer;

pub fn render(frame: &mut Frame, sequencer: &Sequencer) {
    frame.render_widget(Shell { sequencer }, frame.size())
    // let layout = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
    //     .split(frame.size());
}

pub fn handle_events(sequencer: &mut Sequencer) -> Result<bool> {
    Ok(match event::read()? {
        // it's important to check that the event is a key press event as
        // crossterm also emits key release and repeat events on Windows.
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            handle_key_event(sequencer, key_event)
        }
        _ => false,
    })
}

fn handle_key_event(sequencer: &mut Sequencer, key_event: KeyEvent) -> bool {
    match key_event.code {
        KeyCode::Char('q') => true,
        KeyCode::Left => {
            sequencer.semi_tone_down();
            false
        }
        KeyCode::Right => {
            sequencer.semi_tone_up();
            false
        }
        KeyCode::Char(' ') => {
            sequencer.play_note();
            false
        }
        _ => false,
    }
}

struct Shell<'a> {
    sequencer: &'a Sequencer,
}

impl Widget for Shell<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" KentaW Tracker ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
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

        let counter_text = Text::from(vec![Line::from(vec![
            "Frequency: ".into(),
            self.sequencer.frequency.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
