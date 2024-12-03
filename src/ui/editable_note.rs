use ratatui::prelude::*;

use crate::tracker::tone::Tone;

use super::{frame_context::FrameContext, keyboard::InputMessage, message::Message};

fn semitone_up(tone: &mut Option<Tone>, n: u32) {
    if let Some(t) = tone {
        *tone = Some(t.up(n));
    } else {
        *tone = Some(Tone {
            octave: 4,
            semitone: 0,
        });
    }
}

fn semitone_down(tone: &mut Option<Tone>, n: u32) {
    if let Some(t) = tone {
        *tone = Some(t.down(n));
    } else {
        *tone = Some(Tone {
            octave: 4,
            semitone: 0,
        });
    }
}

pub fn editable_note(tone: &mut Option<Tone>, focused: bool, area: Rect, ctx: &mut FrameContext) {
    if focused {
        ctx.process_messages(|msg, _msgs| match msg {
            Message::Input(InputMessage::EditRight) => {
                semitone_up(tone, 1);
                true
            }
            Message::Input(InputMessage::EditLeft) => {
                semitone_down(tone, 1);
                true
            }
            Message::Input(InputMessage::EditUp) => {
                semitone_up(tone, 12);
                true
            }
            Message::Input(InputMessage::EditDown) => {
                semitone_down(tone, 12);
                true
            }
            Message::Input(InputMessage::Clear) => {
                *tone = None;
                true
            }
            _ => false,
        });
    }

    let txt = if let Some(tone) = tone {
        tone.get_string()
    } else {
        "---".into()
    };

    ctx.add(move |buf| {
        let mut line = Line::raw(txt);
        if focused {
            line = line.style(Style::default().fg(Color::Black).bg(Color::White));
        } else {
            line = line.style(Style::default().fg(Color::White));
        }
        line.render(area, buf);
    })
}
