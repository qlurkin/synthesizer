use ratatui::prelude::*;

use super::{block::block, frame_context::FrameContext, state::State};

pub fn console_log<T: Into<Line<'static>>>(state: &mut State, txt: T) {
    let line: Line = txt.into();
    state.logs.push(line);
}

pub fn console(state: &mut State, area: Rect, ctx: &mut FrameContext) {
    let inner = block(
        " Console ".red().bold(),
        None as Option<&str>,
        false,
        area,
        ctx,
    );

    let mut txts = state.logs.clone();

    ctx.add(move |buf| {
        txts.reverse();
        txts.iter().enumerate().for_each(|(i, line)| {
            line.render(
                Rect::new(
                    inner.x,
                    inner.y + inner.height - 1 - i as u16,
                    inner.width,
                    1,
                ),
                buf,
            );
        });
    });
}
