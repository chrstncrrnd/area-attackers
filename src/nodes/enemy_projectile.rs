use macroquad::color::WHITE;
use macroquad::prelude::{draw_texture_ex, DrawTextureParams, Texture2D, Vec2};
use crate::vec2;

pub struct EnemyProjectile{
    texture: Texture2D,
    position: Option<Vec2>,
    shooting: bool,
}

impl EnemyProjectile{
    pub fn new(texture: Texture2D) -> Self{
        Self{
            texture,
            position: None,
            shooting: false
        }
    }

    pub fn render(&mut self){
        if self.shooting{
            if let Some(mut pos) = self.position{
                pos.x -= 10_f32;
                draw_texture_ex(
                    self.texture,
                    pos.x,
                    pos.y,
                    WHITE,
                    DrawTextureParams{
                        dest_size: Option::from(vec2(10.0, 20.0)),
                        ..Default::default()
                    }
                );
                if pos.x < 0_f32{
                    self.shooting = false;
                    self.position = None;
                }
            }
        }
    }

    pub fn shoot(&mut self, position: Vec2){
        self.shooting = true;
        self.position = Some(position);

    }
}

