use cpal::traits::{DeviceTrait, HostTrait};

mod app;
mod event;
mod math;
mod tracker;
mod ui;

use app::App;

fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let config = device.default_output_config().unwrap();

    match config.sample_format() {
        cpal::SampleFormat::I8 => App::new().run::<i8>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::I16 => App::new().run::<i16>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::I32 => App::new().run::<i32>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::I64 => App::new().run::<i64>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U8 => App::new().run::<u8>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U16 => App::new().run::<u16>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U32 => App::new().run::<u32>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U64 => App::new().run::<u64>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::F32 => App::new().run::<f32>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::F64 => App::new().run::<f64>(&device, &config.into()).unwrap(),
        _ => panic!("Unsupported format"),
    }

    Ok(())
}

// todo:
// - Add OFF to phrase view
// - edit a basic modular instrument
// - can have multiple instrument and use them in phrase view
// - add chains
// - Use all tracks
// - rework tracker internals: reverb, chorus and delay should not need to be rebuilt for each
//   change. Internals should be modular
