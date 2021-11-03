use macroquad::color::WHITE;
use macroquad::math::{Vec2, vec2};
use macroquad::prelude::{draw_texture_ex, DrawTextureParams, screen_height, Texture2D};
use macroquad::window::screen_width;

//amount of enemies
const AMOUNT_OF_ENEMIES: u8 = 40;
//how many layers there will be of enemies
const LAYERS_OF_ENEMIES: u8 = 4;
//calculates how many enemies there are per layer
const ENEMIES_PER_LAYER: u8 = AMOUNT_OF_ENEMIES / LAYERS_OF_ENEMIES;
//padding between enemies
const PADDING_X: i32 = 100;
const PADDING_Y: i32 = 100;


/// #Enemies struct
/// has a vector of Enemy for the enemies
/// it also has a Texture2D which will
/// be passed to the individual enemies
/// for rendering
pub struct Enemies {
    pub enemies: Vec<Enemy>,
    pub enemy_texture: Texture2D,
}

impl Enemies{
    /// Create a new enemies thing
    pub fn new(enemy_texture: Texture2D) -> Self{
        // Temporary arr
        let mut enemies: Vec<Enemy> = Vec::new();
        for i in 0..AMOUNT_OF_ENEMIES{
            enemies.push(Enemy::new(i, &enemy_texture))
        }
        Self{
            enemies,
            enemy_texture
        }
    }

    /// Function to call per frame
    pub fn render(&mut self){
        //for each enemy just render
        for e in self.enemies.iter_mut() {
            e.render()
        }
    }
}

/// # Enemy
/// has a position, size and texture
/// very cool
pub struct Enemy{
    position: Vec2,
    size: Vec2,
    index: u8,
    texture: Texture2D
}

impl Enemy{
    //render enemy, takes texture as param
    pub fn render(&mut self){
        //this is actually spaghetti and not good
        //draw the enemy
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

    /// Create a new enemy with only its index and its texture
    /// this contains all of the setup code
    pub fn new(index: u8, texture: &Texture2D) -> Self{
        //this is a bit spaghetti will try to clean up but hopefully well commented
        //declare the size because we are going to need it later
        let size = vec2(20.,35.);
        //get the width of enemies (we subtract 1 below because it will count 1 extra that is at the edge multiply it by the space between and you get width
        let width_of_enemies = (ENEMIES_PER_LAYER - 1) as i32 * PADDING_X;
        //offset is the screen width take the width of the enemies and divided by two
        //we then take the size of the sprite divided by two because the position isnt at the center
        let offset = ((screen_width() as i32 - width_of_enemies) / 2 ) - (size.x / 2.) as i32;
        //calculates what layer this enemy is on
        let layer = (index / ENEMIES_PER_LAYER);
        //calculates its position but pushed back if its on a new layer
        //that is what layer * ENEMIES_PER_LAYER does
        let position_x: f32 = (((index - layer * ENEMIES_PER_LAYER) as i32 * PADDING_X) + offset) as f32;
        //just creates the vec2 for the position | below is just spacing out the layers in the y axis
        let position = vec2(position_x, (PADDING_Y * layer as i32) as f32);
        Self{
            position,
            //the image is: 120 x 217
            size,
            index,
            texture: *texture
        }
    }
}
