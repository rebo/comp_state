# comp_state: Component State for per compoment state

comp_state is a crate that allows you to store state on a per component basis.
It is designed as a clone of React hooks. 

Here a component is defined as a 'topological aware execution context', this 
means that the component is aware of its own call-site identity and location
in the call tree.

Each component has its own TopoId which is then used as a key to store component
state.

comp_state is generally used within the context of a host framework, for instance
a web frontend compiled to Wasm.

**Example:**

This is a complete counting button with state in the Seed framework:

```rust
fn hook_style_button() -> Node<Msg> {
    topo::call({
        // Declare a new state variable which we'll call "count"
        let (count, count_access) = use_state(|| 0);
        div![
            p![format!("You clicked {} times", count)],
            button![
                on_click( move |_| count_access.set(count + 1)),
                format!("Click Me × {}", count)
            ]
        ]
    })
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

The four most important functions/macros are:
 
* init_root_context() - This needs to be called prior to topo::root! in the 
  application initialisation. It sets up the storage in the root component context.
* use_state(|| .. ) stores component state for the type returned by the closure. 
  Returns a (state, accessor) tuple. 
* topo::root!({}) This macro defines the root of the topological components. 
  Usually called at the start of a render loop.
* topo::call!({}) This macro definies the extent of a component. Everything 
  inside the call! will have its own unique topological id.

**Caveats:**

- This is purely pre-alpha experimental!

- this 100% uses topo code from Moxie (https://github.com/anp/moxie), cut and 
pasted into this example because the latest version of topo was not published 
as a separate crate. 

-  The core technology (topo) is 100% created by and the idea of the Moxie team, 
I have nothing to do with them and just wanted to get topo working with seed.

**How does it work?**

- topo creates a new execution context for every `topo::call!` block. Further calls 
to `topo::root!` re-roots the execution context. The re-rooting allows for consistent 
execution contexts for the same components as long as you re-root at the start of the 
base view function. This means that one can store and retrieve local data for an 
individual component which has been enclosed with  `topo::call!`.

- The execution context is not only determined by the order of calling `topo::call!` 
functions but also the source location of these calls. This means that state is 
consistent  and stable even though branching logic might call topologically 
aware functions in different orders.

- See this awesome talk explaining how topo works: https://www.youtube.com/watch?v=tmM756XZt20

- a type gets stored with : `let (my_string, string_access) = use_state::<String>(||text)` 
which stores `text` in the component for the `String` type. This returns a tuple,
 with `my_string` being a clone of the latest state and string_access being an accessor 
 which can get or set this value. 

- The accessor is useful because it can be passed to callbacks or cloned or called from 
different topological contexts. i.e. `string_acces.set(new_text)` will work no matter 
where it is called.

- currently comp_state only exposes a clone to stored values. 

- currently only 1 type per context is storable, however if you want to store more than 1 
String say, you can create a `HashMap<key,String>` , `Vec`, or NewType and store that.

- After some testing this now seems fairly stable-ish. This is experimental please 
don't rely on it for anything important.

**Why would anyone want to do this?**

- I wanted to see what all the fuss is about with React Hooks and whether it could 
be implemented in Rust.

