use ratatui::{
    prelude::*,
    widgets::{Axis, Chart, Dataset, GraphType},
};

use super::{frame_context::FrameContext, state::State};

pub fn graph(_state: &mut State, area: Rect, ctx: &mut FrameContext) {
    ctx.add(move |state, buf| {
        let points = 1024;
        let points0: Vec<(f64, f64)> = (0..points)
            .map(|i| {
                let y = state.tracker.snoop_out0.at(i) as f64;
                ((points - i) as f64, y)
            })
            .collect();

        let datasets = vec![Dataset::default()
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().cyan())
            .data(points0.as_slice())];

        let x_axis = Axis::default().bounds([0.0, points as f64]);

        let y_axis = Axis::default().bounds([-1.0, 1.0]);

        Chart::new(datasets)
            .x_axis(x_axis)
            .y_axis(y_axis)
            .render(area, buf);
    });
}
