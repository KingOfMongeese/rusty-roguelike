use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

pub fn get_map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Wall; NUM_TILES],
        }
    }

    /// checks if a tile is in bounds (not off the screen)
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    /// checks if players/monsters can move onto a tile
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[get_map_idx(point.x, point.y)] == TileType::Floor
    }

    /// checks if a map cord, is valid and returns the index if it is
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(get_map_idx(point.x, point.y))
        }
    }

    /// renders the map on screen
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(BASE_LAYER);
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.in_bounds(Point::new(x, y)) {
                    let idx = get_map_idx(x, y);
                    match self.tiles[idx] {
                        TileType::Floor => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                DARKGRAY,
                                BLACK,
                                to_cp437('.'),
                            );
                        }
                        TileType::Wall => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                DARK_GRAY,
                                GRAY,
                                to_cp437('#'),
                            );
                        }
                    }
                }
            }
        }
    }
}
