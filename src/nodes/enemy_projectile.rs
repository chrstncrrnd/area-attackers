use macroquad::prelude::Texture2D;

pub struct EnemyProjectile{
    texture: Texture2D
}

impl EnemyProjectile{
    pub fn new(texture: Texture2D) -> Self{
        Self{
            texture
        }
    }

    pub fn render(){

    }
}

