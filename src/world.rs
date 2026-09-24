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
    pub fn spawn(&mut self, ent_type: EntityType) -> Entity {
        let entity = Entity(self.next_entity);
        match ent_type {
            EntityType::Player => {
            self.players.insert(entity, Player);
            }

            EntityType::Enemy => {
            self.enemies.insert(entity, Enemy);
            }
            }
        self.next_entity += 1;
        entity
    }

    pub fn despawn(&mut self, entity: Entity) {
        self.positions.remove(&entity);
        self.velocities.remove(&entity);
        self.healths.remove(&entity);
        self.deaths.remove(&entity);

        self.players.remove(&entity);
        self.enemies.remove(&entity);
    }
}