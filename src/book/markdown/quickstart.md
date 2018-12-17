# Seed

**A Rust framework for creating web apps**

[![](https://meritbadge.herokuapp.com/seed)](https://crates.io/crates/seed)
[![](https://img.shields.io/crates/d/seed.svg)](https://crates.io/crates/seed)
[![API Documentation on docs.rs](https://docs.rs/seed/badge.svg)](https://docs.rs/seed)


## Quickstart

### Setup
This framework requires you to install [Rust](https://www.rust-lang.org/tools/install) - This will
enable the CLI commands below:

 You'll need a recent version of Rust: `rustup update`

The wasm32-unknown-unknown target: `rustup target add wasm32-unknown-unknown`

And wasm-bindgen: `cargo install wasm-bindgen-cli`


### The theoretical minimum
To start, clone [This quickstart repo](https://github.com/David-OConnor/seed-quickstart),
run `build.sh` or `build.ps1` in a terminal, then start a dev server that supports WASM.
For example, with [Python](https://www.python.org/downloads/) installed, run `python serve.py`.
(Linux users may need to run `python3 serve.py`.)
Once you change your package name, you'll
need to tweak the html file and build script, as described below.

### A little deeper
Or, create a new lib with Cargo: `cargo new --lib appname`. Here and everywhere it appears in this guide, `
appname` should be replaced with the name of your app.

If not using the quickstart repo, create an Html file that loads your app's compiled module, 
and provides an element with id 
to load the framework into. It also needs the following code to load your WASM module -
 Ie, the body should contain this:
 
 ```html
 <section id="main"></section>

<script src='./pkg/appname.js'></script>

<script>
    const { render } = wasm_bindgen;
    function run() {
        render();
    }
    wasm_bindgen('./pkg/appname_bg.wasm')
        .then(run)
        .catch(console.error);
</script>
```

The quickstart repo includes this file, but you will need to rename the two 
occurances of `appname`. (If your project name has a hyphen, use an underscore instead here) You will eventually need to modify this file to 
change the page's title, add a description, favicon, stylesheet etc.

`Cargo.toml`, which is a file created by Cargo that describes your app, needs `wasm-bindgen`, `web-sys`, and `
seed` added as depdendencies,
 and crate-type
of `"cdylib"`. (The version in the quickstart repo has these set up already) Example:

```toml
[package]
name = "appname"
version = "0.1.0"
authors = ["Your Name <email@address.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
seed = "^0.1.6"
wasm-bindgen = "^0.2.29"
web-sys = "^0.3.6"

# For serialization, eg sending requests to a server. Otherwise, not required.
serde = "^1.0.80"
serde_derive = "^1.0.80"
serde_json = "1.0.33"

```

### A short example
Here's an example demonstrating structure and syntax; it can be found in working form
under `examples/counter`. Descriptions of its parts are in the
Guide section below. Its structure follows [The Elm Architecture](https://guide.elm-lang.org/architecture/).

*lib.rs*:
```rust

#[macro_use]
extern crate seed;
use seed::prelude::*;
use wasm_bindgen::prelude::*;


// Model

#[derive(Clone)]
struct Model {
    count: i32,
    what_we_count: String
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            count: 0,
            what_we_count: "click".into()
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
    ChangeWWC(String),
}

/// The sole source of updating the model; returns a fresh one.
fn update(history: &mut History<Model, Msg>, msg: Msg, model: Model) -> Model {
    match msg {
        Msg::Increment => Model {count: model.count + 1, ..model},
        Msg::Decrement => Model {count: model.count - 1, ..model},
        Msg::ChangeWWC(what_we_count) => Model {what_we_count, ..model }
    }
}


// View

/// A simple component.
fn success_level(clicks: i32) -> El<Msg> {
    let descrip = match clicks {
        0 ... 3 => "Not very many 🙁",
        4 ... 7 => "An OK amount 😐",
        8 ... 999 => "Good job! 🙂",
        _ => "You broke it 🙃"
    };
    p![ descrip ]
}

/// The top-level component we pass to the virtual dom. Must accept a ref to the model as its
/// only argument, and output a single El.
fn view(model: Model) -> El<Msg> {
    let plural = if model.count == 1 {""} else {"s"};

    // Attrs, Style, Events, and children may be defined separately.
    let outer_style = style!{
            "display" => "flex";
            "flex-direction" => "column";
            "text-align" => "center"
    };

     div![ outer_style,
        h1![ "The Grand Total" ],
        div![
            style!{
                // Example of conditional logic in a style.
                "color" => if model.count > 4 {"purple"} else {"gray"};
                // When passing numerical values to style!, "px" is implied.
                "border" => "2px solid #004422"; "padding" => 20
            },
            // We can use normal Rust code and comments in the view.
            h3![ format!("{} {}{} so far", model.count, model.what_we_count, plural) ],
            button![ simple_ev("click", Msg::Increment), "+" ],
            button![ simple_ev("click", Msg::Decrement), "-" ],

            // Optionally-displaying an element
            if model.count >= 10 { h2![ style!{"padding" => 50}, "Nice!" ] } else { seed::empty() }

            ],
        success_level(model.count),  // Incorporating a separate component

        h3![ "What precisely is it we're counting?" ],
        input![ attrs!{"value" => model.what_we_count}, input_ev("input", Msg::ChangeWWC) ]
    ]
}


#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, view, "main", None);
}
```

### Building and running
To build your app, create a `pkg` subdirectory, and run the following two commands:

```
cargo build --target wasm32-unknown-unknown
```
and 
```
wasm-bindgen target/wasm32-unknown-unknown/debug/appname.wasm --no modules --out-dir ./pkg
```
where `appname` is replaced with your app's name. This compiles your code in the target
folder, and populates the pkg folder with your WASM module, a Typescript definitions file,
and a Javascript file used to link your module from HTML.

You may wish to create a build script with these two lines. (`build.sh` for Linux; `build.ps1` for Windows).
The Quickstart repo includes these, but you'll still need to do the rename. You can then use
`./build.sh` or `.\build.ps1`

For development, you can view your app using a shimmed Python dev server described above.
(Set up [this mime-type shim](https://github.com/David-OConnor/seed-quickstart/blob/master/serve.py)
from the quickstart repo, and run `python serve.py`).

For details, reference [the wasm-bindgen documention](https://rustwasm.github.io/wasm-bindgen/whirlwind-tour/basic-usage.html).
In the future, I'd like the build script and commands above to be replaced by [wasm-pack](https://github.com/rustwasm/wasm-pack).

### Running included examples
To run an example located in the `examples` folder, navigate to that folder in a terminal, 
run the build script for your system (`build.sh` or `build.ps1`), then start a dev server
 as described above. Note that if you copy an example to a separate folder, you'll need
to edit its `Cargo.toml` to point to the package on [crates.io](https://crates.io) instead of locally: Ie replace
`seed = { path = "../../"` with `seed = "^0.1.0"`, and in the build script, remove the leading `../../` on the second
line.