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
#[read_component(FieldOfView)]
#[read_component(Tooltip)]
#[read_component(Player)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    // req entity is the entity that owns the components
    let mut positions = <(Entity, &Point, &Name, &Tooltip)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());

    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(HUD_LAYER);

    let player_fov = fov.iter(ecs).next().unwrap();
    positions
        .iter(ecs)
        .filter(|(_, pos, _, _)| **pos == map_pos && player_fov.visible_tiles.contains(&pos))
        .for_each(|(entity, pos, name, tooltip)| {
            let screen_pos = *mouse_pos * 2; // hud layer is 2 times larger than base layer
            let mut next_y = screen_pos.y - 1;

            // title
            let title_pos = Point::new(screen_pos.x, next_y);
            let title_contents = format!("{}", &name.0);
            draw_batch.print_color(
                fit_tooltip_to_screen(&title_pos, title_contents.len()),
                title_contents,
                ColorPair::new(WHITE, RED),
            );
            next_y += 1;

            // process hp tooltip if needed
            match ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                Ok(health) => {
                    let bar_pos = Point::new(screen_pos.x, next_y);
                    draw_batch.bar_horizontal(
                        fit_tooltip_to_screen(&bar_pos, health.max as usize),
                        health.max,
                        health.current,
                        health.max,
                        ColorPair::new(GREEN, RED),
                    );
                    next_y += 1;

                    let hp_pos = Point::new(screen_pos.x, next_y);
                    let hp_contents = format!("HP: {}/{}", &health.current, &health.max);
                    draw_batch.print_color(
                        fit_tooltip_to_screen(&hp_pos, hp_contents.len()),
                        hp_contents,
                        ColorPair::new(WHITE, RED),
                    );
                    next_y += 1;
                }
                Err(_) => {
                    // dont process health if no hp component
                    ()
                }
            };

            // process tooltip (all templates have a tooltip, just like name)
            let tooltip_pos = Point::new(screen_pos.x, next_y);
            let tooltip_contents = format!("<{}>", &tooltip.0);
            draw_batch.print_color(
                fit_tooltip_to_screen(&tooltip_pos, tooltip_contents.len()),
                tooltip_contents,
                ColorPair::new(WHITE, RED),
            );
            next_y += 1;

            let pos_tooltip_pos = Point::new(screen_pos.x, next_y);
            let tooltip_contents = format!("x,y: ({}, {})", pos.x, pos.y);
            draw_batch.print_color(
                fit_tooltip_to_screen(&pos_tooltip_pos, tooltip_contents.len()),
                tooltip_contents,
                ColorPair::new(WHITE, RED),
            );
        });

    draw_batch.submit(10100).expect("Batch Error");
}
