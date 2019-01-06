# Integration with Rust (backend) servers

If pairing Seed with a Rust backend server, we can simplify passing data between
server and frontend by using a layout similar to that in the [server_integration example](https://github.com/David-OConnor/seed/tree/master/examples/server_interaction)
Here, we demonstrate using a single struct for both frontend and server, with [Rocket](https://rocket.rs/).
as the server. This is useful for reducing duplication of data structures, and allows
`Serde` to elegantly handle [de]serialization.
For example, we can use use the same struct which represents a 
database item on a server in Seed, without redefining or changing it.

Highlights from the example:

- We set up the frontend and backend as independent crates, with the frontend folder
inside the backend one. Alternatively, we could set them up at the same nest level.
- We place the shared data structures in a barebones third crate called `shared`. We can't access
data on the backend crate due to it being incompatible with the `wasm32-unknown-unknown` target.
We can't do the reverse due to being unable to import `"cdylib"` crates.
- With `Rocket`, we must use the nightly toolchain for the backend.
- We set the server and client to use different ports
- The Rocket server is set up with CORS, to accept requests from localhost only.
- We are unable to share a workspace between backend and frontend due to incompatible
compile targets.

To run the example, navigate to the server_integration example, and run `cargo +nightly run`.
In a different terminal, navigate to its `frontend` subdirectory, and run a build script and dev server
as you would normally for a seed app.

Folder structure:
```
backend: Our server crate, in this case Rocket
 └── frontend: A normal seed crate
 └── shared: Contains data structures shared between frontend and backend
 
```

Backend Cargo.toml. A normal `Rocket` one, with a relative-path `shared` dependency, and CORS support:
```toml
[package]
name = "backend"
version = "0.1.0"
authors = ["Your Name <email@address.com>"]
edition = "2018"

[dependencies]
rocket = "^0.4.0-rc.1"
serde_json = "^1.0.33"
rocket_cors = "^0.4.0"
shared = { path = "shared" }
```
Frontend Cargo.toml. The only difference from a normal Seed crate is the `shared` dependency.
Note that we don't need to import `Serde` directly, in this case.
```toml
[package]
name = "frontend"
version = "0.1.0"
authors = ["Your Name <email@address.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
seed = "^0.2.1"
wasm-bindgen = "^0.2.29"
web-sys = "^0.3.6"
shared = { path = "../shared"}
```

Shared Cargo.toml:
```toml
[package]
name = "shared"
version = "0.1.0"
authors = ["Your Name <email@address.com>"]
edition = "2018"

[dependencies]
serde = { version = "^1.0.80", features = ['derive'] }
```

In `shared/lib.rs`, we set up serializable data structures:
```rust
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Data {
    pub val: i8,
    pub text: String,
}
```

In the frontend and backend, we import `shared`, and use these structures normally:

backend:
use shared::Data;

```rust
#[get("/data", format = "application/json")]
fn data_api() -> String {
    let data = Data {
        val: 7,
        text: "Test data".into(),
    };

    serde_json::to_string(&data).unwrap()
}
```

frontend:
```rust
use shared::Data;


// Model

#[derive(Clone)]
struct Model {
    pub data: Data,
}

fn get_data(state: seed::App<Msg, Model>) {
    let url = "http://localhost:8001/data";
    let callback = move |json: JsValue| {
        let data: Data = json.into_serde().unwrap();
        state.update(Msg::Replace(data));
    };

    seed::get(url, None, Box::new(callback));
}

#[derive(Clone)]
enum Msg {
    GetData(seed::App<Msg, Model>),
    Replace(Data),
}

fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::GetData(state) => {
            get_data(state);
            model
        },
        Msg::Replace(data) => Model {data}
    }
}

```