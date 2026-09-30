mod components;
mod world;
mod system;
mod math;
mod constants;
mod combat;

use components::*;
use system::*;
use world::*;
use combat::*;

use std::collections::HashMap;

use macroquad::prelude::*;

#[macroquad::main("ECS")]
async fn main() {
    let mut world = World {
        next_entity: 0,
        positions: HashMap::new(),
        velocities: HashMap::new(),
        healths: HashMap::new(),
        deaths: HashMap::new(),
        enemies: HashMap::new(),
        players: HashMap::new(),
        attacks: HashMap::new(),
        defenses: HashMap::new(),
        cooldowns: HashMap::new(),
    };

    let player = world.spawn(
        EntityType::Player, 
        Position { x: 50.0, y: 50.0 },
        Health { hp: 100 },
        Attack { atk: 20 },
        Defense { def: 90 },
    );

    for i in 0..10 {
        world.spawn(
            EntityType::Enemy,
            Position {
                x: 100.0 + i as f32 * 30.0,
                y: 100.0,
            },
            Health { hp: 50 },
            Attack { atk: 10 },
            Defense { def: 0 },
        );
    }


    /*
    -------------------------------
    GAME LOOP
    -------------------------------
    */

    loop {
        clear_background(BLACK);
        let dt = get_frame_time();

        if let Some(vel) = world.velocities.get_mut(&player) {
            let mut input_dir = Vec2::ZERO;

            if is_key_down(KeyCode::W) {
                input_dir.y -= 1.0;
            }
            if is_key_down(KeyCode::S) {
                input_dir.y += 1.0;
            }
            if is_key_down(KeyCode::A) {
                input_dir.x -= 1.0;
            }
            if is_key_down(KeyCode::D) {
                input_dir.x += 1.0;
            }
            // 2. Normalize the vector so its diagonal length is exactly 1.0
            // .normalize_or_zero() safely handles the case where no keys are pressed without crashing
            let move_dir = input_dir.normalize_or_zero();

            // 3. Scale by your actual player speed constant
            vel.x = move_dir.x * constants::PLAYER_SPEED;
            vel.y = move_dir.y * constants::PLAYER_SPEED;
        }

        gameloop(&mut world, dt);

        render_world(&world);

        next_frame().await;
    }
}


pub fn gameloop(world: &mut World, dt: f32) {
    ai_chase(world);
    update_movement(world, dt);
    run_combat(world);
    death_check(world);
    bring_out_your_dead(world);
}