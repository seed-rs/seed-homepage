# View

 Visual layout (ie HTML/DOM elements) is described declaratively in Rust, and uses 
[macros]( https://doc.rust-lang.org/book/appendix-04-macros.html) to simplify syntax.

The view's defined by a function that's passed to `seed::run`. This takes a `&Model`
as its parameter, and outputs something that implements the ` View` trait, which is imported in the prelude.
Usually, this is a `Node`, or `Vec<Node>`, representing all nodes that will be inserted as children
on the top-level one. (The top-level `Node` is in the html file, and specified with
`seed::App::build.mount()`, or as a default, the element with id `app`).
 It may composed into sub-functions, which can be thought of like components in other frameworks. 

Examples:
```rust
fn view(model: &Model) -> Node<Msg> {
    h1![ "Let there be light" ],
}
```

```rust
fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        h1![ "Let there be light" ],
        h2![ "Let it be both a particle and a wave" ]
    ]
}
`````
In either of those examples, you could use the signature: `fn view(model: &Model) -> impl View<Msg>` instead.
This allows you to change between them without changing the function signature.

## The Node Enum
The Virtual DOM is represnted by nested [Nodes](https://docs.rs/seed/0.1.6/seed/dom_types/enum.None.html).
`Node` has 3 variants: 
- `Text` holds a [Text](https://docs.rs/seed/0.1.6/seed/dom_types/struct.Text.html)
struct. Mostly for internal use, but can be created with `Node::new_text()`.
- `Element` wraps an [El](https://docs.rs/seed/0.1.6/seed/dom_types/struct.El.html), which is
the main component of our VDOM. Created using macros, described below.
- `Empty` is a placeholder that doens't render anything; useful in conditional/ternary logic.
Created using the `empty![]` macro, or `seed::empty()`.


## Elements, attributes, styles
Elements are created using macros, named by the lowercase name of
each element, and imported into the global namespace. Eg `div!` above. We use this code to import them:
```rust
#[macro_use]
extern crate seed;
```

These macros accept any combination of the following parameters:
- One [Attrs](https://docs.rs/seed/0.1.6/seed/dom_types/struct.Attrs.html) struct
- One [Style](https://docs.rs/seed/0.1.6/seed/dom_types/struct.Style.html) struct
- One or more [Listener](https://docs.rs/seed/0.1.6/seed/dom_types/struct.Listener.html) structs, which handle events
- One or more `Vec`s of `Listener` structs
- One `String` or `&str` representing a node text
- One or more [Node](https://docs.rs/seed/0.1.6/seed/dom_types/enum.Node.html) structs, representing a child
- One or more Vecs of `Node` structs, representing multiple children
- A `Map`, ie the result of `map()`, yielding `Node`s or `Listener`s, without having to explicitly `collect`.

The parameters can be passed in any order; the compiler knows how to handle them
based on their types. Children are rendered in the order passed.

Views are described using [El](https://docs.rs/seed/0.1.6/seed/dom_types/struct.El.html) structs, 
defined in the [seed::dom_types](https://docs.rs/seed/0.1.6/seed/dom_types/index.html) module.

`Attrs` and `Style` are thinly-wrapped hashmaps created with their own macros: `attrs!{}` and `style!{}`
respectively.

Example:
```rust
fn view(model: &Model) -> impl View<Msg> {
    let things = vec![ h4![ "thing1" ], h4![ "thing2" ] ];
    
    let other_things = vec![1, 2];

    div![ attrs!{At::Class => "hardly-any"}, 
        things,  // Vec<Node<Msg>
        other_things.map(|t| h4![t.to_string()]),  // Map
        h4![ "thing3?" ],  // El
    ]
}
```
Note that you can create any of the above items inside an element macro, or create it separately,
and pass it in. You can separate different items by comma, semicolon, or space.

Keys passed to `attrs!` can be `Seed::At`s, `String`s, or `&str`s. 
Keys passed to `style!` can be `Seed::St`s, `String`s, or `&str`s.
Values passed to `attrs!`, and `style!` macros can 
be owned `Strings`, `&str`s, or for `style!`, `unit`s. 
Eg: `input![ attrs!{At::Disabled => false]` and `input![ attrs!{"disabled" => "false"]` 
are equivalent. You use the `unit!` macro to apply units. There's a `px` function for the
special case where the unit is pixels:
```rust
style!{St::Width => unit!(20, px);}
style!{St::Width => px(20);}  // equivalent
```

For boolean attributes that are handled by presense or absense, like `disabled`,
use can use `.as_at_value`: `input![ attrs!{At::Disabled => false.as_at_value() ]`

We can set multiple values for an attribute using `Attribute.add_multiple`. This
is useful for setting multiple classes. Note that we must set this up outside of
the view macro, since it involves modifying a variable:
```rust
fn a_component() -> Node<Msg> {
    let mut attributes = attrs!{};
    attributes.add_multiple(At::Class, vec!["A-modicum-of", "hardly-any"]);

    div![ attributes ]
}
```

Seed validates attributes [against this list](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes);
The `At` enum includes only these values, and `&strs` passed are converted into `At`s. If you
wish to use a custom attribute, use `At::Custom(name)`, where `name` is a `String` of your
attribute's name. In `attrs!` when using `&str`s, inserting an unrecognized attribute
will do the same.

The `class!` and `id!` convenience macros allow settings
attributes as a list of classes, or a single id, if no other attributes are required.
Do not mix and match these with each other, or with attrs!; all but the last-passed
will be thrown out.
```rust
fn a_component() -> Node<Msg> {
    // ...
    span![ class!["calculus", "chemistry", "literature"] ],
    span![ id!("unique-element") ],
    // ...
}
```

You can conditionally add classes with the `class!` macro:
```rust
let active = true;

class![
    "blue",
    "highlighted" => active,
    "confusing" => 0.99999 == 1
    
]
```

Styles and Attrs can be passed as refs as well, which is useful if you need to pass
the same one more than once:
```rust
fn a_component() -> Node<Msg> {
    let item_style = style!{
        St::MarginTop => px(10);
        St::FontSize => unit!(1.2, em)
    };

    div![
        ul![
            li![ &item_style, "Item 1", ],
            li![ &item_style, "Item 2", ],
        ]
    ]
}
```

Setting an InputElement's `checked`, or `autofocus` property is done through normal attributes:
```rust
fn a_component() -> Node<Msg> {
    // ...
    input![ attrs!{At::Typed => "checkbox"; At::Checked => true} ]
    input![ attrs!{At::Autofocus => true} ]
    // ...
}
```

To change Attrs or Styles you've created, edit their .vals HashMap. To add
a new part to them, use their .add method:
```rust
let mut attributes = attrs!{};
attributes.add(At::Class, "truckloads");
```

Example of the style tag, and how you can use pattern-matching in views:
```rust
fn view(model: &Model) -> impl View<Msg> {
    div![ style!{
        St:Display => "grid";
        St::GridTemplateColumns => "auto";
        St::GridTemplateRows => "100px auto 100px"
        },
        section![ style!{St::GridRow => "1 / 2"},
            header(),
        ],
        section![ attrs!{St::GridRow => "2 / 3"},
            match model.page {
                Page::Guide => guide(),
                Page::Changelog => changelog(),
            },
        ],
        section![ style!{St::GridRow => "3 / 4"},
            footer()
        ]
    ]
}
```

We can combine Attrs and Style instances using their `merge` methods, which take
an &Attrs and &Style respectively. This can be used to compose styles from reusable parts. 
Example:
```rust
fn a_component() -> Node<Msg> {
    let base_style = !style{"color" => "lavender"};

    div![
        h1![ &base_style.merge(&style!{St::GridRow => "1 / 2"}) "First row" ],
        h1![ &base_style.merge(&style!{St::GridRow => "2 / 3"}) "Second row" ],
    ]
}
```

Overall: we leverage of Rust's strict type system to flexibly-create the view
using normal Rust code.W


`El` has several helper methods which can be chained together:
```rust
let my_el = div![]
    .add_text("Words")
    .add_class("complete")
    .add_attr("alt".to_string(), "a description".to_string())
    .add_style(St::Height, "20px".to_string())
    .replace_text("Oops, not complete");oo

```

## Svg
You can create `SVG` elements in the same way as normal `Html` elements.
Setting the `xmlns` attribute isn't required; it's set automatically when using the macro.

Example using macros:
```rust
svg![
    rect![
        attrs!{
            At::X => "5",
            At::Y =>"5",
            At::Width => "20",
            At::Height => "20",
            At::Stroke => "green",
            At::StrokeWidth => "4",
        }
    ]
]
```

The same exmaple using `from_html`:
```rust
Node::from_html(
r#"
<svg>
    <rect x="#5" y="5" width="20" height="20" stroke="green" stroke-width="4" />
</svg>
"#)
```

Another example, showing it in the `View` fn:
```rust
fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        svg![
            attrs!{
                At::Width => "100%";
                At::Height => "100%";
                At::ViewBox => "0 0 512 512";
            },
            path![ 
                attrs!{
                    At::Fill => "lightgrey";
                    At::D => "M345.863,281.853c19.152-8.872,38.221-15.344,56.1"  // etc
                }
            ],
            // More elements as required, eg mesh, polyline, circle
        ]
    ]
}
```