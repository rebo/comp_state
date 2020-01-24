use crate::state_functions::*;
use std::marker::PhantomData;

///  Accessor struct that provides access to getting and setting the
///  state of the stored type
#[derive(Clone, Copy, Debug)]
pub struct StateAccess<T> {
    pub id: topo::Id,
    _phantom_data: PhantomData<T>,
}

impl<T> StateAccess<T>
where
    T: 'static + Clone,
{
    pub fn new(id: topo::Id) -> StateAccess<T> {
        StateAccess {
            id,
            _phantom_data: PhantomData,
        }
    }

    // stores a value of type T in a backing Store
    pub fn set(&self, value: T) {
        set_state_with_topo_id(value, self.id);
    }

    /// updates the stored state in place
    /// using the provided function
    pub fn update<F: FnOnce(&mut T) -> ()>(&self, func: F) {
        let item = &mut self.get().unwrap();
        func(item);
        self.set(item.clone());
    }

    /// returns a option clone of the stored state.
    pub fn get(&self) -> Option<T> {
        get_state_with_topo_id::<T>(self.id)
    }

    /// returns a clone of the stored state panics if not stored.
    pub fn hard_get(&self) -> T {
        get_state_with_topo_id::<T>(self.id).unwrap()
    }
}
