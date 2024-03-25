use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph,
    },
};

use crate::sequencer::Sequencer;

pub struct Ui {
    pub sequencer: Sequencer,
}

impl Ui {
    pub fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size())
        // let layout = Layout::default()
        //     .direction(Direction::Horizontal)
        //     .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
        //     .split(frame.size());
    }

    pub fn handle_events(&mut self) -> Result<bool> {
        Ok(match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => false,
        })
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Char('q') => true,
            KeyCode::Left => {
                self.sequencer.semi_tone_down();
                false
            }
            KeyCode::Right => {
                self.sequencer.semi_tone_up();
                false
            }
            KeyCode::Char(' ') => {
                self.sequencer.play_note();
                false
            }
            _ => false,
        }
    }
}

impl Widget for &Ui {
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
            self.sequencer.tone.get_frequency().to_string().yellow(),
            " Semitone: ".into(),
            self.sequencer.tone.semitone.to_string().yellow(),
            " Octave: ".into(),
            self.sequencer.tone.octave.to_string().yellow(),
            " Notation: ".into(),
            self.sequencer.tone.get_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
