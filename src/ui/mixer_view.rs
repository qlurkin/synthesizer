use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

use crate::{tracker::Tracker, ui::meter::Meter};

use super::{
    component::Component, editablevalue::EditableValue, focusmanager::FocusManager, Message,
};

fn snoop_maxer(snoop: &fundsp::hacker::Snoop, samples_nb: usize) -> f32 {
    if let Some(max) = (0..samples_nb)
        .map(|i| snoop.at(i).abs())
        .max_by(|a, b| a.total_cmp(b))
    {
        max
    } else {
        0.0
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum MixerControl {
    Track(usize),
    Chorus,
    Delay,
    Reverb,
}

pub struct MixerView {
    focusmanager: FocusManager<MixerControl>,
}

impl MixerView {
    pub fn new() -> Self {
        let mut focusmanager = FocusManager::new(MixerControl::Track(0));

        for i in 0..8 {
            focusmanager.add(
                MixerControl::Track(i),
                Box::new(EditableValue::new(
                    Box::new(move |tracker: &Tracker| tracker.tracks[i].mix_level.value()),
                    Box::new(move |tracker: &mut Tracker, value: f32| {
                        tracker.tracks[i].mix_level.set(value)
                    }),
                    0.0,
                    1.0,
                )),
            );
        }

        focusmanager.add(
            MixerControl::Chorus,
            Box::new(EditableValue::new(
                Box::new(|tracker: &Tracker| tracker.chorus_mix_level.value()),
                Box::new(|tracker: &mut Tracker, value: f32| tracker.chorus_mix_level.set(value)),
                0.0,
                1.0,
            )),
        );

        focusmanager.add(
            MixerControl::Delay,
            Box::new(EditableValue::new(
                Box::new(|tracker: &Tracker| tracker.delay_mix_level.value()),
                Box::new(|tracker: &mut Tracker, value: f32| tracker.delay_mix_level.set(value)),
                0.0,
                1.0,
            )),
        );

        focusmanager.add(
            MixerControl::Reverb,
            Box::new(EditableValue::new(
                Box::new(|tracker: &Tracker| tracker.reverb_mix_level.value()),
                Box::new(|tracker: &mut Tracker, value: f32| tracker.reverb_mix_level.set(value)),
                0.0,
                1.0,
            )),
        );

        Self { focusmanager }
    }
}

impl Component for MixerView {
    fn update(&mut self, tracker: &mut Tracker, msg: Message) -> Vec<Message> {
        self.focusmanager.update(tracker, msg)
    }

    fn render(&mut self, tracker: &Tracker, area: Rect, buf: &mut Buffer) {
        let title = Span::from(" Mixer ").bold().red();
        let block = Block::default()
            .title_top(Line::from(title).centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK);
        let inner = block.inner(area);
        block.render(area, buf);

        fn render_track(
            focusmanager: &mut FocusManager<MixerControl>,
            tracker: &Tracker,
            control: MixerControl,
            meter0: f32,
            meter1: f32,
            label: String,
            area: Rect,
            buf: &mut Buffer,
        ) {
            Meter::new(meter0).render(Rect::new(area.x, area.y, 1, area.height - 2), buf);
            Meter::new(meter1).render(Rect::new(area.x + 1, area.y, 1, area.height - 2), buf);

            focusmanager.render_component(
                control,
                tracker,
                Rect::new(area.x, area.bottom() - 2, 2, 1),
                buf,
            );

            Line::from(vec![label.gray()]).render(Rect::new(area.x, area.bottom() - 1, 2, 1), buf);
        }

        for i in 0..8 {
            render_track(
                &mut self.focusmanager,
                tracker,
                MixerControl::Track(i),
                snoop_maxer(&tracker.tracks[i].snoop0, 2048),
                snoop_maxer(&tracker.tracks[i].snoop1, 2048),
                format!("T{}", i),
                Rect::new(inner.x + 1 + i as u16 * 3, inner.y, 2, 6),
                buf,
            );
        }

        render_track(
            &mut self.focusmanager,
            tracker,
            MixerControl::Chorus,
            snoop_maxer(&tracker.snoop_chorus0, 2048),
            snoop_maxer(&tracker.snoop_chorus1, 2048),
            "CH".into(),
            Rect::new(inner.x + 25, inner.y, 2, 6),
            buf,
        );

        render_track(
            &mut self.focusmanager,
            tracker,
            MixerControl::Delay,
            snoop_maxer(&tracker.snoop_delay0, 2048),
            snoop_maxer(&tracker.snoop_delay1, 2048),
            "DE".into(),
            Rect::new(inner.x + 28, inner.y, 2, 6),
            buf,
        );

        render_track(
            &mut self.focusmanager,
            tracker,
            MixerControl::Reverb,
            snoop_maxer(&tracker.snoop_reverb0, 2048),
            snoop_maxer(&tracker.snoop_reverb1, 2048),
            "DE".into(),
            Rect::new(inner.x + 31, inner.y, 2, 6),
            buf,
        );
    }
}
