use ratatui::layout::Rect;

use super::{
    block::block, editable_note::editable_note, frame_context::FrameContext,
    keyboard::process_raw_input, state::State,
};

pub fn render_app(state: &mut State, area: Rect, ctx: &mut FrameContext) {
    process_raw_input(&mut state.keyboard, ctx);

    let inner = block("TermTracker", area, ctx);

    editable_note(&mut state.tracker.tone, inner, ctx);
}
