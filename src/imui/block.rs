use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

use super::frame_context::FrameContext;

pub fn block<T: Into<Line<'static>>, S: Into<Line<'static>>>(
    title: T,
    bottom: Option<S>,
    strong: bool,
    area: Rect,
    ctx: &mut FrameContext,
) -> Rect {
    let title = title.into();
    let mut bl = Block::default()
        .title_top(title.centered())
        .borders(Borders::ALL);

    if let Some(bottom) = bottom {
        let bottom = bottom.into();
        bl = bl.title_bottom(bottom.centered())
    }

    let bl = if strong {
        bl.border_set(symbols::border::THICK)
    } else {
        bl.border_set(symbols::border::PLAIN)
    };

    let inner = bl.inner(area);

    ctx.add(move |buf| {
        bl.render(area, buf);
    });

    inner
}
