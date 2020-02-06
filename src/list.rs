use crate::state_access::{CloneState, StateAccess};
use crate::state_functions::use_state;
use slotmap::{new_key_type, DenseSlotMap, Key};

new_key_type! {
    pub struct ListKey;
}

pub fn use_list<T, F>(initial_list_fn: F) -> ListControl<T>
where
    F: FnOnce() -> Vec<T>,
    T: Clone,
{
    let list_access = use_state(|| List::new(initial_list_fn()));

    ListControl::new(list_access)
}

#[derive(Clone)]
pub struct ListControl<T>
where
    T: Clone + 'static,
{
    list_access: StateAccess<List<T>>,
}

impl<T> ListControl<T>
where
    T: Clone + 'static,
{
    fn new(list_access: StateAccess<List<T>>) -> ListControl<T> {
        ListControl { list_access }
    }

    pub fn get_list(&self) -> List<T> {
        self.list_access.get()
    }

    pub fn clear(&self) {
        self.list_access.update(|list| {
            list.items_map = ListKeyDenseSlotMap::new();
            list.items_order = vec![];
        });
    }

    // brain always gets this messed up so I have to write it down!
    // 0 1 2 3 4 5 6
    // a b c d e f g

    // I want to move c after d (which should be remove 2 put in 3)
    // remove(2)

    // 0 1 2 3 4 5 6
    // a b d e f g

    // insert(3)

    // 0 1 2 3 4 5 6
    // a b d e f g
    //
    //
    // 0 1 2 3 4 5 6
    // a b c d e f g

    // I want to move f after d (which should be remove 5 put in 4)
    // remove(2)

    // 0 1 2 3 4 5 6
    // a b d e f g

    // insert(3)

    // 0 1 2 3 4 5 6
    // a b d e f g
    pub fn move_item_to_position(&self, old_idx: usize, new_idx: usize) {
        let mut list = self.list_access.get();
        if new_idx > list.items_order.len() || old_idx > list.items_order.len() - 1 {
            return;
        }

        let old_item = list.items_order.remove(old_idx);
        use std::cmp::Ordering;
        match old_idx.cmp(&new_idx) {
            Ordering::Less => list.items_order.insert(new_idx - 1, old_item),
            Ordering::Greater => list.items_order.insert(new_idx, old_item),
            Ordering::Equal => {}
        }
        self.list_access.set(list);
    }

    pub fn move_item_up(&self, old_idx: usize) {
        if old_idx == 0 {
            return;
        }
        self.move_item_to_position(old_idx, old_idx - 1);
    }

    pub fn move_item_down(&self, old_idx: usize) {
        self.move_item_to_position(old_idx, old_idx + 2);
    }

    pub fn insert(&self, idx: usize, item: T) {
        let mut list = self.list_access.get();
        let inserted_key = list.items_map.0.insert(item);
        list.items_order.insert(idx, inserted_key);
        self.list_access.set(list);
    }

    pub fn remove(&self, idx: usize) -> T {
        let mut list = self.list_access.get();
        let removed_key = list.items_order.remove(idx);
        let obj = list.items_map.0.remove(removed_key).unwrap();
        self.list_access.set(list);
        obj
    }

    pub fn replace(&self, idx: usize, item: T) -> T {
        let mut list = self.list_access.get();
        let inserted_key = list.items_map.0.insert(item);
        list.items_order.insert(idx, inserted_key);
        let replaced_key = list.items_order.remove(idx + 1);
        let obj = list.items_map.0.remove(replaced_key).unwrap();
        self.list_access.set(list);
        obj
    }

    pub fn push(&self, item: T) {
        let mut list = self.list_access.get();
        let pushed_key = list.items_map.0.insert(item);
        list.items_order.push(pushed_key);
        self.list_access.set(list);
    }

    pub fn unselect_by_key(&self, key: ListKey) {
        let mut list = self.list_access.get();

        list.selected_keys.retain(|k| *k != key);

        self.list_access.set(list);
    }

    pub fn unselect_all(&self) {
        let mut list = self.list_access.get();
        list.selected_keys = vec![];
        self.list_access.set(list);
    }

    pub fn select_all(&self) {
        let mut list = self.list_access.get();
        for key in &list.items_order {
            list.selected_keys.push(*key)
        }

        self.list_access.set(list);
    }

    pub fn unselect(&self, idx: usize) {
        let mut list = self.list_access.get();

        list.selected_keys.remove(idx);

        self.list_access.set(list);
    }
    pub fn select(&self, idx: usize) {
        let mut list = self.list_access.get();

        let key = list.items_order[idx];
        list.selected_keys.push(key);

        self.list_access.set(list);
    }

    pub fn toggle_select(&self, idx: usize) {
        let mut list = self.list_access.get();

        let key = list.items_order[idx];
        if list.selected_keys.contains(&key) {
            list.selected_keys.remove(idx);
        } else {
            list.selected_keys.push(key);
        }

        self.list_access.set(list);
    }

    pub fn select_only(&self, idx: usize) {
        let mut list = self.list_access.get();

        let key = list.items_order[idx];
        list.selected_keys = vec![];
        list.selected_keys.push(key);

        self.list_access.set(list);
    }

    pub fn select_only_by_key(&self, key: ListKey) {
        let mut list = self.list_access.get();
        if !key.is_null() {
            list.selected_keys = vec![];
            list.selected_keys.push(key);
        }
        self.list_access.set(list);
    }

    pub fn select_by_key(&self, key: ListKey) {
        let mut list = self.list_access.get();

        if !key.is_null() {
            list.selected_keys.push(key);
        }

        self.list_access.set(list);
    }
}

#[derive(Clone, Default)]
pub struct ListKeyDenseSlotMap<T>(pub DenseSlotMap<ListKey, T>);

impl<T> ListKeyDenseSlotMap<T> {
    pub fn new() -> ListKeyDenseSlotMap<T> {
        ListKeyDenseSlotMap(DenseSlotMap::<ListKey, T>::with_key())
    }
}

#[derive(Clone, PartialEq)]
pub struct List<T>
where
    T: Clone + 'static,
{
    pub items_map: ListKeyDenseSlotMap<T>,
    pub items_order: Vec<ListKey>,
    pub selected_keys: Vec<ListKey>,
}

impl<T> PartialEq for ListKeyDenseSlotMap<T>
where
    T: Clone,
{
    fn eq(&self, other: &Self) -> bool {
        let mut self_keys = self.0.keys().collect::<Vec<ListKey>>();
        let mut other_keys = other.0.keys().collect::<Vec<ListKey>>();
        self_keys.sort();
        other_keys.sort();
        self_keys == other_keys
        // self.isbn == other.isbn
    }
}

impl<T> List<T>
where
    T: Clone + 'static,
{
    fn new(mut items: Vec<T>) -> List<T> {
        let mut sm = DenseSlotMap::default();
        for item in items.drain(..) {
            sm.insert(item);
        }
        let keys = sm.keys().collect::<Vec<_>>();
        List {
            items_map: ListKeyDenseSlotMap(sm),
            items_order: keys,
            selected_keys: vec![],
        }
    }

    // an iterator over all items in the list
    pub fn items(&self) -> impl Iterator<Item = &T> {
        self.items_order
            .iter()
            .filter_map(move |list_key| self.items_map.0.get(*list_key))
    }
    // an iterator over all selected items
    pub fn selected(&self) -> impl Iterator<Item = &T> {
        let items_map = &self.items_map.0;
        self.selected_keys
            .iter()
            .filter_map(move |key| items_map.get(*key))
    }
}
