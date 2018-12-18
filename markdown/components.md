# Components
The analog of components in frameworks like React are normal Rust functions that that return Els.
The parameters these functions take are not treated in a way equivalent
to attributes on native DOM elements; they just provide a way to 
organize your code. In practice, they feel similar to components in React, but are just
functions used to create elements that end up in the `children` property of
parent elements.

For example, you could break up one of the above examples like this:

```rust
    fn text_display(text: &str) -> El<Msg> {
        h3![ text ]
    }  
    
    div![ style!{"display" => "flex"; flex-direction: "column"},
        text_display("Some things"),
        button![ simple_ev("click", Msg::SayHi), "Click me!" ]
    ]
```

The text_display() component returns a single El that is inserted into its parents'
`children` Vec; you can use this in patterns as you would in React. You can also use
functions that return Vecs of Els, which you can incorporate into other components
using normal Rust code. See Fragments
section below. Rust's type system
ensures that only `El`s  can end up as children, so if your app compiles,
you haven't violated any rules.
 
Note that unlike in JSX, there's a clear syntax delineation here between natural HTML
elements (element macros), and custom components (function calls).

## Fragments
Fragments (`<>...</>` syntax in React and Yew) are components that represent multiple
elements without a parent. This is useful to avoid
unecessary divs, which may be undesirable on their own, and breaks things like tables and CSS-grid. 
There's no special syntax; just have your component return a Vec of `El`s instead of 
one, and add it to the parent's element macro; on its own like in the example below,
 or with other children, or Vecs of children.

```rust
fn cols() -> Vec<El<Msg>> {
    vec![
        td![ "1" ],
        td![ "2" ],
        td![ "3" ]
    ]
}

fn items() -> El<Msg> {
    table![
        tr![ cols() ]
    ]
}
```

## Dummy elements
When performing ternary and related operations instead an element macro, all
branches must return `El`s to satisfy Rust's type system. Seed provides the
`empty()` function, which creates a VDOM element that will not be rendered:

```rust
div![
    if model.count >= 10 { h2![ style!{"padding" => 50}, "Nice!" ] } else { seed::empty() }
]
```
For more complicated construsts, you may wish to create the `children` Vec separately,
push what components are needed, and pass it into the element macro.