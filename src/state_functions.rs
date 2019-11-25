use crate::state_access::StateAccess;
use crate::Store;
use std::cell::RefCell;
use std::collections::HashSet;

/// Important function that sets up the Storage required for uses_state to work.
///
/// All state changing and accessing calls will panic unless this is setup.
pub fn init_root_context() {
    if topo::Env::get::<RefCell<Store>>().is_none() {
        topo::Env::add(RefCell::new(Store::default()));
    }
}

///
/// Constructs a T and T accessor pair. T is stored keyed to the current topological context.
/// The accessor always references this context therefore can you can set/update/ or get this T
/// from anywhere.
///
///  If T has already been stored on subsequent revists, T will be a clone of the latest stored T.
///
///  Only one type per context can be stored in this way.
///
/// # Examples
///
/// ```
/// let (my_string, my_string_access) =  use_state(|| "foo".to_string());
/// ...
/// ...
///  // Maybe in a Callback...
/// my_string_access.set("bar")
/// ```
///
/// This stores a string "foo" in the current topological context,
/// which is later set to "bar", in some other part of the program.
///
pub fn use_state<T: 'static + Clone, F: FnOnce() -> T>(data_fn: F) -> (T, StateAccess<T>) {
    let current_id = topo::Id::current();

    // returns a clone of the curent stored type. If the type has not been stored before
    // set it with the closure passed to use_state.
    if let Some(stored_data) = clone_state::<T>() {
        (stored_data, StateAccess::new(current_id))
    } else {
        let data = data_fn();
        set_state_with_topo_id::<T>(data.clone(), current_id);
        (data, StateAccess::new(current_id))
    }
}

/// sets the state of type T keyed to the local context.
pub fn set_state<T: 'static + Clone>(data: T) {
    let current_id = topo::Id::current();
    assert!(topo::Env::get::<RefCell<Store>>().is_some());

    let store = topo::Env::get::<RefCell<Store>>();
    store
        .expect("'Store' not present, did you forget to call init_root_context() first?")
        .borrow_mut()
        .set_state_with_topo_id::<T>(data, current_id);
}

/// Sets the state of type T keyed to the given TopoId
pub fn set_state_with_topo_id<T: 'static + Clone>(data: T, current_id: topo::Id) {
    let store = topo::Env::get::<RefCell<Store>>();
    store
        .expect("'Store' not present, did you forget to call init_root_context() first?")
        .borrow_mut()
        .set_state_with_topo_id::<T>(data, current_id);
}

/// Clones the state of type T keyed to the given TopoId
pub fn get_state_with_topo_id<T: 'static + Clone>(id: topo::Id) -> Option<T> {
    let store = topo::Env::get::<RefCell<Store>>();
    store
        .expect("'Store' not present, did you forget to call init_root_context() first?")
        .borrow_mut()
        .get_state_with_topo_id::<T>(id)
        .cloned()
}

/// Provides mutable access to the stored state type T.
///
/// Example:
///
/// ```
/// update_state_with_topo_id::<Vec<String>>( topo::Id::current(), |v|
///     v.push("foo".to_string()
/// )
///
pub fn update_state_with_topo_id<T: Clone + 'static, F: FnOnce(&mut T) -> ()>(
    id: topo::Id,
    func: F,
) {
    let item = &mut get_state_with_topo_id::<T>(id)
        .expect("You are trying to update a type state that doesnt exist in this context!");
    func(item);
    set_state_with_topo_id(item.clone(), id);
}

/// Clones the state of a type keyed to the current topological context
pub fn clone_state<T: 'static + Clone>() -> Option<T> {
    let store = topo::Env::get::<RefCell<Store>>();
    store
        .expect("'Store' not present, did you forget to call init_root_context() first?")
        .borrow_mut()
        .get_state::<T>()
        .cloned()
}

/// Rudamentary Garbage Collection
/// purges all unseen ids' state
/// then resets the suneen ids list.
pub fn purge_and_reset_unseen_ids() {
    purge_unseen_ids();
    reset_unseen_id_list();
}

/// Rudamentary Garbage Collection
///
/// Copies all ids in the storage map to an unseen_id list.
/// Each Id is then removed if accessed
///
/// Paired with purge_unseen_ids to remove state for ids that have not been accessed
pub fn reset_unseen_id_list() {
    let store = topo::Env::get::<RefCell<Store>>();
    let store =
        store.expect("'Store' not present, did you forget to call init_root_context() first?");
    let mut store_mut = store.borrow_mut();

    store_mut.unseen_ids = HashSet::new();
    let ids = store_mut.id_to_key_map.keys().cloned().collect::<Vec<_>>();
    for id in ids {
        store_mut.unseen_ids.insert(id);
    }
}

/// Rudamentary Garbage Collection
///
/// Purges all state keyed to ids remaining in unseed ids list
fn purge_unseen_ids() {
    let store = topo::Env::get::<RefCell<Store>>();
    let store =
        store.expect("'Store' not present, did you forget to call init_root_context() first?");
    let mut store_mut = store.borrow_mut();

    let ids = store_mut.unseen_ids.iter().cloned().collect::<Vec<_>>();

    for id in ids {
        let key = store_mut.id_to_key_map.remove(&id);
        if let Some(key) = key {
            store_mut.primary_slotmap.remove(key);
        }
    }
}

// pub fn state_getter<T: 'static + Clone>() -> Arc<dyn Fn() -> Option<T>> {
//     let current_id = topo::Id::current();
//     Arc::new(move || get_state_with_topo_id::<T>(current_id))
// }