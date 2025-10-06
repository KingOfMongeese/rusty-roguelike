use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let attackers_defenders_pairs: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack_msg)| (*entity, attack_msg.defender))
        .collect();

    attackers_defenders_pairs
        .iter()
        .for_each(|(message, defender)| {
            // get a ref to the defender components
            // check if player is 1 of those components
            let is_player = ecs
                .entry_ref(*defender)
                .unwrap()
                .get_component::<Player>()
                .is_ok();
            if let Ok(health) = ecs
                .entry_mut(*defender)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current -= 1;
                if health.current < 1 && !is_player {
                    commands.remove(*defender);
                }
            }

            commands.remove(*message);
        });
}
