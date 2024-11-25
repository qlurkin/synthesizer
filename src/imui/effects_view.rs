use ratatui::prelude::*;

use super::{
    block::block,
    editable_value::editable_value,
    focus_calculator::{view_process_focus_message, FocusCalculator},
    frame_context::FrameContext,
    label::label,
    state::State,
    title::title,
};

pub fn effects_view(state: &mut State, area: Rect, ctx: &mut FrameContext) {
    let inner = block(" Effects ".red().bold(), None as Option<&str>, area, ctx);

    let mut focus_calculator = FocusCalculator::new(state.effects_focused);

    title("Chorus", Rect::new(inner.x, inner.y, 8, 1), ctx);

    label("Mod Frequency", Rect::new(inner.x + 8, inner.y, 15, 1), ctx);
    let (focused, rect) = focus_calculator.add(Rect::new(inner.x + 23, inner.y, 2, 1));
    let mut value = state.tracker.chorus_mod_frequency;
    editable_value(&mut value, 0.0, 20.0, focused, rect, ctx);
    if value != state.tracker.chorus_mod_frequency {
        state.tracker.chorus_mod_frequency = value;
        state.tracker.rebuild_chorus();
    }

    label(
        "Separation",
        Rect::new(inner.x + 8, inner.y + 1, 15, 1),
        ctx,
    );
    let (focused, rect) = focus_calculator.add(Rect::new(inner.x + 23, inner.y + 1, 2, 1));
    let mut value = state.tracker.chorus_separation;
    editable_value(&mut value, 0.0, 5.0, focused, rect, ctx);
    if value != state.tracker.chorus_separation {
        state.tracker.chorus_separation = value;
        state.tracker.rebuild_chorus();
    }

    label("Variation", Rect::new(inner.x + 8, inner.y + 2, 15, 1), ctx);
    let (focused, rect) = focus_calculator.add(Rect::new(inner.x + 23, inner.y + 2, 2, 1));
    let mut value = state.tracker.chorus_variation;
    editable_value(&mut value, 0.0, 5.0, focused, rect, ctx);
    if value != state.tracker.chorus_variation {
        state.tracker.chorus_variation = value;
        state.tracker.rebuild_chorus();
    }

    label(
        "Reverb Send",
        Rect::new(inner.x + 8, inner.y + 3, 15, 1),
        ctx,
    );
    let (focused, rect) = focus_calculator.add(Rect::new(inner.x + 23, inner.y + 3, 2, 1));
    let mut value = state.tracker.chorus_to_reverb_level.value();
    editable_value(&mut value, 0.0, 1.0, focused, rect, ctx);
    if value != state.tracker.chorus_to_reverb_level.value() {
        state.tracker.chorus_to_reverb_level.set(value);
    }

    title("Delay", Rect::new(inner.x, inner.y + 5, 8, 1), ctx);

    label("Time", Rect::new(inner.x + 8, inner.y + 5, 15, 1), ctx);
    let (focused, rect) = focus_calculator.add(Rect::new(inner.x + 23, inner.y + 5, 2, 1));
    let mut value = state.tracker.delay_time;
    editable_value(&mut value, 0.0, 5.0, focused, rect, ctx);
    if value != state.tracker.delay_time {
        state.tracker.delay_time = value;
        state.tracker.rebuild_delay();
    }

    label("Decay", Rect::new(inner.x + 8, inner.y + 6, 15, 1), ctx);
    let (focused, rect) = focus_calculator.add(Rect::new(inner.x + 23, inner.y + 6, 2, 1));
    let mut value = state.tracker.delay_decay;
    editable_value(&mut value, 0.0, 40.0, focused, rect, ctx);
    if value != state.tracker.delay_decay {
        state.tracker.delay_decay = value;
        state.tracker.rebuild_delay();
    }

    label(
        "Reverb Send",
        Rect::new(inner.x + 8, inner.y + 7, 15, 1),
        ctx,
    );
    let (focused, rect) = focus_calculator.add(Rect::new(inner.x + 23, inner.y + 7, 2, 1));
    let mut value = state.tracker.delay_to_reverb_level.value();
    editable_value(&mut value, 0.0, 1.0, focused, rect, ctx);
    if value != state.tracker.delay_to_reverb_level.value() {
        state.tracker.delay_to_reverb_level.set(value);
    }

    title("Reverb", Rect::new(inner.x, inner.y + 9, 8, 1), ctx);

    label("Room Size", Rect::new(inner.x + 8, inner.y + 9, 15, 1), ctx);
    let (focused, rect) = focus_calculator.add(Rect::new(inner.x + 23, inner.y + 9, 2, 1));
    let mut value = state.tracker.reverb_room_size;
    editable_value(&mut value, 10.0, 30.0, focused, rect, ctx);
    if value != state.tracker.reverb_room_size {
        state.tracker.reverb_room_size = value;
        state.tracker.rebuild_reverb();
    }

    label("Time", Rect::new(inner.x + 8, inner.y + 10, 15, 1), ctx);
    let (focused, rect) = focus_calculator.add(Rect::new(inner.x + 23, inner.y + 10, 2, 1));
    let mut value = state.tracker.reverb_time;
    editable_value(&mut value, 0.0, 5.0, focused, rect, ctx);
    if value != state.tracker.reverb_time {
        state.tracker.reverb_time = value;
        state.tracker.rebuild_reverb();
    }

    label(
        "Diffusion",
        Rect::new(inner.x + 8, inner.y + 11, 15, 1),
        ctx,
    );
    let (focused, rect) = focus_calculator.add(Rect::new(inner.x + 23, inner.y + 11, 2, 1));
    let mut value = state.tracker.reverb_diffusion;
    editable_value(&mut value, 0.0, 1.0, focused, rect, ctx);
    if value != state.tracker.reverb_diffusion {
        state.tracker.reverb_diffusion = value;
        state.tracker.rebuild_reverb();
    }

    label(
        "Mod Speed",
        Rect::new(inner.x + 8, inner.y + 12, 15, 1),
        ctx,
    );
    let (focused, rect) = focus_calculator.add(Rect::new(inner.x + 23, inner.y + 12, 2, 1));
    let mut value = state.tracker.reverb_modulation_speed;
    editable_value(&mut value, 0.0, 1.0, focused, rect, ctx);
    if value != state.tracker.reverb_modulation_speed {
        state.tracker.reverb_modulation_speed = value;
        state.tracker.rebuild_reverb();
    }

    label(
        "Filter Freq",
        Rect::new(inner.x + 8, inner.y + 13, 15, 1),
        ctx,
    );
    let (focused, rect) = focus_calculator.add(Rect::new(inner.x + 23, inner.y + 13, 2, 1));
    let mut value = state.tracker.reverb_filter_frequency;
    editable_value(&mut value, 20.0, 4000.0, focused, rect, ctx);
    if value != state.tracker.reverb_filter_frequency {
        state.tracker.reverb_filter_frequency = value;
        state.tracker.rebuild_reverb();
    }

    view_process_focus_message(&mut state.effects_focused, &focus_calculator, ctx);
}
