mod context;
mod helpers;
mod list;
mod memo;
mod state_access;
mod state_functions;
mod store;

//experimental
pub mod mailbox;

// Re exports
pub use context::use_parent_memo;
pub use context::{get_context, set_context};
pub use helpers::do_once;
pub use list::{use_list, List, ListControl};
pub use memo::{use_memo, watch};
pub use state_access::StateAccess;
pub use state_functions::{
    clone_state, get_state_with_topo_id, init_root_context, purge_and_reset_unseen_ids, set_state,
    set_state_with_topo_id, update_state_with_topo_id, use_state,
};
pub use store::Store;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 3, 4);
    }
}
