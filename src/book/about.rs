pub fn text() -> String {
r#"
<h1 id="about">About</h1>
<h2 id="reference">Reference</h2>
<ul>
<li><a href="https://rustwasm.github.io/wasm-bindgen/introduction.html">wasm-bindgen guide</a></li>
<li><a href="https://developer.mozilla.org/en-US/">Mozilla MDN web docs</a></li>
<li><a href="https://rustwasm.github.io/wasm-bindgen/api/web_sys/">web-sys api</a> (A good partner for the MDN docs - most DOM items have web-sys equivalents used internally)</li>
<li><a href="https://doc.rust-lang.org/book/index.html">Rust book</a></li>
<li><a href="https://doc.rust-lang.org/std/">Rust standard library api</a></li>
<li><a href="https://docs.rs/seed">Seed's API docs</a></li>
<li><a href="https://www.rust-lang.org/learn">Learn Rust</a></li>
</ul>
<h2 id="goals">Goals</h2>
<ul>
<li><p>Learning the syntax, creating a project, and building it should be easy - regardless of your familiarity with Rust.</p></li>
<li><p>Complete documentation that always matches the current version. Getting examples working, and starting a project should be painless, and require nothing beyond this guide.</p></li>
<li><p>Concise, flexibilty vew syntax that's easy to read and write.</p></li>
</ul>
<h2 id="a-note-on-view-syntax">A note on view syntax</h2>
<p>This project takes a different approach to describing how to display DOM elements than others. It neither uses completely natural (ie macro-free) Rust code, nor an HTML-like abstraction (eg JSX or templates). My intent is to make the code close to natural Rust, while streamlining the syntax in a way suited for creating a visual layout with minimal repetition. The macros used here are thin wrappers for constructors, and don't conceal much. Specifically, the element-creation macros allow for accepting a variable number of arguments, and the attrs/style marcros are essentially HashMap literals, with wrappers that let el macros know to distinguish them.</p>
<p>The relative lack of resemblance to HTML be offputting at first, but the learning curve is shallow, and I think the macro syntax used to create elements, attributes etc is close-enough to normal Rust syntax that it's easy to reason about how the code should come together, without compartmentalizing it into logic code and display code. This lack of separation in particlar is a subjective, controversial decision, but I think the benefits are worth it.</p>
<h2 id="where-to-start-if-youre-familiar-with-existing-frontend-frameworks">Where to start if you're familiar with existing frontend frameworks</h2>
<p>The <a href="https://github.com/David-OConnor/seed/tree/master/examples/todomvc">todomvc example</a> is an implementation of the <a href="http://todomvc.com/">TodoMVC project</a>, which has example code in my frameworks that do the same thing. Compare the example in this project to one on that page that uses a framework you're familiar with.</p>
<h2 id="suggestions-critique-submit-an-issue-or-pull-request-on-github">Suggestions? Critique? Submit an issue or pull request on Github</h2>
<h2 id="influences">Influences</h2>
<p>This project is strongly influenced by Elm, React, and Redux. The overall layout of Seed apps mimicks that of The Elm Architecture.</p>
<h2 id="why-another-entry-in-a-saturated-field">Why another entry in a saturated field?</h2>
<h3 id="there-are-already-several-rustwasm-frameworks-why-add-another">There are already several Rust/WASM frameworks; why add another?</h3>
<p>My goal is for this to be easy to pick up from looking at a tutorial or documentation, regardless of your level of experience with Rust. I'm distinguising this package through clear examples and documentation (see goals above), and using <code>wasm-bindgen</code> internally. I started this project after being unable to get existing frameworks to work due to lack of documented examples, and inconsistency between documentation and published versions. My intent is for anyone who's proficient in a frontend framework to get a standalone app working in the browser within a few minutes, using just the quickstart guide.</p>
<p>Seed approaches HTML-display syntax differently from existing packages: rather than use an HTML-like markup similar to JSX, it uses Rust builtin types, thinly-wrapped by a macro for each DOM element. This decision may not appeal to everyone, but I think it integrates more naturally with the language.</p>
<h3 id="why-build-a-frontend-in-rust-over-elm-or-javascript-based-frameworks">Why build a frontend in Rust over Elm or Javascript-based frameworks?</h3>
<p>You may prefer writing in Rust, and using packages from Cargo vis npm. Getting started with this framework will, in most cases be faster, and require less config and setup overhead than with JS frameworks. You like the advantages of compile-time error-checking.</p>
<p>You may choose this approach over Elm if you're already comfortable with Rust, want the performance benefits, or don't want to code business logic in a purely-functional langauge.</p>
<p>Compared with React, you may appreciate the consistency of how to write apps: There's no distinction between logic and display code; no restrictions on comments; no distinction between components and normal functions. The API is flexible, and avoids OOP boilerplate.</p>
<p>I also hope that config, building, and dependency-management is cleaner with Cargo and wasm-bindgen than with npm.</p>
<h3 id="shoutouts">Shoutouts</h3>
<ul>
<li>The <a href="https://github.com/rustwasm/wasm-bindgen">WASM-Bindgen</a> team: For building the tools this project relies on</li>
<li>Alex Chrichton, for being extraodinarily helpful in the Rust / WASM community</li>
<li>The <a href="https://elm-lang.org/">Elm</a> team: For creating and standardizing the Elm architecture</li>
<li>Denis Kolodin: for creating the inspirational <a href="https://github.com/DenisKolodin/yew">Yew framework</a></li>
<li>Utkarsh Kukreti, for through his <a href="https://github.com/utkarshkukreti/draco">Draco repo</a>, helping me understand how wasm-bindgen's closure system can be used to update state.</li>
<li>Tim Robinson, for being very helpful on the <a href="https://gitter.im/rust-lang/rust">Rust Gitter</a>.</li>
</ul>
<h3 id="features-to-add">Features to add</h3>
<ul>
<li>Router</li>
<li>High-level fetch API</li>
<li>Composable or mergable styles and attrs</li>
<li>Virtual DOM optimization</li>
<li>Docs/tutorial website example to replace this readme</li>
<li>High-level CSS-grid/Flexbox API ?</li>
</ul>
<p>### Bugs to fix - Text renders above children instead of below</p>
"#.into()
}