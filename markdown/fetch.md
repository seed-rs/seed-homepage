# Http requests (fetch), and updating state

We use the [seed::Request](https://docs.rs/seed/0.1.12/seed/fetch/struct.Request.html) struct
to make HTTP requests in the browser, wrapping the [Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API).
To use this, we need to include `futures = "^0.1.26"` in `Cargo.toml`. The [Fetch module](https://docs.rs/seed/0.2.3/seed/fetch/index.html)
is standalone: It can be used with any wasm-bindgen program.

Example, where we update the state on initial load (similar to the `server_interaction` example in the repo)
from a server. It demonstrates a `GET` request, and deserializing JSON data.
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
    FetchData,
    DataFetched(fetch::FetchObject<Branch>),
    OnFetchError {
        label: &'static str,
        fail_reason: fetch::FailReason,
    },
}

fn fetch_data() -> impl Future<Item = Msg, Error = Msg> {
    let url = "https://api.github.com/repos/david-oconnor/seed/branches/master";
    Request::new(url.into()).fetch_json(Msg::DataFetched)
}

fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::FetchData => {
            orders
                .skip()
                .perform_cmd(fetch_data());
        }

        Msg::DataFetched(fetch_object) => {
            match fetch_object.response() {
                Ok(response) => model.branch = response.data,
                Err(fail_reason) => {
                    orders
                        .send_msg(Msg::OnFetchError {
                            label: "Fetching repository info failed",
                            fail_reason,
                        })
                        .skip();
                }
            }
        }

        Msg::OnFetchError { label, fail_reason } => {
            error!(format!("Fetch error - {} - {:#?}", label, fail_reason));
            orders.skip();
        }
    }
}

fn view(model: &Model) -> El<Msg> {
    div![format!(
        "Repo info: Name: {}, SHA: {}",
        model.branch.name, model.branch.commit.sha
    )]
}


#[wasm_bindgen]
pub fn render() {
    let app = seed::App::build(Model::default(), update, view)
        .finish()
        .run();

    app.update(Msg::FetchData);
}

```
On page load, we trigger an update using `Msg::FetchData`, which points the `update`
function to use the `Orders.perform_cmd` method. This allows state to be
update asynchronosly, when the request is complete. `skip()` is a convenience method that
sets `Update::ShouldRender` to `Skip`; sending the request doesn't trigger a render.
We pattern-match the response in the `update` function's`DataFetched` arm: If successful, we update the model.
If not, we update recursively to the `OnFetchError` branch using `.send_msg()`, in this case
displaying an error in the console.

We've set up nested structs that have fields matching the names of the JSON fields of
the response, which `Serde` deserializes the response into, through the `fetch_json` method of
 `Request`. Note that even though more data than 
what's contained in our Branch struct is included
in the response, Serde automatically applies only the info matching our struct's fields.

 
If we wish to trigger
this update from a normal event instead of on load, we can do something like this:
```rust
fn view(model: &Model) -> Vec<El<Msg>> {
    vec![
        div![format!(
            "Repo info: Name: {}, SHA: {}",
            model.branch.name, model.branch.commit.sha
        )],
        button![ raw_ev(Ev::Click, move |_| Msg::FetchData), "Update"]
    ]
}
```

Example showing a POST request where we send data to a server and receive the response, 
and a header:
```rust
#[derive(Serialize)]
struct RequestBody {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ResponseBody {
    pub success: bool,
}

#[derive(Clone)]
enum Msg {
    SendMessage,
    MessageSent(fetch::FetchObject<ResponseBody>),
    OnFetchError {
        label: &'static str,
        fail_reason: fetch::FailReason,
    },
}

fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::SendMessage => {
            orders.skip().perform_cmd(send_message());
        }

        Msg::MessageSent(fetch_object) => match fetch_object.response() {
            Ok(response) => {
                log!(format!("Response data: {:#?}", response.data));
                orders.skip();
            }
            Err(fail_reason) => {
                orders
                    .send_msg(Msg::OnFetchError {
                        label: "Sending message failed",
                        fail_reason,
                    })
                    .skip();
            }
        },

        Msg::OnFetchError { label, fail_reason } => {
            log!(format!("Fetch error - {} - {:#?}", label, fail_reason));
            orders.skip();
        }
    }
}

fn send_message() -> impl Future<Item = Msg, Error = Msg> {
    let message = RequestBody {
        name: "Mark Watney".into(),
        email: "mark@crypt.kk".into(),
        message: "I wanna be like Iron Man".into(),
    };

    Request::new(CONTACT_URL.into())
        .method(Method::Post)
        .header("Content-Type", "application/json")
        .send_json(&message)
        .fetch_json(Msg::MessageSent)
}

fn view(model: &Model) ->El<Msg> {
    button![
        simple_ev(Ev::Click, Msg::SendMessage),
        "Send an urgent message (see console log)"
    ]
}

```

Reference the `Request` API docs (linked above) for a full
list of methods available to configure the request, and links to the `MDN` docs describing
them. (eg: `credentials`, `mode`, `integrity`)


## Updating state
## Todo: This section is out of date, and the behavior it describes will change in the future.

To update the model outside of the element-based event system, we call `update_state` on
our state var, which is the first parameter in our view func. A consequence of this is
that we must pass state to any components that need to update state in this way. This
may require calling `state.clone()`, to use it in multiple places. Note that we also need
to prepend our closures with `move`, as above, any time `state` is used in one.

Here's an example of using set_interval to update the state once every second. It uses
`seed::set_interval`. `seed::set_timeout` also exists, and works the same way:
```rust
fn view(model: &Model) -> El<Msg> {  
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