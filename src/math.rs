// pub fn _hex_amp(hex: u8) -> f64 {
//     hex as f64 / 255.0
// }
//
// pub fn _db_to_volume(db: f32) -> f32 {
//     (10.0_f32).powf(0.05 * db)
// }
//
// pub fn _volume_to_db(volume: f32) -> f32 {
//     20.0_f32 * volume.log10()
// }

use fundsp::math::{amp_db, db_amp};

pub fn hex_db(hex: u8) -> f64 {
    (hex as f64 - 224.0) / 4.0
}

pub fn db_hex(db: f64) -> u8 {
    (db * 4.0 + 224.0).round() as u8
}

pub fn inc_hex_db_amp(amp: f64, inc: i16) -> f64 {
    let db = amp_db(amp);
    let mut inced = db_hex(db) as i16 + inc;
    if inced > 255 {
        inced = 255;
    }
    if inced < 0 {
        inced = 0;
    }

    db_amp(hex_db(inced as u8))
}

pub fn hex_db_amp(hex: u8) -> f64 {
    if hex == 0 {
        0.0
    } else {
        db_amp(hex_db(hex))
    }
}
