mod tank;
mod resources;

use macroquad::prelude::*;

#[macroquad::main("Area Attackers")]
async fn main() {
    println!("Loading game...");
    //the loading stuff
    let game_resources = resources::Resources::new().await.unwrap();

    let mut tank = tank::Tank::new(game_resources.tank);
    println!("Done loading!");
    loop {
        clear_background(WHITE);

        tank.render();

        next_frame().await;
    }
}
