use ratatui::prelude::*;

use super::{
    block::block, editable_note::editable_note, frame_context::FrameContext,
    keyboard::process_raw_input, message::Message, state::State,
};

pub fn render_app(state: &mut State, area: Rect, ctx: &mut FrameContext) {
    process_raw_input(&mut state.keyboard, ctx);

    ctx.process_messages(|msg, _msgs| {
        if let Message::Input(super::keyboard::InputMessage::Play) = msg {
            state.tracker.play_note();
            return true;
        }
        false
    });

    let title = Span::from(" TermTracker ").bold().red();
    let instructions = Line::from(vec![
        " Play ".into(),
        "<Space>".blue().bold(),
        " Edit ".into(),
        "<C>".blue().bold(),
        " Option ".into(),
        "<X>".blue().bold(),
        " Shift ".into(),
        "<Z>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ]);
    let inner = block(title, Some(instructions), area, ctx);

    editable_note(&mut state.tracker.tone, inner, ctx);
}
