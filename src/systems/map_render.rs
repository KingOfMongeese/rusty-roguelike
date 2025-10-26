use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn map_render(ecs: &SubWorld, #[resource] map: &Map, #[resource] camera: &Camera) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).next().unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(BASE_LAYER);
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            let idx = get_map_idx(x, y);
            if map.in_bounds(pt)
                && (player_fov.visible_tiles.contains(&pt) || map.revealed_tiles[idx])
            {
                let tint = if player_fov.visible_tiles.contains(&pt) {
                    WHITE
                } else {
                    DARK_GRAY
                };

                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };
                let render_pos = pt - offset;

                draw_batch.set(render_pos, ColorPair::new(tint, BLACK), glyph);
            }
        }
    }
    draw_batch.submit(0).expect("BATCH ERROR MAP");
}
