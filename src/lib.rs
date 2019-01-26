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

impl Page {
    fn to_string(self) -> String {
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
    element: El<Msg>
}

#[derive(Clone)]
struct Model {
    page: Page,
    guide_page: usize,  // Index of our guide sections.
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
            ("Http requests", crate::book::fetch::text()),
            ("Lifecycle hooks", crate::book::lifecycle::text()),
            ("Routing", crate::book::routing::text()),
            ("Misc features", crate::book::misc::text()),
            ("Release and debugging", crate::book::release_and_debugging::text()),
            ("Element deep-dive", crate::book::element_deepdive::text()),
            ("Server integration", crate::book::server_integration::text()),
            ("About", crate::book::about::text()),
        ];

        for (title, md_text) in md_texts {
            let element = El::from_markdown(&md_text);
            guide_sections.push(GuideSection{title: title.to_string(), element});
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
    ChangePage(seed::App<Msg, Model>, Page),
    ChangeGuidePage(seed::App<Msg, Model>, usize),
    RoutePage(Page),
    RouteGuidePage(usize),
}

/// The sole source of updating the model; returns a fresh one.
fn update(msg: Msg, model: Model) -> Update<Model> {
    match msg {
        Msg::ChangePage(state, page) => {
            Render(seed::push_route(state, &page.to_string(), Msg::RoutePage(page)))
        },
        Msg::ChangeGuidePage(state, guide_page) => {
            Render(seed::push_route(state, &format!("guide/{}", guide_page), Msg::RouteGuidePage(guide_page)))
        },

        // This is separate, because nagivating the route triggers state updates, which would
        // trigger an additional push state.
        Msg::RoutePage(page) => Render(Model {page, ..model}),
        Msg::RouteGuidePage(guide_page) => Render(Model {guide_page, page: Page::Guide, ..model}),
    }
}


// View

fn header(state: seed::App<Msg, Model>, _version: &str) -> El<Msg> {
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
            a![ &link_style, "Guide", simple_ev("click", Msg::ChangePage(state.clone(), Page::Guide)) ],
            a![ &link_style, "Changelog", simple_ev("click", Msg::ChangePage(state, Page::Changelog)) ],
            a![ &link_style, "Repo", attrs!{"href" => "https://github.com/David-OConnor/seed"} ],
            a![ &link_style, "Quickstart repo", attrs!{"href" => "https://github.com/David-OConnor/seed-quickstart"} ],
            a![ &link_style, "Crate", attrs!{"href" => "https://crates.io/crates/seed"} ],
            a![ &link_style, "API docs", attrs!{"href" => "https://docs.rs/seed"} ]
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

fn guide(state: seed::App<Msg, Model>, sections: &[GuideSection], guide_page: usize) -> El<Msg> {
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
            &attrs!{"class" => if i == guide_page {"guide-menu-selected"} else {"guide-menu"}},
            simple_ev("click", Msg::ChangeGuidePage(state.clone(), i)),
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

        div![ attrs!{"class" => "guide"},
            style!{"display" => "flex"; "grid-column" => "2 / 3";
            "padding" => 80;},
            sections[guide_page].clone().element
        ]
    ]
}

fn changelog() -> El<Msg> {
    let mut entries = El::from_markdown(
"
## v0.3.0
- Changed render func to use a new pattern (Breaking)
- Default mount point added: \"app\" for element id
- View func now takes a ref to the model instead of the model itself
- Routing refactored; now works dynamically
- Update function now returns an enum that returns Render or Skip,
to allow conditional rendering (Breaking)
-Elements can now store more than 1 text node.

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
    );
    entries.style = style!{
        "grid-column" => "2 / 3";
        "display" => "flex";
        "flex-direction" => "column";
//        "align-content" => "center";
     };

    div![
        attrs!{"class" => "guide"},
        style!{
            "display" => "grid";
            "grid-template-columns" => "1fr 2fr 1fr";

            "padding" => 50;
            "color" => "black";
        },
        entries
    ]
}

fn footer() -> El<Msg> {
    footer![ style!{"display" => "flex"; "justify-content" => "center"},
        h4![ "© 2019 David O'Connor"]
    ]
}



fn view(state: seed::App<Msg, Model>, model: &Model) -> El<Msg> {
    let version = "0.2.23";


        let mut r = state.clone().data.routes.borrow_mut().clone();
//    let mut inner = r.take().expect("Problem taking contents of routes");

//    r.insert("MOOSE IS LOOSE".to_string(), message);
//    r.insert(String::from("/") + path, message);

//    r.replace(inner);
    log!(format!("{:?}", r.keys()));


    div![
        style!{
            "display" => "flex";
            "flex-direction" => "column";
        },

        section![
            header(state.clone(), version)
        ],
        section![
            title()
        ],
        section![
            match model.page {
                Page::Guide => guide(state, &model.guide_sections, model.guide_page),
                Page::Changelog => changelog(),
            }
        ],
        section![
            footer()
        ]
    ]
}


#[wasm_bindgen]
pub fn render() {
    let mut routes = routes!{
        "guide" => Msg::RoutePage(Page::Guide),
        "changelog" => Msg::RoutePage(Page::Changelog),
    };

    for guide_page in 0..12 {
        routes.insert(
            "guide/".to_string() + &guide_page.to_string(),
            Msg::RouteGuidePage(guide_page)
        );
    }

//    let mut routes2 = HashMap::new();
//    routes2.insert(("guide"), Msg::RoutePage(Page::Guide));
//    routes2.insert(("changelog"), Msg::RoutePage(Page::Changelog));
//    routes2.insert(("guide", u32), Msg::RouteGuidePage(page));

//    enum Routes3 {
//        Guide,
//        ChangeLog,
//        GuidePage(guide_page)
//    }

    seed::App::build(Model::default(), update, view)
        .routes(routes)
        .finish()
        .run();
}