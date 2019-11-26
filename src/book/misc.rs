pub fn text() -> String {
r#####"
<h1 id="misc-features">Misc features</h1>
<h2 id="logging-in-the-web-browser">Logging in the web browser</h2>
<p>To output to the web browser’s console (ie <code>console.log()</code> in JS), use the <code>log!</code>, which is imported in the seed prelude: <code>log!("On the shoulders of", 5, "giants".to_string())</code>. You can use the <code>error!</code> macro in a similar way, equivalent to JS’s <code>console.error()</code>.</p>
<h2 id="custom-tags">Custom tags</h2>
<p>Seed generally retricts the element tags allowed by using Enums for the tags, and a predefined set of element-creation macros. If you wish to use a custom tag, you can use using <code>Tag::from</code> (<code>El</code> and <code>Tag</code> are exposed in the prelude), either with the <code>El::empty</code> constructor, or using the <code>custom!</code> element-construction macro, where we pass our custom tag as an argument:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb1-1"><a href="#cb1-1"></a><span class="kw">let</span> <span class="kw">mut</span> custom_el = <span class="pp">El::</span>empty(<span class="pp">Tag::</span>Custom(<span class="st">&quot;mytag&quot;</span>.to_string()));</span>
<span id="cb1-2"><a href="#cb1-2"></a>custom_el.set_text(<span class="st">&quot;Words&quot;</span>);</span>
<span id="cb1-3"><a href="#cb1-3"></a></span>
<span id="cb1-4"><a href="#cb1-4"></a><span class="pp">custom!</span><span class="op">[</span> <span class="pp">Tag::</span>from(<span class="st">&quot;anothertag&quot;</span>.into())</span>
<span id="cb1-5"><a href="#cb1-5"></a>    custom_el,</span>
<span id="cb1-6"><a href="#cb1-6"></a><span class="op">]</span></span></code></pre></div>
<p>An example is provided as part of the <a href="https://github.com/David-OConnor/seed/tree/master/examples/todomvc">window_events</a> example.</p>
<h2 id="local-storage">Local storage</h2>
<p>You can store page state locally using web_sys’s <a href="https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Storage.html">Storage struct</a></p>
<p>Seed provides convenience functions <code>seed::storage::get_storage</code>, which returns the <code>web_sys::storage</code> object, and <code>seed::storage::store_data</code> to store an arbitrary Rust data structure that implements serde’s Serialize. Example use:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb2-1"><a href="#cb2-1"></a><span class="kw">extern</span> <span class="kw">crate</span> serde;</span>
<span id="cb2-2"><a href="#cb2-2"></a><span class="at">#[</span>macro_use<span class="at">]</span></span>
<span id="cb2-3"><a href="#cb2-3"></a><span class="kw">extern</span> <span class="kw">crate</span> serde_derive;</span>
<span id="cb2-4"><a href="#cb2-4"></a><span class="kw">extern</span> <span class="kw">crate</span> serde_json;</span>
<span id="cb2-5"><a href="#cb2-5"></a></span>
<span id="cb2-6"><a href="#cb2-6"></a><span class="co">// ...</span></span>
<span id="cb2-7"><a href="#cb2-7"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="at">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb2-8"><a href="#cb2-8"></a><span class="kw">struct</span> Data <span class="op">{</span></span>
<span id="cb2-9"><a href="#cb2-9"></a>    <span class="co">// Arbitrary data (All sub-structs etc must also implement Serialize and Deserialize)</span></span>
<span id="cb2-10"><a href="#cb2-10"></a><span class="op">}</span></span>
<span id="cb2-11"><a href="#cb2-11"></a></span>
<span id="cb2-12"><a href="#cb2-12"></a><span class="kw">let</span> storage = <span class="pp">seed::storage::</span>get_storage();</span>
<span id="cb2-13"><a href="#cb2-13"></a><span class="pp">seed::storage::</span>store(storage, <span class="st">&quot;my-data&quot;</span>, <span class="pp">Data::</span>new());</span>
<span id="cb2-14"><a href="#cb2-14"></a></span>
<span id="cb2-15"><a href="#cb2-15"></a><span class="co">// ...</span></span>
<span id="cb2-16"><a href="#cb2-16"></a></span>
<span id="cb2-17"><a href="#cb2-17"></a><span class="kw">let</span> loaded_serialized = storage.get_item(<span class="st">&quot;my-data&quot;</span>).unwrap().unwrap();</span>
<span id="cb2-18"><a href="#cb2-18"></a><span class="kw">let</span> data = <span class="pp">serde_json::</span>from_str(&amp;loaded_serialized).unwrap();</span></code></pre></div>
<h2 id="display-markdown-and-raw-html">Display markdown and raw HTML</h2>
<p>Seed supports creating elements from markdown text, using <a href="https://github.com/raphlinus/pulldown-cmark">pulldown-cmark</a> internally. Use the <a href="https://docs.rs/seed/0.4.2/seed/dom_types/struct.Node.html#method.from_markdown">Node::from_markdown()</a> method to create an element that accepts a markdown &amp;str as its only parameter, and displays it normally as html. Note that it does not support syntax highlighting. You can render raw HTML with <code>El::from_html(html)</code>, where <code>html</code> is a &amp;str of HTML. You can also use the <code>raw!</code> and <code>md!</code> macros for <code>from_html</code> and <code>from_markdown</code> respectively.</p>
<p>Example:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb3-1"><a href="#cb3-1"></a><span class="kw">fn</span> view(model: &amp;Model) -&gt; <span class="dt">Vec</span>&lt;Node&lt;Msg&gt;&gt; <span class="op">{</span></span>
<span id="cb3-2"><a href="#cb3-2"></a></span>
<span id="cb3-3"><a href="#cb3-3"></a>    <span class="kw">let</span> markdown = </span>
<span id="cb3-4"><a href="#cb3-4"></a><span class="st">&quot;</span></span>
<span id="cb3-5"><a href="#cb3-5"></a><span class="st">## Hello world</span></span>
<span id="cb3-6"><a href="#cb3-6"></a></span>
<span id="cb3-7"><a href="#cb3-7"></a><span class="st">Let&#39;s set the existence-of-God issue aside for a later volume,</span></span>
<span id="cb3-8"><a href="#cb3-8"></a><span class="st">and just [learn to code](https://play.rust-lang.org/).</span></span>
<span id="cb3-9"><a href="#cb3-9"></a><span class="st">&quot;</span>;</span>
<span id="cb3-10"><a href="#cb3-10"></a></span>
<span id="cb3-11"><a href="#cb3-11"></a>    <span class="kw">let</span> html = </span>
<span id="cb3-12"><a href="#cb3-12"></a><span class="st">&quot;</span></span>
<span id="cb3-13"><a href="#cb3-13"></a><span class="st">&lt;div&gt;</span></span>
<span id="cb3-14"><a href="#cb3-14"></a><span class="st">    &lt;p&gt;It is a truth universally acknowledged, that a single man in </span></span>
<span id="cb3-15"><a href="#cb3-15"></a><span class="st">    possession of a good fortune, must be in want of a good time.&lt;/p&gt;</span></span>
<span id="cb3-16"><a href="#cb3-16"></a><span class="st">&lt;/div&gt;</span></span>
<span id="cb3-17"><a href="#cb3-17"></a><span class="st">&quot;</span>;</span>
<span id="cb3-18"><a href="#cb3-18"></a>    </span>
<span id="cb3-19"><a href="#cb3-19"></a>    <span class="pp">vec!</span><span class="op">[</span></span>
<span id="cb3-20"><a href="#cb3-20"></a>        <span class="pp">Node::</span>from_markdown(markdown)   <span class="co">// or md!(markdown)</span></span>
<span id="cb3-21"><a href="#cb3-21"></a>        <span class="pp">Node::</span>from_html(html)  <span class="co">// or raw!(html)</span></span>
<span id="cb3-22"><a href="#cb3-22"></a>    <span class="op">]</span></span>
<span id="cb3-23"><a href="#cb3-23"></a><span class="op">}</span></span></code></pre></div>
<p>This works for SVG as well:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb4-1"><a href="#cb4-1"></a><span class="kw">fn</span> view(model: &amp;Model) -&gt; Impl View&lt;Msg&gt; <span class="op">{</span></span>
<span id="cb4-2"><a href="#cb4-2"></a>    <span class="pp">Node::</span>from_html(</span>
<span id="cb4-3"><a href="#cb4-3"></a><span class="st">r#&quot;</span></span>
<span id="cb4-4"><a href="#cb4-4"></a><span class="st">&lt;svg xmlns=&quot;http://www.w3.org/2000/svg&quot;&gt;</span></span>
<span id="cb4-5"><a href="#cb4-5"></a><span class="st">    &lt;rect x=&quot;5&quot; y=&quot;5&quot; width=&quot;20&quot; height=&quot;20&quot;&gt;&lt;/rect&gt;</span></span>
<span id="cb4-6"><a href="#cb4-6"></a><span class="st">&lt;/svg&gt;</span></span>
<span id="cb4-7"><a href="#cb4-7"></a><span class="st">&quot;#</span>)</span>
<span id="cb4-8"><a href="#cb4-8"></a><span class="op">}</span></span></code></pre></div>
<h2 id="using-web_sys-to-view-element-data.">Using <code>web_sys</code> to view element data.</h2>
<p><code>web_sys</code>, which Seed uses internally, can be used to view information about elements. For example:</p>
<div class="sourceCode" id="cb5"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb5-1"><a href="#cb5-1"></a><span class="kw">fn</span> get_height(id: &amp;<span class="dt">str</span>) -&gt; <span class="dt">i32</span> <span class="op">{</span>    </span>
<span id="cb5-2"><a href="#cb5-2"></a>    <span class="kw">let</span> html_el = <span class="pp">seed::</span>document().get_element_by_id(<span class="st">&quot;my_el&quot;</span>).unwrap();</span>
<span id="cb5-3"><a href="#cb5-3"></a>    <span class="kw">let</span> h = html_el.client_height();    </span>
<span id="cb5-4"><a href="#cb5-4"></a>    <span class="pp">log!</span>(<span class="st">&quot;Height {:?}&quot;</span>, h); </span>
<span id="cb5-5"><a href="#cb5-5"></a>    h   </span>
<span id="cb5-6"><a href="#cb5-6"></a><span class="op">}</span></span></code></pre></div>
<p>Where we’ve given the element we wish to query id <code>my_el</code>.</p>
<h2 id="some-convenience-functions">Some convenience functions</h2>
<p>You can use <code>seed::document()</code> and <code>seed::window()</code> to access the <code>web_sys</code> document and window functions. Example:</p>
<div class="sourceCode" id="cb6"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb6-1"><a href="#cb6-1"></a><span class="kw">fn</span> view(model: &amp;Model) -&gt; <span class="dt">Vec</span>&lt;Node&lt;Msg&gt;&gt; <span class="op">{</span></span>
<span id="cb6-2"><a href="#cb6-2"></a>    <span class="pp">vec!</span><span class="op">[</span></span>
<span id="cb6-3"><a href="#cb6-3"></a>        <span class="pp">button!</span><span class="op">[</span> </span>
<span id="cb6-4"><a href="#cb6-4"></a>            simple_ev(<span class="st">&quot;click&quot;</span>, <span class="pp">Msg::</span>Increment), </span>
<span id="cb6-5"><a href="#cb6-5"></a>            <span class="pp">format!</span>(<span class="st">&quot;Hello, World × {}&quot;</span>, model.val),</span>
<span id="cb6-6"><a href="#cb6-6"></a>            did_mount(|_| <span class="op">{</span></span>
<span id="cb6-7"><a href="#cb6-7"></a>                <span class="pp">seed::</span>document().set_title(<span class="st">&quot;New title&quot;</span>)</span>
<span id="cb6-8"><a href="#cb6-8"></a>            <span class="op">}</span>)</span>
<span id="cb6-9"><a href="#cb6-9"></a>        <span class="op">]</span></span>
<span id="cb6-10"><a href="#cb6-10"></a>    <span class="op">]</span></span>
<span id="cb6-11"><a href="#cb6-11"></a><span class="op">}</span></span></code></pre></div>
<p>Additionally, use <code>seed::html_document()</code> in the same way, to return a <a href="https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlDocument.html">HtmlDocument</a></p>
<p>We also include <code>seed::canvas()</code>, and <code>seed::canvas_context()</code>. (Unreleased)</p>
<p>You can call <code>seed::cookies()</code> to retrieve all cookies from the current <code>HtmlDocument</code>.</p>
<h2 id="input-elements-are-controlled">Input elements are controlled</h2>
<p><code>input</code>, <code>textarea</code>, and <code>select</code> elements are always controlled, in the vein of <code>React</code>’s <a href="https://reactjs.org/docs/forms.html#controlled-components">controlled components</a>. This means that even if there’s no event associated with user input to these fields, their value will always stay in sync with the model, which may mean ignoring text input if not set up with a <code>Ev::Input</code> event.</p>
"#####.into()
}