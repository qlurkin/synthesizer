use fundsp::math::amp_db;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

use crate::{
    math::{db_hex, inc_hex_db_amp},
    tracker::Tracker,
    ui::keyboard::InputMessage,
};

use super::{component::Component, focusmanager::FocusableComponent, message::Message};

pub struct EditableValue {
    get_callback: Box<dyn Fn(&Tracker) -> f32>,
    set_callback: Box<dyn Fn(&mut Tracker, f32)>,
    focused: bool,
}

impl EditableValue {
    pub fn new(
        get_callback: Box<dyn Fn(&Tracker) -> f32>,
        set_callback: Box<dyn Fn(&mut Tracker, f32)>,
    ) -> Self {
        Self {
            set_callback,
            get_callback,
            focused: false,
        }
    }
}

impl Component for EditableValue {
    fn update(&mut self, tracker: &mut Tracker, msg: Message) -> Vec<Message> {
        if let Message::Input(input) = msg {
            let inc: i16 = match input {
                InputMessage::EditUp => 16,
                InputMessage::EditDown => -16,
                InputMessage::EditRight => 1,
                InputMessage::EditLeft => -1,
                _ => 0,
            };

            (self.set_callback)(tracker, inc_hex_db_amp((self.get_callback)(tracker), inc));
            vec![]
        } else {
            vec![]
        }
    }

    fn render(&mut self, tracker: &Tracker, area: Rect, buf: &mut Buffer) {
        let value = db_hex(amp_db((self.get_callback)(tracker)));

        let mut line = Line::raw(format!("{:02x}", value).to_uppercase());
        if self.focused {
            line = line.style(Style::default().fg(Color::Black).bg(Color::White));
        }
        line.render(area, buf);
    }
}

impl FocusableComponent for EditableValue {
    fn focus(&mut self, focused: bool) {
        self.focused = focused;
    }
}
