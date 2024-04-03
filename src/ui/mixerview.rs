use crate::tracker::Tracker;
use ratatui::{
    prelude::*,
    widgets::{block::Title, Block, Paragraph},
};

pub struct MixerView<'a> {
    pub tracker: &'a Tracker,
}

impl Widget for MixerView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Title::from(" Mixer ".bold());
        let block = Block::default().title(title.alignment(Alignment::Center));
        let inner = block.inner(area);
        block.render(area, buf);

        let text = Text::from(vec![Line::from(vec![
            "Frequency: ".into(),
            self.tracker.tone.get_frequency().to_string().yellow(),
            " Notation: ".into(),
            self.tracker.tone.get_string().yellow(),
        ])]);

        Paragraph::new(text).centered().render(inner, buf);
    }
}
