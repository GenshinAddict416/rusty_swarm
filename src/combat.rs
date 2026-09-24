use crate::math::*;
use crate::world::*;
use crate::components::*;
use std::collections::HashMap;

fn run_combat(world: &mut World) {
    let mut combat_positions = HashMap::new();
    let mut combat_attacks = HashMap::new();
    let mut combat_defenses = HashMap::new();
    let mut attackers: Vec<Entity> = Vec::new();

    for (ent, pos) in &world.positions {
        combat_positions.insert(
            ent,
            pos,
        );
    }

    for (ent, atk) in &world.attacks {
        combat_attacks.insert(
            ent,
            atk,
        );
    }

    for (ent, def) in &world.defenses {
        combat_defenses.insert(
            ent,
            def,
        );
    }






}


pub fn do_damage(world: &mut World, attacker: &Entity, target: &Entity) {

    if attacker.0 == target.0 {return;}
    // pos checks
    let Some(pos_a) = world.positions.get(attacker) else {return;};
    let Some(pos_t) = world.positions.get(target) else {return;};

    if distance(pos_a, pos_t) < 2_f32.sqrt() {
        let Some(health) = world.healths.get_mut(target) else {return;};
        health.hp -= 10;
    } 
}
