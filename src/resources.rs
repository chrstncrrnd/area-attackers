use macroquad::miniquad::FilterMode;
use macroquad::prelude::{FileError, load_texture, Texture2D};

pub struct Resources{
    pub enemy: Texture2D,
    pub enemy_projectile: Texture2D,
    pub tank: Texture2D,
    pub tank_projectile: Texture2D,
}


impl Resources{
    pub async fn new() -> Result<Resources, FileError> {
        println!("Starting resource loading");

        let enemy = load_texture("assets/enemy.png").await.unwrap();
        enemy.set_filter(FilterMode::Nearest);
        println!("Loaded 1/4 textures");

        let enemy_projectile = load_texture("assets/enemy_projectile.png").await.unwrap();
        enemy_projectile.set_filter(FilterMode::Nearest);
        println!("Loaded 2/4 textures");

        let tank = load_texture("assets/tank.png").await.unwrap();
        tank.set_filter(FilterMode::Nearest);
        println!("Loaded 3/4 textures");

        let tank_projectile = load_texture("assets/tank_projectile.png").await.unwrap();
        tank_projectile.set_filter(FilterMode::Nearest);
        println!("Finished loading resources");
        Ok(
            Resources{
                enemy,
                enemy_projectile,
                tank,
                tank_projectile
            }
        )

    }
}
