//! The Seed homepage - hosting the guide, and acting as an example. Includes
//! simple interactions, markdown elements, basic routing, and lots of view markup.

mod book;

#[macro_use]
extern crate seed;
use seed::prelude::*;


// Model

#[derive(Copy, Clone)]
enum Page {
    Guide,
    Changelog
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

#[derive(Clone)]
struct GuideSection {
    title: String,
    elements: Vec<El<Msg>>
}

struct Model {
    page: Page,
    guide_page: usize,  // Index of our guide sections.
    guide_sections: Vec<GuideSection>,
}

//fn p<Ms>(e: &El<Ms>) {
//    log!(e.tag.as_str(), e.get_text());
//    for c in &e.children {
//        p(c);
//    }
//}

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
            ("Release and debugging", crate::book::release_and_debugging::text()),
            ("Element deep-dive", crate::book::element_deepdive::text()),
            ("Server integration", crate::book::server_integration::text()),
            ("About", crate::book::about::text()),
        ];

        for (title, md_text) in md_texts {
            let elements = El::from_markdown(&md_text);
//
//            for e in &elements {
//                p(e);
//            }
            guide_sections.push(GuideSection{title: title.to_string(), elements});
        }

        Self {
            page: Page::Guide,
            guide_page: 0,
            guide_sections,
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    ChangePage(Page),
    ChangeGuidePage(usize),
}

/// The sole source of updating the model; returns a fresh one.
fn update(msg: Msg, model: &mut Model) -> Update<Msg> {
    match msg {
        Msg::ChangePage(page) => model.page = page,
        Msg::ChangeGuidePage(guide_page) => {
            model.page = Page::Guide;
            model.guide_page = guide_page;
        },
    }
    Render.into()
}

// View

fn header(_version: &str) -> El<Msg> {
    let link_style = style!{
        "margin-left" => 20;
        "margin-right" => 20;
        "font-weight" => "bold";
        "font-size" => "1.2em";
        "color" => "black";
        "cursor" => "pointer";
    };

    header![ style!{"display" => "flex"; "justify-content" => "flex-end"},
        ul![
            a![ &link_style, "Guide", attrs!{At::Href => "/guide"} ],
            a![ &link_style, "Changelog", attrs!{At::Href => "/changelog"} ],

            a![ &link_style, "Repo", attrs!{At::Href => "https://github.com/David-OConnor/seed"} ],
            a![ &link_style, "Quickstart repo", attrs!{At::Href => "https://github.com/David-OConnor/seed-quickstart"} ],
            a![ &link_style, "Crate", attrs!{At::Href => "https://crates.io/crates/seed"} ],
            a![ &link_style, "API docs", attrs!{At::Href => "https://docs.rs/seed"} ]
        ]
    ]
}

fn title() -> El<Msg> {
    div![ style!{
            // todo look up areas
            "display" => "grid";
            "grid-template-rows" => "auto 160px";
            "grid-template-columns" => "1fr 1fr 1fr";
            "text-align" => "center";
            "align-items" => "center";
            },
        div![ style!{"grid-row" => "1/2"; "grid-column" => "1 / 4"},
            h1![ style!{"font-size" => "2em"},"Seed" ],
            h2![ "A Rust framework for creating web apps" ],
        ],
        div![  style!{"grid-row" => "2/3"; "grid-column" => "1 / 2"},
            h2![ "Expressive view syntax"]
        ],
        div![  style!{"grid-row" => "2/3"; "grid-column" => "2 / 3"},
            h2![ "Compile-time error checking" ]
        ],
        div![  style!{"grid-row" => "2/3"; "grid-column" => "3 / 4"},
            h2![ "Clean architecture" ]
        ],

    ]
}

fn guide(sections: &[GuideSection], guide_page: usize) -> El<Msg> {
    let menu_item_style = style!{
        "display" => "flex";  // So we can vertically center
        "align-items" => "center";
        "padding" => 4;
        "cursor" => "pointer";
        "height" => 40;
        "margin-bottom" => 0;
        "width" => "100%";
        "color" => "black";
        "font-size" => "1.2em";
    };

    let menu_items: Vec<El<Msg>> = sections
        .iter()
        .enumerate()
        .map(|(i, s)|
        h4![
            &menu_item_style,
            attrs!{
                At::Class => if i == guide_page {"guide-menu-selected"} else {"guide-menu"};
                At::Href => "/guide/".to_string() + &i.to_string()
            },
            s.title
        ]
    ).collect();

    div![ style! {
        "display" => "grid";
        "grid-template-columns" => "200px auto";
        "color" => "black";
        "grid-auto-rows" => "1fr";
        "align-items" => "start";
    },
        div![ style!{"display" => "flex"; "flex-direction" => "column";
                     "grid-column" => "1 / 2";
                      "justify-content" => "flex-start";
                     "padding" => 10;},
            menu_items
        ],

        div![
            class!["guide"],
            style!{
                "display" => "flex";
                "flex-direction" => "column";
                "grid-column" => "2 / 3";
                "padding" => 80;
            },
            sections[guide_page].clone().elements
        ]
    ]
}

fn changelog() -> El<Msg> {
    let mut entries = span![ El::from_markdown(
"
## v0.3.1
- `Top level view functions now return `Vec<El<Ms>>` instead of `El<Ms>`. (Breaking)
- `push_route()` now can accept a `Vec<&str>`, depreciating `push_path()`.

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
- Routing overhauled; modelled after react-reason. Cleaner syntax, and more flexible.
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
- Elements can now store more than 1 text node.

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
    ) ];
    entries.style = style!{
        "grid-column" => "2 / 3";
     };

    div![
        class!["guide"],
        style!{
            "display" => "grid";
            "grid-template-columns" => "1fr 2fr 1fr";

            "padding" => 50;
            "color" => "black";
        },
        entries,
    ]
}

fn footer() -> El<Msg> {
    footer![ style!{"display" => "flex"; "justify-content" => "center"},
        h4![ "© 2019 David O'Connor"]
    ]
}



fn view(model: &Model) -> Vec<El<Msg>> {
    let version = "0.3.1";

    vec![
        div![
            style!{
                "display" => "flex";
                "flex-direction" => "column";
            },

            section![
                header(version)
            ],
            section![
                title()
            ],
            section![
                match model.page {
                    Page::Guide => guide(&model.guide_sections, model.guide_page),
                    Page::Changelog => changelog(),
                }
            ],
            section![
                footer()
            ],
        ]
    ]
}

fn routes(url: &seed::Url) -> Msg {
    if url.path.is_empty() {
        return Msg::ChangePage(Page::Guide)
    }

    match url.path[0].as_ref() {
        "guide" => {
            match url.path.get(1).as_ref() {
                Some(page) => Msg::ChangeGuidePage(page.parse::<usize>().unwrap()),
                None => Msg::ChangePage(Page::Guide)
            }
        },
        "changelog" => Msg::ChangePage(Page::Changelog),
        _ => Msg::ChangePage(Page::Guide),
    }
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .routes(routes)
        .finish()
        .run();
}