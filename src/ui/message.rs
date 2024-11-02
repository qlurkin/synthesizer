use super::keyboard::{InputMessage, RawInputMessage};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Message {
    RawInput(RawInputMessage),
    Input(InputMessage),
    Quit,
    Refresh,
}
