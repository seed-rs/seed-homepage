# Routing
Seed supports basic routings: You can trigger state changes that update the address bar,
 and can be nagivated to/from using forward and back buttons. This works for custom landing-page
  routing as well, provided your server is configured to support this. (eg sub-paths will redirect to
  the main path instead of throwing errors.)
  
  As an example, let's say our site has three pages:
a home page, a guide, and a changelog, accessible by `http://seed-rs.org/`, `http://seed-rs.org/guide`,
and `http://seed-rs.org/changelog` respectively. First, we need to set up our backend server so that
all three of these endpoints point towards our app. We describe the page by a `page`
field in our model, which is an integer: 0 for homepage, 1 for guide, or 2 for changelog.
(An enum would work as well). 

To set up the initial routing, we pass a HashMap<&str, Msg> describing the possible routings
as the last parameter of [Seed::run](https://docs.rs/seed/0.1.7/seed/fn.run.html):

```rust
#[wasm_bindgen]
pub fn render() {
    let mut route_map = HashMap::new();
    route_map.insert("guide", Msg::RoutePage(1));
    route_map.insert("changelog", Msg::RoutePage(2));

    seed::run(Model::default(), update, view, "main", Some(route_map));
}
```
Once this is configured, intial routing on page load will work as expected: The page will
load with the default state, then immediately trigger the update prescribed by the RoutePage
message. std::collections::HashMap is included in the Seed prelude.

In order to trigger our route change through an event (eg clicking a link or pushing a button), our update function 
includes the following logic:
```rust
#[derive(Clone)]
enum Msg {
    ChangePage(u32),
    RoutePage(u32),
}

fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::ChangePage(page) => {
            // An enum, with a to_string() method might be a more elegant way
            // to store page state.
            let page_name = match page {
                0 => "",
                1 => "guide",
                2 => "changelog"
            };
            seed::push_route(page_name);
            update(Msg::RoutePage(page), model)
        },
        // This is separate, because nagivating the route triggers state updates, which would
        // trigger an additional push state.
        Msg::RoutePage(page) => Model {page, ..model},
}
```
[seed::push_route](https://docs.rs/seed/0.1.8/seed/fn.push_route.html) accepts a single parameter:
a path &str corresponding to what will be appended to the url. Currently, it must match one of the
keys in the route map.

When a page is loaded, or naviation occurs (eg back button), Seed searches each of the route_map keys for 
a matching path name (url suffix). If it finds one,
it updates the model based on its associated message. If not, no action will be taken. 
In our example, we assume the model initialized to page=0, for the homepage.

Notice how we keep ChangePage and RoutePage separate in our example: RoutePage performs
the action associated with routing, while ChangePage updates our route history, then
recursively calls RoutePage. If you were to attempt this in the same message, each
navigation event would add a redundant route history entry, interfering with navigation. We call
RoutePage from ChangePage, and in the route map. We call ChangePage from an event, like this:

```rust
h2![ simple_ev("click", Msg::ChangePage(1)), "Guide" ]
```

Dynamic routes are not yet supported.