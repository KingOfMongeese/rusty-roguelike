use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Carried)]
#[read_component(Item)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld) {
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

    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();

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

    draw_batch.submit(1000).expect("Batch ERROR");
}
