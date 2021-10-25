mod tank;

use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let tank_texutre = load_texture("player_texture_default.png").await.unwrap();
    let mut tank = tank::Tank::new(tank_texutre);
    loop {
        clear_background(WHITE);

        draw_text("Wow very space invaders!", 100., 100., 100., BLACK);

        tank.render();

        next_frame().await
    }
}
