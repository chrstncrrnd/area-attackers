mod nodes;
mod resources;

use crate::nodes::enemies::{Enemies, Enemy};
use crate::nodes::tank_projectile::TankProjectile;
use macroquad::prelude::*;
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
    let game_resources = resources::Resources::load().await.unwrap();
    //new tank
    let mut tank = tank::Tank::new(game_resources.tank, game_resources.tank_projectile);
    //new enemies
    let mut enemies = Enemies::new(game_resources.enemy, game_resources.enemy_projectile);
    println!("Done loading!");
    loop {
        //clear the background
        clear_background(GRAY);
        //render the background
        draw_texture(game_resources.background, 0.0, 0.0, WHITE);
        //render the tank
        tank.render();
        enemies.render();

        let mut enemy_to_remove: Option<usize> = None;

        for (index, enemy) in enemies.enemies.iter_mut().enumerate() {
            if enemy_tproj_overlap(enemy, &tank.projectile) {
                enemy_to_remove = Some(index);
                tank.projectile.hit_an_enemy();
            }
        }

        if let Some(e) = enemy_to_remove {
            enemies.enemies.remove(e);
        };

        // fps
        // draw_text(get_fps().to_string().as_str(), 10., 100., 100., WHITE);

        //next frame
        next_frame().await;
    }
}

fn enemy_tproj_overlap(enemy: &Enemy, tproj: &TankProjectile) -> bool {
    overlaps(
        enemy.position,
        vec2(
            enemy.position.x + enemy.size.x,
            enemy.position.y - enemy.size.y,
        ),
        tproj.position,
        vec2(
            tproj.position.x + tproj.size.x,
            tproj.position.y - tproj.size.y,
        ),
    )
}

fn overlaps(tl_1: Vec2, br_1: Vec2, tl_2: Vec2, br_2: Vec2) -> bool {
    tl_1.y > br_2.y && br_1.y < tl_2.y && br_1.x > tl_2.x && tl_1.x < br_2.x
}
