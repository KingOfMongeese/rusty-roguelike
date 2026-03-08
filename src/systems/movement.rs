use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(FieldOfView)]
#[read_component(Point)]
#[read_component(Health)]
#[read_component(WantsToMove)]
pub fn movement(
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    let mut want_move = <(Entity, &WantsToMove)>::query();

    // We update the pos on the component, but they dont take affect until we flush the buffer, so we store a list of them here
    let mut existing_positions: Vec<Point> =
            <(Entity, &Point, &Health)>::query().iter(ecs).map(|(_, pos, _)| *pos).collect();

    want_move.iter(ecs).for_each(|(entity, want_move)| {

        if map.can_enter_tile(want_move.destination)
            && !existing_positions.contains(&want_move.destination)
        {

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

                    // remove the old pos from the list as its no longer occupied
                    if let Ok(old_pos) = entry.get_component::<Point>() {
                        let index = existing_positions.iter().enumerate().find(|(_, pos)| **pos == *old_pos).map(|(i, _)| i);
                        if let Some(index) = index {
                            existing_positions.remove(index);
                        }
                    }
                }
            }
            // add the new pos to the list, as the pos will be occupied on flush
            existing_positions.push(want_move.destination);

            // dest replaces pos in the entity, must be after we remove its old pos
            commands.add_component(want_move.entity, want_move.destination);
        }

        // remove msg entity, contains just the msg component
        commands.remove(*entity);
    });
}
