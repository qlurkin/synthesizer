use ratatui::prelude::*;

use crate::tracker::Tone;

use super::{frame_context::FrameContext, keyboard::InputMessage, message::Message};

pub fn editable_note(tone: &mut Tone, area: Rect, ctx: &mut FrameContext) {
    ctx.process_messages(|msg, _msgs| match msg {
        Message::Input(InputMessage::Up) => {
            *tone = tone.up(1);
            true
        }
        _ => false,
    });

    let tone = *tone;

    ctx.add(move |_state, buf| {
        let line = Line::raw(tone.get_string());
        line.style(Style::default().fg(Color::White))
            .render(area, buf);
    })
}
