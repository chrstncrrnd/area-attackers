use macroquad::color::WHITE;
use macroquad::prelude::{draw_texture_ex, DrawTextureParams, screen_height, Texture2D, Vec2};
use macroquad::time::get_frame_time;

use crate::vec2;

pub struct EnemyProjectile {
    pub position: Vec2,
    pub size: Vec2,
    texture: Texture2D,
    shooting: bool,
}

impl EnemyProjectile {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            position: vec2(0., 0.),
            size: vec2(10.0, 20.0),
            texture,
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
                    dest_size: Some(self.size),
                    ..Default::default()
                },
            );
            if self.position.y > screen_height() {
                self.shooting = false;
            }
        }
    }

    pub fn retract_projectile(&mut self){
        self.shooting = false;
        self.position = vec2(0., 0.);
    }

    pub fn shoot(&mut self, position: Vec2) {
        if !self.shooting {
            self.shooting = true;
            self.position = position;
        }
    }
}
