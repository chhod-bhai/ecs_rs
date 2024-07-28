use std::{cell::RefCell, collections::BTreeMap};

use super::entity::{Entity, EntityTag};
type SharedEntity = RefCell<Entity>;

pub struct EntityManager {
    entity_ctr: u32,
    pub entities: BTreeMap<u32, SharedEntity>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: BTreeMap::<u32, SharedEntity>::new(),
            entity_ctr: 0,
        }
    }
    pub fn add(&mut self, tag: EntityTag) -> u32 {
        let id = self.entity_ctr;
        let entity = Entity {
            id,
            tag: tag,
            shape: None,
            color: None,
            transform: None,
            label: None,
        };
        self.entities.insert(id, RefCell::new(entity));
        self.entity_ctr = self.entity_ctr + 1;
        return id;
    }
    pub fn remove(&mut self, entity_id: u32) {
        let removed = self.entities.remove(&entity_id);
        if removed.is_none() {
            eprintln!("No such entity found with given id");
        }
    }
    pub fn get_by_id(&self, entity_id: u32) -> Option<&RefCell<Entity>> {
        return self.entities.get(&entity_id);
    }
}
