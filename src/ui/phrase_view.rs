use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

use crate::{
    math::to_hex_str,
    tracker::{Tone, Tracker},
};

use super::{
    component::Component,
    editablenote::EditableNote,
    focusmanager::{FocusManager, FocusableComponent},
    Message,
};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum PhraseControl {
    Note(usize),
}

pub struct PhraseView {
    focusmanager: FocusManager<PhraseControl>,
    focused: bool,
    phrase_id: usize,
}

impl PhraseView {
    pub fn new() -> Self {
        let mut focusmanager = FocusManager::new(PhraseControl::Note(0));

        focusmanager.add(
            PhraseControl::Note(0),
            Box::new(EditableNote::new(
                Box::new(|tracker: &Tracker| tracker.tone),
                Box::new(|tracker: &mut Tracker, value: Tone| tracker.tone = value),
            )),
        );

        Self {
            focusmanager,
            focused: false,
            phrase_id: 0,
        }
    }
}

impl Component for PhraseView {
    fn update(&mut self, tracker: &mut Tracker, msg: Message) -> Vec<Message> {
        self.focusmanager.update_and_navigate(tracker, msg)
    }

    fn render(&mut self, tracker: &Tracker, area: Rect, buf: &mut Buffer) {
        let title = Span::from(format!(" Phrase {} ", to_hex_str(self.phrase_id as u8)))
            .bold()
            .red();
        let block = Block::default()
            .title_top(Line::from(title).centered())
            .borders(Borders::ALL);

        let block = if self.focused {
            block.border_set(symbols::border::THICK)
        } else {
            block.border_set(symbols::border::PLAIN)
        };

        let inner = block.inner(area);
        block.render(area, buf);

        self.focusmanager.render_component(
            PhraseControl::Note(0),
            tracker,
            Rect::new(inner.x, inner.y, 3, 1),
            buf,
        );
    }
}

impl FocusableComponent for PhraseView {
    fn focus(&mut self, focused: bool) {
        self.focused = focused;
    }
}
