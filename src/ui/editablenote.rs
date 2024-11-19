use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

use crate::{
    tracker::{Tone, Tracker},
    ui::keyboard::InputMessage,
};

use super::{component::Component, focusmanager::FocusableComponent, message::Message};

type ToneSetCallback = Box<dyn Fn(&mut Tracker, Tone)>;
type ToneGetCallback = Box<dyn Fn(&Tracker) -> Tone>;

pub struct EditableNote {
    get_callback: ToneGetCallback,
    set_callback: ToneSetCallback,
    focused: bool,
}

impl EditableNote {
    pub fn new(get_callback: ToneGetCallback, set_callback: ToneSetCallback) -> Self {
        Self {
            get_callback,
            set_callback,
            focused: false,
        }
    }

    pub fn get_tone(&self, tracker: &Tracker) -> Tone {
        (self.get_callback)(tracker)
    }

    pub fn set_tone(&self, tracker: &mut Tracker, value: Tone) {
        (self.set_callback)(tracker, value);
    }

    pub fn octave_up(&self, tracker: &mut Tracker) {
        let mut tone = self.get_tone(tracker);
        tone = tone.up(12);
        self.set_tone(tracker, tone);
    }

    pub fn octave_down(&self, tracker: &mut Tracker) {
        let mut tone = self.get_tone(tracker);
        tone = tone.down(12);
        self.set_tone(tracker, tone);
    }

    pub fn semitone_up(&self, tracker: &mut Tracker) {
        let mut tone = self.get_tone(tracker);
        tone = tone.up(1);
        self.set_tone(tracker, tone);
    }

    pub fn semitone_down(&self, tracker: &mut Tracker) {
        let mut tone = self.get_tone(tracker);
        tone = tone.down(1);
        self.set_tone(tracker, tone);
    }
}

impl Component for EditableNote {
    fn update(&mut self, tracker: &mut Tracker, msg: Message) -> Vec<Message> {
        if let Message::Input(input) = msg {
            match input {
                InputMessage::EditUp => self.octave_up(tracker),
                InputMessage::EditDown => self.octave_down(tracker),
                InputMessage::EditRight => self.semitone_up(tracker),
                InputMessage::EditLeft => self.semitone_down(tracker),
                _ => {}
            };

            vec![]
        } else {
            vec![]
        }
    }

    fn render(&mut self, tracker: &Tracker, area: Rect, buf: &mut Buffer) {
        let tone = self.get_tone(tracker);
        let mut line = Line::raw(tone.get_string());
        if self.focused {
            line = line.style(Style::default().fg(Color::Black).bg(Color::White));
        } else {
            line = line.style(Style::default().fg(Color::White));
        }
        line.render(area, buf);
    }
}

impl FocusableComponent for EditableNote {
    fn focus(&mut self, focused: bool) {
        self.focused = focused;
    }
}
