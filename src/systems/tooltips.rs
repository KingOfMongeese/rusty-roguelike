use crate::prelude::*;

fn fit_tooltip_to_screen(pos: &Point, tooltip_size_x: usize) -> Point {
    let mut adjusted_point = Point::from_tuple(pos.to_tuple());

    if (pos.x + tooltip_size_x as i32) > SCREEN_WIDTH {
        adjusted_point.x = SCREEN_WIDTH - tooltip_size_x as i32;
    }
    adjusted_point
}

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
                        fit_tooltip_to_screen(&bar_pos, health.max as usize),
                        health.max,
                        health.current,
                        health.max,
                        ColorPair::new(GREEN, RED),
                    );

                    // 2 higher than entitiy
                    let title_pos = Point::new(screen_pos.x, screen_pos.y - 1);
                    let tooltip_str = format!("{}: {}", &name.0, health.current);
                    draw_batch.print_color(
                        fit_tooltip_to_screen(&title_pos, tooltip_str.len()),
                        tooltip_str,
                        ColorPair::new(WHITE, RED),
                    );
                }
                Err(_) => {
                    draw_batch.print(fit_tooltip_to_screen(&screen_pos, name.0.clone().len()), name.0.clone());
                }
            };
        });

    draw_batch.submit(10100).expect("Batch Error");
}
