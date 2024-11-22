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
    }
}

struct FocusCalculator<C: Eq + Hash + Copy> {
    rects: HashMap<C, Rect>,
}

impl<C: Eq + Hash + Copy> FocusCalculator<C> {
    pub fn new() -> Self {
        Self {
            rects: HashMap::new(),
        }
    }

    pub fn add(&mut self, control: C, rect: Rect) -> Rect {
        self.rects
            .entry(control)
            .and_modify(|r| *r = rect)
            .or_insert(rect);
        rect
    }

    pub fn update(&self, focused: &mut C, direction: Direction) -> Result<(), ()> {
        let may_be_rect = self.rects.get(focused);

        if let Some(rect) = may_be_rect {
            let focused_center = Point {
                x: rect.x + rect.width / 2,
                y: rect.y + rect.height / 2,
            };

            let mut best: Option<C> = None;
            let mut best_dist: f32 = f32::INFINITY;

            for (control, rect) in &self.rects {
                if control != focused {
                    let rect_center = Point {
                        x: rect.x + rect.width / 2,
                        y: rect.y + rect.height / 2,
                    };
                    let dist = distance(focused_center, rect_center, direction);

                    if dist < best_dist {
                        best_dist = dist;
                        best = Some(*control);
                    }
                }
            }

            if let Some(control) = best {
                *focused = control;
                Ok(())
            } else {
                Err(())
            }
        } else {
            Ok(())
        }
    }
}
