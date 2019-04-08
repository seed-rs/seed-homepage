# Http requests (fetch), and updating state

We use the [seed::Request](https://docs.rs/seed/0.1.12/seed/fetch/struct.Request.html) struct
to make HTTP requests in the browser, wrapping the [Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API).
To use this, we need to include `futures = "^0.1.20"` in `Cargo.toml`. The [Fetch module](https://docs.rs/seed/0.2.3/seed/fetch/index.html)
is standalone: It can be used with any wasm-bindgen program.

Example, where we update the state on initial load:
```rust
use futures::Future;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[derive(Clone)]
enum Msg {
    GetData,
    Replace(Branch),
    OnFetchErr(JsValue),
}

fn update(msg: Msg, model: &mut Model) -> Update<Msg> {
    match msg {
        Msg::GetData => Update::with_future_msg(get_data()).skip(),
        
        Msg::Replace(data) => {
            model.data = data;
            Render.into()
        },
        Msg::OnFetchErr(err) => {
            log!(format!("Fetch error: {:?}", err));
            Skip.into()
        }
    }
}

fn get_data() -> impl Future<Item = Msg, Error = Msg> {
    let url = "https://api.github.com/repos/david-oconnor/seed/branches/master";

    seed::Request::new(url)
        .method(seed::Method::Get)
        .fetch_json()
        .map(Msg::Replace)
        .map_err(Msg::OnFetchErr)
}

<<<<<<< HEAD
fn view(state: seed::App<Msg, Model>, model: &Model) -> Vec<El<Msg>> {
    div![ format!("name: {}, sha: {}", model.data.name, model.data.commit.sha),
        did_mount(move |_| spawn_local(get_data(state.clone())))
     ]
}
=======
// ...

#[wasm_bindgen]
pub fn render() {
    let state = seed::App::build(Model::default(), update, view)
        .finish()
        .run();

    state.update(Msg::GetData);

>>>>>>> 120884dccd13e179489134a53d4d28393bd19370
```
On page load, we trigger an update using `Msg::GetData`, which points the `update`
function to use the `Update::with_future_msg` method. This allows state to be
update asynchronosly, when the request is complete. `skip()` is a convenience method that
sets `Update::ShouldRender` to `Skip`; sending the request doesn't trigger a render.
We use `Request::map` to point to an enum that handles successful retrieval, and wraps
the struct of the response. In this case, `Msg::Replace`. We use `Request::map_err` in a similar
way to handle http failures; the enum it points to wraps a `wasm_bindgen::JsValue`.

This
a `get` request by passing the url, options like headers (In this example, we don't use any),
and a callback to be executed once the data's received.

We've set up nested structs that have fields matching the names of the JSON fields of
the response, which `Serde` deserializes the response into, through the `fetch_json` method of
 `Request`. Note that even though more data than 
what's contained in our Branch struct is included
in the response, Serde automatically applies only the info matching our struct's fields.

 
If we wish to trigger
this update from a normal event instead of on load, we can do something like this:
```rust
#[derive(Clone)]
enum Msg {
    Replace(Branch),
    GetData(seed::App<Msg, Model>),
}

fn update(msg: Msg, model: Model) -> Update<Msg, Model> {
    match msg {
        Msg::Replace(data) => Render(Model {data}),
        Msg::GetData(state) => {
            spawn_local(get_data(state));
            Render(model)
        },
    }
}

fn view(state: seed::App<Msg, Model>, model: &Model) -> El<Msg> {
    div![
        div![ format!("Hello World. name: {}, sha: {}", model.data.name, model.data.commit.sha) ],
        button![ raw_ev(Ev::Click, move |_| Msg::GetData(state.clone())), "Update from the internet"]
    ]
}
```

Example showing POST, and headers:
```rust
#[derive(Serialize)]
struct Message {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[derive(Deserialize, Debug)]
struct ServerResponse {
    pub success: bool,
}

fn send() -> impl Future<Item = (), Error = JsValue> {
    let url = "https://infinitea.herokuapp.com/api/contact";

    let message = Message {
        name: "Mark Watney".into(),
        email: "mark@crypt.kk".into(),
        message: "I wanna be like Iron Man".into(),
    };

    Request::new(url)
        .method(Method::Post)
        .header("Content-Type", "application/json")
        .body_json(&message)
        .fetch_json()
        .map(|result: ServerResponse| {
            log!(format!("Response: {:?}", result));
        })
}
```
Note how we pass a ref to the struct we wish to serialize (the payload) to the `.body_json()` method;
serialization happens out of sight. Reference the `Request` API docs (linked above) for a full
list of methods available to configure the request, and links to the `MDN` docs describing
them. (eg: `credentials`, `mode`, `integrity`)


## Updating state
To update the model outside of the element-based event system, we call `update_state` on
our state var, which is the first parameter in our view func. A consequence of this is
that we must pass state to any components that need to update state in this way. This
may require calling `state.clone()`, to use it in multiple places. Note that we also need
to prepend our closures with `move`, as above, any time `state` is used in one.

Here's an example of using set_interval to update the state once every second. It uses
`seed::set_interval`. `seed::set_timeout` also exists, and works the same way:
```rust
fn view(state: seed::App<Msg, Model>, model: &Model) -> El<Msg> {  
    div![
        did_mount(move |_| {
            let state2 = state.clone();

            let callback = move || {
                state2.update(Msg::Increment);
            };

            seed::set_interval(Box::new(callback), 1000);
        }),
        
        button![
            simple_ev(Ev::Click, Msg::Increment),
            format!("Hello, World × {}", model.val)
        ]
    ]
}
```

`App::build` returns an instance of `seed::App`, which we can use to updated state from the `render` function. Example:
```rust
fn open_websockets(state: seed::App<Msg, Model>) {

  // setup websockets ...

  let on_message = Box::new(move|ev: MessageEvent| {
    let txt = ev.data().as_string().unwrap();
    let json: JsonMsg = serde_json::from_str(&text).unwrap();
    state.update(Msg::Json(json));
  });
}

pub fn render() {
    let state = App::build(Model::default(), update, view)
        .finish()
        .run();
    open_websockets(state);
}
```

Re-examining our initial example, instead of loading the data when the top-level element mounts,
we can load it in `render` like this:

```rust
#[wasm_bindgen]
pub fn render() {
    let state = seed::App::build(Model::default(), update, view)
        .finish()
        .run();

    spawn_local(get_data(state));
}
```

See the [server_interaction example](https://github.com/David-OConnor/seed/tree/master/examples/server_interaction)
for a full example.

Props to Pauan for writing the Fetch module.