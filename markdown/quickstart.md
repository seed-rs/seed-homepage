# Quickstart

## Setup
This framework requires you to first install [Rust](https://www.rust-lang.org/tools/install).

You'll need a recent version of Rust: `rustup update`

The wasm32-unknown-unknown target: `rustup target add wasm32-unknown-unknown`

And wasm-bindgen: `cargo install wasm-bindgen-cli`

If you run into errors while installing `wasm-bindgen-cli`, you may need to install C++
build tools. On linux, run `sudo apt install build-essential`. On Windows, download and install
[Visual Studio 2017](https://visualstudio.microsoft.com/downloads/); when asked in the installer,
include the C++ workload.

## The theoretical minimum
To start, clone [This quickstart repo](https://github.com/David-OConnor/seed-quickstart),
run `build.sh` or `build.ps1` in a terminal, then start a dev server that supports WASM.
For example, with [Python](https://www.python.org/downloads/) installed, run `python serve.py`.
(Linux users may need to run `python3 serve.py`.)
Once you change your package name, you'll need to tweak the html file and build script, as described below.


## A little deeper
Alternatively, create a new lib with Cargo: `cargo new --lib appname`. Here and everywhere it appears in this guide, `
appname` should be replaced with the name of your app.

If not using the quickstart repo, create an Html file with a body that contains this:
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
The first line above is an empty element with id: It's where your app will render. The subsequent ones load your app's wasm modules.

The quickstart repo includes this file, but you will need to rename the two 
occurances of `appname`. (If your project name has a hyphen, use an underscore instead here) You will eventually need to modify this file to 
change the page's title, add a description, favicon, stylesheet etc.

`Cargo.toml`, which is a file created by Cargo that describes your app, needs `wasm-bindgen`, `web-sys`, and `
seed` as depdendencies,
 and crate-type
of `"cdylib"`. The version in the quickstart repo has these set up already. Example:

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
```

## A short example
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
fn update(msg: Msg, model: Model) -> Model {
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

/// The top-level component we pass to the virtual dom. Must accept the model as its
/// only parameter, and output a single El.
fn view(app: seed::App<Msg, Model>, model: Model) -> El<Msg> {
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
    // The final parameter is an optional routing map.
    seed::run(Model::default(), update, view, "main", None);
}
```
For truly minimimal example, see [lib.rs in the quickstart repo](https://github.com/David-OConnor/seed-quickstart/blob/master/src/lib.rs)

## Building and running
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
and a JS file used to link your module from HTML.

You may wish to create a build script with these two lines. (`build.sh` for Linux; `build.ps1` for Windows).
The quickstart repo includes these, but you'll still need to do the rename. You can then use
`./build.sh` or `.\build.ps1` If you run into permission errors on `build.sh`, try this command
to allow executing the file:`chmod +x build.sh`. If you run into persmission errors on `build.ps`,
open Powershell as an administrator, and enter this command: `Set-ExecutionPolicy RemoteSigned`.

For development, you can view your app using a shimmed Python dev server, as described above.
(Set up [this mime-type shim](https://github.com/David-OConnor/seed-quickstart/blob/master/serve.py)
from the quickstart repo, and run `python serve.py`).

In the future, the build script and commands above may be replaced by [wasm-pack](https://github.com/rustwasm/wasm-pack).

## Running included examples
To run an example located in the [examples folder](https://github.com/David-OConnor/seed/tree/master/examples),
navigate to that folder in a terminal, 
run the build script for your system (`build.sh` or `build.ps1`), then start a dev server
 as described above. Note that if you copy an example to a separate folder, you'll need
to edit its `Cargo.toml` to point to the package on [crates.io](https://crates.io) instead of locally: Ie replace
`seed = { path = "../../"` with `seed = "^0.1.8"`, and in the build script, remove the leading `../../` on the second
line.
