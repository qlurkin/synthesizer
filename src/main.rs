mod app;
mod engine;
mod sequencer;
mod ui;

fn _db_to_volume(db: f32) -> f32 {
    (10.0_f32).powf(0.05 * db)
}

fn _volume_to_db(volume: f32) -> f32 {
    20.0_f32 * volume.log10()
}

fn main() -> anyhow::Result<()> {
    app::App::new()?.run()
}
