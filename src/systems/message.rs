use crate::prelude::*;

/// prints msgs
#[system]
#[read_component(Enemy)]
pub fn message(ecs: &mut SubWorld) {
    let enemy_count = get_enemy_count(ecs);
    display_enemy_count(enemy_count);
    display_all_monsters_cleared(enemy_count);
}

fn get_enemy_count(ecs: &mut SubWorld) -> usize {
    let enemies: Vec<&Enemy> = <&Enemy>::query().iter(ecs).collect();
    enemies.len()
}

fn display_enemy_count(enemy_count: usize) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(MESSAGE_LAYER);

    draw_batch.print(Point::new(0, 1), format!("enemies: {enemy_count}"));
    draw_batch.submit(7000).expect("Batch Error");
}

fn display_all_monsters_cleared(enemy_count: usize) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(MESSAGE_LAYER);

    if enemy_count == 0 {
        draw_batch.print_centered(4, "All monsters cleared!");
    }

    draw_batch.submit(6000).expect("Batch Error");
}
