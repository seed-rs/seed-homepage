# Lifecycle hooks

You can use lifecycle hooks, like those in React, to introduce side effects on DOM
elements when an element is rendered for the first time, upates, or de-renders. We do
this by passing one of the following structs to the element macro:

- [dom_types::DidMount](https://docs.rs/seed/0.1.11/seed/dom_types/struct.DidMount.html)
- [dom_types::DidUpdate](https://docs.rs/seed/0.1.11/seed/dom_types/struct.DidUpdate.html)
- [dom_types::WillUnmount](https://docs.rs/seed/0.1.11/seed/dom_types/struct.WillUnmount.html)

These are inspired by, and act similar to [functions of similar names](https://reactjs.org/docs/react-component.html#componentdidmount)
in React. Each of these structs is a thin-wrapper for a closure that takes the
[web_sys element](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html)
as its input, and has no output. We use them to perform side-effects (eg actions that don't change state), like setup and teardown 
operations on the DOM elements (eg focusing).

We create them using the following functions respectively, imported in the prelude:

- [did_mount](https://docs.rs/seed/0.1.11/seed/fn.did_mount.html)
- [did_update](https://docs.rs/seed/0.1.11/seed/fn.did_update.html)
- [will_unmount](https://docs.rs/seed/0.1.11/seed/fn.will_unmount.html)

Each of these functions takes a single argument: A closure containing the actions to perform,
and doesn't return anything.

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
],
```