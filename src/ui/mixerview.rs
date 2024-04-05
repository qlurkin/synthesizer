use crate::tracker::Tracker;
use ratatui::{
    prelude::*,
    widgets::{block::Title, Block},
};

fn snoop_averager(snoop: &fundsp::hacker::Snoop<f64>, samples_nb: usize) -> f64 {
    let sum: f64 = (0..samples_nb).map(|i| snoop.at(i).abs()).sum();
    sum / samples_nb as f64
}

pub struct MixerView {}

impl MixerView {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(self, area: Rect, buf: &mut Buffer, tracker: &Tracker) {
        let title = Title::from(" Mixer ".bold());
        let block = Block::default().title(title.alignment(Alignment::Center));
        let inner = block.inner(area);
        block.render(area, buf);

        VerticalSlider::new(
            tracker.tracks[0].mix_level,
            snoop_averager(&tracker.tracks[0].snoop0, 2048),
            snoop_averager(&tracker.tracks[0].snoop1, 2048),
            "T1".into(),
        )
        .render(Rect::new(inner.x + 1, inner.y + 2, 3, 8), buf);
        VerticalSlider::new(tracker.tracks[1].mix_level, 0.0, 0.0, "T2".into())
            .render(Rect::new(inner.x + 4, inner.y + 2, 3, 8), buf);
        VerticalSlider::new(tracker.tracks[2].mix_level, 0.0, 0.0, "T3".into())
            .render(Rect::new(inner.x + 7, inner.y + 2, 3, 8), buf);
        VerticalSlider::new(tracker.tracks[3].mix_level, 0.0, 0.0, "T4".into())
            .render(Rect::new(inner.x + 10, inner.y + 2, 3, 8), buf);
        VerticalSlider::new(tracker.tracks[4].mix_level, 0.0, 0.0, "T5".into())
            .render(Rect::new(inner.x + 13, inner.y + 2, 3, 8), buf);
        VerticalSlider::new(tracker.tracks[5].mix_level, 0.0, 0.0, "T6".into())
            .render(Rect::new(inner.x + 16, inner.y + 2, 3, 8), buf);
        VerticalSlider::new(tracker.tracks[6].mix_level, 0.0, 0.0, "T7".into())
            .render(Rect::new(inner.x + 19, inner.y + 2, 3, 8), buf);
        VerticalSlider::new(tracker.tracks[7].mix_level, 0.0, 0.0, "T8".into())
            .render(Rect::new(inner.x + 22, inner.y + 2, 3, 8), buf);
        VerticalSlider::new(
            tracker.chorus_mix_level.value(),
            snoop_averager(&tracker.snoop_chorus0, 2048),
            snoop_averager(&tracker.snoop_chorus1, 2048),
            "CH".into(),
        )
        .render(Rect::new(inner.x + 25, inner.y + 2, 3, 8), buf);
        VerticalSlider::new(
            tracker.delay_mix_level.value(),
            snoop_averager(&tracker.snoop_delay0, 2048),
            snoop_averager(&tracker.snoop_delay1, 2048),
            "DE".into(),
        )
        .render(Rect::new(inner.x + 28, inner.y + 2, 3, 8), buf);
        VerticalSlider::new(
            tracker.reverb_mix_level.value(),
            snoop_averager(&tracker.snoop_reverb0, 2048),
            snoop_averager(&tracker.snoop_reverb1, 2048),
            "RE".into(),
        )
        .render(Rect::new(inner.x + 31, inner.y + 2, 3, 8), buf);
    }
}

struct VerticalSlider {
    ratio: f64,
    meter0: f64,
    meter1: f64,
    line_set: symbols::line::Set,
    label: String,
}

impl VerticalSlider {
    fn new(ratio: f64, meter0: f64, meter1: f64, label: String) -> Self {
        Self {
            ratio,
            meter0,
            meter1,
            line_set: symbols::line::THICK,
            label,
        }
    }
}

impl Widget for VerticalSlider {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let level0 = ((area.height - 2) as f64 * self.meter0).floor() as u16;
        let level1 = ((area.height - 2) as f64 * self.meter1).floor() as u16;
        let bottom = area.bottom() - 2;
        let value = (255_f64 * self.ratio).round() as u16;
        for i in area.y..(bottom - level0) {
            buf.get_mut(area.x, i)
                .set_symbol(self.line_set.vertical)
                .set_style(
                    Style::default()
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                );
        }
        for i in (bottom - level0)..bottom {
            buf.get_mut(area.x, i)
                .set_symbol(self.line_set.vertical)
                .set_style(Style::default().add_modifier(Modifier::BOLD));
        }
        for i in area.y..(bottom - level1) {
            buf.get_mut(area.x + 1, i)
                .set_symbol(self.line_set.vertical)
                .set_style(
                    Style::default()
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                );
        }
        for i in (bottom - level1)..bottom {
            buf.get_mut(area.x + 1, i)
                .set_symbol(self.line_set.vertical)
                .set_style(Style::default().add_modifier(Modifier::BOLD));
        }
        Line::raw(format!("{:02x}", value).to_uppercase())
            .render(Rect::new(area.x, bottom, 2, 1), buf);
        Line::from(vec![self.label.gray()]).render(Rect::new(area.x, bottom + 1, 2, 1), buf);
    }
}
