use rodio::{OutputStream, Source};
use std::time::Duration;

const SAMPLE_RATE: u32 = 44100;

fn _db_to_volume(db: f32) -> f32 {
    (10.0_f32).powf(0.05 * db)
}

fn _volume_to_db(volume: f32) -> f32 {
    20.0_f32 * volume.log10()
}

#[inline]
fn w(freq: f32) -> f32 {
    freq * 2.0 * std::f32::consts::PI
}

#[derive(Clone, Debug)]
struct Oscillator {
    freq: f32,
    sample_rate: u32,
    num_sample: usize,
}

impl Oscillator {
    #[inline]
    fn new(freq: f32, sample_rate: u32) -> Self {
        Self {
            freq,
            sample_rate,
            num_sample: 0,
        }
    }
}

impl Iterator for Oscillator {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);
        Some((w(self.freq) * self.num_sample as f32 / self.sample_rate as f32).sin() * 0.3)
    }
}

impl Source for Oscillator {
    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

fn main() {
    let freq: f32 = 440.0;

    let oscillator = Oscillator::new(freq, SAMPLE_RATE);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let _restult = stream_handle.play_raw(oscillator);

    std::thread::sleep(Duration::from_secs(2));
}
