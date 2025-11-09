use crate::prelude::*;
use super::MapArchitect;

pub struct RoomArchitect{}

impl MapArchitect for RoomArchitect {
    fn construct(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {

        // book doesnt have defualt
        let mut mb = MapBuilder::default();

        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb.amulet_of_yala_start = mb.find_most_distant();
        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.center());
        }

        mb
    }
}