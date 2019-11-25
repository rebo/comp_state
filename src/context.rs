#![allow(clippy::map_clone)]

use crate::use_state;
use std::any::Any;
use std::ops::Deref;

// Simply a wrap of topo::env::add
// stores a value of type T in the current context
// accessible in all child components
pub fn set_context<T: 'static>(context: T) {
    topo::Env::add(context);
}

// Simply a wrap of topo::env::add
// retrieves a reference to a value of type T in the current (and parent) contexts
pub fn get_context<E>() -> Option<impl Deref<Target = E> + 'static>
where
    E: Any + 'static,
{
    topo::Env::get::<E>()
}

#[derive(Clone)]
pub struct TopoIdMemo(pub topo::Id);

// retreives the parents id (as long as it has been memoized) and sets the current parent.
// This is useful for child parent communication.
pub fn use_parent_memo() -> Option<TopoIdMemo> {
    // if there is already a parent_id do nothing
    // as it has already been stored.context
    let (parent_id, _parent_id_accesor) =
        use_state(|| topo::Env::get::<TopoIdMemo>().map(|d| d.clone()));
    topo::Env::add(TopoIdMemo(topo::Id::current()));
    parent_id
}
