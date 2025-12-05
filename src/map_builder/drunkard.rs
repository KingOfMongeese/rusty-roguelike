use crate::prelude::*;
use super::MapArchitect;

const STAGGER_DISTANCE: usize = 400;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR: usize = NUM_TILES / 3;

pub struct DrunkardsWalkArchitect {}

impl MapArchitect for DrunkardsWalkArchitect {
    fn construct(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::default();

        mb.fill(TileType::Wall);

        // start the first iter
        let center = Point::new(SCREEN_WIDTH /2 , SCREEN_HEIGHT / 2);
        self.drunkard(&center, rng, &mut mb.map);

        // while there is not enough open tiles, go again
        while mb.map.tiles.iter().filter(|t| **t == TileType::Floor).count() < DESIRED_FLOOR {

            // spawn a miner at a random point
            self.drunkard(&Point::new(rng.range(0, SCREEN_WIDTH), rng.range(0, SCREEN_HEIGHT)), rng, &mut mb.map);

            // calucate all the inaccessible parts, as the miners dont always start in the center (ie have a valid path from center)
            let dijsktra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &vec![mb.map.point2d_to_index(center)], &mb.map, 1024.0);

            // fill inaccessible parts with walls
            dijsktra_map.map.iter().enumerate().filter(|(_, distance)| *distance > &2000.0).for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);
        }
        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.amulet_of_yala_start = mb.find_most_distant();

        mb
    }
}

impl DrunkardsWalkArchitect {

    /// spawn a drunkard, moves until off the map or runs out of mv points
    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut dunkard_pos = start.clone();
        let mut distance_staggered = 0;

        loop {

            // turn current tile into floor
            let drunk_idx = map.point2d_to_index(dunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            match rng.range(0, 4) {
                0 => dunkard_pos.x -= 1,
                1 => dunkard_pos.x += 1,
                2 => dunkard_pos.y -= 1,
                _ => dunkard_pos.y += 1
            }
            
            distance_staggered += 1;
            if !map.in_bounds(dunkard_pos) || distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}