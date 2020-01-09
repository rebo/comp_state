use crate::state_functions::use_state;

/// call the provided function once and once only
///
/// Example
///
/// do_once(||{
///     println!("This will print only once");
/// });
pub fn do_once<F: Fn() -> ()>(func: F) {
    topo::call(|| {
        let (has_done, has_done_access) = use_state(|| false);
        if !has_done {
            func();
            has_done_access.set(true);
        }
    });
}
