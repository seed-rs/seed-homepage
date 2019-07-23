# Lifecycle hooks

You can use lifecycle hooks, like those in React, to introduce side effects on DOM
elements when an element is rendered, upates, or de-renders. We do
this by passing one of the following structs to the element macro:

- [DidMount](https://docs.rs/seed/0.1.11/seed/dom_types/struct.DidMount.html)
- [DidUpdate](https://docs.rs/seed/0.1.11/seed/dom_types/struct.DidUpdate.html)
- [WillUnmount](https://docs.rs/seed/0.1.11/seed/dom_types/struct.WillUnmount.html)

These are inspired by, and act similar to [functions of similar names](https://reactjs.org/docs/react-component.html#componentdidmount)
in React. Each of these is a thin-wrapper for a closure that takes a ref to the associated
[web_sys element](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html)
as its only parameter, and doesn't return anything. We use them to perform side-effects (eg actions that don't change state), like setup and teardown 
operations on DOM elements.

We create them using the following functions respectively, imported in the prelude:

- [did_mount](https://docs.rs/seed/0.1.11/seed/dom_types/fn.did_mount.html)
- [did_update](https://docs.rs/seed/0.1.11/seed/dom_types/fn.did_update.html)
- [will_unmount](https://docs.rs/seed/0.1.11/seed/dom_types/fn.will_unmount.html)

Each of these takes a single parameter: the closure described above.

Example:
```rust
h3![ num_clicks, did_update(|_| log!("This shows when we increment")) ],

// ...

if model.count >= 10 {
    h2![ "Nice!",
         did_mount(|_| log!("This shows when clicks reach 10")),
         will_unmount(|_| log!("This shows when clicks drop below 10")),
    ]
} else { seed::empty() }
```

An example of updating the associated DOM element:
```rust
button![
    "Autofocuses on load",
    did_mount(|el| {
        let html_el = seed::to_html_el(&el);
        html_el.focus().unwrap();
    })
]
```

You can define the closure separately, either inside the view/component func:
```rust
// You may have to specify type in the closure, as below.
let autofocus = |el: &web_sys::Element| {
    let html_el = seed::to_html_el(&el);
    html_el.focus().unwrap();
};

button![
    "Autofocuses on load",
    autofocus
]
```
or as a separate function:
```rust
fn autofocus(el: &web_sys::Element) {
    let html_el = seed::to_html_el(&el);
    html_el.focus().unwrap();
}

fn component() -> Node<Msg> {
    button![
        "Autofocuses on load",
        autofocus
    ]
}
```

