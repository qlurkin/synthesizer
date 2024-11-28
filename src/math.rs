use fundsp::math::{amp_db, db_amp};

#[allow(dead_code)]
pub fn hex_db(hex: u8) -> f32 {
    (hex as f32 - 224.0) / 4.0
}

#[allow(dead_code)]
pub fn db_hex(db: f32) -> u8 {
    (db * 4.0 + 224.0).round() as u8
}

#[allow(dead_code)]
pub fn inc_hex_db_amp(amp: f32, inc: i16) -> f32 {
    let db = amp_db(amp);
    let mut inced = db_hex(db) as i16 + inc;
    inced = inced.clamp(0, 255);

    db_amp(hex_db(inced as u8))
}

pub fn _hex_amp(min: f32, max: f32, hex: u8) -> f32 {
    min + hex as f32 * (max - min) / 255.0
}

#[allow(dead_code)]
pub fn amp_hex(min: f32, max: f32, amp: f32) -> u8 {
    let amp = if amp > max { max } else { amp };
    let amp = if amp < min { min } else { amp };

    (255.0 * (amp - min) / (max - min)).round() as u8
}

pub fn _inc_hex_amp(min: f32, max: f32, amp: f32, inc: i16) -> f32 {
    let mut inced = amp_hex(min, max, amp) as i16 + inc;
    inced = inced.clamp(0, 255);
    _hex_amp(min, max, inced as u8)
}

pub fn to_hex_str_2(value: u8) -> String {
    format!("{:02x}", value).to_uppercase()
}

#[allow(dead_code)]
pub fn to_hex_str_1(value: u8) -> String {
    format!("{:x}", value).to_uppercase()
}
