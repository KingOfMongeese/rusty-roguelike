use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    // req entity is the entity that owns the components
    let mut positions = <(Entity, &Point, &Name)>::query();

    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(HUD_LAYER);

    positions
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos)
        .for_each(|(entity, _, name)| {
            let screen_pos = *mouse_pos * 2; // hud layer is 2 times larger than base layer
            match ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                Ok(health) => {
                    // 1 lower than entity
                    let bar_pos = Point::new(screen_pos.x, screen_pos.y - 2);
                    draw_batch.bar_horizontal(
                        bar_pos,
                        health.max,
                        health.current,
                        health.max,
                        ColorPair::new(GREEN, RED),
                    );

                    // 2 higher than entitiy
                    let title_pos = Point::new(screen_pos.x, screen_pos.y - 1);
                    draw_batch.print_color(
                        title_pos,
                        format!("{}: {} / {}", &name.0, health.current, health.max),
                        ColorPair::new(WHITE, RED),
                    );
                }
                Err(_) => {
                    draw_batch.print(screen_pos, name.0.clone());
                }
            };
        });

    draw_batch.submit(10100).expect("Batch Error");
}
