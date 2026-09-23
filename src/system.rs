use crate::world::*;
use crate::math::distance;
use crate::components::*;


pub fn update_movement(world: &mut World) {
    for (name, velocity) in &mut world.velocities {
        if let Some(pos) = world.positions.get_mut(name) {
            pos.x += velocity.x;
            pos.y += velocity.y;
        }
    }
}

pub fn do_damage(world: &mut World, attacker: &Entity, target: &Entity) {

    // pos checks
    let Some(pos_a) = world.positions.get(attacker) else {return;};
    let Some(pos_t) = world.positions.get(target) else {return;};

    if distance(pos_a, pos_t) < 10.0 {
        let Some(health) = world.healths.get_mut(target) else {return;};
        health.hp -= 10;
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
    }
}

pub fn bring_out_your_dead(world: &mut World) {
    let dead_entities: Vec<Entity> =
        world.deaths.keys().copied().collect();
    
    for entity in dead_entities {
        world.despawn(entity);
    }
}