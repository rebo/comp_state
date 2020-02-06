use crate::state_functions::*;
use std::marker::PhantomData;

///  Accessor struct that provides access to getting and setting the
///  state of the stored type
///
#[derive(Debug)]
pub struct StateAccess<T> {
    pub id: topo::Id,
    _phantom_data: PhantomData<T>,
}

impl<T> std::fmt::Display for StateAccess<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?})", self.id)
    }
}

impl<T> Copy for StateAccess<T> {}
impl<T> Clone for StateAccess<T> {
    fn clone(&self) -> StateAccess<T> {
        StateAccess::<T> {
            id: self.id,
            _phantom_data: PhantomData::<T>,
        }
    }
}

impl<T> StateAccess<T>
where
    T: 'static,
{
    pub fn new(id: topo::Id) -> StateAccess<T> {
        StateAccess {
            id,
            _phantom_data: PhantomData,
        }
    }

    // stores a value of type T in a backing Store
    pub fn set(self, value: T) {
        set_state_with_topo_id(value, self.id);
    }

    pub fn remove(self) -> Option<T> {
        remove_state_with_topo_id(self.id)
    }

    /// updates the stored state in place
    /// using the provided function
    pub fn update<F: FnOnce(&mut T) -> ()>(self, func: F) {
        update_state_with_topo_id(self.id, func);
    }

    pub fn state_exists(self) -> bool {
        state_exists_for_topo_id::<T>(self.id)
    }

    pub fn get_with<F: FnOnce(&T) -> R, R>(self, func: F) -> R {
        read_state_with_topo_id(self.id, func)
    }
}

pub trait CloneState<T>
where
    T: Clone + 'static,
{
    fn get(&self) -> T;

    fn soft_get(&self) -> Option<T>;
}

impl<T> CloneState<T> for StateAccess<T>
where
    T: Clone + 'static,
{
    /// returns a clone of the stored state panics if not stored.
    fn get(&self) -> T {
        clone_state_with_topo_id::<T>(self.id).expect("state should be present")
    }

    fn soft_get(&self) -> Option<T> {
        clone_state_with_topo_id::<T>(self.id)
    }
}
