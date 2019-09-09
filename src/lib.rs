//! The Seed homepage - hosting the guide, and acting as an example. Includes
//! simple interactions, markdown elements, basic routing, and lots of view markup.

mod book;

#[macro_use]
extern crate seed;
use seed::prelude::*;

// Model

#[derive(Copy, Clone, Debug)]
enum Page {
    Guide,
    Changelog,
}

impl ToString for Page {
    fn to_string(&self) -> String {
        // Eg for url routing
        match self {
            Page::Guide => "guide".into(),
            Page::Changelog => "changelog".into(),
        }
    }
}

#[derive(Clone, Debug)]
struct GuideSection {
    title: String,
    content: String,
}

struct Model {
    page: Page,
    guide_page: usize, // Index of our guide sections.
    guide_sections: Vec<GuideSection>,
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        let mut guide_sections = Vec::new();
        let md_texts = vec![
            ("Quickstart", crate::book::quickstart::text()),
            ("Prereqs", crate::book::prereqs::text()),
            ("Structure", crate::book::structure::text()),
            ("Events", crate::book::events::text()),
            ("Components", crate::book::components::text()),
            ("Http requests and state", crate::book::fetch::text()),
            ("Lifecycle hooks", crate::book::lifecycle::text()),
            ("Routing", crate::book::routing::text()),
            ("Misc features", crate::book::misc::text()),
            (
                "Release and debugging",
                crate::book::release_and_debugging::text(),
            ),
//            ("Element deep-dive", crate::book::element_deepdive::text()),
            ("Complex apps", crate::book::complex_apps::text()),
            (
                "Server integration",
                crate::book::server_integration::text(),
            ),
            ("About", crate::book::about::text()),
        ];

        for (title, md_text) in md_texts {
            guide_sections.push(GuideSection {
                title: title.to_string(),
                content: md_text,
            });
        }

        Self {
            page: Page::Guide,
            guide_page: 0,
            guide_sections,
        }
    }
}

#[derive(Clone, Debug)]
enum Msg {
    ChangePage(Page),
    ChangeGuidePage(usize),
}

/// The sole source of updating the model; returns a fresh one.
fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangePage(page) => model.page = page,
        Msg::ChangeGuidePage(guide_page) => {
            model.page = Page::Guide;
            model.guide_page = guide_page;
        }
    }
}

fn header(_version: &str) -> Node<Msg> {
    let link_style = style! {
        "margin-left" => unit!(20, px);
        "margin-right" => unit!(20, px);
        "font-weight" => "bold";
        "font-size" => unit!(1.2, em);
        "color" => "black";
        "cursor" => "pointer";
    };

    header![
        style! {"display" => "flex"; "justify-content" => "flex-end"},
        ul![
            a![&link_style, "Guide", attrs! {At::Href => "/guide"}],
            a![&link_style, "Changelog", attrs! {At::Href => "/changelog"}],
            a![
                &link_style,
                "Repo",
                attrs! {At::Href => "https://github.com/David-OConnor/seed"}
            ],
            a![
                &link_style,
                "Quickstart repo",
                attrs! {At::Href => "https://github.com/David-OConnor/seed-quickstart"}
            ],
            a![
                &link_style,
                "Crate",
                attrs! {At::Href => "https://crates.io/crates/seed"}
            ],
            a![
                &link_style,
                "API docs",
                attrs! {At::Href => "https://docs.rs/seed"}
            ]
        ]
    ]
}

fn title() -> Node<Msg> {
    div![
        style! {
        // todo look up areas
        "display" => "grid";
        "grid-template-rows" => "auto 160px";
        "grid-template-columns" => "1fr 1fr 1fr";
        "text-align" => "center";
        "align-items" => "center";
        },
        div![
            style! {"grid-row" => "1/2"; "grid-column" => "1 / 4"},
            img![
                attrs! {At::Src => "public/seed_logo.svg"; At::Width => 256; At::Alt => "Seed"},
                style! {"margin-top" => unit!(30, px)},
            ],
            h2!["A Rust framework for creating web apps"],
        ],
        div![
            style! {"grid-row" => "2/3"; "grid-column" => "1 / 2"},
            h2!["Expressive view syntax"]
        ],
        div![
            style! {"grid-row" => "2/3"; "grid-column" => "2 / 3"},
            h2!["Compile-time error checking"]
        ],
        div![
            style! {"grid-row" => "2/3"; "grid-column" => "3 / 4"},
            h2!["Clean architecture"]
        ],
    ]
}

