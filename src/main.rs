mod resources;
mod nodes;


use nodes::tank;
use macroquad::prelude::*;
use nodes::background;

//window config does nothing more than it did now but
//good to change it later
fn window_config() -> Conf {
    Conf {
        window_title: String::from("Area Attackers"),
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    println!("Loading game...");
    //the loading stuff
    let game_resources = resources::Resources::new().await.unwrap();
    //new tank
    let mut tank = tank::Tank::new(game_resources.tank, game_resources.tank_projectile);
    println!("Done loading!");
    loop {
        //clear the background
        clear_background(Color::new(0.46, 0.46, 0.46, 1.));
        //render the background
        background::render();
        //render the tank
        tank.render();
        //next frame
        next_frame().await;
    }
}
