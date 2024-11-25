use anyhow::Result;
use core::f32;
use std::{collections::HashMap, hash::Hash};

use ratatui::layout::Rect;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point {
    x: u16,
    y: u16,
}

fn distance(from: Point, to: Point, direction: Direction) -> f32 {
    match direction {
        Direction::Up => {
            if to.y < from.y {
                (to.x as f32 - from.x as f32).abs() + (to.y as f32 - from.y as f32).abs()
            } else {
                f32::INFINITY
            }
        }
        Direction::Down => {
            if to.y > from.y {
                (to.x as f32 - from.x as f32).abs() + (to.y as f32 - from.y as f32).abs()
            } else {
                f32::INFINITY
            }
        }
        Direction::Left => {
            if to.x < from.x {
                (to.x as f32 - from.x as f32).abs() + (to.y as f32 - from.y as f32).abs()
            } else {
                f32::INFINITY
            }
        }
        Direction::Right => {
            if to.x > from.x {
                (to.x as f32 - from.x as f32).abs() + (to.y as f32 - from.y as f32).abs()
            } else {
                f32::INFINITY
            }
        }
        Direction::None => f32::INFINITY,
    }
}

pub struct FocusCalculator {
    rects: Vec<Rect>,
    focused: usize,
}

impl FocusCalculator {
    pub fn new(focused: usize) -> Self {
        Self {
            rects: Vec::new(),
            focused,
        }
    }

    pub fn add(&mut self, rect: Rect) -> (bool, Rect) {
        let id = self.rects.len();
        self.rects.push(rect);
        (self.focused == id, rect)
    }

    pub fn update(&self, direction: Direction) -> Result<usize, ()> {
        let rect = self.rects[self.focused];

        let focused_center = Point {
            x: rect.x + rect.width / 2,
            y: rect.y + rect.height / 2,
        };

        let mut best = self.focused;
        let mut best_dist: f32 = f32::INFINITY;

        for (control, rect) in self.rects.iter().enumerate() {
            if control != self.focused {
                let rect_center = Point {
                    x: rect.x + rect.width / 2,
                    y: rect.y + rect.height / 2,
                };
                let dist = distance(focused_center, rect_center, direction);

                if dist < best_dist {
                    best_dist = dist;
                    best = control;
                }
            }
        }

        if best != self.focused {
            Ok(best)
        } else {
            Err(())
        }
    }
}
