pub fn text() -> String {
r#"
<h1 id="prerequisites">Prerequisites</h1>
<h2 id="rust">Rust</h2>
<p>Proficiency in Rust isn't required to get started using this framework. It helps, but I think you'll be able to build a usable webapp using this guide, and example code alone. For business logic behind the GUI, more study may be required. The official <a href="https://doc.rust-lang.org/book/index.html">Rust Book</a> is a good place to start.</p>
<p>You should be able to get by with just the basic Rust syntax common to most programming languages, eg conditionals, equalities, iteration, collections - and how Rust's borrow system applies to strings. A skim through the first few chapters of the Book, and the examples here should provide what you need. Rust's advanced and specialized features like lifetimes, generics, smartpointers, and traits aren't required to build an interactive GUI. If you run into a syntax or borrowing/ownership problem, the compiler will often recommend a fix.</p>
<h2 id="web-fundamentals">Web fundamentals</h2>
<p>Experience building websites using HTML/CSS or other frameworks is required. Neither this guide nor the API docs describes how web pages are structured, or what different HTML/DOM elements, attributes, styles etc do. You'll need to know these before getting started. Seed provides tools used to assemble and manipulate these fundamentals. Mozilla's <a href="https://developer.mozilla.org/en-US/docs/Learn">MDN web docs</a> is a good place to start.</p>
<h2 id="other-frontend-frameworks">Other frontend frameworks</h2>
<p>The design principles Seed uses are similar to those used by React, Elm, and Yew. People familiar with how to set up interactive web pages using these tools will likely have an easy time learning this.</p>
"#.into()
}