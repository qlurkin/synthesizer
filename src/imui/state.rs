use crate::tracker::Tracker;

use super::keyboard::Keyboard;

pub struct State {
    pub keyboard: Keyboard,
    pub tracker: Tracker,
}
