# Routing
For now, the only supported routing feature is in setting up how to load the inintial
page, based on the entry-point URL. As an example, let's say our site has three pages:
a home page, a guide, and a changelog, accessible by `http://seed-rs.org/`, `http://seed-rs.org/guide`,
and `http://seed-rs.org/changelog` respectively. First, we need to set up our backend server so that
all three of these endpoints point towards our app. We describe the page by a `page`
field in our model, which is an integer: 0 for homepage, 1 for guide, or 2 for changelog.
(An enum would work as well). Our update function includes this logic, triggered by
clicking a link, or pushing a button:
```rust
match msg {
        Msg::ChangePage(page) => {
            Model {page, ..model}
        },
```
To set up the initial routing, we pass a HashMap<&str, Msg> describing the possible routings
as the last parameter of [Seed::run](https://docs.rs/seed/0.1.7/seed/fn.run.html):

```rust
#[wasm_bindgen]
pub fn render() {
    let mut route_map = HashMap::new();
    route_map.insert("/guide", Msg::ChangePage(1));
    route_map.insert("/changelog", Msg::ChangePage(2));

    seed::run(Model::default(), update, view, "main", Some(route_map));
}
```
Seed searches each of the route_map keys for a matching path name (url suffix). If it finds one,
it updates the model based on its associated message. If not, no action will be taken. In our example, we assume the model initialized to page=0, for the homepage.
