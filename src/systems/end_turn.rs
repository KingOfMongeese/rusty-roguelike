use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
pub fn end_turn(ecs: &mut SubWorld, #[resource] turn_state: &mut TurnState, #[resource] map: &Map) {
    let current_state = *turn_state;

    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());

    let amulet_default = Point::new(-1, -1);
    let amulet_pos = amulet.iter(ecs).next().unwrap_or(&amulet_default);

    let mut player = <(&Health, &Point)>::query().filter(component::<Player>());
    player.iter(ecs).for_each(|(hp, pos)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }

        if pos == amulet_pos {
            new_state = TurnState::Victory;
        }
        if map.tiles[map.point2d_to_index(*pos)] == TileType::Exit {
            new_state = TurnState::NextLevel;
        }
    });

    *turn_state = new_state;
}
