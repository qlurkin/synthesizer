use crate::{tracker::Tracker, ui::Ui};
use anyhow::{anyhow, Result};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, SizedSample,
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    prelude::Stylize,
    symbols,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph, Widget,
    },
};
use std::io::stdout;

use crossterm::{execute, terminal::*};
use fundsp::hacker::*;
use ratatui::prelude::*;

pub fn stream_setup_for(
    backend: fundsp::realseq::SequencerBackend64,
) -> Result<(cpal::Host, cpal::Device, cpal::StreamConfig, cpal::Stream)> {
    let (host, device, config) = host_device_setup()?;

    let (stream, config) = match config.sample_format() {
        cpal::SampleFormat::I8 => make_stream::<i8>(&device, config.into(), backend),
        cpal::SampleFormat::I16 => make_stream::<i16>(&device, config.into(), backend),
        cpal::SampleFormat::I32 => make_stream::<i32>(&device, config.into(), backend),
        cpal::SampleFormat::I64 => make_stream::<i64>(&device, config.into(), backend),
        cpal::SampleFormat::U8 => make_stream::<u8>(&device, config.into(), backend),
        cpal::SampleFormat::U16 => make_stream::<u16>(&device, config.into(), backend),
        cpal::SampleFormat::U32 => make_stream::<u32>(&device, config.into(), backend),
        cpal::SampleFormat::U64 => make_stream::<u64>(&device, config.into(), backend),
        cpal::SampleFormat::F32 => make_stream::<f32>(&device, config.into(), backend),
        cpal::SampleFormat::F64 => make_stream::<f64>(&device, config.into(), backend),
        sample_format => Err(anyhow!("Unsupported sample format '{sample_format}'")),
    }?;

    Ok((host, device, config, stream))
}

pub fn host_device_setup() -> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig)> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow!("Default output device is not available"))?;

    let config = device.default_output_config()?;

    Ok((host, device, config))
}

pub fn make_stream<T>(
    device: &cpal::Device,
    config: cpal::StreamConfig,
    mut backend: fundsp::realseq::SequencerBackend64,
) -> Result<(cpal::Stream, cpal::StreamConfig)>
where
    T: SizedSample + FromSample<f64>,
{
    let num_channels = config.channels as usize;

    let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

    let mut pipeline = multipass() & (5.0 * reverb_stereo(10.0, 0.5));

    let mut next_sample = move || {
        let (left_sample, right_sample) = backend.get_stereo();
        pipeline.filter_stereo(left_sample, right_sample)
    };

    let stream = device.build_output_stream(
        &config,
        move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(output, num_channels, &mut next_sample)
        },
        err_fn,
        None,
    )?;

    Ok((stream, config))
}

fn write_data<T: SizedSample + FromSample<f64>>(
    output: &mut [T],
    channels: usize,
    next_sample: &mut dyn FnMut() -> (f64, f64),
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

// fn process_frame<SampleType>(output: &mut [SampleType], engine: &mut Engine, num_channels: usize)
// where
//     SampleType: Sample + FromSample<f32>,
// {
//     for frame in output.chunks_mut(num_channels) {
//         let value: SampleType = SampleType::from_sample(engine.tick());
//
//         // copy the same value to all channels
//         for sample in frame.iter_mut() {
//             *sample = value;
//         }
//     }
// }

pub struct App {
    _host: cpal::Host,
    _device: cpal::Device,
    _config: cpal::StreamConfig,
    _stream: cpal::Stream,
    frequency: f32,
    exit: bool,
    ui: Ui,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut frontend = fundsp::sequencer::Sequencer64::new(false, 2);
        let backend = frontend.backend();
        let (host, device, config, stream) = stream_setup_for(backend)?;
        stream.play()?;

        Ok(Self {
            _host: host,
            _device: device,
            _config: config,
            _stream: stream,
            frequency: 440.0,
            exit: false,
            ui: Ui {
                sequencer: Tracker::new(frontend),
            },
        })
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self) -> Result<()> {
        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        while !self.exit {
            // terminal.draw(|frame| self.render_frame(frame))?;
            terminal.draw(|frame| self.ui.render_frame(frame))?;
            self.exit = self.ui.handle_events()?;
        }
        execute!(stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" KentaW Tracker ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Frequency: ".into(),
            self.frequency.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
