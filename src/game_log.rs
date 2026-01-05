use crate::prelude::*;
use std::collections::VecDeque;

const MAX_EVENTS: usize = 5;

pub struct GameLog {
    log: VecDeque<GameLogEvent>,
}

impl GameLog {
    pub fn new() -> Self {
        Self {
            log: VecDeque::with_capacity(MAX_EVENTS),
        }
    }

    pub fn log(&mut self, event: GameLogEvent) {
        if self.log.len() + 1 > MAX_EVENTS {
            self.log.pop_front();
        }
        self.log.push_back(event);
    }

    pub fn render(&self, y: usize) {
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(HUD_LAYER);

        let mut pos = Point::new(0, y);
        draw_batch.print(pos, "Game Events");
        pos.y += 1;

        draw_batch.print(pos, "------------");
        self.log.iter().for_each(|event| {
            pos.y += 1;
            draw_batch.print_color(pos, event.message.clone(), event.color);
        });

        draw_batch.submit(800).expect("batch error: gamelog render");
    }
}

pub struct GameLogEvent {
    pub color: ColorPair,
    pub message: String,
}

impl GameLogEvent {
    pub fn new(color: ColorPair, message: String) -> Self {
        Self { color, message }
    }
}
