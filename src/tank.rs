use macroquad::prelude::*;


///# Tank struct
/// The tank is what the player will be controlling
/// pos: position,
///
/// speed: the tank speed,
/// 
/// sprite: the texture that will be rendered
pub struct Tank{
    pub pos: Vec2,
    pub speed: u8,
    pub sprite: Texture2D,
    pub size: Vec2
}


impl Tank{
    /// New function to sort of standardise the way it will
    /// be initialised
    /// takes its sprite as an argument
    pub fn new(sprite: Texture2D) -> Self{
        Self{
            pos: Vec2::new(0.,0.),
            speed: 10,
            sprite,
            size: Vec2::new(100., 100.)
        }
    }

    /// The render function
    /// this renders the tank
    /// also responsible for
    /// handling logic and stuff
    /// yeah
    pub fn render(&mut self){
        draw_texture_ex(
            self.sprite,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams{
                dest_size: Option::from(self.size),
                ..Default::default()
            }
        )
    }

}