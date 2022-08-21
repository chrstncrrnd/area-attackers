use macroquad::prelude::*;

use nodes::tank;

use crate::nodes::enemies::{Enemies, Enemy, AMOUNT_OF_ENEMIES};
use crate::nodes::enemy_projectile::EnemyProjectile;
use crate::nodes::tank_projectile::TankProjectile;
use crate::tank::Tank;

mod nodes;
mod resources;

const TOTAL_PLAYER_LIVES: u8 = 3;

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

    let mut player_lives: u8 = TOTAL_PLAYER_LIVES;
    let mut points: u8 = 0;

    println!("Done loading!");
    loop {
        
        clear_background(GRAY);

        // if the player is dead
        if player_lives <= 0 {
            draw_centered_text("You died!", 100., Some(vec2(0., -100.)));
            draw_centered_text(format!("Score: {}", points).as_str(), 75., None);
            draw_centered_text("Press [space] to restart", 80., Some(vec2(0., 100.,)));
            
            if is_key_released(KeyCode::Space){
                player_lives = TOTAL_PLAYER_LIVES;
                points = 0;
                enemies.reset();
            }

        }
        else if points >= AMOUNT_OF_ENEMIES{
            draw_centered_text("You won!", 100., Some(vec2(0., -100.)));
            draw_centered_text(format!("Press [space] to restart").as_str(), 90., None);
            if is_key_released(KeyCode::Space){
                player_lives = TOTAL_PLAYER_LIVES;
                points = 0;
                enemies.reset();
            }
        }
        // main game
        else{//clear the background
            //render the background
            draw_texture(game_resources.background, 0.0, 0.0, WHITE);
            //render the tank
            tank.render();
            enemies.render();

            draw_text(format!("Score: {}/{}", points, AMOUNT_OF_ENEMIES).as_str(), 0., 30., 30., WHITE);
            draw_text(format!("Lives: {}/{}", player_lives, TOTAL_PLAYER_LIVES).as_str(), 0., 60., 30., WHITE);
        
            // new scope to handle enemy death logic (yes i know that im only creating enemy_to_remove and
            // its the only thing that will get dropped, the new scope is just here to organise a bit more idk
            // ENEMY DEATH
            {
                let mut enemy_to_remove: Option<usize> = None;
    
                for (index, enemy) in enemies.enemies.iter_mut().enumerate() {
                    if enemy_tproj_overlap(enemy, &tank.projectile) {
                        enemy_to_remove = Some(index);
                        tank.projectile.hit_an_enemy();
                        points += 1;
                    }
                }
    
                if let Some(e) = enemy_to_remove {
                    enemies.enemies.remove(e);
                };
            }
            // another scope to see if the tank is hit by an enemy projectile
            // TANK DEATH
            {
                let mut hit: bool = false;
                for enemy in enemies.enemies.iter_mut(){
                    let eproj = &mut enemy.projectile;
    
                    if tank_eproj_overlap(&tank, eproj){
                        println!("Hit by an enemy projectile!");
                        hit = true;
                        player_lives -= 1;
                    }
                }

                if hit{
                    enemies.retract_all_projectiles();
                }
            }
        }

        // fps
        draw_text(get_fps().to_string().as_str(), 10., 100., 100., WHITE);

        //next frame
        next_frame().await;
    }
}


fn draw_centered_text(text: &str, font_size: f32, offset_: Option<Vec2>) {
    let text_size = measure_text(text, None, font_size as u16, 1.);
    
    let win_width = window_config().window_width as f32;
    let win_height = window_config().window_height as f32;

    let mut pos_x = (win_width / 2.) - (text_size.width / 2.);
    let mut pos_y = (win_height / 2.) - (text_size.height / 2.);

    if let Some(offset) = offset_{
        pos_x += offset.x;
        pos_y += offset.y;
    } 

    draw_text(text, pos_x, pos_y, font_size, WHITE);
}

//function to see if an enemy is hit by a tank projectile
#[inline]
fn enemy_tproj_overlap(enemy: &Enemy, tproj: &TankProjectile) -> bool {
    overlaps(
        enemy.position,
        vec2(
            enemy.position.x + enemy.size.x,
            enemy.position.y + enemy.size.y,
        ),
        tproj.position,
        vec2(
            tproj.position.x + tproj.size.x,
            tproj.position.y + tproj.size.y,
        ),
    )
}

//function to see if the tank is hit by an enemy projectile
#[inline]
fn tank_eproj_overlap(tank: &Tank, eproj: &EnemyProjectile) -> bool {
    overlaps(
        tank.pos,
        vec2(
            tank.pos.x + tank.size.x,
            tank.pos.y + tank.size.y
        ),
        eproj.position,
        vec2(
            eproj.position.x + eproj.size.x,
            eproj.position.y + eproj.size.y,
        ),
    )
}

#[inline]
fn overlaps(tl_1: Vec2, br_1: Vec2, tl_2: Vec2, br_2: Vec2) -> bool {
    br_1.x > tl_2.x && br_2.x > tl_1.x && br_1.y > tl_2.y && br_2.y > tl_1.y
}
