/// Exports the following:
///
/// use_state associates a type T's state with a current topoolical context
///     returning a copy of that state as well as an accsssor method to update
///     that state
///
/// topo - re-export of topo crate. Needed to ensure a single version of topo
/// is used throughout so that user topo::Ids match comp_state topo::Ids.
///
/// do_once - a function to do a block once and once only
///
/// set_state - set the state of type T in the current topological context
///
/// clone_state - clone the state of a type T in the current topological context
///
/// get_state_with_topo_id - clone the state of type T in the given topological context
///
/// set_state_with_topo_id - set the state of type T in the given topological context
///
/// update_state_with_topo_id - update the state of type T in the given topological
///     context
///
/// purge_and_reset_unseed_ids - rudamentary gabrage collection, purgets any
///     topological context state that has not been accessed since the last time
///     this function was run
///
///  StateAccess - the access struct that enables a state to be udated or retrieved
pub use topo;

// Re exports
pub use crate::helpers::do_once;
pub use crate::state_access::{ChangedState, CloneState, StateAccess};
pub use crate::state_functions::{
    clone_state_with_topo_id, execute_and_remove_unmounts, new_state, on_unmount,
    purge_and_reset_unseen_ids, reset_unseen_id_list, set_state_with_topo_id,
    state_exists_for_topo_id, unseen_ids, update_state_with_topo_id, use_state, use_state_current,
};
pub use crate::unmount::{StateAccessUnmount, Unmount};
