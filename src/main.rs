mod nodes;
mod resources;

use crate::nodes::enemies::Enemies;
use macroquad::prelude::*;
use nodes::background;
use nodes::tank;

//we do a little configuration
fn window_config() -> Conf {
    Conf {
        window_title: String::from("Area Attackers"),
        window_resizable: false,
        window_height: 800,
        window_width: 1000,
        ..Default::default()
    }
}

//main (very useful comment)
#[macroquad::main(window_config)]
async fn main() {
    println!("Loading game...");
    //the loading stuff
    let game_resources = resources::Resources::new().await.unwrap();
    //new tank
    let mut tank = tank::Tank::new(game_resources.tank, game_resources.tank_projectile);

    let mut enemies = Enemies::new(game_resources.enemy, game_resources.enemy_projectile);
    println!("Done loading!");
    loop {
        //clear the background
        clear_background(GRAY);
        //render the background
        background::render();
        //render the tank
        tank.render();
        enemies.render();

        //next frame
        next_frame().await;
    }
}
