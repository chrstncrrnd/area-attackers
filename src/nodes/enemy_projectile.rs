use macroquad::color::WHITE;
use macroquad::prelude::{
    draw_texture_ex, screen_height, DrawTextureParams, Texture2D, Vec2,
};
use macroquad::time::get_frame_time;

use crate::vec2;

pub struct EnemyProjectile {
    texture: Texture2D,
    position: Vec2,
    shooting: bool,
}

impl EnemyProjectile {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            texture,
            position: vec2(0., 0.),
            shooting: false,
        }
    }

    pub fn render(&mut self) {
        if self.shooting {
            self.position.y += 1000. * get_frame_time();
            draw_texture_ex(
                self.texture,
                self.position.x,
                self.position.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Option::from(vec2(10.0, 20.0)),
                    ..Default::default()
                },
            );
            if self.position.y > screen_height() {
                self.shooting = false;
            }
        }
    }

    pub fn shoot(&mut self, position: Vec2) {
        if !self.shooting {
            self.shooting = true;
            self.position = position;
        }
    }
}
