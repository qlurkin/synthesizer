use crate::{ui::keyboard::InputMessage, math::to_hex_str_2};

use super::{frame_context::FrameContext, message::Message};
use ratatui::prelude::*;

pub fn editable_value(
    value: &mut f32,
    min: f32,
    max: f32,
    focused: bool,
    area: Rect,
    ctx: &mut FrameContext,
) {
    let mut byte = (255.0 * (*value - min) / (max - min)).round() as u8;

    if focused {
        ctx.process_messages(|msg, _msgs| {
            if let Message::Input(input) = msg {
                let inc: i16 = match input {
                    InputMessage::EditUp => 16,
                    InputMessage::EditDown => -16,
                    InputMessage::EditRight => 1,
                    InputMessage::EditLeft => -1,
                    _ => 0,
                };

                byte = (byte as i16 + inc).clamp(0, 255) as u8;
                let ratio = byte as f32 / 255.0;
                *value = ratio * (max - min) + min;
                inc != 0
            } else {
                false
            }
        });
    }

    let hex = to_hex_str_2(byte);

    ctx.add(move |buf| {
        let mut line = Line::raw(hex);
        if focused {
            line = line.style(Style::default().fg(Color::Black).bg(Color::White));
        } else {
            line = line.style(Style::default().fg(Color::White));
        }
        line.render(area, buf);
    })
}
