use ratatui::prelude::*;

use super::{
    block::block, frame_context::FrameContext, graph::graph, keyboard::process_raw_input,
    message::Message, mixer_view::mixer_view, state::State,
};

pub fn render_app(state: &mut State, area: Rect, ctx: &mut FrameContext) {
    process_raw_input(&mut state.keyboard, ctx);

    ctx.process_messages(|msg, _msgs| {
        match msg {
            Message::Input(super::keyboard::InputMessage::Play) => {
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
    let inner = block(title, Some(instructions), area, ctx);

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

    mixer_view(state, layout[0], ctx);
}
