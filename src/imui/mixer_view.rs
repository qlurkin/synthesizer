use ratatui::prelude::*;

use super::{
    block::block, editable_note::editable_note, frame_context::FrameContext, state::State,
};

pub fn mixer_view(state: &mut State, area: Rect, ctx: &mut FrameContext) {
    let inner = block(" Mixer ".red().bold(), None as Option<&str>, area, ctx);
    editable_note(&mut state.tracker.tone, inner, ctx);
}
