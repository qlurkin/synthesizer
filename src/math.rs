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

pub fn _hex_amp(min: f64, max: f64, hex: u8) -> f64 {
    min + hex as f64 * (max - min) / 255.0
}

pub fn amp_hex(min: f64, max: f64, amp: f64) -> u8 {
    let amp = if amp > max { max } else { amp };
    let amp = if amp < min { min } else { amp };

    (255.0 * (amp - min) / (max - min)).round() as u8
}

pub fn _inc_hex_amp(min: f64, max: f64, amp: f64, inc: i16) -> f64 {
    let mut inced = amp_hex(min, max, amp) as i16 + inc;
    if inced > 255 {
        inced = 255;
    }
    if inced < 0 {
        inced = 0;
    }
    _hex_amp(min, max, inced as u8)
}
