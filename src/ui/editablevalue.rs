use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

use crate::{tracker::Tracker, ui::keyboard::InputMessage};

use super::{component::Component, focusmanager::FocusableComponent, message::Message};

pub struct EditableValue {
    get_callback: Box<dyn Fn(&Tracker) -> f32>,
    set_callback: Box<dyn Fn(&mut Tracker, f32)>,
    focused: bool,
    min: f32,
    max: f32,
}

impl EditableValue {
    pub fn new(
        get_callback: Box<dyn Fn(&Tracker) -> f32>,
        set_callback: Box<dyn Fn(&mut Tracker, f32)>,
        min: f32,
        max: f32,
    ) -> Self {
        Self {
            set_callback,
            get_callback,
            focused: false,
            min,
            max,
        }
    }

    pub fn get(&self, tracker: &Tracker) -> f32 {
        let value = (self.get_callback)(tracker);
        let value = value.clamp(self.min, self.max);
        value
    }

    pub fn get_as_u8(&self, tracker: &Tracker) -> u8 {
        let value = self.get(tracker);
        let value = (255.0 * (value - self.min) / (self.max - self.min)).round() as u8;
        value
    }

    pub fn get_as_hex(&self, tracker: &Tracker) -> String {
        let value = self.get_as_u8(tracker);
        format!("{:02x}", value).to_uppercase()
    }

    pub fn set(&self, tracker: &mut Tracker, value: f32) {
        let value = value.clamp(self.min, self.max);
        (self.set_callback)(tracker, value);
    }

    pub fn set_from_u8(&self, tracker: &mut Tracker, value: u8) {
        let value = value as f32 / 255.0;
        let value = value * (self.max - self.min) + self.min;
        (self.set_callback)(tracker, value);
    }

    pub fn inc_as_u8(&self, tracker: &mut Tracker, inc: i16) {
        let value = self.get_as_u8(tracker) as i16 + inc;
        let value = value.clamp(0, 255) as u8;
        self.set_from_u8(tracker, value);
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

            self.inc_as_u8(tracker, inc);
            vec![]
        } else {
            vec![]
        }
    }

    fn render(&mut self, tracker: &Tracker, area: Rect, buf: &mut Buffer) {
        let mut line = Line::raw(self.get_as_hex(tracker));
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
