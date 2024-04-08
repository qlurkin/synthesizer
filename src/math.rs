pub fn _hex_amp(hex: u8) -> f64 {
    hex as f64 / 255.0
}

pub fn _db_to_volume(db: f32) -> f32 {
    (10.0_f32).powf(0.05 * db)
}

pub fn _volume_to_db(volume: f32) -> f32 {
    20.0_f32 * volume.log10()
}
