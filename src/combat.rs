use crate::math::*;
use crate::world::*;
use std::collections::HashMap;

// called in game loop after updating positions
pub fn run_combat(world: &mut World) {

    // make borrow checker happy
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

    for (ent, _entype) in &world.enemies {
        attackers.push(
            *ent
        );
    }

    for (play, _entype) in &world.players {
        attackers.push(
            *play
        );
    }

    for attacker in &attackers {
        for target in &attackers {
            do_damage(world, attacker, target);
        }
    }


}


pub fn do_damage(world: &mut World, attacker: &Entity, target: &Entity) {

    // check if the attacker is accidentally the target
    if attacker.0 == target.0 {return;}

    // pos checks
    let Some(pos_a) = world.positions.get(attacker) else {return;};
    let Some(pos_t) = world.positions.get(target) else {return;};

    // get attack and defense
    let Some(atk) = world.attacks.get(attacker) else {return;};
    let Some(def) = world.defenses.get(target) else {return;};

    // if within range of attack (the sqrt of 2)
    if distance(pos_a, pos_t) < 2_f32.sqrt() {
        // fetch health and subtract atk - def ONLY IF we are subtracting health
        let Some(health) = world.healths.get_mut(target) else {return;};
        health.hp -= atk.atk.saturating_sub(def.def);
    } 
}
