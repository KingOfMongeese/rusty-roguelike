use crate::prelude::*;
use super::MapArchitect;

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn construct(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {

        // book doesnt have defualt
        let mut mb = MapBuilder::default();

        mb.fill(TileType::Floor);
        mb.player_start = Point::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);
        mb.amulet_of_yala_start = mb.find_most_distant();
        for _ in 0..50 {
            mb.monster_spawns.push(Point::new(rng.range(1, SCREEN_WIDTH), rng.range(1, SCREEN_HEIGHT)));
        }
        mb
    }
}