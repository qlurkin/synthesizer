use std::collections::HashSet;

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
    Clear,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum RawInputMessage {
    Press(Key),
    Release(Key),
}

pub struct Keyboard {
    keys: HashSet<Key>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: HashSet::new(),
        }
    }
}

pub fn process_raw_input(keyboard: &mut Keyboard, ctx: &mut FrameContext) {
    ctx.process_messages(|msg, msgs| {
        if let Message::RawInput(msg) = msg {
            match msg {
                RawInputMessage::Press(key) => {
                    let handled = false;
                    let handled = if keyboard.keys.contains(&Key::Option) {
                        match key {
                            Key::Edit => {
                                msgs.push(Message::Input(InputMessage::Clear));
                                true
                            }
                            _ => handled,
                        }
                    } else {
                        handled
                    };

                    let handled = if !handled && keyboard.keys.contains(&Key::Edit) {
                        match key {
                            Key::Up => {
                                msgs.push(Message::Input(InputMessage::EditUp));
                                true
                            }
                            Key::Down => {
                                msgs.push(Message::Input(InputMessage::EditDown));
                                true
                            }
                            Key::Left => {
                                msgs.push(Message::Input(InputMessage::EditLeft));
                                true
                            }
                            Key::Right => {
                                msgs.push(Message::Input(InputMessage::EditRight));
                                true
                            }
                            _ => handled,
                        }
                    } else {
                        handled
                    };

                    let handled = if !handled && keyboard.keys.contains(&Key::Shift) {
                        match key {
                            Key::Up => {
                                msgs.push(Message::Input(InputMessage::ShiftUp));
                                true
                            }
                            Key::Down => {
                                msgs.push(Message::Input(InputMessage::ShiftDown));
                                true
                            }
                            Key::Left => {
                                msgs.push(Message::Input(InputMessage::ShiftLeft));
                                true
                            }
                            Key::Right => {
                                msgs.push(Message::Input(InputMessage::ShiftRight));
                                true
                            }
                            _ => handled,
                        }
                    } else {
                        handled
                    };

                    let _handled = if !handled {
                        match key {
                            Key::Up => {
                                msgs.push(Message::Input(InputMessage::Up));
                                true
                            }
                            Key::Down => {
                                msgs.push(Message::Input(InputMessage::Down));
                                true
                            }
                            Key::Left => {
                                msgs.push(Message::Input(InputMessage::Left));
                                true
                            }
                            Key::Right => {
                                msgs.push(Message::Input(InputMessage::Right));
                                true
                            }
                            Key::Play => {
                                msgs.push(Message::Input(InputMessage::Play));
                                true
                            }
                            _ => handled,
                        }
                    } else {
                        handled
                    };

                    keyboard.keys.insert(*key);
                }
                RawInputMessage::Release(key) => {
                    keyboard.keys.remove(key);
                }
            };
            true
        } else {
            false
        }
    });
}
