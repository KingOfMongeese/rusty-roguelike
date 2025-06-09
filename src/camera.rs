use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,

    // margins are line where trigger camera movement
    pub margin_x: i32,
    pub margin_y: i32,
}

impl Camera {
    pub fn new(player_pos: Point) -> Self {
        Self {
            left_x: player_pos.x - DISPLAY_WIDTH / 2,
            right_x: player_pos.x + DISPLAY_WIDTH / 2,
            top_y: player_pos.y - DISPLAY_HEIGHT / 2,
            bottom_y: player_pos.y + DISPLAY_HEIGHT / 2,
            margin_x: DISPLAY_WIDTH / 4,
            margin_y: DISPLAY_HEIGHT / 4,
        }
    }

    pub fn on_player_move(&mut self, player_pos: Point) {

        // Check horizontal boundaries
        if player_pos.x < self.left_x + self.margin_x {
            let shift = player_pos.x - (self.left_x + self.margin_x);
            self.left_x += shift;
            self.right_x += shift;
            // camera debug
            //println!("Shift LEFT\nPlayer: {}, {}\nShift Offset: {shift}\nCamera Bounds X (left, right): {}, {}\n---------\n", player_pos.x, player_pos.y, self.left_x, self.right_x);
        } else if player_pos.x > self.right_x - self.margin_x {
            let shift = player_pos.x - (self.right_x - self.margin_x);
            self.left_x += shift;
            self.right_x += shift;
            // camera debug
            //println!("Shift RIGHT\nPlayer: {}, {}\nShift Offset: {shift}\nCamera Bounds X (left, right): {}, {}\n---------\n", player_pos.x, player_pos.y, self.left_x, self.right_x);
        }

        // Check vertical boundaries
        if player_pos.y < self.top_y + self.margin_y {
            let shift = player_pos.y - (self.top_y + self.margin_y);
            self.top_y += shift;
            self.bottom_y += shift;
            // camera debug
            //println!("Shift UP\nPlayer: {}, {}\nShift Offset: {shift}\nCamera Bounds Y (top, bottom): {}, {}\n---------\n", player_pos.x, player_pos.y, self.top_y, self.bottom_y);
        } else if player_pos.y > self.bottom_y - self.margin_y {
            let shift = player_pos.y - (self.bottom_y - self.margin_y);
            self.top_y += shift;
            self.bottom_y += shift;
            // camera debug
            //println!("Shift DOWN\nPlayer: {}, {}\nShift Offset: {shift}\nCamera Bounds Y (top, bottom): {}, {}\n---------\n", player_pos.x, player_pos.y, self.top_y, self.bottom_y);
        }
    }
}
