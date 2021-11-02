use macroquad::color::WHITE;
use macroquad::math::{Vec2, vec2};
use macroquad::prelude::{draw_texture_ex, DrawTextureParams, Texture2D};


const AMOUNT_OF_ENEMIES: u8 = 10;
const LAYERS_OF_ENEMIES: u8 = 1;


pub struct Enemies {
    pub enemies: Vec<Enemy>,
    pub enemy_texture: Texture2D,

}



impl Enemies{
    pub fn setup(&mut self){
        for i in 0..AMOUNT_OF_ENEMIES{
            self.enemies.push(Enemy::new(&self.enemy_texture, i))
        }
    }

    pub fn render(&mut self){
        for e in self.enemies.iter_mut() {
            e.render()
        }
    }
}


pub struct Enemy{
    position: Vec2,
    size: Vec2,
    texture: Texture2D,
}


impl Enemy{
    pub fn render(&mut self){
        draw_texture_ex(
            self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams{
                dest_size: Option::from(self.size),
                ..Default::default()
            }
        )
    }
    pub fn new(texture: &Texture2D, index: u8) -> Self{
        Self{
            position: vec2(10. + (index  as f32 * 100.), 10.),
            size: vec2(10., 20.),
            texture: *texture
        }
    }
}
