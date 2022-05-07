//! # This is the resource file loading file
//! yeah it loads resources from disk
use macroquad::prelude::{FileError, Font, load_texture, Texture2D};
use macroquad::text::load_ttf_font;


/// # Resources struct
/// this contains all of the game required resources.
/// there arent many so its fine for now
pub struct Resources {
    pub enemy: Texture2D,
    pub enemy_projectile: Texture2D,
    pub tank: Texture2D,
    pub tank_projectile: Texture2D,
    pub background: Texture2D,
    pub font: Font,
}

// Macro for DRY loading
macro_rules! load {
    ($($var_name:ident),+) => {
        $(
            let var_name_str = stringify!($var_name);
            let path = format!("assets/{}.png", var_name_str);
            let $var_name = macroquad::prelude::load_texture(path.as_str()).await.unwrap();
            $var_name.set_filter(macroquad::prelude::FilterMode::Nearest);
        )+
    };
}




/// actual loader for the resources
impl Resources {
    //new function, returns the resources or a file error if something goes wrong
    pub async fn load() -> Result<Resources, FileError> {
        load!(enemy, enemy_projectile, tank, tank_projectile, background);
        // not really worth putting font in the load macro.
        let font = load_ttf_font("assets/font.ttf").await.unwrap();
        //if everything goes well return resources with all of the previous variables
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

