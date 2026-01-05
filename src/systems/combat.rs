use ron::de;

use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[read_component(Damage)]
#[read_component(Carried)]
#[read_component(Name)]
#[write_component(Health)]
pub fn combat(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] game_log: &mut GameLog,
) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let attacks: Vec<(Entity, Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack_msg)| (*entity, attack_msg.attacker, attack_msg.defender))
        .collect();

    attacks.iter().for_each(|(message, attacker, defender)| {
        // get a ref to the defender components
        // check if player is 1 of those components
        let is_player = ecs
            .entry_ref(*defender)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        let base_damage = if let Ok(attacker_ref) = ecs.entry_mut(*attacker) {
            if let Ok(dmg) = attacker_ref.get_component::<Damage>() {
                dmg.0
            } else {
                0
            }
        } else {
            0
        };

        let weapon_damage: i32 = <(&Carried, &Damage)>::query()
            .iter(ecs)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, dmg)| dmg.0)
            .sum();

        let final_damage = base_damage + weapon_damage;

        if let Ok(health) = ecs
            .entry_mut(*defender)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= final_damage;
            if health.current < 1 && !is_player {
                commands.remove(*defender);
            }
        }

        let attacker_name =
            if let Ok(name) = ecs.entry_ref(*attacker).unwrap().get_component::<Name>() {
                name.0.to_string()
            } else {
                "<name>".to_string()
            };

        let defender_name =
            if let Ok(name) = ecs.entry_ref(*defender).unwrap().get_component::<Name>() {
                name.0.to_string()
            } else {
                "<name>".to_string()
            };

        let log_event = if attacker_name == "You" {
            GameLogEvent::new(
                ColorPair::new(GREEN, BLACK),
                format!("You -> {defender_name}: {final_damage} dmg"),
            )
        } else {
            GameLogEvent::new(
                ColorPair::new(RED, BLACK),
                format!("{attacker_name} -> {defender_name}: {final_damage} dmg"),
            )
        };

        game_log.log(log_event);

        commands.remove(*message);
    });
}
