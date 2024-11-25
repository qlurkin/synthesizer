use crate::tracker::Tracker;

use super::keyboard::Keyboard;

pub struct State {
    pub keyboard: Keyboard,
    pub tracker: Tracker,
    pub mixer_focused: usize,
    pub effects_focused: usize,
}

impl State {
    pub fn new(tracker: Tracker) -> Self {
        Self {
            tracker,
            keyboard: Keyboard::new(),
            mixer_focused: 0,
            effects_focused: 0,
        }
    }
}
