use ratatui::{buffer::Buffer, layout::Rect, Frame};

use super::{message::Message, state::State};

type DrawCall = dyn FnOnce(&mut Buffer);

pub struct FrameContext {
    messages: Vec<Message>,
    next_messages: Vec<Message>,
    draw_calls: Vec<Box<DrawCall>>,
}

impl FrameContext {
    pub fn render(
        frame: &mut Frame,
        state: &mut State,
        messages: Vec<Message>,
        fun: fn(&mut State, area: Rect, ctx: &mut Self),
    ) {
        let mut ctx = Self {
            messages,
            next_messages: Vec::new(),
            draw_calls: Vec::new(),
        };

        while !ctx.messages.is_empty() {
            ctx.draw_calls.clear();
            fun(state, frame.area(), &mut ctx);
            ctx.messages.clear();
            std::mem::swap(&mut ctx.messages, &mut ctx.next_messages);
        }

        ctx.draw(frame.buffer_mut());
    }

    pub fn send(&mut self, msg: Message) {
        self.next_messages.push(msg);
    }

    pub fn add(&mut self, draw_call: impl FnOnce(&mut Buffer) + 'static) {
        self.draw_calls.push(Box::new(draw_call));
    }

    pub fn process_messages(&mut self, mut f: impl FnMut(&Message, &mut Vec<Message>) -> bool) {
        let mut msgs: Vec<Message> = Vec::new();
        self.messages.retain(|msg| !f(msg, &mut msgs));
        self.next_messages.append(&mut msgs);
    }

    fn draw(&mut self, buf: &mut Buffer) {
        while let Some(call) = self.draw_calls.pop() {
            (call)(buf);
        }
    }

    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            next_messages: Vec::new(),
            draw_calls: Vec::new(),
        }
    }

    pub fn append_draw_calls(&mut self, other: &mut Self) {
        self.draw_calls.append(&mut other.draw_calls);
    }
}
