use crate::world::*;
use crate::math::*;
use crate::components::*;
use crate::constants;


pub fn update_movement(world: &mut World) {
    for (name, velocity) in &mut world.velocities {
        if let Some(pos) = world.positions.get_mut(name) {
            pos.x += velocity.x;
            pos.y += velocity.y;
        }
    }
}


pub fn death_check(world: &mut World) {
    let mut dead_entities = Vec::new();

    for (entity, health) in &world.healths {
        if health.hp <= 0 && !world.deaths.contains_key(entity) {
            dead_entities.push(*entity);
        }
    }

    for entity in dead_entities {
        world.deaths.insert(entity, Dead);
        println!("Entity {:?} died", entity)
    }
}

pub fn bring_out_your_dead(world: &mut World) {
    let dead_entities: Vec<Entity> =
        world.deaths.keys().copied().collect();
    
    for entity in dead_entities {
        world.despawn(entity);
    }
}

pub fn ai_chase(world: &mut World) {
    let (player_key, _) = world.players.iter().next().unwrap();
    
    let player_pos = match world.positions.get(player_key) {
        Some(pos) => (pos.x, pos.y),
        None => return,
    };

    let mut movables: Vec<Entity> = Vec::new();
    for (current_enm, _) in &world.velocities {
        if world.enemies.contains_key(current_enm) {
            movables.push(*current_enm);
        }
    }

    for enm in movables {
        let Some(enm_pos) = world.positions.get(&enm) else { continue; };
        
        let x_mod = sign(enm_pos.x, player_pos.0);
        let y_mod = sign(enm_pos.y, player_pos.1);
        
        if let Some(enm_vel) = world.velocities.get_mut(&enm) {
            enm_vel.x = x_mod * constants::ENEMY_SPEED;
            enm_vel.y = y_mod * constants::ENEMY_SPEED;
        }
    }
}
