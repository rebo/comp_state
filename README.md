# comp_state: store state on components

comp_state is a crate that allows you to store state on a per component basis.
It is designed as a clone of React Hooks, principally the useState hook.

Here a component is defined as a 'topological aware execution context', this 
means that the component is aware of its own call-site identity and location
in the call tree.

comp_state is generally used within the context of a host framework, for instance
a web frontend compiled to Wasm.

**Example:**

This is a complete counting button with state implemented in in the Seed framework:

```rust
use comp_state::{topo, use_state};

#[topo::nested]
fn my_button() -> Node<Msg> {
    let count = use_state(|| 3);

    div![
        count,
        button![count.mouse_ev(Ev::Click, |count, _| *count += 1), "Click me"],
    ]
}
```

vs ReactJs:

```javascript
import React, { useState } from 'react';
function Example() {
  // Declare a new state variable, which we'll call "count"
  const [count, setCount] = useState(0);

  return (
    <div>
      <p>You clicked {count} times</p>
      <button onClick={() => setCount(count + 1)}>
        Click me
      </button>
    </div>
  );
}
```

The two most important functions are:
 
* use_state(|| .. ) stores component state for the type returned by the closure. 
  Returns a state accessor. 
* `#[topo::nested]` function annotation definies the a topologically aware function. Everything 
  executed within the function will have its own unique topological id. The outermost nested function
  acts as a "root" which resets the topology and enables specific components to have
  a "topological identity".

**Caveats:**

This is purely alpha experimental!

Each component has its own "topo::Id" which is then used as a key to store component
state. topo is a crate from the Moxie team who are creating a GUI framework for rust.
There is an interesting talk about moxie and how topo works [here](https://www.youtube.com/watch?v=tmM756XZt20).

**How does it work?**

- this relies on the `#![feature(track_caller)]` feature gate to be activated.

- topo creates a new execution context for every `#[topo::nested]` function or every `topo::call` block. The outermost call
re-roots the execution context. The re-rooting allows for consistent 
execution contexts for the same components as long as you re-root at the start of the 
base view function. This means that one can store and retrieve local data for an 
individual component annotated by `#[topo::nested]`.

- The execution context is not only determined by the order of calling a  
functions but also the source location of these calls. This means that state is 
consistent and stable even though branching logic might call topologically 
aware functions in different orders.

- See this awesome talk explaining how topo works: https://www.youtube.com/watch?v=tmM756XZt20

- a type gets stored with : `let string  = use_state::<String>(||text)` 
which stores `text` in the component for the `String` type. This returns a 
 state accessor struct respomsible for getting and setting of the state.

- The accessor is useful because it can be passed to callbacks or cloned or called from 
different topological contexts. i.e. `string_acces.set(new_text)` will work no matter 
where it is called.

- currently comp_state exposes a clone to stored values via `get()` and to non-Clone types with `get_with()`

- After some testing this now seems fairly stable-ish. This is experimental please 
don't rely on it for anything important.

**Why would anyone want to do this?**

- I wanted to see what all the fuss is about with React Hooks and whether it could 
be implemented in Rust.

