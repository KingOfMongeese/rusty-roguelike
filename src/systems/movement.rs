use crate::prelude::*;

// for each WantsToMove
#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        // dest replaces pos in the entnity
        commands.add_component(want_move.entity, want_move.destination);

        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                // queue fov for update by marking it as dirty, entity moves so we need to update later in the fov system
                commands.add_component(want_move.entity, fov.clone_dirty());
            }

            if entry.get_component::<Player>().is_ok() {
                camera.on_player_move(want_move.destination);
            }
        }
    }
    // remove msg entity, contains just the msg component
    commands.remove(*entity);
}
