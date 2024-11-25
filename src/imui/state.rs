use crate::tracker::Tracker;

use super::keyboard::Keyboard;

pub struct State {
    pub keyboard: Keyboard,
    pub tracker: Tracker,
    pub mixer_focused: usize,
}

impl State {
    pub fn new(tracker: Tracker) -> Self {
        Self {
            tracker,
            keyboard: Keyboard::new(),
            mixer_focused: 0,
        }
    }
}
