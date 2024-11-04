use crate::tracker::Tracker;

use super::{
    component::Component, editablevalue::EditableValue, focusmanager::FocusManager, Message,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
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

pub struct EffectsView {
    focusmanager: FocusManager<EffectControl>,
}

impl EffectsView {
    pub fn new() -> Self {
        let mut focusmanager = FocusManager::new(EffectControl::ChorusToReverb);

        focusmanager.add(
            EffectControl::ChorusToReverb,
            Box::new(EditableValue::new(
                Box::new(|tracker: &Tracker| tracker.chorus_to_reverb_level.value()),
                Box::new(|tracker: &mut Tracker, value: f32| {
                    tracker.chorus_to_reverb_level.set(value)
                }),
                0.0,
                1.0,
            )),
        );

        focusmanager.add(
            EffectControl::ChorusModFrequency,
            Box::new(EditableValue::new(
                Box::new(|tracker: &Tracker| tracker.chorus_mod_frequency),
                Box::new(|tracker: &mut Tracker, value: f32| {
                    tracker.chorus_mod_frequency = value;
                    tracker.rebuild_chorus();
                }),
                0.0,
                20.0,
            )),
        );

        focusmanager.add(
            EffectControl::ChorusSeparation,
            Box::new(EditableValue::new(
                Box::new(|tracker: &Tracker| tracker.chorus_separation),
                Box::new(|tracker: &mut Tracker, value: f32| {
                    tracker.chorus_separation = value;
                    tracker.rebuild_chorus();
                }),
                0.0,
                5.0,
            )),
        );

        focusmanager.add(
            EffectControl::ChorusVariation,
            Box::new(EditableValue::new(
                Box::new(|tracker: &Tracker| tracker.chorus_variation),
                Box::new(|tracker: &mut Tracker, value: f32| {
                    tracker.chorus_variation = value;
                    tracker.rebuild_chorus();
                }),
                0.0,
                5.0,
            )),
        );

        focusmanager.add(
            EffectControl::DelayTime,
            Box::new(EditableValue::new(
                Box::new(|tracker: &Tracker| tracker.delay_time),
                Box::new(|tracker: &mut Tracker, value: f32| {
                    tracker.delay_time = value;
                    tracker.rebuild_delay();
                }),
                0.0,
                5.0,
            )),
        );

        focusmanager.add(
            EffectControl::DelayDecay,
            Box::new(EditableValue::new(
                Box::new(|tracker: &Tracker| tracker.delay_decay),
                Box::new(|tracker: &mut Tracker, value: f32| {
                    tracker.delay_decay = value;
                    tracker.rebuild_delay();
                }),
                0.0,
                40.0,
            )),
        );

        focusmanager.add(
            EffectControl::DelayToReverb,
            Box::new(EditableValue::new(
                Box::new(|tracker: &Tracker| tracker.delay_to_reverb_level.value()),
                Box::new(|tracker: &mut Tracker, value: f32| {
                    tracker.delay_to_reverb_level.set(value);
                }),
                0.0,
                1.0,
            )),
        );

        focusmanager.add(
            EffectControl::ReverbRoomSize,
            Box::new(EditableValue::new(
                Box::new(|tracker: &Tracker| tracker.reverb_room_size),
                Box::new(|tracker: &mut Tracker, value: f32| {
                    tracker.reverb_room_size = value;
                    tracker.rebuild_reverb();
                }),
                10.0,
                30.0,
            )),
        );

        focusmanager.add(
            EffectControl::ReverbTime,
            Box::new(EditableValue::new(
                Box::new(|tracker: &Tracker| tracker.reverb_time),
                Box::new(|tracker: &mut Tracker, value: f32| {
                    tracker.reverb_time = value;
                    tracker.rebuild_reverb();
                }),
                0.0,
                5.0,
            )),
        );

        focusmanager.add(
            EffectControl::ReverbDiffusion,
            Box::new(EditableValue::new(
                Box::new(|tracker: &Tracker| tracker.reverb_diffusion),
                Box::new(|tracker: &mut Tracker, value: f32| {
                    tracker.reverb_diffusion = value;
                    tracker.rebuild_reverb();
                }),
                0.0,
                1.0,
            )),
        );

        focusmanager.add(
            EffectControl::ReverbModulationSpeed,
            Box::new(EditableValue::new(
                Box::new(|tracker: &Tracker| tracker.reverb_modulation_speed),
                Box::new(|tracker: &mut Tracker, value: f32| {
                    tracker.reverb_modulation_speed = value;
                    tracker.rebuild_reverb();
                }),
                0.0,
                1.0,
            )),
        );

        focusmanager.add(
            EffectControl::ReverbFilterFrequency,
            Box::new(EditableValue::new(
                Box::new(|tracker: &Tracker| tracker.reverb_filter_frequency),
                Box::new(|tracker: &mut Tracker, value: f32| {
                    tracker.reverb_filter_frequency = value;
                    tracker.rebuild_reverb();
                }),
                20.0,
                4000.0,
            )),
        );

        Self { focusmanager }
    }
}

impl Component for EffectsView {
    fn update(&mut self, tracker: &mut Tracker, msg: Message) -> Vec<Message> {
        self.focusmanager.update(tracker, msg)
    }

    fn render(&mut self, tracker: &Tracker, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Effects ".bold().red()).centered();
        let block = Block::default()
            .title_top(title)
            .borders(Borders::ALL)
            .border_set(symbols::border::PLAIN);
        let inner = block.inner(area);
        block.render(area, buf);

        fn render_control(
            focusmanager: &mut FocusManager<EffectControl>,
            control: EffectControl,
            tracker: &Tracker,
            label: String,
            area: Rect,
            buf: &mut Buffer,
        ) {
            Line::from(vec![label.gray()]).render(area, buf);
            focusmanager.render_component(control, tracker, area, buf);
        }

        Line::from(vec!["Chorus".red()]).render(Rect::new(inner.x, inner.y, 8, 1), buf);

        Line::from(vec!["Mod Frequency".gray()])
            .render(Rect::new(inner.x + 8, inner.y, 15, 1), buf);
        self.focusmanager.render_component(
            EffectControl::ChorusModFrequency,
            tracker,
            Rect::new(inner.x + 23, inner.y, 2, 1),
            buf,
        );

        Line::from(vec!["Separation".gray()])
            .render(Rect::new(inner.x + 8, inner.y + 1, 15, 1), buf);
        self.focusmanager.render_component(
            EffectControl::ChorusSeparation,
            tracker,
            Rect::new(inner.x + 23, inner.y + 1, 2, 1),
            buf,
        );

        Line::from(vec!["Variation".gray()])
            .render(Rect::new(inner.x + 8, inner.y + 2, 15, 1), buf);
        self.focusmanager.render_component(
            EffectControl::ChorusVariation,
            tracker,
            Rect::new(inner.x + 23, inner.y + 2, 2, 1),
            buf,
        );

        Line::from(vec!["Reverb Send".gray()])
            .render(Rect::new(inner.x + 8, inner.y + 3, 15, 1), buf);
        self.focusmanager.render_component(
            EffectControl::ChorusToReverb,
            tracker,
            Rect::new(inner.x + 23, inner.y + 3, 2, 1),
            buf,
        );

        Line::from(vec!["Delay".red()]).render(Rect::new(inner.x, inner.y + 5, 8, 1), buf);

        Line::from(vec!["Time".gray()]).render(Rect::new(inner.x + 8, inner.y + 5, 15, 1), buf);
        self.focusmanager.render_component(
            EffectControl::DelayTime,
            tracker,
            Rect::new(inner.x + 23, inner.y + 5, 2, 1),
            buf,
        );

        Line::from(vec!["Decay".gray()]).render(Rect::new(inner.x + 8, inner.y + 6, 15, 1), buf);
        self.focusmanager.render_component(
            EffectControl::DelayDecay,
            tracker,
            Rect::new(inner.x + 23, inner.y + 6, 2, 1),
            buf,
        );

        Line::from(vec!["Reverb Send".gray()])
            .render(Rect::new(inner.x + 8, inner.y + 7, 15, 1), buf);
        self.focusmanager.render_component(
            EffectControl::DelayToReverb,
            tracker,
            Rect::new(inner.x + 23, inner.y + 7, 2, 1),
            buf,
        );

        Line::from(vec!["Reverb".red()]).render(Rect::new(inner.x, inner.y + 9, 8, 1), buf);

        Line::from(vec!["Room Size".gray()])
            .render(Rect::new(inner.x + 8, inner.y + 9, 15, 1), buf);
        self.focusmanager.render_component(
            EffectControl::ReverbRoomSize,
            tracker,
            Rect::new(inner.x + 23, inner.y + 9, 2, 1),
            buf,
        );

        Line::from(vec!["Time".gray()]).render(Rect::new(inner.x + 8, inner.y + 10, 15, 1), buf);
        self.focusmanager.render_component(
            EffectControl::ReverbTime,
            tracker,
            Rect::new(inner.x + 23, inner.y + 10, 2, 1),
            buf,
        );

        Line::from(vec!["Diffusion".gray()])
            .render(Rect::new(inner.x + 8, inner.y + 11, 15, 1), buf);
        self.focusmanager.render_component(
            EffectControl::ReverbDiffusion,
            tracker,
            Rect::new(inner.x + 23, inner.y + 11, 2, 1),
            buf,
        );

        Line::from(vec!["Mod Speed".gray()])
            .render(Rect::new(inner.x + 8, inner.y + 12, 15, 1), buf);
        self.focusmanager.render_component(
            EffectControl::ReverbModulationSpeed,
            tracker,
            Rect::new(inner.x + 23, inner.y + 12, 2, 1),
            buf,
        );

        Line::from(vec!["Filter Freq".gray()])
            .render(Rect::new(inner.x + 8, inner.y + 13, 15, 1), buf);
        self.focusmanager.render_component(
            EffectControl::ReverbFilterFrequency,
            tracker,
            Rect::new(inner.x + 23, inner.y + 13, 2, 1),
            buf,
        );
    }
}
