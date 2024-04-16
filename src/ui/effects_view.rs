use crate::math::amp_hex;

use super::{Message, State};
use anyhow::Result;
use ratatui::{
    prelude::*,
    widgets::{block::Title, Block, Borders},
};

#[derive(PartialEq, Clone)]
pub enum EffectControl {
    ChorusToReverb,
    ChorusModFrequency,
    ChorusSeparation,
    ChorusVariation,
    DelayTime,
    DelayDecay,
    DelayToReverb,
    ReverbRoomSize,
    ReverbTime,
    ReverbDiffusion,
    ReverbModulationSpeed,
    ReverbFilterFrequency,
}

pub enum EffectMessage {}

pub struct EffectState {
    pub focused: EffectControl,
}

impl Default for EffectState {
    fn default() -> Self {
        Self {
            focused: EffectControl::ChorusModFrequency,
        }
    }
}

fn control_style(state: &State, control: EffectControl) -> Style {
    if state.effect_state.focused == control {
        Style::default().fg(Color::Black).bg(Color::White)
    } else {
        Style::default()
    }
}

pub fn update_effect(state: &mut State, msg: Message) -> Result<Vec<Message>> {
    match msg {
        _ => Ok(vec![]),
    }
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

    Line::from(vec!["Chorus".red()]).render(Rect::new(inner.x, inner.y, 8, 1), buf);

    Line::from(vec!["Mod Frequency".gray()]).render(Rect::new(inner.x + 8, inner.y, 15, 1), buf);
    Line::from(vec![format!(
        "{:02x}",
        amp_hex(0.0, 20.0, tracker.chorus_mod_frequency)
    )
    .to_uppercase()
    .set_style(control_style(state, EffectControl::ChorusModFrequency))])
    .render(Rect::new(inner.x + 23, inner.y, 15, 1), buf);

    Line::from(vec!["Separation".gray()]).render(Rect::new(inner.x + 8, inner.y + 1, 15, 1), buf);
    Line::from(vec![format!(
        "{:02x}",
        amp_hex(0.0, 5.0, tracker.chorus_separation)
    )
    .to_uppercase()
    .set_style(control_style(state, EffectControl::ChorusSeparation))])
    .render(Rect::new(inner.x + 23, inner.y + 1, 15, 1), buf);

    Line::from(vec!["Variation".gray()]).render(Rect::new(inner.x + 8, inner.y + 2, 15, 1), buf);
    Line::from(vec![format!(
        "{:02x}",
        amp_hex(0.0, 5.0, tracker.chorus_variation)
    )
    .to_uppercase()
    .set_style(control_style(state, EffectControl::ChorusVariation))])
    .render(Rect::new(inner.x + 23, inner.y + 2, 15, 1), buf);

    Line::from(vec!["Reverb Send".gray()]).render(Rect::new(inner.x + 8, inner.y + 3, 15, 1), buf);
    Line::from(vec![format!(
        "{:02x}",
        amp_hex(0.0, 1.0, tracker.chorus_to_reverb_level.value())
    )
    .to_uppercase()
    .set_style(control_style(state, EffectControl::ChorusToReverb))])
    .render(Rect::new(inner.x + 23, inner.y + 3, 15, 1), buf);

    Line::from(vec!["Delay".red()]).render(Rect::new(inner.x, inner.y + 5, 8, 1), buf);

    Line::from(vec!["Time".gray()]).render(Rect::new(inner.x + 8, inner.y + 5, 15, 1), buf);
    Line::from(vec![format!(
        "{:02x}",
        amp_hex(0.0, 5.0, tracker.delay_time)
    )
    .to_uppercase()
    .set_style(control_style(state, EffectControl::DelayTime))])
    .render(Rect::new(inner.x + 23, inner.y + 5, 15, 1), buf);

    Line::from(vec!["Decay".gray()]).render(Rect::new(inner.x + 8, inner.y + 6, 15, 1), buf);
    Line::from(vec![format!(
        "{:02x}",
        amp_hex(0.0, 40.0, tracker.delay_decay)
    )
    .to_uppercase()
    .set_style(control_style(state, EffectControl::DelayDecay))])
    .render(Rect::new(inner.x + 23, inner.y + 6, 15, 1), buf);

    Line::from(vec!["Reverb Send".gray()]).render(Rect::new(inner.x + 8, inner.y + 7, 15, 1), buf);
    Line::from(vec![format!(
        "{:02x}",
        amp_hex(0.0, 1.0, tracker.delay_to_reverb_level.value())
    )
    .to_uppercase()
    .set_style(control_style(state, EffectControl::DelayToReverb))])
    .render(Rect::new(inner.x + 23, inner.y + 7, 15, 1), buf);

    Line::from(vec!["Reverb".red()]).render(Rect::new(inner.x, inner.y + 9, 8, 1), buf);

    Line::from(vec!["Room Size".gray()]).render(Rect::new(inner.x + 8, inner.y + 9, 15, 1), buf);
    Line::from(vec![format!(
        "{:02x}",
        amp_hex(10.0, 30.0, tracker.reverb_room_size)
    )
    .to_uppercase()
    .set_style(control_style(state, EffectControl::ReverbRoomSize))])
    .render(Rect::new(inner.x + 23, inner.y + 9, 15, 1), buf);

    Line::from(vec!["Time".gray()]).render(Rect::new(inner.x + 8, inner.y + 10, 15, 1), buf);
    Line::from(vec![format!(
        "{:02x}",
        amp_hex(0.0, 5.0, tracker.reverb_time)
    )
    .to_uppercase()
    .set_style(control_style(state, EffectControl::ReverbTime))])
    .render(Rect::new(inner.x + 23, inner.y + 10, 15, 1), buf);

    Line::from(vec!["Diffusion".gray()]).render(Rect::new(inner.x + 8, inner.y + 11, 15, 1), buf);
    Line::from(vec![format!(
        "{:02x}",
        amp_hex(0.0, 1.0, tracker.reverb_diffusion)
    )
    .to_uppercase()
    .set_style(control_style(state, EffectControl::ReverbDiffusion))])
    .render(Rect::new(inner.x + 23, inner.y + 11, 15, 1), buf);

    Line::from(vec!["Mod Speed".gray()]).render(Rect::new(inner.x + 8, inner.y + 12, 15, 1), buf);
    Line::from(vec![format!(
        "{:02x}",
        amp_hex(0.0, 1.0, tracker.reverb_modulation_speed)
    )
    .to_uppercase()
    .set_style(control_style(state, EffectControl::ReverbModulationSpeed))])
    .render(Rect::new(inner.x + 23, inner.y + 12, 15, 1), buf);

    Line::from(vec!["Filter Freq".gray()]).render(Rect::new(inner.x + 8, inner.y + 13, 15, 1), buf);
    Line::from(vec![format!(
        "{:02x}",
        amp_hex(20.0, 40000.0, tracker.reverb_filter_frequency)
    )
    .to_uppercase()
    .set_style(control_style(state, EffectControl::ReverbFilterFrequency))])
    .render(Rect::new(inner.x + 23, inner.y + 13, 15, 1), buf);
}
