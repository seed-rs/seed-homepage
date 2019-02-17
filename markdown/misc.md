# Misc features

## Logging in the web browser
To output to the web browser's console (ie `console.log()` in JS), use `web_sys::console_log1`,
or the `log` macro that wraps it, which is imported in the seed prelude: 
`log!("On the shoulders of", 5, "giants".to_string())`

## Custom tags
Seed generally retricts the element tags allowed by using Enums for the tags, and
a predefined set of element-creation macros. If you wish to use a custom tag, you can
use using `Tag::Custom` (`El` and `Tag` are
exposed in the prelude), either with the `El::empty` constructor, or using the `custom!`
element-construction macro, where we pass our custom tag as an argument:
```rust
let mut custom_el = El::empty(Tag::Custom("mytag".to_string()));
custom_el.set_text("Words");

custom![ Tag::Custom("anothertag".into())
    custom_el,
]
```
An example is provided as part of the [window_events](https://github.com/David-OConnor/seed/tree/master/examples/todomvc)
example.

## Local storage
You can store page state locally using web_sys's [Storage struct](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Storage.html)

Seed provides convenience functions `seed::storage::get_storage`, which returns 
the `web_sys::storage` object, and `seed::storage::store_data` to store an arbitrary
Rust data structure that implements serde's Serialize. Example use:

```rust
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

// ...
#[derive(Serialize, Deserialize)]
struct Data {
    // Arbitrary data (All sub-structs etc must also implement Serialize and Deserialize)
}

let storage = seed::storage::get_storage();
seed::storage::store(storage, "my-data", Data::new());

// ...

let loaded_serialized = storage.get_item("my-data").unwrap().unwrap();
let data = serde_json::from_str(&loaded_serialized).unwrap();
```

## Display markdown and raw HTML
Seed supports creating elements from markdown text, using [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)
internally. Use the [El::from_markdown()](https://docs.rs/seed/0.1.6/seed/dom_types/struct.El.html#method.from_markdown)
method to create an element that accepts a markdown &str as its only parameter, and displays
it normally as html. Note that it does not support syntax highlighting. You can render raw HTML with `El::from_html(html)`, where `html` is a 
&str of HTML.

Example:
```rust
fn view(app: seed::App<Msg, Model>, model: Model) -> El<Msg> {

    let markdown = 
"
## Hello world

Let's set the existence-of-God issue aside for a later volume,
and just [learn to code](https://play.rust-lang.org/).
"
;

    let html = 
"
<div>
    <p>It is a truth universally acknowledged, that a single man in 
    possession of a good fortune, must be in want of a good time.</p>
</div>
"
;
    
    div![
        El::from_markdown(markdown) 
        El::from_html(html) 
    ]
}

```

## Some convenience functions
You can use `seed::document` and `seed::window` to access the `web_sys` document
and window functions. Example:
```rust
fn view(state: seed::App<Msg, Model>, model: Model) -> El<Msg> {
    button![ 
        simple_ev("click", Msg::Increment), 
        format!("Hello, World × {}", model.val),
        did_mount(|_| {
            seed::document().set_title("New title")
        })
    ]
}
```

## Input elements are controlled
`input`, `textarea`, and `select` elements are always controlled, in the vein of React.
This means that even if there's no event associated with user input to these fields, their
value will always stay in sync with the model, which may mean ignoring text input if
not set up with a `Ev::Input` event.


## Debugging elements
`El`s implement the `Debug` trait, so you can view outputs using `log!`: `log!(format!("{:?}", my_el));`
In order to take advantage of this, you must implement `Debug` for your message type, and 
any sub-types. Example:

```rust
#[derive(Copy, Clone, Debug)]
enum Page {
    Guide,
    Changelog
}

#[derive(Clone, Debug)]
enum Msg {
    RoutePage(Page),
    ChangePage(Page),
}
```
