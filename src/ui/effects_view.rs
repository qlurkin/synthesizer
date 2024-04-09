use super::{Message, State};
use anyhow::Result;
use ratatui::{
    prelude::*,
    widgets::{block::Title, Block, Borders},
};

#[derive(PartialEq, Clone)]
pub enum EffectControl {
    ChorusToReverb,
    DelayToReverb,
}

pub enum EffectMessage {}

pub struct EffectState {
    pub focused: EffectControl,
}

impl Default for EffectState {
    fn default() -> Self {
        Self {
            focused: EffectControl::ChorusToReverb,
        }
    }
}

fn is(state: &State, control: EffectControl) -> bool {
    state.effect_state.focused == control
}

pub fn update_effect(state: &mut State, msg: EffectMessage) -> Result<Vec<Message>> {
    match msg {}
}

pub fn render_effect(area: Rect, buf: &mut Buffer, state: &State) {
    let title = Title::from(" Effects ".bold().red());
    let block = Block::default()
        .title(title.alignment(Alignment::Center))
        .borders(Borders::ALL)
        .border_set(symbols::border::THICK);
    let inner = block.inner(area);
    block.render(area, buf);
    let tracker = &state.tracker;
}
