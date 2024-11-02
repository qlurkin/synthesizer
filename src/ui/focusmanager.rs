use std::{collections::HashMap, f32::INFINITY, hash::Hash};

use anyhow::Result;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::tracker::Tracker;

use super::{component::Component, keyboard::InputMessage, message::Message};

pub trait FocusableComponent: Component {
    fn focus(&mut self, _focused: bool) {}
}

pub struct FocusManager<C: Eq + Hash> {
    rects: HashMap<C, Rect>,
    components: HashMap<C, Box<dyn FocusableComponent>>,
    focused: C,
}

struct Point {
    x: u16,
    y: u16,
}

impl<C: Eq + Hash + Copy> FocusManager<C> {
    pub fn new(focused: C) -> Self {
        Self {
            rects: HashMap::new(),
            components: HashMap::new(),
            focused,
        }
    }

    pub fn add(&mut self, control: C, component: Box<dyn FocusableComponent>) {
        self.components.insert(control, component);
    }

    pub fn update_rect(&mut self, control: C, rect: Rect) -> Rect {
        self.rects
            .entry(control)
            .and_modify(|r| *r = rect)
            .or_insert(rect);
        rect
    }

    pub fn render_component(
        &mut self,
        control: C,
        tracker: &Tracker,
        rect: Rect,
        buf: &mut Buffer,
    ) {
        self.update_rect(control, rect);
        let focused = self.is_focused(control);
        let mut binding = self.components.get_mut(&control);
        let component = binding.as_mut().unwrap();

        component.focus(focused);
        component.render(tracker, rect, buf);
    }

    pub fn update(&mut self, tracker: &mut Tracker, msg: Message) -> Vec<Message> {
        let mut binding = self.components.get_mut(&self.focused);
        let component = binding.as_mut().unwrap();

        let mut msgs = component.update(tracker, msg);
        msgs.append(&mut self.process_input(msg));
        msgs
    }

    pub fn is_focused(&self, control: C) -> bool {
        control == self.focused
    }

    pub fn process_input(&mut self, msg: Message) -> Vec<Message> {
        match msg {
            Message::Input(InputMessage::Up) => match self.up() {
                Ok(()) => vec![],
                Err(()) => vec![],
            },
            Message::Input(InputMessage::Down) => match self.down() {
                Ok(()) => vec![],
                Err(()) => vec![],
            },
            Message::Input(InputMessage::Left) => match self.left() {
                Ok(()) => vec![],
                Err(()) => vec![],
            },
            Message::Input(InputMessage::Right) => match self.right() {
                Ok(()) => vec![],
                Err(()) => vec![],
            },
            _ => vec![],
        }
    }

    pub fn up(&mut self) -> Result<(), ()> {
        let may_be_rect = self.rects.get(&self.focused);

        if let Some(rect) = may_be_rect {
            let focused_center = Point {
                x: rect.x + rect.width / 2,
                y: rect.y + rect.height / 2,
            };

            let mut best: Option<C> = None;
            let mut best_dist: f32 = INFINITY;

            for (control, rect) in &self.rects {
                if *control != self.focused {
                    let rect_center = Point {
                        x: rect.x + rect.width / 2,
                        y: rect.y + rect.height / 2,
                    };
                    if rect_center.y < focused_center.y {
                        let dist = (rect_center.x as f32 - focused_center.x as f32).abs()
                            + (rect_center.y as f32 - focused_center.y as f32).abs();

                        if dist < best_dist {
                            best_dist = dist;
                            best = Some(*control);
                        }
                    }
                }
            }

            if let Some(control) = best {
                self.focused = control;
                Ok(())
            } else {
                Err(())
            }
        } else {
            Ok(())
        }
    }

    pub fn right(&mut self) -> Result<(), ()> {
        let may_be_rect = self.rects.get(&self.focused);

        if let Some(rect) = may_be_rect {
            let focused_center = Point {
                x: rect.x + rect.width / 2,
                y: rect.y + rect.height / 2,
            };

            let mut best: Option<C> = None;
            let mut best_dist: f32 = INFINITY;

            for (control, rect) in &self.rects {
                if *control != self.focused {
                    let rect_center = Point {
                        x: rect.x + rect.width / 2,
                        y: rect.y + rect.height / 2,
                    };
                    if rect_center.x > focused_center.x {
                        let dist = (rect_center.x as f32 - focused_center.x as f32).abs()
                            + (rect_center.y as f32 - focused_center.y as f32).abs();

                        if dist < best_dist {
                            best_dist = dist;
                            best = Some(*control);
                        }
                    }
                }
            }

            if let Some(control) = best {
                self.focused = control;
                Ok(())
            } else {
                Err(())
            }
        } else {
            Ok(())
        }
    }

    pub fn down(&mut self) -> Result<(), ()> {
        let may_be_rect = self.rects.get(&self.focused);

        if let Some(rect) = may_be_rect {
            let focused_center = Point {
                x: rect.x + rect.width / 2,
                y: rect.y + rect.height / 2,
            };

            let mut best: Option<C> = None;
            let mut best_dist: f32 = INFINITY;

            for (control, rect) in &self.rects {
                if *control != self.focused {
                    let rect_center = Point {
                        x: rect.x + rect.width / 2,
                        y: rect.y + rect.height / 2,
                    };
                    if rect_center.y > focused_center.y {
                        let dist = (rect_center.x as f32 - focused_center.x as f32).abs()
                            + (rect_center.y as f32 - focused_center.y as f32).abs();

                        if dist < best_dist {
                            best_dist = dist;
                            best = Some(*control);
                        }
                    }
                }
            }

            if let Some(control) = best {
                self.focused = control;
                Ok(())
            } else {
                Err(())
            }
        } else {
            Ok(())
        }
    }

    pub fn left(&mut self) -> Result<(), ()> {
        let may_be_rect = self.rects.get(&self.focused);

        if let Some(rect) = may_be_rect {
            let focused_center = Point {
                x: rect.x + rect.width / 2,
                y: rect.y + rect.height / 2,
            };

            let mut best: Option<C> = None;
            let mut best_dist: f32 = INFINITY;

            for (control, rect) in &self.rects {
                if *control != self.focused {
                    let rect_center = Point {
                        x: rect.x + rect.width / 2,
                        y: rect.y + rect.height / 2,
                    };
                    if rect_center.x < focused_center.x {
                        let dist = (rect_center.x as f32 - focused_center.x as f32).abs()
                            + (rect_center.y as f32 - focused_center.y as f32).abs();

                        if dist < best_dist {
                            best_dist = dist;
                            best = Some(*control);
                        }
                    }
                }
            }

            if let Some(control) = best {
                self.focused = control;
                Ok(())
            } else {
                Err(())
            }
        } else {
            Ok(())
        }
    }
}
