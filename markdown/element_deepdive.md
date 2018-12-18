# Element-creation macros, under the hood
The following code returns an `El` representing a few DOM elements displayed
in a flexbox layout:
```rust
    div![ style!{"display" => "flex"; "flex-direction" => "column"},
        h3![ "Some things" ],
        button![ "Click me!" ]
    ]
```

The only magic parts of this are the macros used to simplify syntax for creating these
things: text are [Options](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values)
 of Rust borrowed Strings; `Listeners` are stored in Vecs; children are elements and/or Vecs of;
`Attr`s and `Style` are thinly-wrapped HashMaps. They can be created independently, and
passed to the macros separately. The following code is equivalent; it uses constructors
from the El struct. Note that `El` type is imported with the Prelude.

```rust
    use seed::dom_types::{El, Attrs, Style, Tag};
    
    // heading and button here show two types of element constructors
    let mut heading = El::new(
        Tag::H2, 
        Attrs::empty(), 
        Style::empty(), 
        Vec::new(),
        "Some things",
        Vec::New()
    );  
    
    let mut button = El::empty(Tag::Button);
    let children = vec![heading, button];
    
    let mut elements = El::empty(Tag::Div);
    elements.add_style("display", "flex");
    elements.add_style("flex-direction", "column");
    elements.children = children;
    
    elements
```

The following equivalent example shows creating the required structs without constructors,
to demonstrate that the macros and constructors above represent normal Rust structs,
and provides insight into what abstractions they perform:

```rust
// We didn't provide an example of a Listener/style: These are
// more complicated to show using literals.
use seed::dom_types::{El, Attrs, Style, Tag};

// Rust has no built-in HashMap literal syntax.
let mut style = HashMap::new();
style.insert("display", "flex");  
style.insert("flex-direction", "column");  

El {
    tag: Tag::Div,
    attrs: Attrs { vals: HashMap::new() },
    style: Style { vals: style },
    events: Events { vals: Vec::new() },
    text: None,
    children: vec![
        El {
            tag: Tag::H2,
            attrs: Attrs { vals: HashMap::new() },
            style: Style { vals: HashMap::new() },
            listeners: Vec::new();
            text: Some(String::from("Some Things")),
            children: Vec::new()
        },
        El {
            tag: Tag::button,
            attrs: Attrs { vals: HashMap::new() },
            style: Style { vals: HashMap::new() },
            listeners: Vec::new();
            text: None,
            children: Vec::new(),
        } 
    ]
}
```

For most uses, the first example (using macros) will be the easiest to read and write.
You can mix in constructors (or struct literals) in components as needed, depending on your code structure.