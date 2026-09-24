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
        defenses: HashMap::new()
    };

    let player = world.spawn(EntityType::Player);
    world.positions.insert(
        player,
        Position {
            x: 0.0,
            y: 0.0,
        },
    );
    world.velocities.insert(
        player,
        Velocity {
            x: constants::PLAYER_SPEED,
            y: 0.0,
        },
    );
    world.healths.insert(
        player,
        Health {
            hp: 100,
        }
    );

    let goblin = world.spawn(EntityType::Enemy);
    world.positions.insert(
        goblin,
        Position {
            x: 25.0,
            y: 0.0,
        },
    );
    world.velocities.insert(
        goblin,
        Velocity {
            x: 0.0,
            y: 0.0,
        },
    );
    world.healths.insert(
        goblin,
        Health {
            hp: 50,
        }
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

    while i < 60 {
        ai_chase(&mut world);
        update_movement(&mut world);
        // do_damage(&mut world, &player, &goblin);
        death_check(&mut world);
        bring_out_your_dead(&mut world);
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