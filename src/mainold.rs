use std::collections::HashMap;

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

struct Velocity {
    x: f32,
    y: f32,
}

struct Health {
    hp: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Entity(u32);

struct World {
    next_entity: u32,
    positions: HashMap<Entity, Position>,
    velocities: HashMap<Entity, Velocity>,
    healths: HashMap<Entity, Health>,
    deaths: HashMap<Entity, Dead>,
}

impl World {
    fn spawn(&mut self) -> Entity {
        let entity = Entity(self.next_entity);
        self.next_entity += 1;
        entity
    }

    fn despawn(&mut self, entity: Entity) {
        self.positions.remove(&entity);
        self.velocities.remove(&entity);
        self.healths.remove(&entity);
        self.deaths.remove(&entity);
    }
}

struct Dead;

fn main() {
    let mut world = World {
        next_entity: 0,
        positions: HashMap::new(),
        velocities: HashMap::new(),
        healths: HashMap::new(),
        deaths: HashMap::new(),
    };

    let player = world.spawn();
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
            x: 50.0 / 60.0, // pixels per second
            y: 0.0,
        },
    );
    world.healths.insert(
        player,
        Health {
            hp: 100,
        }
    );

    let goblin = world.spawn();
    world.positions.insert(
        goblin,
        Position {
            x: 100.0,
            y: 0.0,
        },
    );
    world.velocities.insert(
        goblin,
        Velocity {
            x: -0.5,
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

    if let Some(vel) = world.velocities.get(&player) {
        println!("{:?} moving at {}, {}", player, vel.x, vel.y);
    } else {
        println!("Player has no velocity components!");
    }

    let mut i = 0;

    while (i < 60) {
        update_movement(&mut world);
        do_damage(&mut world, &player, &goblin);
        death_check(&mut world);
        bring_out_your_dead(&mut world);
        i += 1;
        println!("tick {}", i);
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

fn update_movement(world: &mut World) {
    for (name, velocity) in &mut world.velocities {
        if let Some(pos) = world.positions.get_mut(name) {
            pos.x += velocity.x;
            pos.y += velocity.y;
        }
    }
}

fn do_damage(world: &mut World, attacker: &Entity, target: &Entity) {

    // pos checks
    let Some(pos_a) = world.positions.get(attacker) else {return;};
    let Some(pos_t) = world.positions.get(target) else {return;};

    if distance(pos_a, pos_t) < 10.0 {
        let Some(health) = world.healths.get_mut(target) else {return;};
        health.hp -= 10;
    } 
}

fn distance(a: &Position, b: &Position) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    (dx * dx + dy * dy).sqrt()
}

fn death_check(world: &mut World) {
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

fn bring_out_your_dead(world: &mut World) {
    let dead_entities: Vec<Entity> =
        world.deaths.keys().copied().collect();
    
    for entity in dead_entities {
        world.despawn(entity);
    }
}