# Events
Events are created by passing a a [Listener](https://docs.rs/seed/0.1.6/seed/dom_types/struct.Listener.html),
or vec of Listeners, created using the following four functions exposed in the prelude: `simple_ev`,
`input_ev`, `keyboard_ev`, and `raw_ev`. The first is demonstrated in the example in the quickstart section,
and all are demonstrated in the todomvc example.

`simple_ev` takes two arguments: an event trigger (eg "click", "contextmenu" etc), and an instance
of your `Msg` enum. (eg Msg::Increment). The other three event-creation-funcs
take a trigger, and a [closure](https://doc.rust-lang.org/book/ch13-01-closures.html) (An anonymous function,
similar to an arrow func in JS) that returns a Msg enum.

`simple_ev` does not pass any information about the event, only that it fired.
Example: 
```rust
enum Msg {
    ClickClick
}
// ...
simple_ev("dblclick", Msg::ClickClick)`
```

`input_ev` passes the event target's value field, eg what a user typed in an input field.
Example: 
```rust
enum Msg {
    NewWords(String)
}
// ...
input_ev("input", Msg::NewWords)
```

`keyboard_ev` returns a [web_sys::KeyboardEvent](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.KeyboardEvent.html),
which exposes several getter methods like `key_code` and `key`.
Example:
```rust
enum Msg {
    PutTheHammerDown(web_sys::KeyboardEvent)
}
// ...
keyboard_ev("input", Msg::PutTheHammerDown)
```

Note that in the examples for input_ev and keyboard_ev, the syntax is simplified since
we're only passing the field text, and keyboard event respectively to the Msg. The input_ev
example is Rust shorthand for ```input_ev("input, |text| Msg::NewWords(text)```. If you were
to pass something other than, or more than just the input text (Or KeyboardEvent for keyboard_ev, 
or Event for raw_ev described below),
you can't use this shorthand, and would have to do something like this intead,
explicitly writing the closure:
```rust
enum Msg {
    NewWords(String, u32)
}
// ...
input_ev("input", move |text| Msg::NewWords(text, 0))
```

`raw_ev` returns a [web_sys::Event](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Event.html). 
It lets you access any part of any type of
event, albeit with more verbose syntax.
If you wish to do something like prevent_default(), or anything not listed above, 
you need to take this approach. Note that for many common operations, like taking
the value of an input element after an `input` or `change` event, you have to deal with
casting from a generic event or target to the specific one. Seed provides convenience
functions to handle this. They wrap wasm-bindgen's .dyn_ref() and .dyn_into(), from its
[JsCast](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html) trait.

Example syntax showing how you might use raw_ev; processing an input and handling a keyboard
event, while using prevent_default:
```rust
// (in update func)
Msg::KeyPress(event) => {
    event.prevent_default();
    let code = seed::to_kbevent(&ev).key_code();
    // ..
    let target = event.target().unwrap();
    let text = seed::to_input(&target).value();
    
    // ...
    // In view
    raw_ev("input", Msg::KeyPress),
}
```
Seed also provides `to_textarea` and `to_select` functions, which you'd use as
`to_input`.

This extra step is caused by a conflict between Rust's type system, and the way DOM events
are handled. For example, you may wish to pull text from an input field by reading the event target's
value field. However, not all targets contain value; it may have to be represented as
an `HtmlInputElement`. (See [the web-sys ref](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.EventTarget.html), 
and [Mdn ref](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget); there's no value field)) Another example:
If we wish to read the key_code of an event, we must first cast it as a KeyboardEvent; pure Events
(web_sys and DOM) do not contain this field.

It's likely you'll be able to do most of what you wish with the simpler event funcs.
If there's a type of event or use you think would benefit from a similar func, submit
an issue or PR. In the descriptions above for all event-creation funcs, we assumed minimal code in the closure,
and more code in the update func's match arms. For example, to process a keyboard event,
these two approaches are equivalent:

```rust
enum Msg {
    KeyDown(web_sys::KeyboardEvent)
}

// ... (in update)
KeyDown(event) => {
    let code = event.key_code()
    // ...
}

// ... In view
keyboard_ev("keydown", Msg::KeyDown)
```
and
```rust
enum Msg {
    KeyDown(u32)
}

// ... (in update)
KeyDown(code) => {
    // ...
}

// ... In view
keyboard_ev("keydown", |ev| KeyDown(ev.key_code()))
```

You can pass more than one variable to the `Msg` enum via the closure, as long
as it's set up appropriate in `Msg`'s definition. Note that if you pass a value to the enum
other than what's between ||, you may receive an error about lifetimes. This is corrected by
making the closure a move type. Eg:
```rust
keyboard_ev("keydown", move |ev| Msg::EditKeyDown(id, ev.key_code()))
```
Where `id` is a value defined earlier.

Event syntax may be improved later with the addition of a single macro that infers what the type of event 
is based on the trigger, and avoids the use of manually creating a `Vec` to store the
`Listener`s. For examples of all of the above (except raw_ev), check out the [todomvc example](https://github.com/David-OConnor/seed/tree/master/examples/todomvc).

The [todomvc example](https://github.com/David-OConnor/seed/tree/master/examples/todomvc) has a number of event-handling examples, including use of raw_ev, 
where it handles text input triggered by a key press, and uses prevent_default().