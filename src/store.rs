use anymap::any::Any;
use slotmap::{DefaultKey, Key, SecondaryMap, SlotMap};

use std::collections::HashMap;
pub use std::collections::HashSet;
use topo::*;

#[derive(Debug)]
pub struct Store {
    pub id_to_key_map: HashMap<topo::Id, DefaultKey>,
    pub primary_slotmap: SlotMap<DefaultKey, Id>,
    pub anymap: anymap::Map<dyn Any>,
    pub unseen_ids: HashSet<topo::Id>,
}

impl Store {
    pub(crate) fn new() -> Store {
        Store {
            id_to_key_map: HashMap::new(),
            primary_slotmap: SlotMap::new(),
            anymap: anymap::Map::new(),
            unseen_ids: HashSet::new(),
        }
    }

    pub(crate) fn state_exists_with_topo_id<T: 'static>(&self, id: topo::Id) -> bool {
        match (self.id_to_key_map.get(&id), self.get_secondarymap::<T>()) {
            (Some(existing_key), Some(existing_secondary_map)) => {
                existing_secondary_map.contains_key(*existing_key)
            }
            (_, _) => false,
        }
    }

    pub(crate) fn get_state_with_topo_id<T: 'static>(
        &mut self,
        current_id: topo::Id,
    ) -> Option<&T> {
        self.unseen_ids.remove(&current_id);
        match (
            self.id_to_key_map.get(&current_id),
            self.get_secondarymap::<T>(),
        ) {
            (Some(existing_key), Some(existing_secondary_map)) => {
                existing_secondary_map.get(*existing_key)
            }
            (_, _) => None,
        }
    }

    pub fn mark_id_as_active(&mut self, id: topo::Id) {
        self.unseen_ids.remove(&id);
    }

    pub(crate) fn remove_state_with_topo_id<T: 'static>(
        &mut self,
        current_id: topo::Id,
    ) -> Option<T> {
        // /self.unseen_ids.remove(&current_id);

        //unwrap or default to keep borrow checker happy
        let key = self
            .id_to_key_map
            .get(&current_id)
            .copied()
            .unwrap_or_default();

        if key.is_null() {
            None
        } else if let Some(existing_secondary_map) = self.get_mut_secondarymap::<T>() {
            existing_secondary_map.remove(key)
        } else {
            None
        }
    }

    // pub(crate) fn remove_topo_id(&mut self, id: topo::Id) {
    //     let key = self.id_to_key_map.get(&id).copied().unwrap_or_default();
    //     if !key.is_null() {
    //         self.primary_slotmap.remove(key);
    //         self.id_to_key_map.remove(&id);
    //     }
    // }

    pub(crate) fn set_state_with_topo_id<T: 'static>(&mut self, data: T, current_id: topo::Id) {
        self.unseen_ids.remove(&current_id);

        //unwrap or default to keep borrow checker happy
        let key = self
            .id_to_key_map
            .get(&current_id)
            .copied()
            .unwrap_or_default();

        if key.is_null() {
            let key = self.primary_slotmap.insert(current_id);
            self.id_to_key_map.insert(current_id, key);
            if let Some(sec_map) = self.get_mut_secondarymap::<T>() {
                sec_map.insert(key, data);
            } else {
                self.register_secondarymap::<T>();
                self.get_mut_secondarymap::<T>().unwrap().insert(key, data);
            }
        } else if let Some(existing_secondary_map) = self.get_mut_secondarymap::<T>() {
            existing_secondary_map.insert(key, data);
        } else {
            self.register_secondarymap::<T>();
            self.get_mut_secondarymap::<T>().unwrap().insert(key, data);
        }
    }

    fn get_secondarymap<T: 'static>(&self) -> Option<&SecondaryMap<DefaultKey, T>> {
        self.anymap.get::<SecondaryMap<DefaultKey, T>>()
    }

    fn get_mut_secondarymap<T: 'static>(&mut self) -> Option<&mut SecondaryMap<DefaultKey, T>> {
        self.anymap.get_mut::<SecondaryMap<DefaultKey, T>>()
    }

    pub fn register_secondarymap<T: 'static>(&mut self) {
        let sm: SecondaryMap<DefaultKey, T> = SecondaryMap::new();
        self.anymap.insert(sm);
    }
}
