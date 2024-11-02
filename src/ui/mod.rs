pub mod component;
// mod effects_view;
mod focusmanager;
pub mod keyboard;
pub mod message;
mod mixer_view;

use component::Component;
use keyboard::Keyboard;
use message::Message;
use mixer_view::MixerView;
use ratatui::{
    prelude::*,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
};

use crate::tracker::Tracker;

// pub struct State {
//     pub tracker: Tracker,
//     pub exit: bool,
//     pub mixer_state: MixerState,
//     pub effect_state: EffectState,
//     pub keyboard: HashMap<Key, bool>,
// }
//

pub struct Ui {
    keyboard: Keyboard,
    pub running: bool,
    mixer_view: MixerView,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            keyboard: Keyboard::new(),
            running: true,
            mixer_view: MixerView::new(),
        }
    }

    pub fn render_frame(&mut self, tracker: &Tracker, frame: &mut Frame) {
        self.render(tracker, frame.area(), frame.buffer_mut());
    }
}

impl Component for Ui {
    fn update(&mut self, tracker: &mut Tracker, msg: Message) -> Vec<Message> {
        match msg {
            Message::Refresh => {
                for i in 0..8 {
                    tracker.tracks[i].snoop0.update();
                    tracker.tracks[i].snoop1.update();
                }
                tracker.snoop_chorus0.update();
                tracker.snoop_chorus1.update();
                tracker.snoop_delay0.update();
                tracker.snoop_delay1.update();
                tracker.snoop_reverb0.update();
                tracker.snoop_reverb1.update();
                tracker.snoop_out0.update();
                tracker.snoop_out1.update();

                vec![]
            }
            Message::Input(keyboard::InputMessage::Play) => {
                tracker.play_note();
                Vec::new()
            }
            Message::Quit => {
                self.running = false;
                Vec::new()
            }
            _ => {
                let mut msgs = self.mixer_view.update(tracker, msg);
                msgs.append(&mut self.keyboard.update(tracker, msg));
                msgs
            }
        }
    }

    fn render(&mut self, tracker: &Tracker, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" TermTracker ".bold().red()).centered();
        let instructions = Line::from(vec![
            " Play ".into(),
            "<Space>".blue().bold(),
            " Edit ".into(),
            "<X>".blue().bold(),
            " Option ".into(),
            "<Z>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ])
        .centered();
        let block = Block::default()
            .title_top(title)
            .title_bottom(instructions)
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK);

        let inner_area = block.inner(area);
        block.render(area, buf);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(inner_area);

        // let graph_area = Rect::new(inner_area.x, inner_area.y, inner_area.width, 10);

        let points = 1024;
        let points0: Vec<(f64, f64)> = (0..points)
            .map(|i| {
                let y = tracker.snoop_out0.at(i) as f64;
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
            .render(layout[0], buf);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(20)])
            .split(layout[1]);

        self.mixer_view.render(tracker, layout[0], buf);
    }
}

// impl State {
//     pub fn new(tracker: Tracker) -> Self {
//         Self {
//             tracker,
//             exit: false,
//             mixer_state: MixerState::default(),
//             effect_state: EffectState::default(),
//             keyboard: HashMap::new(),
//         }
//     }
// }

// pub fn render(state: &State, frame: &mut Frame) {
//     render_app(state, frame.area(), frame.buffer_mut());
// }

