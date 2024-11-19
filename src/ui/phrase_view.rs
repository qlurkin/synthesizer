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

        (0..16).for_each(|i| {
            focusmanager.add(
                PhraseControl::Note(i),
                Box::new(EditableNote::new(
                    Box::new(|tracker: &Tracker| Some(tracker.tone)),
                    Box::new(|tracker: &mut Tracker, value: Option<Tone>| {
                        if let Some(tone) = value {
                            tracker.tone = tone
                        }
                    }),
                )),
            )
        });

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

        (0..16).for_each(|i| {
            Line::from(vec![format!("{:x}", i).to_uppercase().gray()])
                .render(Rect::new(inner.x, inner.y + i as u16, 2, 1), buf);
            self.focusmanager.render_component(
                PhraseControl::Note(i),
                tracker,
                Rect::new(inner.x + 2, inner.y + i as u16, 3, 1),
                buf,
            );
        });
    }
}

impl FocusableComponent for PhraseView {
    fn focus(&mut self, focused: bool) {
        self.focused = focused;
    }
}
