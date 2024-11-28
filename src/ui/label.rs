use super::frame_context::FrameContext;
use ratatui::prelude::*;

pub fn label(txt: &str, area: Rect, ctx: &mut FrameContext) {
    let txt = String::from(txt).gray();
    ctx.add(move |buf| {
        Line::from(vec![txt]).render(area, buf);
    })
}
