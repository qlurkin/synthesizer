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

type ToneSetCallback = Box<dyn Fn(&mut Tracker, Option<Tone>)>;
type ToneGetCallback = Box<dyn Fn(&Tracker) -> Option<Tone>>;

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

    pub fn get_tone(&self, tracker: &Tracker) -> Option<Tone> {
        (self.get_callback)(tracker)
    }

    pub fn set_tone(&self, tracker: &mut Tracker, value: Option<Tone>) {
        (self.set_callback)(tracker, value);
    }

    pub fn octave_up(&self, tracker: &mut Tracker) {
        self.semitone_up(tracker, 12);
    }

    pub fn octave_down(&self, tracker: &mut Tracker) {
        self.semitone_down(tracker, 12);
    }

    pub fn semitone_up(&self, tracker: &mut Tracker, n: u32) {
        let tone = self.get_tone(tracker);
        if let Some(tone) = tone {
            let tone = tone.up(n);
            self.set_tone(tracker, Some(tone));
        } else {
            self.set_tone(
                tracker,
                Some(Tone {
                    octave: 4,
                    semitone: 0,
                }),
            );
        }
    }

    pub fn semitone_down(&self, tracker: &mut Tracker, n: u32) {
        let tone = self.get_tone(tracker);
        if let Some(tone) = tone {
            let tone = tone.down(n);
            self.set_tone(tracker, Some(tone));
        } else {
            self.set_tone(
                tracker,
                Some(Tone {
                    octave: 4,
                    semitone: 0,
                }),
            );
        }
    }
}

impl Component for EditableNote {
    fn update(&mut self, tracker: &mut Tracker, msg: Message) -> Vec<Message> {
        if let Message::Input(input) = msg {
            match input {
                InputMessage::EditUp => self.octave_up(tracker),
                InputMessage::EditDown => self.octave_down(tracker),
                InputMessage::EditRight => self.semitone_up(tracker, 1),
                InputMessage::EditLeft => self.semitone_down(tracker, 1),
                _ => {}
            };

            vec![]
        } else {
            vec![]
        }
    }

    fn render(&mut self, tracker: &Tracker, area: Rect, buf: &mut Buffer) {
        let tone = self.get_tone(tracker);

        let txt = if let Some(tone) = tone {
            tone.get_string()
        } else {
            "---".into()
        };

        let mut line = Line::raw(txt);
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
