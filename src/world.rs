use std::collections::HashMap;

use crate::components::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub u32);

pub enum EntityType {
    Player,
    Enemy,
}

pub struct World {
    pub next_entity: u32,

    pub positions: HashMap<Entity, Position>,
    pub velocities: HashMap<Entity, Velocity>,
    pub healths: HashMap<Entity, Health>,
    pub deaths: HashMap<Entity, Dead>,
    pub players: HashMap<Entity, Player>,
    pub enemies: HashMap<Entity, Enemy>,
    pub attacks: HashMap<Entity, Attack>,
    pub defenses: HashMap<Entity, Defense>,
}

impl World {
    pub fn spawn(&mut self, 
        ent_type: EntityType, 
        init_pos: Position, 
        init_health: Health, 
        init_atk: Attack,
        init_def: Defense,
    ) -> Entity {

        let entity = Entity(self.next_entity);
        self.positions.insert(
            entity, init_pos
        );
        self.healths.insert(
            entity, init_health
        );
        self.attacks.insert(
            entity, init_atk
        );
        self.defenses.insert(
            entity, init_def
        );
        match ent_type {
            EntityType::Player => {
            self.players.insert(entity, Player);
            self.velocities.insert(entity, Velocity {x: 0.0 , y: 0.0});
            }

            EntityType::Enemy => {
            self.enemies.insert(entity, Enemy);
            self.velocities.insert(entity, Velocity {x: 0.0 , y: 0.0});
            }
            }
        self.next_entity += 1;
        println!("Entity {:?} created!", entity);
        entity
    }

    pub fn despawn(&mut self, entity: Entity) {

        println!("Entity {:?} despawned", entity);
        self.positions.remove(&entity);
        self.velocities.remove(&entity);
        self.healths.remove(&entity);
        self.deaths.remove(&entity);

        self.players.remove(&entity);
        self.enemies.remove(&entity);

        self.healths.remove(&entity);
        self.defenses.remove(&entity);

        
    }
}