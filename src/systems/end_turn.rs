use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
pub fn end_turn(ecs: &mut SubWorld, #[resource] turn_state: &mut TurnState) {
    let current_state = turn_state.clone();

    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
    // only ever 1 amulet
    let amulet_pos = amulet.iter(ecs).nth(0).unwrap();

    // TODO this should only be checked during game. no need to check when in menus
    let mut player = <(&Health, &Point)>::query().filter(component::<Player>());
    player.iter(ecs).for_each(|(hp, pos)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }

        if pos == amulet_pos {
            new_state = TurnState::Victory;
        }
    });

    *turn_state = new_state;
}