// pub fn update(state: &mut State, msg: Message) -> Vec<Message> {
//     match msg {
//         Message::Refresh => {
//             for i in 0..8 {
//                 state.tracker.tracks[i].snoop0.update();
//                 state.tracker.tracks[i].snoop1.update();
//             }
//             state.tracker.snoop_chorus0.update();
//             state.tracker.snoop_chorus1.update();
//             state.tracker.snoop_delay0.update();
//             state.tracker.snoop_delay1.update();
//             state.tracker.snoop_reverb0.update();
//             state.tracker.snoop_reverb1.update();
//             state.tracker.snoop_out0.update();
//             state.tracker.snoop_out1.update();
//
//             vec![]
//         }
//         Message::Play => {
//             state.tracker.play_note();
//             Vec::new()
//         }
//         Message::Quit => {
//             state.exit = true;
//             Vec::new()
//         }
//         Message::Press(key) => {
//             let key_state = state.keyboard.entry(key).or_insert(false);
//             if !*key_state {
//                 *key_state = true;
//             }
//             vec![]
//         }
//         Message::Release(key) => {
//             state.keyboard.insert(key, false);
//
//             if let Some(down) = state.keyboard.get(&Key::Edit) {
//                 if *down {
//                     return match key {
//                         Key::Up => vec![Message::EditUp],
//                         Key::Down => vec![Message::EditDown],
//                         Key::Left => vec![Message::EditLeft],
//                         Key::Right => vec![Message::EditRight],
//                         _ => vec![],
//                     };
//                 }
//             }
//
//             match key {
//                 Key::Quit => vec![Message::Quit],
//                 Key::Up => vec![Message::Up],
//                 Key::Down => vec![Message::Down],
//                 Key::Left => vec![Message::Left],
//                 Key::Right => vec![Message::Right],
//                 Key::Play => vec![Message::Play],
//                 _ => vec![],
//             }
//         }
//         Message::EffectMessage(_) => update_effect(state, msg),
//         Message::MixerMessage(_) => update_mixer(state, msg),
//         Message::Up => update_mixer(state, msg),
//         Message::Down => update_mixer(state, msg),
//         Message::Left => update_mixer(state, msg),
//         Message::Right => update_mixer(state, msg),
//         Message::EditUp => update_mixer(state, msg),
//         Message::EditDown => update_mixer(state, msg),
//         Message::EditLeft => update_mixer(state, msg),
//         Message::EditRight => update_mixer(state, msg),
//     }
// }

// fn render_app(state: &State, area: Rect, buf: &mut Buffer) {
//     let title = Title::from(" TermTracker ".bold().red());
//     let instructions = Title::from(Line::from(vec![
//         " Play ".into(),
//         "<Space>".blue().bold(),
//         " Edit ".into(),
//         "<X>".blue().bold(),
//         " Option ".into(),
//         "<Z>".blue().bold(),
//         " Quit ".into(),
//         "<Q> ".blue().bold(),
//     ]));
//     let block = Block::default()
//         .title(title.alignment(Alignment::Center))
//         .title(
//             instructions
//                 .alignment(Alignment::Center)
//                 .position(Position::Bottom),
//         )
//         .borders(Borders::ALL)
//         .border_set(symbols::border::THICK);
//
//     let inner_area = block.inner(area);
//     block.render(area, buf);
//
//     let layout = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
//         .split(inner_area);
//
//     // let graph_area = Rect::new(inner_area.x, inner_area.y, inner_area.width, 10);
//
//     let points = 1024;
//     let points0: Vec<(f64, f64)> = (0..points)
//         .map(|i| {
//             let y = state.tracker.snoop_out0.at(i) as f64;
//             ((points - i) as f64, y)
//         })
//         .collect();
//
//     let datasets = vec![Dataset::default()
//         .marker(symbols::Marker::Braille)
//         .graph_type(GraphType::Line)
//         .style(Style::default().cyan())
//         .data(points0.as_slice())];
//
//     let x_axis = Axis::default().bounds([0.0, points as f64]);
//
//     let y_axis = Axis::default().bounds([-1.0, 1.0]);
//
//     Chart::new(datasets)
//         .x_axis(x_axis)
//         .y_axis(y_axis)
//         .render(layout[0], buf);
//
//     let layout = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(20)])
//         .split(layout[1]);
//
//     render_mixer(layout[0], buf, state);
//     render_effect(layout[1], buf, state);
// }
