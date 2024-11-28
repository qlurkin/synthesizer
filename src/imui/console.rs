use ratatui::prelude::*;
use std::sync::Mutex;

static LOGS: Mutex<Vec<Line<'static>>> = Mutex::new(Vec::new());

use super::{block::block, frame_context::FrameContext};

pub fn console_log<T: Into<Line<'static>>>(txt: T) {
    let line: Line = txt.into();
    LOGS.lock().unwrap().push(line);
}

pub fn console(area: Rect, ctx: &mut FrameContext) {
    let inner = block(
        " Console ".red().bold(),
        None as Option<&str>,
        false,
        area,
        ctx,
    );

    let mut txts = LOGS.lock().unwrap().clone();

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
