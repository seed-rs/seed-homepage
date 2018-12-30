# Http requests (fetch), and updating state

We use the [seed::fetch](https://docs.rs/seed/0.1.12/seed/fetch/fn.fetch.html) function
to make HTTP requests in the browser, wrapping the [Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API).
`fetch` takes 3 parameters: 

- The [request method](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods): a
[seed::Method](https://docs.rs/seed/0.1.12/seed/fetch/enum.Method.html)
- The url, an `&str`
- An optional [seed::RequstOpts](https://docs.rs/seed/0.1.12/seed/fetch/struct.RequestOpts.html) struct, where you
can set things like headers, payload, and credentials.
- A callback that performs actions once the request is complete. It accepts
a [JsValue](https://docs.rs/wasm-bindgen/0.2.29/wasm_bindgen/), and returns nothing.

The convenience functions [seed::get](https://docs.rs/seed/0.1.12/seed/fetch/fn.get.html) and
[seed::post](https://docs.rs/seed/0.1.12/seed/fetch/fn.post.html) are also available;
these are the same as `fetch`, but ommit the method parameter. Additionally, `seed::post`
uses a non-serialized payload as a second parameter: This is any Rust struct which implements
`serde::Serialize`. It overrides the payload defined in `RequestOpts`.

Example, where we update the state on initial app load:
```rust
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
    Replace(Branch),
}

fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::Replace(data) => Model {data},
    }
}

fn get_data(app: seed::App<Msg, Model>) {
    let url = "https://api.github.com/repos/david-oconnor/seed/branches/master";
    let callback = move |json: JsValue| {
        let data: Branch = json.into_serde().unwrap();
        app.update(Msg::Replace(data));
    };
    seed::get(url, None, Box::new(callback));
}

fn view(state: seed::App<Msg, Model>, model: Model) -> El<Msg> {
    div![ format!("name: {}, sha: {}", model.data.name, model.data.commit.sha),
        did_mount(move |_| get_data(state.clone()))
     ]
}
```
When the top-level element is rendered for the first time (`did_mount`), we make
a `get` request by passing the url, options like headers (In this example, we don't use any),
and a callback to be executed once the data's received. In this case, we update our
state by sending a message which contains the data to `state.update`.

We've set up nested structs that have fields matching the names of the JSON fields of
the response, which Serde deserializes the response into. Note that even though more data than 
what's contained in our Branch struct is included
in the response, Serde automatically applies only the info matching our struct's fields.
In order to update our state outside of a normal event, we used `did_mount`. If we wish to trigger
this update from a normal event instead of on load, we can do something like this:

```rust
#[derive(Clone)]
enum Msg {
    Replace(Branch),
    GetData(seed::App<Msg, Model>)
}

fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::Replace(data) => Model {data},
        Msg::GetData(app) => {
            let url = "https://api.github.com/repos/david-oconnor/seed/branches/master";
            let callback = move |json: JsValue| {
                let data: Branch = json.into_serde().unwrap();
                app.update(Msg::Replace(data));
            };
            seed::get(url, None, Box::new(callback));
            model
        }
    }
}

fn view(state: seed::App<Msg, Model>, model: Model) -> El<Msg> {
    div![
        div![ format!("Hello World. name: {}, sha: {}", model.data.name, model.data.commit.sha) ],
        button![ raw_ev("click", move |_| Msg::GetData(state.clone())), "Update from the internet"]
    ]
}
```

## Updating state
To update the model outside of the element-based event system, we call `update_state` on
our state var, which is the first parameter in our view func. A consequence of this is
that we must pass state to any components that need to update state in this way. This
may require calling `state.clone()`, to use it in multiple places. Note that we also need
to prepend our closures with `move`, as above, any time `state` is used in one.

Example showing POST, and headers:
```rust
use serde_json;

// ...

#[derive(Serialize, Deserialize)]
struct Message {
    pub name: String,
    pub email: String,
    pub message: String,
}

fn post_data() {
    let message = Message {
        name: "Mark Watney".into(),
        email: "mark@crypt.kk".into(),
        message: "I wanna be like Iron Man".into(),
    };
    
    let mut opts = seed::RequestOpts::new();
    opts.headers.insert("Content-Type".into(), "application/json".into());
    
    // We can handle the server's response in the callback, as in the Get example.
    let callback = move |json: JsValue| {};
    seed::post(url, message, Some(opts), Box::new(callback));
}
```
Note how we pass the struct we wish to serialize (the payload) as the second parameter to `post`;
serialization happens out of sight. If a payload is included in `RequestOpts`, it's replaced by this.
Alternatively, we could use `fetch`, and pass an arbitrary payload `String` in `opts`. 
Here's an example, also demonstrating use 
of the `hashmap_string!` macro for brevity: a HashMap literal, which converts
both key and value to Strings (eg we avoid repetitive `insert`, and `into()` as in above):

```rust
fn post_data() {
    let message = Message {
        name: "Mark Watney".into(),
        email: "mark@crypt.kk".into(),
        message: "I wanna be like Iron Man".into(),
    };
    
    let mut opts = seed::RequestOpts::new();
    opts.headers = hashmap_string!{
        "Content-Type" => "application/json",
    };
    opts.payload = Some(serde_json::to_string(&message).unwrap());
    
    let callback = move |json: JsValue| {};
    seed::fetch(seed::Method::Post, url, Some(opts), Box::new(callback));
}
```

See the [server_interaction example](https://github.com/David-OConnor/seed/tree/master/examples/server_interaction)
for a full example.