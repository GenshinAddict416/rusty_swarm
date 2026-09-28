use crate::math::*;
use crate::world::*;

// called in game loop after updating positions
pub fn run_combat(world: &mut World) {

    
    let mut attackers: Vec<Entity> = Vec::new();

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
    if attacker == target {
        return;
    }

    // check for friendly fire
    let attacker_is_player =
        world.players.contains_key(attacker);
        
    let target_is_player =
        world.players.contains_key(target);
        
    if attacker_is_player == target_is_player {
        return;
    }

    update_cooldown(world);

    let Some(cooldown) =
        world.cooldowns.get_mut(attacker)
    else { return; };

    if cooldown.active > 0 {
        return;
    }

    // pos checks
    let Some(pos_a) = world.positions.get(attacker) else {return;};
    let Some(pos_t) = world.positions.get(target) else {return;};

    // get attack and defense
    let Some(atk) = world.attacks.get(attacker) else {return;};
    let Some(def) = world.defenses.get(target) else {return;};

    // if within range of attack (the sqrt of 2)
    if distance(pos_a, pos_t) < 2_f32.sqrt() && cooldown.active == 0 {
        // fetch health and subtract atk - def ONLY IF we are subtracting health
        let Some(health) = world.healths.get_mut(target) else {return;};
        health.hp -= atk.atk.saturating_sub(def.def);
        
        println!(
            "{:?} hit {:?} for {} damage with cooldown {}",
            attacker,
            target,
            atk.atk.saturating_sub(def.def),
            cooldown.active,
        );
        cooldown.active = cooldown.cd;
    }
    
}


fn update_cooldown(world: &mut World) {
    for (_ent, cd) in & mut world.cooldowns{
        if cd.active > 0 {
            cd.active -= 1;
        }
    }
}
