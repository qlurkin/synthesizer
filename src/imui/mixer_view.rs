use ratatui::prelude::*;

use crate::imui::{
    editable_value::editable_value,
    focus_calculator::{view_process_focus_message, FocusCalculator},
    label::label,
    vertical_meter::vertical_meter,
};

use super::{block::block, frame_context::FrameContext, state::State};

fn snoop_maxer(snoop: &fundsp::hacker::Snoop, samples_nb: usize) -> f32 {
    (0..samples_nb)
        .map(|i| snoop.at(i).abs())
        .max_by(|a, b| a.total_cmp(b))
        .unwrap_or(0.0)
}

pub fn mixer_view(state: &mut State, area: Rect, ctx: &mut FrameContext) {
    let inner = block(" Mixer ".red().bold(), None as Option<&str>, area, ctx);

    let mut focus_calculator = FocusCalculator::new(state.mixer_focused);

    fn render_track(
        value: &mut f32,
        meter0: f32,
        meter1: f32,
        txt: &str,
        area: Rect,
        focus_calculator: &mut FocusCalculator,
        ctx: &mut FrameContext,
    ) {
        vertical_meter(meter0, Rect::new(area.x, area.y, 1, area.height - 2), ctx);
        vertical_meter(
            meter1,
            Rect::new(area.x + 1, area.y, 1, area.height - 2),
            ctx,
        );

        let (focused, rect) = focus_calculator.add(Rect::new(area.x, area.bottom() - 2, 2, 1));
        editable_value(value, 0.0, 1.0, focused, rect, ctx);

        label(txt, Rect::new(area.x, area.bottom() - 1, 2, 1), ctx);
    }

    for i in 0..8 {
        let mut value = state.tracker.tracks[i].mix_level.value();
        render_track(
            &mut value,
            snoop_maxer(&state.tracker.tracks[i].snoop0, 2048),
            snoop_maxer(&state.tracker.tracks[i].snoop1, 2048),
            &format!("T{}", i),
            Rect::new(inner.x + 1 + i as u16 * 3, inner.y, 2, 6),
            &mut focus_calculator,
            ctx,
        );
        state.tracker.tracks[i].mix_level.set(value);
    }

    let mut value = state.tracker.chorus_mix_level.value();
    render_track(
        &mut value,
        snoop_maxer(&state.tracker.snoop_chorus0, 2048),
        snoop_maxer(&state.tracker.snoop_chorus1, 2048),
        "CH",
        Rect::new(inner.x + 25, inner.y, 2, 6),
        &mut focus_calculator,
        ctx,
    );
    state.tracker.chorus_mix_level.set(value);

    let mut value = state.tracker.delay_mix_level.value();
    render_track(
        &mut value,
        snoop_maxer(&state.tracker.snoop_delay0, 2048),
        snoop_maxer(&state.tracker.snoop_delay1, 2048),
        "DE",
        Rect::new(inner.x + 28, inner.y, 2, 6),
        &mut focus_calculator,
        ctx,
    );
    state.tracker.delay_mix_level.set(value);

    let mut value = state.tracker.reverb_mix_level.value();
    render_track(
        &mut value,
        snoop_maxer(&state.tracker.snoop_reverb0, 2048),
        snoop_maxer(&state.tracker.snoop_reverb1, 2048),
        "RE",
        Rect::new(inner.x + 31, inner.y, 2, 6),
        &mut focus_calculator,
        ctx,
    );
    state.tracker.reverb_mix_level.set(value);

    view_process_focus_message(&mut state.mixer_focused, &focus_calculator, ctx);
}
