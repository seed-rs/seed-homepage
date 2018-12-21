# Routing
Seed supports basic routing: You can trigger state changes that update the address bar,
 and can be nagivated to/from using forward and back buttons. This works for landing-page
routing as well, provided your server is configured to support. For an example of routes in use,
see the [homepage example](https://github.com/David-OConnor/seed/tree/master/examples/homepage).
  
As an example, let's say our site has three pages:
a home page, a guide, and a changelog, accessible by `http://seed-rs.org/`, `http://seed-rs.org/guide`,
and `http://seed-rs.org/changelog` respectively. We describe the page by a `page`
field in our model, which is an integer: 0 for homepage, 1 for guide, or 2 for changelog.
(An enum would work as well). 

To set up the initial routing, we pass a HashMap<String, Msg> describing the possible routings
as the last parameter of [Seed::run](https://docs.rs/seed/0.1.10/seed/fn.run.html). We can
create it using the `routes!` macro, which is for convenience: Rust doesn't include a
HashMap literal syntax, and this macro automatically converts the keys to Strings, eg
in the case of the &strs we use in the example below:
```rust
#[wasm_bindgen]
pub fn render() {
    let routes = routes!{
        "guide" => Msg::RoutePage(Page::Guide),
        "changelog" => Msg::RoutePage(Page::Changelog),
    };

    seed::run(Model::default(), update, view, "main", Some(routes));
}
```
This syntax resembles that of the `attrs!` and `style!` macros, but uses commas
for separation.
(Note: In the latest version on Crates.io, the comma separation used in this macro must
be semicolons; this will change next publish)

To make landing-page routing work, configure your server so that all three of these path point towards the app,
or that any (sub)path points towards it, instead of returning an error. Once this is configured, intial 
routing on page load will work as expected: The page will load with the default state, then immediately 
trigger the update prescribed by the RoutePage message.

In order to trigger our route change through in-app naviation (eg clicking a link or pushing a button), include
logic like this in the update function:
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
            // Recursively update with a Msg that performs the update, but does not include seed::push_route
            update(Msg::RoutePage(page), model)
        },
        // This is separate, because in-app naviation needs to call push_route,
        // but we don't want to call it from browser navigation. (eg back button)
        Msg::RoutePage(page) => Model {page, ..model},
}
```
[seed::push_route](https://docs.rs/seed/0.1.8/seed/fn.push_route.html) accepts a single parameter:
a path &str corresponding to what will be appended to the url. Currently, it must match one of the
keys in the route map.

When a page is loaded or browser naviation occurs (eg back button), Seed searches each of the route map keys for 
a matching path name (url suffix). If it finds one,
it updates the model based on its associated message. If not, no action is taken. 
In our example, we assume the model initialized to page=0, for the homepage.

Notice how we keep ChangePage and RoutePage separate in our example: RoutePage performs
the action associated with routing, while ChangePage updates our route history, then
recursively calls RoutePage. If you were to attempt this in the same message, each
browser navigation event would add a redundant route history entry, interfering with navigation. We call
RoutePage from ChangePage, and in the route map. We call ChangePage from an in-app navigation event, like this:

```rust
h2![ simple_ev("click", Msg::ChangePage(1)), "Guide" ]
```

Dynamic routes are not yet supported.
