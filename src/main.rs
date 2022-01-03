use macroquad::prelude::*;

use nodes::tank;

use crate::nodes::enemies::{Enemies, Enemy};
use crate::nodes::enemy_projectile::EnemyProjectile;
use crate::nodes::tank_projectile::TankProjectile;
use crate::tank::Tank;

mod nodes;
mod resources;

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
        // new scope to handle enemy death logic (yes i know that im only creating enemy_to_remove and
        // its the only thing that will get dropped, the new scope is just here to organise a bit more idk
        {
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
        }
        // another scope to see if the tank is hit by an enemy projectile
        {
            for enemy in enemies.enemies.iter_mut(){
                let eproj = &mut enemy.projectile;

                if tank_eproj_overlap(&tank, eproj){
                    println!("Hit by an enemy projectile!");
                    eproj.retract_projectile();
                }

            }
        }

        // fps
        // draw_text(get_fps().to_string().as_str(), 10., 100., 100., WHITE);

        //next frame
        next_frame().await;
    }
}


//function to see if an enemy is hit by a tank projectile
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

//function to see if the tank is hit by an enemy projectile
fn tank_eproj_overlap(tank: &Tank, eproj: &EnemyProjectile) -> bool {
    overlaps(
        tank.pos,
        vec2(
            tank.pos.x + tank.size.x,
            tank.pos.y - tank.size.y
        ),
        eproj.position,
        vec2(
            eproj.position.x + eproj.size.x,
            eproj.position.y - eproj.size.y,
        ),
    )
}

fn overlaps(tl_1: Vec2, br_1: Vec2, tl_2: Vec2, br_2: Vec2) -> bool {
    tl_1.y > br_2.y && br_1.y < tl_2.y && br_1.x > tl_2.x && tl_1.x < br_2.x
}
