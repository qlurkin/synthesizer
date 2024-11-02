use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    symbols,
    widgets::Widget,
};

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
