use super::frame_context::FrameContext;
use ratatui::prelude::*;

pub fn title(txt: &str, area: Rect, ctx: &mut FrameContext) {
    let txt = String::from(txt).red();
    ctx.add(move |buf| {
        Line::from(vec![txt]).render(area, buf);
    })
}
