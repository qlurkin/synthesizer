use crate::{
    math::to_hex_str_1,
    tracker::{step::Step, tone::Tone},
};

use super::{
    block::block,
    editable_note::editable_note,
    focus_calculator::{view_process_focus_message, FocusCalculator},
    frame_context::FrameContext,
    label::label,
    state::State,
};
use ratatui::prelude::*;

fn get_tone(state: &mut State, phrase_id: usize, step_id: usize) -> Option<Tone> {
    Some(
        state.tracker.get_phrase(phrase_id).steps[step_id]
            .as_ref()?
            .tone,
    )
}

fn set_tone(state: &mut State, phrase_id: usize, step_id: usize, value: Option<Tone>) {
    if let Some(tone) = value {
        if state.tracker.get_phrase(phrase_id).steps[step_id].is_none() {
            state.tracker.get_phrase(phrase_id).steps[step_id] = Some(Step {
                tone,
                instrument: 0,
                velocity: 64,
            })
        }
        state.tracker.get_phrase(phrase_id).steps[step_id]
            .as_mut()
            .unwrap()
            .tone = tone;
    } else {
        state.tracker.get_phrase(phrase_id).steps[step_id] = None;
    }
}

pub fn phrase_view(state: &mut State, focused: bool, area: Rect, ctx: &mut FrameContext) {
    let inner = block(
        " Mixer ".red().bold(),
        None as Option<&str>,
        focused,
        area,
        ctx,
    );

    let mut focus_calculator = FocusCalculator::new(state.phrase_focused);

    (0..16).for_each(|i| {
        label(
            &to_hex_str_1(i),
            Rect::new(inner.x, inner.y + i as u16, 2, 1),
            ctx,
        );

        let mut value = get_tone(state, 0, i as usize);

        let (focused, rect) =
            focus_calculator.add(Rect::new(inner.x + 2, inner.y + i as u16, 3, 1));
        editable_note(&mut value, focused, rect, ctx);

        set_tone(state, 0, i as usize, value);
    });

    let step_cursor = state.tracker.tracks[0].step_cursor;

    ctx.add(move |buf| {
        Line::from(">").render(
            Rect::new(inner.x + 1, inner.y + step_cursor as u16, 1, 1),
            buf,
        );
    });

    view_process_focus_message(&mut state.phrase_focused, &focus_calculator, ctx);
}