fn guide(sections: &[GuideSection], guide_page: usize) -> Node<Msg> {
    let menu_item_style = style! {
        "display" => "flex";  // So we can vertically center
        "align-items" => "center";
        "padding" => unit!(4, px);
        "cursor" => "pointer";
        "height" => unit!(40, px);
        "margin-bottom" => 0;
        "width" => unit!(100, %);
        "color" => "black";
        "font-size" => unit!(1.2, em);
    };

    let menu_items = sections.iter().enumerate().map(|(i, s)| {
        h4![
            &menu_item_style,
            attrs! {
                At::Class => if i == guide_page {"guide-menu-selected"} else {"guide-menu"};
                At::Href => "/guide/".to_string() + &i.to_string()
            },
            s.title
        ]
    });

    div![
        style! {
            "display" => "grid";
            "grid-template-columns" => "200px auto";
            "color" => "black";
            "grid-auto-rows" => "1fr";
            "align-items" => "start";
        },
        div![
            style! {"display" => "flex"; "flex-direction" => "column";
            "grid-column" => "1 / 2";
             "justify-content" => "flex-start";
            "padding" => unit!(10, px);},
            menu_items
        ],
        div![
            class!["guide"],
            style! {
                "display" => "flex";
                "flex-direction" => "column";
                "grid-column" => "2 / 3";
                "padding" => unit!(80, px);
            },
            raw![&sections[guide_page].content],
        ]
    ]
}

