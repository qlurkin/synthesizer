use ratatui::layout::Rect;

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

    let inner = block("TermTracker", area, ctx);

    editable_note(&mut state.tracker.tone, inner, ctx);
}
