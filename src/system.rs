use crate::world::*;
use crate::components::*;
use crate::constants;
use macroquad::prelude::*;



pub fn update_movement(world: &mut World, dt: f32) {
    for (name, velocity) in &mut world.velocities {
        if let Some(pos) = world.positions.get_mut(name) {
            pos.x += velocity.x * dt;
            pos.y += velocity.y * dt;
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
    // get player's pos and put it in a Vec2
    let Some((player_key, _)) = world.players.iter().next() else { return; };
    let Some(p_pos) = world.positions.get(player_key) else { return; };
    let player_vec = Vec2::new(p_pos.x, p_pos.y);

    // make borrow checker happy
    let mut movables: Vec<Entity> = Vec::new();
    for (current_enm, _) in &world.velocities {
        if world.enemies.contains_key(current_enm) {
            movables.push(*current_enm);
        }
    }

    // 3. Update each enemy vector toward the player
    for enm in movables {
        let Some(enm_pos) = world.positions.get(&enm) else { continue; };
        let enemy_vec = Vec2::new(enm_pos.x, enm_pos.y);
        
        // Calculate the raw distance/direction vector from enemy to player
        let to_player = player_vec - enemy_vec;
        
        // Normalize it so the length becomes exactly 1.0 diagonally or horizontally
        let move_dir = to_player.normalize_or_zero();
        
        if let Some(enm_vel) = world.velocities.get_mut(&enm) {
            // Scale by your enemy speed constant
            enm_vel.x = move_dir.x * constants::ENEMY_SPEED;
            enm_vel.y = move_dir.y * constants::ENEMY_SPEED;
        }
    }
}


pub fn render_world(world: &World) {
    for (entity, pos) in &world.positions {
        if world.players.contains_key(entity) {
            draw_circle(pos.x, pos.y, 10.0, BLUE);
        }

        if world.enemies.contains_key(entity) {
            draw_circle(pos.x, pos.y, 10.0, RED);
        }
        if let Some(health) = world.healths.get(entity) {
            draw_text(
                &health.hp.to_string(),
                pos.x - 10.0,
                pos.y - 15.0,
                20.0, 
                GREEN
            ); 
        }
    }
}
