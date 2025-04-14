use crate::prelude::*;


pub struct Debugger {
    on: bool,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            on: false,
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm, player: &Player, camera: &Camera) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::F1 => {
                    self.on = !self.on;
                },
                _ => (),
            }
        }
        if self.on {
            ctx.set_active_console(DEBUG_LAYER);

            ctx.print_centered(0, "Dev Screen");
            ctx.print(0, 1, "Camera");
            ctx.print(0, 2, format!("left_x: {}", camera.left_x));
            ctx.print(0, 3, format!("right_x: {}", camera.right_x));
            ctx.print(0, 4, format!("top_y: {}", camera.top_y));
            ctx.print(0, 5, format!("bottom_y: {}", camera.bottom_y));

            ctx.print(0, 7, "Player");
            ctx.print(0, 8, format!("position: {:?}", player.position));

            ctx.set_active_console(BASE_LAYER);
        }
    }
}
