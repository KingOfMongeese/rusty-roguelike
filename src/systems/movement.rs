use crate::prelude::*;

// for each WantsToMove
#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
#[read_component(Point)]
#[read_component(Health)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    let existing_positions: Vec<(&Entity, &Point, &Health)> = <(Entity, &Point, &Health)>::query().iter(ecs).collect();


    if map.can_enter_tile(want_move.destination) && !is_occupied(&existing_positions, &want_move.destination){
        // dest replaces pos in the entnity
        commands.add_component(want_move.entity, want_move.destination);

        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                // queue fov for update by marking it as dirty, entity moves so we need to update later in the fov system
                commands.add_component(want_move.entity, fov.clone_dirty());

                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(want_move.destination);
                    fov.visible_tiles.iter().for_each(|pos| {
                        map.revealed_tiles[get_map_idx(pos.x, pos.y)] = true;
                    });
                }
            }
        }
    }
    // remove msg entity, contains just the msg component
    commands.remove(*entity);
}

fn is_occupied(existing_pos: &Vec<(&Entity, &Point, &Health)>, destination: &Point) -> bool {
    let mut is_occupied = false;
    if existing_pos.iter().map(|(_, pos, _)| **pos).filter(|pos| *pos == *destination).count() > 0 {
        println!("cant move monster");
        is_occupied = true;
    }

    is_occupied
}