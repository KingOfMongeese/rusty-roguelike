mod camera;
mod map;
mod map_builder;
mod player;
mod debug_layer;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const BASE_LAYER: usize = 0;
    pub const ENTITY_LAYER: usize = 1;
    pub const DEBUG_LAYER: usize = 2;
    pub use crate::camera::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use crate::debug_layer::*;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
    camera: Camera,
    debugger: Debugger,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start),
            debugger: Debugger::new(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(BASE_LAYER);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(DEBUG_LAYER);
        ctx.cls();
        self.player.update(ctx, &self.map, &mut self.camera);
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
        self.debugger.render(ctx, &self.player, &self.camera);

    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Rusty Roguelike")
        .with_fps_cap(60.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "terminal8x8.png")
        .with_fitscreen(true)
        .build()?;

    main_loop(context, State::new())
}
