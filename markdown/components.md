# Components
The analog of components in frameworks like React are normal Rust functions that that return
[El](https://docs.rs/seed/0.1.8/seed/dom_types/struct.El.html)s.
These functions take parameters that are not treated in a way equivalent
to attributes on native DOM elements; they just provide a way to 
organize your code. In practice, they're used in a way similar to components in React.

For example, you could organize one of the examples in the Structure section of the guide like this:
```rust
    fn text_display(text: &str) -> El<Msg> {
        h3![ text ]
    }  
    
    div![ style!{"display" => "flex"; "flex-direction" => "column"},
        text_display("Some things"),
        button![ simple_ev("click", Msg::SayHi), "Click me!" ]
    ]
```

The text_display component returns a single `El` that is inserted into its parents'
`children` Vec; you can use this in patterns as you would in React. You can also use
functions that return `Vec`s of`El`s, which you can incorporate into other `El`s
using normal Rust code. See the Fragments section below. Rust's type system
ensures that only `El`s  can end up as children, so if your app compiles,
you haven't violated any rules.
 
Unlike in JSX, there's a clear syntax delineation between natural DOM
elements (element macros), and custom components (function calls): We called text_display
above as `text_display("Some things")`, not `text_display![ "Some things" ]`.

## Fragments
Fragments (`<>...</>` syntax in React and Yew) are components that represent multiple
elements without a parent. They're useful to avoid
unecessary divs, which clutter teh DOM, and breaks things like tables and CSS-grid. 
There's no special fragment syntax: have your component return a `Vec` of `El`s instead of 
one. Add it to the parent's element macro:
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

You can mix `El` `Vec`s with `Els` in macros:
```rust
fn items() -> El<Msg> {
    // You may wish to keep complicated or dynamic logic separate.
    let mut more_cols = vec![ td![ "another col" ], td![ "and another" ] ];
    more_cols.push(td![ "yet another" ]);

    table![
        tr![
            td![ "first col" ],  // A lone element
            cols(),  // A "fragment" component.
            td![ "an extra col" ], // A element after the fragment
            // A Vec of Els, not in a separate func
            vec![ td![ "another col" ], td![ "and another" ] ],
            more_cols  // A vec of Els created separately.
        ]
    ]
}
```

## Dummy elements
When performing ternary operations inside an element macro, all
branches must return an `El` (Or `Vec` of `El`s) to satisfy Rust's type system. Seed provides the
[empty](https://docs.rs/seed/0.1.8/seed/fn.empty.html) function, which creates an element that will not be rendered:
```rust
div![
    if model.count >= 10 { h2![ style!{"padding" => 50}, "Nice!" ] } else { seed::empty() }
]

```
