# Routing
Seed includes flexible routing, inspired by 
[React-Reason](https://github.com/reasonml/reason-react/blob/master/docs/router.md): 
You can trigger state changes that update the address bar,
 and can be nagivated to/from using forward and back buttons. This works for landing-page
routing as well, provided your server is configured to support. See the
 [homepage](https://github.com/David-OConnor/seed/tree/master/examples/homepage) and
[todomvc](https://github.com/David-OConnor/seed/tree/master/examples/todomvc) examples.
  
Let's say our site the following pages:
a guide, which can have subpages, and a changelog, accessible by `http://seed-rs.org/changelog`,
`http://seed-rs.org/guide`, and `http://seed-rs.org/guide/3 (where 3 is the page we want) respectively. 
We describe the page by a `page`
field in our model, which is an integer: 0 for guide, 1 for changelog, and an additional
number for the guide page. An enum would be cleaner, but we don't wish to complicate this example.

To set up the initial routing, we pass a `routes` function describing how to handle
routing, to [App::build](https://docs.rs/seed/0.2.5/seed/struct.App.html#method.build)'s 
`routes` method.
```rust
fn routes(url: &seed::Url) -> Msg {
    if url.path.is_empty() {
        return Msg::ChangePage(0)
    }

    match url.path[0].as_ref() {
        "guide" => {
            // Determine if we're at the main guide page, or a subpage
            match url.path.get(1).as_ref() {
                Some(page) => Msg::ChangeGuidePage(page.parse::<usize>().unwrap()),
                None => Msg::ChangePage(0)
            }
        },
        "changelog" => Msg::ChangePage(1),
        _ => Msg::ChangePage(0),
    }
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .routes(routes)
        .finish()
        .run();
}
```
Your `routes` function outputs the message that handles the routing, and accepts a ref to a 
[Url struct](https://docs.rs/seed/0.2.4/seed/routing/struct.Url.html)
describing the route, which routes has the following fields:
```rust
pub struct Url {
    pub path: Vec<String>,
    pub hash: Option<String>,
    pub search: Option<String>,
    pub title: Option<String>,
}
```
`path` contains the path heirarchy from top to bottom. For example, the `changelog` page above's path
is `vec![String::from("changelog")]`, representing `/changelog/`, and guide page 3's is 
`vec![String::from("guide"), 3.to_string()]`, representing `/guide/3/`. It's likely all you'll need.
The other three properties aren't as common; `hash` describes text after a `#`; `search` describes
text after a `?`, but before `#`, and title is a descriptive title, unimplemented in current web browsers, but may
see use in the future.

In order to trigger our route change through in-app naviation (eg clicking a link or pushing a button), include
logic like this in the `update` function:
```rust
#[derive(Clone)]
enum Msg {
    RoutePage(u32),
    RouteGuidePage(u32),
    ChangePage(u32),
    ChangeGuidePage(u32),
}

fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::RoutePage(page) => {
            seed::push_path(vec![page]);
            update(Msg::ChangePage(page), model)
        },
        Msg::RouteGuidePage(guide_page) => {
            seed::push_path(vec!["guide", guide_page]);
            update(Msg::ChangeGuidePage(guide_page), model)
        },
        // This is separate, because nagivating the route triggers state updates, which would
        // trigger an additional push state.
        Msg::ChangePage(page) => Render(Model {page, ..model}),
        Msg::ChangeGuidePage(guide_page) => Render(Model {guide_page, page: Page::Guide, ..model}),
    }
}
```

Notice how the `Route` messages above call [seed::push_path](https://docs.rs/seed/0.2.5/seed/routing/fn.push_path.html), 
and the `Change` messages are called in the `routes` function, and are recursively called in the
update function. `push_path` is a convenience function for 
[seed::push_route](https://docs.rs/seed/0.1.8/seed/routing/fn.push_route.html).
`push_route` accepts a single parameter: a `Url` struct, which you can create with
 [seed::Url::new](https://docs.rs/seed/0.2.5/seed/routing/struct.Url.html#method.new) .  It
accepts the path as a `Vec` of items that implement `ToString` (eg `String`, `&str`, numbers),
and makes the rest of the fields `None`. If you wish to define one of these fields, there are additional
methods you can chain together, eg: 

```rust
seed::push_route(
    seed::Url::new(vec!["myurl"])
        .hash("textafterhash")
        .search("textafterquestionmark")
)
```

`push_path` doesn't support `hash`, `search`, or `title`, but has cleaner syntax; it takes the path,
as described above, as its only parameter. 
When a page is loaded or browser naviation occurs (eg back button), Seed uses the `routes`
func you provided to determine which message to call. 

Notice how we keep ChangePage and RoutePage separate in our example. Do not
call `push_path` or `push_route` from one of these messages, or you'll end up with recusions/unwanted behavior:
 `ChangePage` in our example performs
the action associated with routing, while `RoutePage` updates our route history, then
recursively calls `ChangePage`. If you were to attempt this in the same message, each
browser navigation event would add a redundant route history entry, interfering with navigation. `

We call routing messages from in-app navigation events, like this:

```rust
h2![ simple_ev(Ev::Click, Msg::RoutePage(0)), "Guide" ]
```

Or programatically using lifecycle hooks:

```rust
    did_mount(move |_| {
        if model.logged_in {
            state.update(Msg::RoutePage(0))
        }
    })
```

To make landing-page routing work, configure your server so that all relevant paths towards the 
root or html file,
 instead of returning an error. The `serve.py` script
included in the quickstart repo and examples is set up for this. Once this is configured, intial 
routing on page load will work as expected: The page will initialize with the default state, then immediately 
update based on the message returned by the `routes` function.
