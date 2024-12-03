use super::step::Step;

pub struct Phrase {
    pub steps: Vec<Option<Step>>,
}

impl Phrase {
    pub fn new() -> Self {
        Self {
            steps: std::iter::repeat_with(|| None).take(16).collect(),
        }
    }
}
