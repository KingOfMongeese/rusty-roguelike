use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Carried)]
#[read_component(Item)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld, #[resource] game_log: &GameLog) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).next().unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(HUD_LAYER);
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH,
        player_health.current,
        player_health.max,
        ColorPair::new(GREEN, RED),
    );

    draw_batch.print_color_centered(
        0,
        format!("Health: {} / {}", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );

    let (player, map_level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, player)| Some((*entity, player.map_level)))
        .unwrap();

    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH, 1),
        format!("Dungeon Level: {}/3", map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    let mut item_query = <(&Item, &Name, &Carried)>::query();
    let mut y = 4;
    item_query
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .zip(1..) // start enumerate from 1
        .for_each(|((_, name, _), item_cnt)| {
            draw_batch.print(Point::new(0, y), format!("{} : {} ", item_cnt, name.0));
            y += 1;
        });

    if y > 4 {
        draw_batch.print_color(
            Point::new(0, 3),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    } else {
        draw_batch.print_color(
            Point::new(0, 3),
            "Press 'G' to pickup items",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    game_log.render((SCREEN_HEIGHT - 8) as usize);
    draw_batch.submit(1000).expect("Batch ERROR");
}
