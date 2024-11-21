use std::collections::HashMap;

use super::{frame_context::FrameContext, message::Message};

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
    Shift,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum InputMessage {
    Play,
    Up,
    Down,
    Left,
    Right,
    ShiftUp,
    ShiftDown,
    ShiftLeft,
    ShiftRight,
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

pub fn process_raw_input(keyboard: &mut Keyboard, ctx: &mut FrameContext) {
    ctx.process_messages(|msg, msgs| {
        if let Message::RawInput(msg) = msg {
            match msg {
                RawInputMessage::Press(key) => {
                    let key_state = keyboard.keys.entry(*key).or_insert(false);
                    if !*key_state {
                        *key_state = true;
                    }
                }
                RawInputMessage::Release(key) => {
                    keyboard.keys.insert(*key, false);

                    if let Some(down) = keyboard.keys.get(&Key::Edit) {
                        if *down {
                            match key {
                                Key::Up => msgs.push(Message::Input(InputMessage::EditUp)),
                                Key::Down => msgs.push(Message::Input(InputMessage::EditDown)),
                                Key::Left => msgs.push(Message::Input(InputMessage::EditLeft)),
                                Key::Right => msgs.push(Message::Input(InputMessage::EditRight)),
                                _ => {}
                            };
                        }
                    } else if let Some(down) = keyboard.keys.get(&Key::Shift) {
                        if *down {
                            match key {
                                Key::Up => msgs.push(Message::Input(InputMessage::ShiftUp)),
                                Key::Down => msgs.push(Message::Input(InputMessage::ShiftDown)),
                                Key::Left => msgs.push(Message::Input(InputMessage::ShiftLeft)),
                                Key::Right => msgs.push(Message::Input(InputMessage::ShiftRight)),
                                _ => {}
                            };
                        }
                    } else {
                        match key {
                            Key::Up => msgs.push(Message::Input(InputMessage::Up)),
                            Key::Down => msgs.push(Message::Input(InputMessage::Down)),
                            Key::Left => msgs.push(Message::Input(InputMessage::Left)),
                            Key::Right => msgs.push(Message::Input(InputMessage::Right)),
                            Key::Play => msgs.push(Message::Input(InputMessage::Play)),
                            _ => {}
                        }
                    };
                }
            };
            true
        } else {
            false
        }
    });
}
