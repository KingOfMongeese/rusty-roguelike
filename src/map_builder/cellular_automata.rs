use super::MapArchitect;
use crate::prelude::*;

pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
    fn construct(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::default();
        self.random_noise_map(rng, &mut mb.map);
        for _ in 0..10 {
            self.interation(&mut mb.map);
        }

        mb.player_start = self.find_start(&mb.map);
        mb.monster_spawns = mb.spawn_monsters(&mb.player_start, rng);
        mb.amulet_of_yala_start = mb.find_most_distant();
        mb
    }
}

impl CellularAutomataArchitect {
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);
            if roll > 55 {
                *t = TileType::Floor;
            } else {
                *t = TileType::Wall;
            }
        });
    }

    /// preform an interation of cellular automata
    fn interation(&mut self, map: &mut Map) {
        // iterate on a copy, as we are changing the map
        let mut new_tiles = map.tiles.clone();
        for y in 1..SCREEN_HEIGHT - 1 {
            for x in 1..SCREEN_WIDTH - 1 {
                let adjacent_walls = self.count_wall_neighbors(x, y, map);
                let index = get_map_idx(x, y);
                if adjacent_walls > 4 || adjacent_walls == 0 {
                    new_tiles[index] = TileType::Wall;
                } else {
                    new_tiles[index] = TileType::Floor;
                }
            }
        }

        map.tiles = new_tiles;
    }

    /// count all walls that are adjacent (including diagonal)
    fn count_wall_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        // ix is offet distance
        let mut nieghbors = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                if !(ix == 0 && iy == 0) && map.tiles[get_map_idx(x + ix, y + iy)] == TileType::Wall
                {
                    nieghbors += 1;
                }
            }
        }
        nieghbors
    }

    /// finds the closest spawnable point to the center for the player
    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        let closest_point = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor) // only compare floors
            .map(|(idx, _)| {
                (
                    idx,
                    DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx)),
                )
            }) // get distance for each idx to center
            .min_by(|(_, distance), (_, distance2)| distance.partial_cmp(distance2).unwrap())// get the smallest value
            .map(|(idx, _)| idx)// transform tuple(usize, f32) -> usize
            .unwrap();
        map.index_to_point2d(closest_point)
    }
}
