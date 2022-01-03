//! # This is the resource file loading file
//! yeah it loads resources from disk
use macroquad::miniquad::FilterMode;
use macroquad::prelude::{FileError, Font, load_texture, Texture2D};
use macroquad::text::load_ttf_font;

/// # Resources struct
/// this contains all of the game required resources.
/// there arent many so its fine for now
/// TODO: Add a texture atlas for more efficient loading
pub struct Resources {
    pub enemy: Texture2D,
    pub enemy_projectile: Texture2D,
    pub tank: Texture2D,
    pub tank_projectile: Texture2D,
    pub background: Texture2D,
    pub font: Font,
}

/// actual loader for the resources
impl Resources {
    //new function, returns the resources or a file error if something goes wrong
    pub async fn load() -> Result<Resources, FileError> {
        println!("Starting resource loading");

        //all of the image loadings do the same thing might be able to add a for loop idk
        //1) set a non mut variable to load the texture from its file ./ being where src/ and Cargo.toml are
        let enemy = load_texture("assets/enemy.png").await.unwrap();
        //2) set the filter to make sure that its not blurred or anything
        enemy.set_filter(FilterMode::Nearest);
        //3) print it has been successful
        println!("Loaded 1/6 assets");

        let enemy_projectile = load_texture("assets/enemy_projectile.png").await.unwrap();
        enemy_projectile.set_filter(FilterMode::Nearest);
        println!("Loaded 2/6 assets");

        let tank = load_texture("assets/tank.png").await.unwrap();
        tank.set_filter(FilterMode::Nearest);
        println!("Loaded 3/6 assets");

        let tank_projectile = load_texture("assets/tank_projectile.png").await.unwrap();
        tank_projectile.set_filter(FilterMode::Nearest);
        println!("Loaded 4/6 assets");

        let background = load_texture("assets/background.png").await.unwrap();
        background.set_filter(FilterMode::Nearest);
        println!("Loaded 5/6 assets");

        //font is the same as the images except there is no filter
        let font = load_ttf_font("assets/font.ttf").await.unwrap();
        println!("Loaded 6/6 assets");
        println!("Finished loading assets");
        //if everything goes well return resources with all of the previous variables
        //returns it in the shorthand state
        Ok(Resources {
            enemy,
            enemy_projectile,
            tank,
            tank_projectile,
            background,
            font,
        })
    }
}
