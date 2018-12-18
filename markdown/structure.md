# App structure

### Model
Each app must contain a model [struct]( https://doc.rust-lang.org/book/ch05-00-structs.html), 
which contains the app’s state and data. It must derive `Clone`, and should contain 
[owned data](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html). References
with a static [lifetime](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html) may work,
but may be more difficult to work with. Example:

```rust
#[derive(Clone)]
struct Model {
    count: i32,
    what_we_count: String
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            count: 0,
            what_we_count: "click".into()
        }
    }
}
```
 
In this example, we provide 
initialization via Rust’s `Default` trait, in order to keep the initialization code by the
 model itself. When we call `Model.default()`, it initializes with these values. We could 
 also initialize it using a constructor method, or a struct literal. Note the use of `into()` 
 on our string literal, to convert it into an owned string.
 
The model holds all data used by the app, and will be replaced with updated versions when the data changes.
Use owned data in the model; eg `String` instead of `&'static str`.

 The model may be split into sub-structs to organize it – this is especially useful as the app grows. 
Sub-structs must implement `Clone`:
 

```rust
#[derive(Clone)]
struct FormData {
    name: String,
    age: i8,
}

#[derive(Clone)]
struct Misc {
    value: i8,
    descrip: String,
}

#[derive(Clone)]
struct Model {
    form_data: FormData,
    misc: Misc
}
```

### Update
The Message is an [enum]( https://doc.rust-lang.org/book/ch06-00-enums.html) which 
categorizes each type of interaction with the app. Its fields may hold a value, or not. 
We’ve abbreviated it as `Msg` here for brevity. If you're not familiar with enums,
think of one as a set of options; in other languages, you might use an integer, or string 
for this, but an enum is explicitly limited in which values it can take. Example:

```rust
#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
    ChangeDescrip(String),
}
```
 
The update [function]( https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) 
you pass to `seed::run` describes how the state should change, upon
receiving each type of Message. It is the only place where the model is changed. It accepts a message, 
and model as parameters, and returns a model. This function signature cannot be changed.
 Note that it doesn’t update the model in place: It returns a new one.

Example:

```rust
fn update(history: &mut History<Model, Msg>, msg: Msg, model: Model) -> Model {
    match msg {
        Msg::Increment => Model {count: model.count + 1, ..model},
        Msg::SetCount(count) => Model {count, ..model},
    }
}
```

 While the signature of the update function is fixed (Accepts a History struct, Msg and ref to the model; outputs
 a new model), and will usually involve a match pattern, with an arm for each Msg, there
 are many ways you can structure this function. Some may be easier to write, and others may 
 be more efficient, or appeal to specific aesthetics. While the example above
 it straightforward, this becomes import with more complicated updates.
 
 The signature suggests taking an immutable-design/functional approach. This can be verbose
 when modifying collections, but is a common pattern in Elm and Redux. Unlike in a pure functional language,
 side-effects (ie other things that happen other than updating the model) don't require special 
 handling. Example, from the todomvc example:
```rust
fn update(history: &mut History<Model, Msg>, msg: Msg, model: Model) -> Model {
    match msg {
        Msg::ClearCompleted => {
            let todos = model.todos.into_iter()
                .filter(|t| !t.completed)
                .collect();
            Model {todos, ..model}
        },
        Msg::Destroy(posit) => {
            let todos = model.todos.into_iter()
                .enumerate()
                .filter(|(i, t)| i != &posit)
                .map(|(i, t)| t)
                .collect();
            Model {todos, ..model}
        },
        Msg::Toggle(posit) => {
            let mut todos = model.todos;
            let mut todo = todos.remove(posit);
            todo.completed = !todo.completed;
            todos.insert(posit, todo);

            Model {todos, ..model}
        },
        Msg::ToggleAll => {
            let completed = model.active_count() != 0;
            let todos = model.todos.into_iter()
                .map(|t| Todo {completed, ..t})
                .collect();
            Model {todos, ..model}
        }
    }
}
 ```
In this example, we avoid mutating data. In the first two Msgs, we filter the todos,
then pass them to a new model using [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax)
.  In the third Msg, we mutate todos, but don't mutate the model itself. In the fourth,
we build a new todo list using a functional technique. The [docs for Rust Iterators](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
show helpful methods for functional iterator manipulation.

Alternatively, we could write the same update function like this:
```rust
fn update(history: &mut History<Model, Msg>, msg: Msg, model: Model) -> Model {
    let mut model = model;
    match msg {
        Msg::ClearCompleted => {
            model.todos = model.todos.into_iter()
            .filter(|t| !t.completed)
            .collect();
        },
        Msg::Destroy(posit) => {
            model.todos.remove(posit);
        },
        Msg::Toggle(posit) => model.todos[posit].completed = !model.todos[posit].completed,
        Msg::ToggleAll => {
            let completed = model.active_count() != 0;
            for todo in &mut model.todos {
                todo.completed = completed;
        }
    };
    model
}
 ```
This approach, where we mutate the model directly, is much more concise when
handling collections. How-to: Reassign `model` as mutable at the start of `update`. 
Return `model` at the end. Mutate it during the match legs.

As with the model, only one update function is passed to the app, but it may be split into 
sub-functions to aid code organization.

Note that you can perform updates recursively, ie have one update trigger another. For example,
here's a non-recursive approach, where functions do_things() and do_other_things() each
act on an Model, and output a Model:
 ```rust
fn update(fn update(history: &mut History<Model, Msg>, msg: Msg, model: Model) -> Model {
    match msg {
        Msg::A => do_things(model),
        Msg::B => do_other_things(do_things(model)),
    }
}
 ```
Here's a recursive equivalent:
 ```rust
fn update(fn update(history: &mut History<Model, Msg>, msg: Msg, model: Model) -> Model {
    match msg {
        Msg::A => do_things(model),
        Msg::B => do_other_things(update(history, Msg::A, model)),
    }
}
 ```

 The history parameter is currently unused; it will be used for routing in the future.

 
### View
 Visual layout (ie HTML/DOM elements) is described declaratively in Rust, but uses 
[macros]( https://doc.rust-lang.org/book/appendix-04-macros.html) to simplify syntax. 

### Elements, attributes, styles
Elements are created using macros, named by the lowercase name of
each element, and imported into the global namespace:
```rust
#[macro_use]
extern crate seed;

// ...

div![]
```
These macros accept any combination of the following parameters:
- One [Attrs](https://docs.rs/seed/0.1.6/seed/dom_types/struct.Attrs.html) struct
- One [Style](https://docs.rs/seed/0.1.6/seed/dom_types/struct.Style.html) struct
- One or more [Listener](https://docs.rs/seed/0.1.6/seed/dom_types/struct.Listener.html) structs, which handle events
- One or more Vecs of Listener structs
- One String or &str representing a node text
- One or more [El](https://docs.rs/seed/0.1.6/seed/dom_types/struct.El.html) structs, representing a child
- One or more Vecs of El structs, representing multiple children

The parameters can be passed in any order; the compiler knows how to handle them
based on their types. Children are rendered in the order passed.

Views are described using [El structs](https://docs.rs/seed/0.1.6/seed/dom_types/struct.El.html), 
defined in the [seed::dom_types](https://docs.rs/seed/0.1.6/seed/dom_types/index.html) module. They're most-easily created
with a shorthand using macros.

`Attrs` and `Style` are thinly-wrapped hashmaps created with their own macros: `attrs!{}` and `style!{}`
respectively.

Example:
```rust
let things = vec![ h4![ "thing1" ], h4![ "thing2" ] ];

div![ attrs!{"class" => "hardly-any"}, 
    things,
    h4![ "thing3?" ]
]
```
Note that you can create any of the above items inside an element macro, or create it separately,
and pass it in.

Values passed to `attrs`, and `style` macros can be owned `Strings`, `&str`s, or when applicable, numerical and 
boolean values. Eg: `input![ attrs!{"disabled" => false]` and `input![ attrs!{"disabled" => "false"]` 
are equivalent. If a numerical value is used in a `Style`, 'px' will be automatically appended.
If you don't want this behavior, use a `String` or`&str`. Eg: `h2![ style!{"font-size" => 16} ]` , or
`h2![ style!{"font-size" => "1.5em"} ]` for specifying font size in pixels or em respectively. Note that
once created, a `Style` instance holds all its values as `Strings`; eg that `16` above will be stored
as `"16px"`; keep this in mind if editing a style that you made outside an element macro.

Styles and Attrs can be passed as refs as well, which is useful if you need to pass
the same one more than once:
```rust
 let item_style = style!{
        "margin-top" => 10;
        "font-size" => "1.2em"
    };

    div![
        ul![
            li![ &item_style, "Item 1", ],
            li![ &item_style, "Item 2", ],
        ]
    ]
```

Setting an InputElement's `checked` property is done through normal attributes:
```rust
input![ attrs!{"type" => "checkbox"; "checked" => true} ]
```

To change Attrs or Styles you've created, edit their .vals HashMap. To add
a new part to them, use their .add method:
```rust
let mut attributes = attrs!{};
attributes.add("class", "truckloads");
```

Example of the style tag, and how you can use pattern-matching in views:
```rust
fn view(model: Model) -> El<Msg> {
    div![ style!{
        "display" => "grid";
        "grid-template-columns" => "auto";
        "grid-template-rows" => "100px auto 100px"
        },
        section![ style!{"grid-row" => "1 / 2"},
            header(),
        ],
        section![ attrs!{"grid-row" => "2 / 3"},
            match model.page {
                Page::Guide => guide(),
                Page::Changelog => changelog(),
            },
        ],
        section![ style!{"grid-row" => "3 / 4"},
            footer()
        ]
    ]
}
```

We can combine Attrs and Style instances using their `merge` methods, which take
an &Attrs and &Style respectively. This can be used to compose styles from reusable parts. 
Example:
```rust
let base_style = !style{"color" => "lavender"};

div![
    h1![ &base_style.merge(&style!{"grid-row" => "1 / 2"}) "First row" ],
    h1![ &base_style.merge(&style!{"grid-row" => "2 / 3"}) "Second row" ],
]
```

Overall: we leverage of Rust's strict type system to flexibly-create the view
using normal Rust code.

**Initializing
Initializing your app

To start your app, pass an instance of your model, the update function, the top-level component function (not its output), and id of the element (Usually a Div or Section) you wish to mount it to to the seed::run function:

#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, view, "main");
}

This must be wrapped in a function named render, with the #[wasm_bindgen] invocation above. (More correctly, its name must match the func in this line in your html file):

function run() {
    render();
}

Note that you don't need to pass your Msg enum; it's inferred from the update function.

### Initializing
To start yoru app, call the `seed::run` function, which takes the following parameters:
- An instance of your model
- Your update function
- Your top-level component function
- The id of the element you wish to mount it to
- Optionally, a HashMap of routings, used to initialize your state based on url (See the Routing section)

This must be wrapped in a function named render, with the #[wasm_bindgen] invocation above. (More correctly, its name must match the func in this line in your html file):
```javascript
function run() {
    render();
}
```

Example:
```rust
#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, view, "main");
}
```

Note that you don't need to pass your Msg enum; it's inferred from the update function.