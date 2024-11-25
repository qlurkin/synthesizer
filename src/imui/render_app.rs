use ratatui::prelude::*;

use super::{
    block::block,
    effects_view::effects_view,
    focus_calculator::{Direction as Dir, FocusCalculator},
    frame_context::FrameContext,
    graph::graph,
    keyboard::{process_raw_input, InputMessage},
    message::Message,
    mixer_view::mixer_view,
    state::State,
};

pub fn render_app(state: &mut State, area: Rect, ctx: &mut FrameContext) {
    process_raw_input(&mut state.keyboard, ctx);

    let mut focus_calculator = FocusCalculator::new(state.view_focused);

    let mut direction = Dir::None;

    ctx.process_messages(|msg, _msgs| {
        match msg {
            Message::Input(InputMessage::Play) => {
                state.tracker.play_note();
                return true;
            }
            Message::Refresh => {
                for i in 0..8 {
                    state.tracker.tracks[i].snoop0.update();
                    state.tracker.tracks[i].snoop1.update();
                }
                state.tracker.snoop_chorus0.update();
                state.tracker.snoop_chorus1.update();
                state.tracker.snoop_delay0.update();
                state.tracker.snoop_delay1.update();
                state.tracker.snoop_reverb0.update();
                state.tracker.snoop_reverb1.update();
                state.tracker.snoop_out0.update();
                state.tracker.snoop_out1.update();
                return false;
            }
            Message::Input(InputMessage::ShiftRight) => {
                direction = Dir::Right;
                return true;
            }
            Message::Input(InputMessage::ShiftLeft) => {
                direction = Dir::Left;
                return true;
            }
            Message::Input(InputMessage::ShiftUp) => {
                direction = Dir::Up;
                return true;
            }
            Message::Input(InputMessage::ShiftDown) => {
                direction = Dir::Down;
                return true;
            }
            _ => (),
        }
        false
    });

    let title = Span::from(" TermTracker ").bold().red();
    let instructions = Line::from(vec![
        " Play ".into(),
        "<Space>".blue().bold(),
        " Edit ".into(),
        "<C>".blue().bold(),
        " Option ".into(),
        "<X>".blue().bold(),
        " Shift ".into(),
        "<Z>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ]);
    let inner = block(title, Some(instructions), false, area, ctx);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(inner);

    graph(state, layout[0], ctx);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(layout[1]);

    let (focused, rect) = focus_calculator.add(layout[0]);
    if focused {
        mixer_view(state, focused, rect, ctx);
    } else {
        let mut empty_context = FrameContext::new();
        mixer_view(state, focused, rect, &mut empty_context);
        ctx.append_draw_calls(&mut empty_context);
    };

    let (focused, rect) = focus_calculator.add(layout[1]);
    if focused {
        effects_view(state, focused, rect, ctx);
    } else {
        let mut empty_context = FrameContext::new();
        effects_view(state, focused, rect, &mut empty_context);
        ctx.append_draw_calls(&mut empty_context);
    };

    if let Ok(focus_id) = focus_calculator.to(direction) {
        state.view_focused = focus_id;
    }
}
