use macroquad::color::WHITE;
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{draw_texture_ex, DrawTextureParams, get_frame_time, Texture2D};
use macroquad::window::screen_width;
use rand::Rng;

use crate::nodes::enemy_projectile::EnemyProjectile;

//amount of enemies
pub const AMOUNT_OF_ENEMIES: u8 = 32;
//how many layers there will be of enemies
const LAYERS_OF_ENEMIES: u8 = 4;
//calculates how many enemies there are per layer
const ENEMIES_PER_LAYER: u8 = AMOUNT_OF_ENEMIES / LAYERS_OF_ENEMIES;
//padding between enemies
const PADDING_X: i32 = 90;
const PADDING_Y: i32 = 100;

/// #Enemies struct
/// has a vector of Enemy for the enemies
/// it also has a Texture2D which will
/// be passed to the individual enemies
/// for rendering
pub struct Enemies {
    pub enemies: Vec<Enemy>,
    enemy_texture: Texture2D,
    enemy_projectile_texture: Texture2D,
}

impl Enemies {
    /// Create a new enemies thing
    pub fn new(enemy_texture: Texture2D, enemy_projectile_texture: Texture2D) -> Self {
        // Temporary arr
        let mut enemies: Vec<Enemy> = Vec::with_capacity(AMOUNT_OF_ENEMIES as usize);
        for i in 0..AMOUNT_OF_ENEMIES {
            enemies.push(Enemy::new(
                i,
                enemy_texture.clone(),
                enemy_projectile_texture.clone(),
            ))
        }

        Self { enemies, enemy_texture, enemy_projectile_texture }
    }

    pub fn reset(&mut self){
        self.enemies.clear();
        for i in 0..AMOUNT_OF_ENEMIES{
            self.enemies.push(Enemy::new(
               i, 
               self.enemy_texture.clone(), 
               self.enemy_projectile_texture.clone()
            ))
        }
    }

    /// Function to call per frame
    pub fn render(&mut self) {
        for e in self.enemies.iter_mut() {
            e.render();
        }
        //for each enemy just render
    }

    pub fn retract_all_projectiles(&mut self){
        for enemy in self.enemies.iter_mut(){
            enemy.projectile.retract_projectile();
        }
    }
}

/// # Enemy
/// has a position, size and texture
/// very cool
pub struct Enemy {
    pub position: Vec2,
    pub size: Vec2,
    pub projectile: EnemyProjectile,
    texture: Texture2D,
    speed_hor: i16,
    speed_ver: i16,
    start_pos: Vec2,
    frames_until_shoot: u16,
}

impl Enemy {
    //render enemy, takes texture as param
    pub fn render(&mut self) {
        let position_from_start = vec2(
            self.position.x - self.start_pos.x,
            self.position.y - self.start_pos.y,
        );

        if position_from_start.x >= 100. || position_from_start.x <= -100. {
            self.speed_hor *= -1;
        }
        if position_from_start.y >= 50. || position_from_start.y <= -10. {
            self.speed_ver *= -1;
        }

        let distance_to_move_h = self.speed_hor as f32 * get_frame_time();
        let distance_to_move_v = self.speed_ver as f32 * get_frame_time();
        self.position.x += distance_to_move_h;
        self.position.y += distance_to_move_v;
        self.projectile.render();
        //draw the enemy
        draw_texture_ex(
            self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Option::from(self.size),
                ..Default::default()
            },
        );
        if self.frames_until_shoot == 0 {
            self.shoot();
            self.frames_until_shoot = rand::thread_rng().gen_range(500..1000);
        } else {
            self.frames_until_shoot -= 1;
        }
    }
    pub fn shoot(&mut self) {
        self.projectile.shoot(self.position);
    }

    pub fn new(index: u8, texture: Texture2D, projectile_texture: Texture2D) -> Self {
        //this is a bit spaghetti will try to clean up but hopefully well commented
        //declare the size because we are going to need it later
        let size = vec2(24., 43.);
        //get the width of enemies (we subtract 1 below because it will count 1 extra that is at the edge multiply it by the space between and you get width
        let width_of_enemies = (ENEMIES_PER_LAYER - 1) as i32 * PADDING_X;
        //offset is the screen width take the width of the enemies and divided by two
        //we then take the size of the sprite divided by two because the position isnt at the center
        let offset = ((screen_width() as i32 - width_of_enemies) / 2) - (size.x / 2.) as i32;
        //calculates what layer this enemy is on
        let layer = index / ENEMIES_PER_LAYER;
        //calculates its position but pushed back if its on a new layer
        //that is what layer * ENEMIES_PER_LAYER does
        let position_x: f32 =
            (((index - layer * ENEMIES_PER_LAYER) as i32 * PADDING_X) + offset) as f32;
        //just creates the vec2 for the position | below is just spacing out the layers in the y axis
        let position = vec2(position_x, (PADDING_Y * layer as i32) as f32 + 50.);
        let projectile = EnemyProjectile::new(projectile_texture);
        Self {
            position,
            //the image size is: 120 x 217
            size,
            texture,
            speed_hor: 40,
            speed_ver: 15,
            start_pos: position,
            projectile,
            frames_until_shoot: rand::thread_rng().gen_range(0..1500),
        }
    }
}
