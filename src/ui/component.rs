use ratatui::{buffer::Buffer, layout::Rect};

use crate::tracker::Tracker;

use super::message::Message;

pub trait Component {
    fn update(&mut self, _tracker: &mut Tracker, _msg: Message) -> Vec<Message> {
        Vec::new()
    }

    fn render(&mut self, _tracker: &Tracker, _area: Rect, _buf: &mut Buffer) {}
}
