use ratatui::text::Line;

use crate::tracker::Tracker;

use super::keyboard::Keyboard;

pub struct State {
    pub keyboard: Keyboard,
    pub tracker: Tracker,
    pub mixer_focused: usize,
    pub effects_focused: usize,
    pub phrase_focused: usize,
    pub view_focused: usize,
    pub logs: Vec<Line<'static>>,
}

impl State {
    pub fn new(tracker: Tracker) -> Self {
        Self {
            tracker,
            keyboard: Keyboard::new(),
            mixer_focused: 0,
            effects_focused: 0,
            phrase_focused: 0,
            view_focused: 0,
            logs: Vec::new(),
        }
    }
}
