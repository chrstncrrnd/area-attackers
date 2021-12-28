use crate::nodes::tank_projectile::TankProjectile;
use macroquad::prelude::*;

///# Tank struct
/// The tank is what the player will be controlling
/// pos: position,
///
/// speed: the tank speed,
///
/// sprite: the texture that will be rendered
pub struct Tank {
    pub projectile: TankProjectile,
    pos: Vec2,
    speed: u16,
    sprite: Texture2D,
    size: Vec2,
}

impl Tank {
    /// New function to sort of standardise the way it will
    /// be initialised
    /// takes its sprite as an argument
    pub fn new(sprite: Texture2D, projectile_sprite: Texture2D) -> Self {
        Self {
            pos: vec2(screen_width() / 2., screen_height() - 300.),
            speed: 400,
            sprite,
            size: vec2(150., 100.),
            projectile: TankProjectile::new(projectile_sprite),
        }
    }

    /// The render function
    /// this renders the tank
    /// also responsible for
    /// handling logic and stuff
    /// yeah
    pub fn render(&mut self) {
        //movement code
        // TODO: smooth movement
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.pos.x -= self.speed as f32 * get_frame_time();
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.pos.x += self.speed as f32 * get_frame_time();
        }
        //this might be exploitable
        self.pos.y = screen_height() - (self.size.y + 10.);
        if self.pos.x < 0. {
            self.pos.x = 0.;
        } else if self.pos.x > screen_width() - self.size.x {
            self.pos.x = screen_width() - self.size.x;
        }

        //if the space key is pressed shoot the projectile
        if is_key_released(KeyCode::Space) {
            //some stuff to make the projectile launch in the middle of the tank
            self.projectile
                .shoot(vec2(self.pos.x + self.size.x / 2., self.pos.y - 10.))
        }

        //call the render function of the projectile
        self.projectile.render();

        //draw the tank texture
        draw_texture_ex(
            self.sprite,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Option::from(self.size),
                ..Default::default()
            },
        )
    }
}
