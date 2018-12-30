# Misc features

## Logging in the web browser
To output to the web browser's console (ie `console.log()` in JS), use `web_sys::console_log1`,
or the `log` macro that wraps it, which is imported in the seed prelude: 
`log!("On the shoulders of", 5, "giants".to_string())`

## Querying servers using fetch
To send and receive data with a server, use `wasm-bindgen`'s `web-sys` fetch methods,
[described here](https://rustwasm.github.io/wasm-bindgen/examples/fetch.html).

Use the [Serde](https://serde.rs/) crate to serialize and deserialize data, eg
when sending and receiving data from a REST-etc. It supports most popular formats,
including `JSON`, `YAML`, and `XML`.

(Example, and with our integration)

Check out the `server_interaction` examples for an example of how to send and receive
data from the server in JSON.

Seed will implement a high-level fetch API in the future, wrapping web-sys's.

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
