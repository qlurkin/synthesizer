use fundsp::math::amp_db;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

use crate::{
    math::{db_hex, inc_hex_db_amp},
    tracker::Tracker,
};

use super::{
    component::Component,
    focusmanager::{FocusManager, FocusableComponent},
    keyboard::InputMessage,
    Message,
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
                )),
            );
        }

        focusmanager.add(
            MixerControl::Chorus,
            Box::new(EditableValue::new(
                Box::new(|tracker: &Tracker| tracker.chorus_mix_level.value()),
                Box::new(|tracker: &mut Tracker, value: f32| tracker.chorus_mix_level.set(value)),
            )),
        );

        focusmanager.add(
            MixerControl::Delay,
            Box::new(EditableValue::new(
                Box::new(|tracker: &Tracker| tracker.delay_mix_level.value()),
                Box::new(|tracker: &mut Tracker, value: f32| tracker.delay_mix_level.set(value)),
            )),
        );

        focusmanager.add(
            MixerControl::Reverb,
            Box::new(EditableValue::new(
                Box::new(|tracker: &Tracker| tracker.reverb_mix_level.value()),
                Box::new(|tracker: &mut Tracker, value: f32| tracker.reverb_mix_level.set(value)),
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

pub struct Meter {
    ratio: f32,
}

impl Meter {
    pub fn new(ratio: f32) -> Self {
        Self { ratio }
    }
}

impl Widget for Meter {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let bar_set = symbols::bar::NINE_LEVELS;
        let symbols = [
            bar_set.empty,
            bar_set.one_eighth,
            bar_set.one_quarter,
            bar_set.three_eighths,
            bar_set.half,
            bar_set.five_eighths,
            bar_set.three_quarters,
            bar_set.seven_eighths,
        ];

        let level = (area.height) as f32 * self.ratio;
        let full = level.floor();
        let partial = ((level - full) * 8.0).floor() as usize;
        let full = full as u16;
        for i in area.y..(area.bottom() - full - 1) {
            buf.cell_mut((area.x, i))
                .unwrap()
                .set_symbol(bar_set.full)
                .set_style(Style::default().fg(Color::Black));
        }
        buf.cell_mut((area.x, area.bottom() - full - 1))
            .unwrap()
            .set_symbol(symbols[partial])
            .set_style(Style::default().bg(Color::Black).fg(Color::Cyan));
        for i in (area.bottom() - full)..area.bottom() {
            buf.cell_mut((area.x, i))
                .unwrap()
                .set_symbol(bar_set.full)
                .set_style(Style::default().fg(Color::Cyan));
        }
    }
}

pub struct EditableValue {
    get_callback: Box<dyn Fn(&Tracker) -> f32>,
    set_callback: Box<dyn Fn(&mut Tracker, f32)>,
    focused: bool,
}

impl EditableValue {
    pub fn new(
        get_callback: Box<dyn Fn(&Tracker) -> f32>,
        set_callback: Box<dyn Fn(&mut Tracker, f32)>,
    ) -> Self {
        Self {
            set_callback,
            get_callback,
            focused: false,
        }
    }
}

impl Component for EditableValue {
    fn update(&mut self, tracker: &mut Tracker, msg: Message) -> Vec<Message> {
        if let Message::Input(input) = msg {
            let inc: i16 = match input {
                InputMessage::EditUp => 16,
                InputMessage::EditDown => -16,
                InputMessage::EditRight => 1,
                InputMessage::EditLeft => -1,
                _ => 0,
            };

            (self.set_callback)(tracker, inc_hex_db_amp((self.get_callback)(tracker), inc));
            vec![]
        } else {
            vec![]
        }
    }

    fn render(&mut self, tracker: &Tracker, area: Rect, buf: &mut Buffer) {
        let value = db_hex(amp_db((self.get_callback)(tracker)));

        let mut line = Line::raw(format!("{:02x}", value).to_uppercase());
        if self.focused {
            line = line.style(Style::default().fg(Color::Black).bg(Color::White));
        }
        line.render(area, buf);
    }
}

impl FocusableComponent for EditableValue {
    fn focus(&mut self, focused: bool) {
        self.focused = focused;
    }
}
