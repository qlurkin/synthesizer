use super::tone::Tone;

pub struct Step {
    pub tone: Tone,
    pub instrument: usize,
    pub velocity: u8,
}
