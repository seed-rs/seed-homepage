# Http requests (fetch), and updating state

We use the [seed::fetch](https://docs.rs/seed/0.1.12/seed/fetch/fn.fetch.html) function
to make HTTP requests in the browser, wrapping the [Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API).
`fetch` takes 3 parameters: 

- The [request method](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods): a
[seed::Method](https://docs.rs/seed/0.1.12/seed/fetch/enum.Method.html)
- The url, an `&str`
- An optional [seed::RequstOps](https://docs.rs/seed/0.1.12/seed/fetch/struct.RequestOps.html), where you
can set things like headers, payload, and credentials.
- A callback that performs actions once the request is complete. It accepts
a [JsValue](https://docs.rs/wasm-bindgen/0.2.29/wasm_bindgen/), and returns nothing.

The convenience functions [seed::get](https://docs.rs/seed/0.1.12/seed/fetch/fn.get.html) and
[seed::post](https://docs.rs/seed/0.1.12/seed/fetch/fn.post.html) are also available;
these are the same as `fetch`, but ommit the method parameter.

Example, where we update the state on initial app load:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
        Msg::Replace(data) => {
            Model {data}
        },
    }
}

fn get_data(app: seed::App<Msg, Model>) {
    let url = "https://api.github.com/repos/david-oconnor/seed/branches/master";
    let callback = move |json: JsValue| {
        let data: Branch = json.into_serde().unwrap();
        app.update_dom(Msg::Replace(data));
    };
    seed::get(url, None, Box::new(callback));
}


fn view(app: seed::App<Msg, Model>, model: Model) -> El<Msg> {
    div![ format!("name: {}, sha: {}", model.data.name, model.data.commit.sha),
        did_mount(move |_| get_data(app.clone()))
     ]
}
```
Note that even though more data than what's contained in our Branch structure is included
in at the url, Serde automatically applies only the info matching our struct's fields.
In order to update our state outside of a normal event, we use did_mount. We could also
do so from a normal event:

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
            get_data(app);
            model
        }
    }
}

fn view(app: seed::App<Msg, Model>, model: Model) -> El<Msg> {
    div![
        div![ format!("Hello World. name: {}, sha: {}", model.data.name, model.data.commit.sha) ],
        button![ raw_ev("click", move |_| Msg::GetData(app.clone())), "Update state from the internet"]
    ]
}
```

## Updating state
To update the model outside of the element-based event system, we call `update_state` on
our app var, which is the first parameter in our view func. A consequence of this is
that we must pass app to any components that need to update state in this way. This
may require calling `app.clone()`, to use it in multiple places. Note that we also need
to prepend our closures with `move`, as above, any time `app` is used in one.

This is the only use for the app var.

Important: Setting Content-Type is currently broken, which affects many
 POST requests. Standby for a fix.

Example showing POST, and headers:
```rust
fn post_data() {
    let url = "https://infinitea.herokuapp.com/api/contact";

    let opts = seed::RequestOpts {
        payload: Some(
            hashmap_string!{
                "name" => "Mark Watney",
                "email" => "mark@crypt.kn",
                "message" => "I wanna be like Iron Man.",
            }
        ),
        headers: Some(  // todo try without headers
            hashmap_string!{
                "Content-Type" => "application/json",
            }
        ),
        credentials: None,
    };

    let callback = move |json: JsValue| {
        // We can do something with the response here.
    };
    seed::post(url, Some(opts), Box::new(callback));
}
```
Note the use of the hashmap_string! macro. This is a HashMap literal that converts
both key and value to Strings. (eg from &strs here).