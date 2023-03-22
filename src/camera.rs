use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

const CAMERA_INCREMENT: i32 = 5;

impl Camera {
    pub fn new(player_position: Point) -> Self {
        Self {
            left_x: player_position.x - DISPLAY_WIDTH/2,
            right_x: player_position.x + DISPLAY_WIDTH/2,
            top_y: player_position.y - DISPLAY_HEIGHT/2,
            bottom_y: player_position.y + DISPLAY_HEIGHT/2,
        }
    }

    pub fn move_right(&mut self) {
        self.left_x  += CAMERA_INCREMENT;
        self.right_x += CAMERA_INCREMENT;
    }

    pub fn move_left(&mut self) {
        self.left_x  -= CAMERA_INCREMENT;
        self.right_x -= CAMERA_INCREMENT;
    }

    pub fn move_up(&mut self) {
        self.top_y    -= CAMERA_INCREMENT;
        self.bottom_y -= CAMERA_INCREMENT;
    }

    pub fn move_down(&mut self) {
        self.top_y    += CAMERA_INCREMENT;
        self.bottom_y += CAMERA_INCREMENT;
    }

    pub fn on_player_move(&mut self, player_position: Point) {
        if player_position.x - self.left_x < CAMERA_INCREMENT {
            self.move_left();
        }

        if self.right_x - player_position.x < CAMERA_INCREMENT {
            self.move_right();
        }

        if player_position.y - self.top_y < CAMERA_INCREMENT {
            self.move_up();
        }

        if self.bottom_y - player_position.y < CAMERA_INCREMENT {
            self.move_down();
        }
    }
}
