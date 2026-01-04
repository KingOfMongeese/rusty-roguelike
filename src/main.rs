#[warn(clippy::pedantic)]
#[warn(clippy::perf)]
mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const BASE_LAYER: usize = 0;
    pub const ENTITY_LAYER: usize = 1;
    pub const HUD_LAYER: usize = 2;
    pub const MESSAGE_LAYER: usize = 3;
    pub const DEBUG_LAYER: usize = 4;
    pub const ALL_LAYERS: [usize; 5] = [
        BASE_LAYER,
        ENTITY_LAYER,
        HUD_LAYER,
        MESSAGE_LAYER,
        DEBUG_LAYER,
    ];
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use std::collections::HashSet;

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        let exit_idx = map_builder
            .map
            .point2d_to_index(map_builder.amulet_of_yala_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        spawn_level(&mut ecs, &mut rng, 0, &map_builder.monster_spawns);

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::Menu);
        resources.insert(map_builder.theme);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(HUD_LAYER);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "You were slain by a monster, your hero's journey has come to a premature end",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The Amulet of Yala remains unclaimed, and your home town is not saved",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Don't worry the town can send a new hero if you answer the call!",
        );

        self.print_exit_options(ctx);
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.reset(),
                VirtualKeyCode::E => ctx.quitting = true,
                _ => (),
            }
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(HUD_LAYER);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "You put on The Amulet of Yala and feel its power!",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "Your home town is now saved, finally you can rest",
        );

        self.print_exit_options(ctx);
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.reset(),
                VirtualKeyCode::E => ctx.quitting = true,
                _ => (),
            }
        }
    }

    fn menu(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(HUD_LAYER);

        let text = "Plague strikes the land! Thousands fall sick every day as kingdoms lay on the brink of all out war. The only hope for the land is The Amulet of YALA!
Created in ages past by the great enchanter YALA with the power to stop all disease. But before it was used it was locked away in the EVER-SHIFTING DUNGEON OF YALA.
Now adventurers and mercenaries dare to enter the dangerous enchanted dungeon to prove their mettle...";

        let mut y = 2;
        let mut line = String::new();

        text.split(' ').enumerate().for_each(|(cnt, word)| {
            if line.chars().count() + word.chars().count() > (SCREEN_WIDTH - 2) as usize {
                ctx.print_color(0, y, YELLOW3, BLACK, line.to_string());
                y += 2;
                line = "".to_string();
            }

            line = format!("{line} {word}");

            if cnt == text.split(' ').count() - 1 {
                ctx.print_color(0, y, YELLOW, BLACK, line.to_string());
            }
        });

        self.print_credits(ctx);
        self.print_exit_options(ctx);

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.resources.insert(TurnState::AwaitingInput),
                VirtualKeyCode::E => ctx.quitting = true,
                _ => (),
            }
        }
    }

    fn print_exit_options(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(HUD_LAYER);
        self.print_credits(ctx);

        ctx.print_color(0, SCREEN_HEIGHT - 1, GREEN, BLACK, "(p) play");
        ctx.print_color_right(SCREEN_WIDTH - 1, SCREEN_HEIGHT - 1, RED, BLACK, "(e) exit");
    }

    fn print_credits(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(HUD_LAYER);

        let mut starting_y = SCREEN_HEIGHT / 2;

        ctx.print_centered(starting_y, "CREDITS");

        let credits_text = [
            "|---------------------------|",
            "| Programmer KingOfMongeese |",
            "| Backstory  Josh Zillinger |",
            "|---------------------------|",
        ];

        credits_text.iter().for_each(|line| {
            starting_y += 1;
            ctx.print_centered(starting_y, line);
        });
    }

    fn reset(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut self.ecs, map_builder.player_start);
        let exit_idx = map_builder
            .map
            .point2d_to_index(map_builder.amulet_of_yala_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        spawn_level(&mut self.ecs, &mut rng, 0, &map_builder.monster_spawns);

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    fn advance_level(&mut self) {
        self.clean_entities();

        // players is the only one left in the world now
        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.is_dirty = true);

        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = map_builder.player_start.x;
                pos.y = map_builder.player_start.y;
            });

        if map_level == 2 {
            spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_of_yala_start);
        } else {
            // amulet is finish point on map
            let exit_idx = map_builder
                .map
                .point2d_to_index(map_builder.amulet_of_yala_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        spawn_level(
            &mut self.ecs,
            &mut rng,
            map_level as usize,
            &map_builder.monster_spawns,
        );

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    fn clean_entities(&mut self) {
        let player = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&mut self.ecs)
            .next()
            .unwrap();

        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player);

        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_e, carried)| carried.0 == player)
            .map(|(e, _carried)| *e)
            .for_each(|e| {
                entities_to_keep.insert(e);
            });

        let mut cb = CommandBuffer::new(&mut self.ecs);
        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e);
            }
        }

        cb.flush(&mut self.ecs);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        for layer in &ALL_LAYERS {
            ctx.set_active_console(*layer);
            ctx.cls();
        }
        // inserted each frame
        self.resources.insert(ctx.key);

        // call before inserting the mouse
        ctx.set_active_console(BASE_LAYER);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        let current_state = *self.resources.get::<TurnState>().unwrap();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
            TurnState::NextLevel => self.advance_level(),
            TurnState::Menu => self.menu(ctx),
        }
        render_draw_buffer(ctx).expect("Render Error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Rusty Roguelike")
        .with_fps_cap(60.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("terminal8x8.png", 8, 8)
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // Base Layer
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // Entity Layer
        .with_simple_console_no_bg(DISPLAY_WIDTH * 2, DISPLAY_HEIGHT * 2, "terminal8x8.png") // HUD Layer
        .with_simple_console_no_bg(DISPLAY_WIDTH * 2, DISPLAY_HEIGHT * 2, "terminal8x8.png") // Message Layer
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "terminal8x8.png") // Debug Layer
        .with_fitscreen(true)
        .build()?;

    // add system to exit full screen

    main_loop(context, State::new())
}
