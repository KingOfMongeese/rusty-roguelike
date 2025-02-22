use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    /// update player position, checks if pos is valid.
    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::A => Point::new(-1, 0),
                VirtualKeyCode::D => Point::new(1, 0),
                VirtualKeyCode::W => Point::new(0, -1),
                VirtualKeyCode::S => Point::new(0, 1),
                _ => Point::zero(),
            };

            let new_pos = self.position + delta;
            if map.can_enter_tile(new_pos) {
                self.position = new_pos;
            }
        }
    }

    /// render the player
    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }
}