fn changelog() -> Node<Msg> {
    let entries = span![ md!(
"
## v0.4.1
- Added more SVG `At` variants
- Added the `St` enum, for style keys; similar to `At`
- Improved ergonomics of `add_child`, `add_attr`, `add_class`,
`add_style`, `replace_text`, and `add_text`, `Node` methods

## v0.4.0
- `ElContainer`, imported in prelude, renamed to `View`. (Breaking)
- Internal refactor of `El`: Now wrapped in `Node`, along with
`Empty` and `Text`. Creation macros return `Node(Element)`. (Breaking)
- Changed the way special attributes like `disabled`, `autofocus`, and
`checked` are handled (Breaking)
- `MessageMapper` now accepts closures
- `Orders` is a trait now instead of a struct. (Breaking)
- Significant changes to MessageMapper
- Orders has new methods, `clone_app` and `msg_mapper` which can allow access to app instance.
- Added more SVG element macros
- Several minor bux fixes
- Examples updated to reflect these changes
- Improvements to Fetch API, especially regarding error handling
and deserialization

## v0.3.7
- `routes` now accepts `Url` instead of `&Url` (Breaking)
- Improvements to fetch API
- Added `raw!`, `md!`, and `plain!` macros that alias `El::from_html`, `El::from_markdown`,
and `El::new_text` respectively
- `Attrs!` and `Style!` macros can now use commas and whitespace as separators,
in addition to semicolons
- Fixed typos in a few attributes (Breaking)
- Fixed a bug where an HTML namespace was applied to raw html/markdown elements
- New conditional syntax added in `class!` macro, similar to `Elm`'s `classList`
- `Listener` now implements `MessageMapper`
- `El methods` `add_child`, `add_style`, `add_attr`, and `set_text` now return the elements,
allowing chaining
- Fixed a bug with `set_text`. Renamed to `replace_text`. Added `add_text`, which adds
a text node, but doesn't remove existing ones. Added `add_class`. (Breaking)

## v0.3.6
- Fetch module and API heavily changed (breaking)
- Added support for `request​Animation​Frame`, which improves render performance,
especially for animations
- Styles no longer implicitly add `px`. Added `unit!` macro in its place
- `Map` can now be used directly in elements, without needing to annotate type and collect
(ie for child `Elements`, and `Listener`s)
- Fixed a bug where `empty` elements at the top-level were rendering in the wrong order
- Added an `empty!` macro, which is similar to `seed::empty`
- Attributes and style now retain order

## v0.3.5
- Fixed a bug where view functions returning `Vec<El>` weren't rendering properly
- Fixed a typo with the `viewBox` attribute

## v0.3.4
- The `update` fn now accepts a (new) `Orders` struct, and returns nothing. Renders occur implicitly,
with the option to skip rendering, update with an additional message, or perform an asynchronous
action. (Breaking)
- `.mount()` now accepts elements. Deprecated `.mount_el()`
- The `log` function and macro now support items which implement `Debug`
- Removed deprecated `routing::push_path` function (breaking)

## v0.3.3
- Added `seed::update` function, which allows custom events, and updates from JS.

## v0.3.2
- Top level view functions can now return `Vec<El<Ms>>`, `El<Ms>`, or something else implementing
the new ElContainer trait

## v0.3.1
- Top level view functions now return `Vec<El<Ms>>` instead of `El<Ms>`, mounted directly to
 the mount point. (Breaking)
- `push_route()` can now accept a `Vec<&str>`, depreciating `push_path()`
- Fixed a bug where window events couldn't be enabled on initialization

## v0.3.0
- `update` function now takes a mutable ref of the model. (Breaking)
- `Update` (update's return type) is now a struct. (Breaking)
- Async, etc events are now handled through messages, instead of passing `App`
through the view func. (breaking)
- Fixed some bugs with empty elements
- Internal code cleanup
- Added commented-out release command to example build files
- Added more tests

## v0.2.10
- Routing can be triggered by clicking any element containing a `Href` attribute
with value as a relative link
- Internal links no longer trigger a page refresh
- Models no longer need to implement `Clone`
- Fixed a bug introduced in 0.2.9 for `select` elements

## v0.2.9
- Added a `RenderThen` option to `Update`, which allows chaining update messages
- Added a `.model` method to `Update`, allowing for cleaner recursion in updates
- Improved controlled-comonent (sync fields with model) logic

## v0.2.8
- Reflowed `El::from_html` and `El::from_markdown` to return `Vec`s of `El`s, instead of wrapping
them in a single span.
- Added `set_timeout` wrapper
- Improved support for SVG and namespaces

## v0.2.7
- Fixed a bug where `line!` macro interfered with builtin
- Fixed a bug with routing search (ie `?`)

## v0.2.6
- Fixed a bug where children would render out-of-order
- Improved vdom diffing logic

## v0.2.5
- Attributes and Events now can use `At` and `Ev` enums
- Routing overhauled; modelled after react-reason. Cleaner syntax, and more flexible
- Input, Textarea, and Select elements are now \"controlled\" - they always
stay in sync with the model.
- index.html file updated in examples and quickstart to use relative paths,
which fixes landing-page routing

## v0.2.4
- Changed render func to use a new pattern (Breaking)
- Default mount point added: \"app\" for element id
- View func now takes a ref to the model instead of the model itself
- Routing refactored; now works dynamically
- Update function now returns an enum that returns Render or Skip,
to allow conditional rendering (Breaking)
- Elements can now store more than 1 text node

## V0.2.3
- Fixed a bug where initially-empty text won't update
- Added more tests
- Exposed web_sys Document and Window in top level of Seed create, with .expect
- Modified build scripts to keep the wasm output name fixed at 'package', simplifying example/quickstart renames
- Tests now work in Windows due to update in wasm-pack

## V0.2.2
- Overhaul of fetch module
- Added server-integration example

## V0.2.1
- Added support for custom tags
- Added `class!` and `id!` convenience macros for setting style

## v0.2.0

- Added high-level fetch api
- Added routing
- Added element lifecycles (did_mount, did_update, will_unmount)
- Added support for updating state outside events
- Added server_interaction, and homepage (this site) examples

## v0.1.0

- Initial release

"
    ), style! {
       "grid-column" => "2 / 3";
    } ];

    div![
        class!["guide"],
        style! {
            "display" => "grid";
            "grid-template-columns" => "1fr 2fr 1fr";

            "padding" => unit!(50, px);
            "color" => "black";
        },
        entries,
    ]
}

fn footer() -> Node<Msg> {
    footer![
        style! {"display" => "flex"; "justify-content" => "center"},
        h4!["© 2019 David O'Connor"]
    ]
}

fn view(model: &Model) -> Node<Msg> {
    let version = "0.3.1";
    div![
        style! {
            "display" => "flex";
            "flex-direction" => "column";
        },
        section![header(version)],
        section![title()],
        section![match model.page {
            Page::Guide => guide(&model.guide_sections, model.guide_page),
            Page::Changelog => changelog(),
        }],
        section![footer()],
    ]
}

#[allow(clippy::needless_pass_by_value)]
fn routes(url: seed::Url) -> Msg {
    match url.path.get(0).map(String::as_str) {
        Some("guide") => match url.path.get(1).as_ref() {
            Some(page) => Msg::ChangeGuidePage(page.parse::<usize>().unwrap()),
            None => Msg::ChangePage(Page::Guide),
        },
        Some("changelog") => Msg::ChangePage(Page::Changelog),
        _ => Msg::ChangePage(Page::Guide),
    }
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(|_, _| Model::default(), update, view)
        .routes(routes)
        .finish()
        .run();
}
