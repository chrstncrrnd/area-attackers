use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(WHITE);

        draw_text("Wow very space invaders!", 100., 100., 100., BLACK);

        next_frame().await
    }
}
