use crate::state_access::{CloneState, StateAccess};
use crate::state_functions::use_state;

/// call the provided function once and once only
/// returns a droptype which will allow the do_once
/// to repeat if .execute_if_activated() is called on the droptype.
/// Example
///
/// do_once(||{
///     println!("This will print only once");
/// });
#[topo::nested]
pub fn do_once<F: Fn() -> ()>(func: F) -> StateAccess<bool> {
    let has_done = use_state(|| false);
    if !has_done.get() {
        func();
        has_done.set(true);
    }
    has_done
}
