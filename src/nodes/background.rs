use macroquad::prelude::*;


pub fn render(){
    //actual background
    draw_rectangle(0., 0., screen_width(), screen_height(), Color::new(0.46, 0.46, 0.46, 1.));
    //grass foreground
    draw_rectangle(0., screen_height() - 70., screen_width(), 70., Color::new(0.16, 0.50, 0.16, 0.8))
}
