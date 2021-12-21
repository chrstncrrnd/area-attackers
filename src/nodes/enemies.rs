use macroquad::color::WHITE;
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{draw_texture_ex, get_frame_time, DrawTextureParams, Texture2D};
use macroquad::window::screen_width;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::nodes::enemy_projectile::EnemyProjectile;

//amount of enemies
const AMOUNT_OF_ENEMIES: u8 = 32;
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

        Self { enemies }
    }

    /// Function to call per frame
    pub fn render(&mut self) {
        let mut rng = rand::thread_rng();
        let random_num = rng.gen_range(0..AMOUNT_OF_ENEMIES) as usize;
        //for each enemy just render
        for e in self.enemies.iter_mut() {
            e.render();
            e.shoot();
        }
        self.enemies[random_num].shoot();
    }
}

/// # Enemy
/// has a position, size and texture
/// very cool
pub struct Enemy {
    position: Vec2,
    size: Vec2,
    texture: Texture2D,
    speed_hor: i16,
    speed_ver: i16,
    start_pos: Vec2,
    projectile: EnemyProjectile,
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
    }
    pub fn shoot(&mut self){
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
            speed_hor: 50,
            speed_ver: 20,
            start_pos: position,
            projectile,
        }
    }
}
