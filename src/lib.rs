#![feature(track_caller)]
pub mod list;
pub mod mailbox;
pub mod memo;
pub mod prelude;

mod helpers;
mod state_access;
mod state_functions;
mod store;
pub use prelude::*;
// Re export topo so that there will not be any conflicting
// topo versions used.

//experimental

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 3, 4);
    }
}
