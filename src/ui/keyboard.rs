use std::collections::HashMap;

use super::{component::Component, message::Message};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Option,
    Edit,
    Play,
    Quit,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum InputMessage {
    Play,
    Up,
    Down,
    Left,
    Right,
    EditUp,
    EditDown,
    EditLeft,
    EditRight,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum RawInputMessage {
    Press(Key),
    Release(Key),
}

pub struct Keyboard {
    keys: HashMap<Key, bool>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }
}

impl Component for Keyboard {
    fn update(&mut self, _tracker: &mut crate::tracker::Tracker, msg: Message) -> Vec<Message> {
        if let Message::RawInput(msg) = msg {
            match msg {
                RawInputMessage::Press(key) => {
                    let key_state = self.keys.entry(key).or_insert(false);
                    if !*key_state {
                        *key_state = true;
                    }
                    vec![]
                }
                RawInputMessage::Release(key) => {
                    self.keys.insert(key, false);

                    if let Some(down) = self.keys.get(&Key::Edit) {
                        if *down {
                            return match key {
                                Key::Up => vec![Message::Input(InputMessage::EditUp)],
                                Key::Down => vec![Message::Input(InputMessage::EditDown)],
                                Key::Left => vec![Message::Input(InputMessage::EditLeft)],
                                Key::Right => vec![Message::Input(InputMessage::EditRight)],
                                _ => vec![],
                            };
                        }
                    }

                    match key {
                        Key::Quit => vec![Message::Quit],
                        Key::Up => vec![Message::Input(InputMessage::Up)],
                        Key::Down => vec![Message::Input(InputMessage::Down)],
                        Key::Left => vec![Message::Input(InputMessage::Left)],
                        Key::Right => vec![Message::Input(InputMessage::Right)],
                        Key::Play => vec![Message::Input(InputMessage::Play)],
                        _ => vec![],
                    }
                }
            }
        } else {
            vec![]
        }
    }
}
