use crate::{
    tracker::Tracker,
    ui::{render, update, Message, State},
};
use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, StreamTrait},
    FromSample, SizedSample,
};
use std::io::stdout;

use crossterm::{execute, terminal::*};
use fundsp::hacker::*;
use ratatui::prelude::*;

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

/// runs the application's main loop until the user quits
pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<()>
where
    T: SizedSample + FromSample<f64>,
{
    let sample_rate = config.sample_rate.0 as f64;
    let channels = config.channels as usize;

    let (tracker, mut backend) = Tracker::new(sample_rate);

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

    let mut state = State::new(tracker);

    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    while !state.exit {
        terminal.draw(|frame| render(&state, frame))?;
        let mut msgs = vec![Message::Refresh];
        while !msgs.is_empty() {
            msgs = msgs
                .into_iter()
                .flat_map(|msg| update(&mut state, msg).unwrap().into_iter())
                .collect();
        }
    }
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
