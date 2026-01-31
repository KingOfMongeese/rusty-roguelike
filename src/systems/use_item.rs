use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[read_component(Name)]
#[write_component(Health)]
#[read_component(ProvidesDngMap)]
pub fn use_items(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map,
    #[resource] game_log: &mut GameLog,
) {
    let mut healing_to_appy = Vec::<(Entity, i32)>::new();

    <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .for_each(|(entity, activate)| {
            let item = ecs.entry_ref(activate.item);
            if let Ok(item) = item {
                if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                    healing_to_appy.push((activate.used_by, healing.amount));
                }

                if let Ok(_mapper) = item.get_component::<ProvidesDngMap>() {
                    map.revealed_tiles.iter_mut().for_each(|t| *t = true);
                }

                if let Ok(name) = item.get_component::<Name>() {
                    game_log.log(GameLogEvent::new(
                        ColorPair::new(YELLOW, BLACK),
                        format!("You used: {}", name.0),
                    ));
                }
            }

            // the item has been spent, remove it
            commands.remove(activate.item);

            // remove the message of intent
            commands.remove(*entity);
        });

    for (entity, amount) in healing_to_appy {
        if let Ok(mut target) = ecs.entry_mut(entity) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                // dont heal beyond max health
                health.current = i32::min(health.max, health.current + amount);
            }
        }
    }
}
