use crate::state_access::StateAccess;
use crate::state_functions::use_state;
use enclose::enclose as e;

#[derive(Clone)]
pub struct MemoControl(StateAccess<bool>);

impl MemoControl {
    pub fn recalc(&self, trigger: bool) {
        self.0.set(trigger);
    }
}

/// creates a watch object its is own topological context
/// passed the current state of type T
/// if the state is different return a watch object with true the 'changed' field
/// if state is not different just return the existing watch object
///
/// This is then used as the recalc argument for use_memo.Clone
///
///
/// Example:
/// ```
/// let watch_changed = watch(list);
/// ```

pub fn watch<T: 'static + Clone + PartialEq>(current_watched: &T) -> bool {
    topo::call(|| {
        let (watched, watch_access) = use_state(e!((current_watched) || current_watched));
        if &watched != current_watched {
            watch_access.set(current_watched.clone());
            true
        } else {
            false
        }
    })
}

///
///  use_memo hook - accepts a bool that triggers re-evaluation of the given function.
///  if the bool is true the given function is re-called.
///   
///  This function also returns a MemoControl struct that has its own recalc method
///
///  use_memo can be used in conjunction with watch to trigger re-evaluations on change
///  of watched variables.
///
/// Example:
/// ```
/// let rendered_list =  use_memo(
///     watch(list),
///     || expensive_render(list)
/// )
/// ```
#[topo::nested]
pub fn use_memo<T: 'static + Clone, F: Fn() -> T>(recalc: bool, func: F) -> (T, MemoControl) {
    let (update, recalc_trigger_access) = use_state(|| false);

    let new_func = || func();

    // by definition this will keep returning 'value'
    // until update is set to true.

    let (mut value, value_access) = use_state(new_func);

    if update || recalc {
        value = func();
        value_access.set(value.clone());
        recalc_trigger_access.set(false);
    }
    (value, MemoControl(recalc_trigger_access))
}
