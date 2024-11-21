use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

use super::frame_context::FrameContext;

pub fn block(title: &str, area: Rect, ctx: &mut FrameContext) -> Rect {
    let title = Span::from(format!(" {} ", title)).bold().red();
    let bl = Block::default()
        .title_top(Line::from(title).centered())
        .borders(Borders::ALL);

    let bl = bl.border_set(symbols::border::PLAIN);

    let inner = bl.inner(area);

    ctx.add(move |buf| {
        bl.render(area, buf);
    });

    inner
}
