use crate::{
    event::{Event, EventHandler},
    tracker::Tracker,
    ui::{
        component::Component,
        keyboard::{Key, RawInputMessage},
        message::Message,
        Ui,
    },
};
use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, StreamTrait},
    FromSample, SizedSample,
};
use std::io::stdout;

use crossterm::{
    event::{
        KeyCode, KeyEvent, KeyEventKind, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
        PushKeyboardEnhancementFlags,
    },
    execute,
    terminal::*,
};
use fundsp::hacker::*;
use ratatui::prelude::*;

pub struct App {
    pub events: EventHandler,
    ui: Ui,
}

fn handle_key_event(key_event: KeyEvent) -> Vec<Message> {
    let key = match key_event.code {
        KeyCode::Char(' ') => Some(Key::Play),
        KeyCode::Up => Some(Key::Up),
        KeyCode::Down => Some(Key::Down),
        KeyCode::Left => Some(Key::Left),
        KeyCode::Right => Some(Key::Right),
        KeyCode::Char('q') => Some(Key::Quit),
        KeyCode::Char('x') => Some(Key::Edit),
        KeyCode::Char('z') => Some(Key::Option),
        _ => None,
    };

    match key_event.kind {
        KeyEventKind::Press => {
            if let Some(key) = key {
                vec![Message::RawInput(RawInputMessage::Press(key))]
            } else {
                vec![]
            }
        }
        KeyEventKind::Release => {
            if let Some(key) = key {
                vec![Message::RawInput(RawInputMessage::Release(key))]
            } else {
                vec![]
            }
        }
        _ => vec![],
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            events: EventHandler::new(16),
            ui: Ui::new(),
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run<T: SizedSample + FromSample<f32>>(
        &mut self,
        device: &cpal::Device,
        config: &cpal::StreamConfig,
    ) -> Result<()> {
        let sample_rate = config.sample_rate.0 as f64;
        let channels = config.channels as usize;

        let (mut tracker, mut backend) = Tracker::new(sample_rate);

        let mut next_value = move || backend.get_stereo();

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = device.build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                write_data(data, channels, &mut next_value)
            },
            err_fn,
            None,
        )?;
        stream.play()?;

        // let mut state = State::new(tracker);

        execute!(
            stdout(),
            EnterAlternateScreen,
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
            )
        )?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        while self.ui.running {
            let mut msgs = match self.events.next()? {
                Event::Tick => {
                    terminal.draw(|frame| self.ui.render_frame(&tracker, frame))?;
                    vec![Message::Refresh]
                }
                Event::Key(key_event) => handle_key_event(key_event),
                Event::Mouse(_) => vec![],
                Event::Resize(_, _) => vec![],
            };
            while !msgs.is_empty() {
                msgs = msgs
                    .into_iter()
                    .flat_map(|msg| self.ui.update(&mut tracker, msg).into_iter())
                    .collect();
            }
        }
        execute!(stdout(), LeaveAlternateScreen, PopKeyboardEnhancementFlags)?;
        disable_raw_mode()?;

        Ok(())
    }
}

fn write_data<T: SizedSample + FromSample<f32>>(
    output: &mut [T],
    channels: usize,
    next_sample: &mut dyn FnMut() -> (f32, f32),
) {
    for frame in output.chunks_mut(channels) {
        let sample = next_sample();
        let left: T = T::from_sample(sample.0);
        let right: T = T::from_sample(sample.1);

        for (channel, sample) in frame.iter_mut().enumerate() {
            *sample = if channel & 1 == 0 { left } else { right };
        }
    }
}
