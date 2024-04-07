use cpal::traits::{DeviceTrait, HostTrait};

mod app;
mod math;
mod tracker;
mod ui;

fn _db_to_volume(db: f32) -> f32 {
    (10.0_f32).powf(0.05 * db)
}

fn _volume_to_db(volume: f32) -> f32 {
    20.0_f32 * volume.log10()
}

fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let config = device.default_output_config().unwrap();

    match config.sample_format() {
        cpal::SampleFormat::I8 => app::run::<i8>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::I16 => app::run::<i16>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::I32 => app::run::<i32>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::I64 => app::run::<i64>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U8 => app::run::<u8>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U16 => app::run::<u16>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U32 => app::run::<u32>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U64 => app::run::<u64>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::F32 => app::run::<f32>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::F64 => app::run::<f64>(&device, &config.into()).unwrap(),
        _ => panic!("Unsupported format"),
    }

    Ok(())
}
