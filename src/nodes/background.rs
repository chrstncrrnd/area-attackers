use macroquad::prelude::*;

pub fn render(){
    //grass foreground
    draw_rectangle(0., screen_height() - 70., screen_width(), 70., Color::new(0.16, 0.50, 0.16, 0.8));
}
