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

fn main() {
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
        Attack { atk: 50 },
        Defense { def: 30 },
    );

    let goblin = world.spawn(
        EntityType::Enemy,
        Position { x: 20.0, y: 50.0},
        Health { hp: 1500 },
        Attack { atk: 40 },
        Defense { def: 20 },
    );

    
    if let Some(pos) = world.positions.get(&player) {
        println!("{:?} at {}, {}", player, pos.x, pos.y);
    } else {
        println!("Player has no position components!");
    }

    if let Some(vel) = world.positions.get(&goblin) {
        println!("{:?} at {}, {}", goblin, vel.x, vel.y);
    } else {
        println!("Goblin has no velocity components!");
    }

    let mut i = 0;

    /*
    -------------------------------
    GAME LOOP
    -------------------------------
    */

    while i < 600 {
        gameloop(&mut world);
        i += 1;
        println!("tick {}", i);
        if let Some(pos) = world.positions.get(&player) {
            println!("Player: ({}, {})", pos.x, pos.y);
        }

        if let Some(pos) = world.positions.get(&goblin) {
            println!("Goblin: ({}, {})", pos.x, pos.y)
        }
    }

    if let Some(pos) = world.positions.get(&player) {
        println!("{:?} at {}, {}", player, pos.x, pos.y);
    } else {
        println!("Player has no position components!");
    }

    if let Some(pos) = world.positions.get(&goblin) {
        println!("{:?} at {}, {}", goblin, pos.x, pos.y);
    } else {
        println!("Goblin has no position components!");
    }

}


pub fn gameloop(world: &mut World) {
    ai_chase(world);
    update_movement(world);
    run_combat(world);
    death_check(world);
    bring_out_your_dead(world);
}