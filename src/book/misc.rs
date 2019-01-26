pub fn text() -> String {
r#"
<h1 id="misc-features">Misc features</h1>
<h2 id="logging-in-the-web-browser">Logging in the web browser</h2>
<p>To output to the web browser's console (ie <code>console.log()</code> in JS), use <code>web_sys::console_log1</code>, or the <code>log</code> macro that wraps it, which is imported in the seed prelude: <code>log!("On the shoulders of", 5, "giants".to_string())</code></p>
<h2 id="custom-tags">Custom tags</h2>
<p>Seed generally retricts the element tags allowed by using Enums for the tags, and a predefined set of element-creation macros. If you wish to use a custom tag, you can use using <code>Tag::Custom</code> (<code>El</code> and <code>Tag</code> are exposed in the prelude), either with the <code>El::empty</code> constructor, or using the <code>custom!</code> element-construction macro, where we pass our custom tag as an argument:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb1-1" title="1"><span class="kw">let</span> <span class="kw">mut</span> custom_el = <span class="pp">El::</span>empty(<span class="pp">Tag::</span>Custom(<span class="st">&quot;mytag&quot;</span>.to_string()));</a>
<a class="sourceLine" id="cb1-2" title="2">custom_el.set_text(<span class="st">&quot;Words&quot;</span>);</a>
<a class="sourceLine" id="cb1-3" title="3"></a>
<a class="sourceLine" id="cb1-4" title="4"><span class="pp">custom!</span><span class="op">[</span> <span class="pp">Tag::</span>Custom(<span class="st">&quot;anothertag&quot;</span>.into())</a>
<a class="sourceLine" id="cb1-5" title="5">    custom_el,</a>
<a class="sourceLine" id="cb1-6" title="6"><span class="op">]</span></a></code></pre></div>
<p>An example is provided as part of the <a href="https://github.com/David-OConnor/seed/tree/master/examples/todomvc">window_events</a> example.</p>
<h2 id="local-storage">Local storage</h2>
<p>You can store page state locally using web_sys's <a href="https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Storage.html">Storage struct</a></p>
<p>Seed provides convenience functions <code>seed::storage::get_storage</code>, which returns the <code>web_sys::storage</code> object, and <code>seed::storage::store_data</code> to store an arbitrary Rust data structure that implements serde's Serialize. Example use:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb2-1" title="1"><span class="kw">extern</span> <span class="kw">crate</span> serde;</a>
<a class="sourceLine" id="cb2-2" title="2"><span class="at">#[</span>macro_use<span class="at">]</span></a>
<a class="sourceLine" id="cb2-3" title="3"><span class="kw">extern</span> <span class="kw">crate</span> serde_derive;</a>
<a class="sourceLine" id="cb2-4" title="4"><span class="kw">extern</span> <span class="kw">crate</span> serde_json;</a>
<a class="sourceLine" id="cb2-5" title="5"></a>
<a class="sourceLine" id="cb2-6" title="6"><span class="co">// ...</span></a>
<a class="sourceLine" id="cb2-7" title="7"><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="at">,</span> Deserialize<span class="at">)]</span></a>
<a class="sourceLine" id="cb2-8" title="8"><span class="kw">struct</span> Data <span class="op">{</span></a>
<a class="sourceLine" id="cb2-9" title="9">    <span class="co">// Arbitrary data (All sub-structs etc must also implement Serialize and Deserialize)</span></a>
<a class="sourceLine" id="cb2-10" title="10"><span class="op">}</span></a>
<a class="sourceLine" id="cb2-11" title="11"></a>
<a class="sourceLine" id="cb2-12" title="12"><span class="kw">let</span> storage = <span class="pp">seed::storage::</span>get_storage();</a>
<a class="sourceLine" id="cb2-13" title="13"><span class="pp">seed::storage::</span>store(storage, <span class="st">&quot;my-data&quot;</span>, <span class="pp">Data::</span>new());</a>
<a class="sourceLine" id="cb2-14" title="14"></a>
<a class="sourceLine" id="cb2-15" title="15"><span class="co">// ...</span></a>
<a class="sourceLine" id="cb2-16" title="16"></a>
<a class="sourceLine" id="cb2-17" title="17"><span class="kw">let</span> loaded_serialized = storage.get_item(<span class="st">&quot;my-data&quot;</span>).unwrap().unwrap();</a>
<a class="sourceLine" id="cb2-18" title="18"><span class="kw">let</span> data = <span class="pp">serde_json::</span>from_str(&amp;loaded_serialized).unwrap();</a></code></pre></div>
<h2 id="display-markdown-and-raw-html">Display markdown and raw HTML</h2>
<p>Seed supports creating elements from markdown text, using <a href="https://github.com/raphlinus/pulldown-cmark">pulldown-cmark</a> internally. Use the <a href="https://docs.rs/seed/0.2.4/seed/dom_types/struct.El.html#method.from_markdown">El::from_markdown()</a> method to create an element that accepts a markdown &amp;str as its only parameter, and displays it normally as html. Note that it does not support syntax highlighting. You can render raw HTML with <code>El::from_html(html)</code>, where <code>html</code> is a &amp;str of HTML.</p>
<p>Example:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb3-1" title="1"><span class="kw">fn</span> view(app: <span class="pp">seed::</span>App&lt;Msg, Model&gt;, model: Model) -&gt; El&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb3-2" title="2"></a>
<a class="sourceLine" id="cb3-3" title="3">    <span class="kw">let</span> markdown = </a>
<a class="sourceLine" id="cb3-4" title="4"><span class="st">&quot;</span></a>
<a class="sourceLine" id="cb3-5" title="5"><span class="st">## Hello world</span></a>
<a class="sourceLine" id="cb3-6" title="6"></a>
<a class="sourceLine" id="cb3-7" title="7"><span class="st">Let&#39;s set the existence-of-God issue aside for a later volume,</span></a>
<a class="sourceLine" id="cb3-8" title="8"><span class="st">and just [learn to code](https://play.rust-lang.org/).</span></a>
<a class="sourceLine" id="cb3-9" title="9"><span class="st">&quot;</span></a>
<a class="sourceLine" id="cb3-10" title="10">;</a>
<a class="sourceLine" id="cb3-11" title="11"></a>
<a class="sourceLine" id="cb3-12" title="12">    <span class="kw">let</span> html = </a>
<a class="sourceLine" id="cb3-13" title="13"><span class="st">&quot;</span></a>
<a class="sourceLine" id="cb3-14" title="14"><span class="st">&lt;div&gt;</span></a>
<a class="sourceLine" id="cb3-15" title="15"><span class="st">    &lt;p&gt;It is a truth universally acknowledged, that a single man in </span></a>
<a class="sourceLine" id="cb3-16" title="16"><span class="st">    possession of a good fortune, must be in want of a good time.&lt;/p&gt;</span></a>
<a class="sourceLine" id="cb3-17" title="17"><span class="st">&lt;/div&gt;</span></a>
<a class="sourceLine" id="cb3-18" title="18"><span class="st">&quot;</span></a>
<a class="sourceLine" id="cb3-19" title="19">;</a>
<a class="sourceLine" id="cb3-20" title="20">    </a>
<a class="sourceLine" id="cb3-21" title="21">    <span class="pp">div!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb3-22" title="22">        <span class="pp">El::</span>from_markdown(markdown) </a>
<a class="sourceLine" id="cb3-23" title="23">        <span class="pp">El::</span>from_html(html) </a>
<a class="sourceLine" id="cb3-24" title="24">    <span class="op">]</span></a>
<a class="sourceLine" id="cb3-25" title="25"><span class="op">}</span></a></code></pre></div>
<h2 id="some-convenience-functions">Some convenience functions</h2>
<p>You can use <code>seed::document</code> and <code>seed::window</code> to access the <code>web_sys</code> document and window functions. Example:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb4-1" title="1"><span class="kw">fn</span> view(state: <span class="pp">seed::</span>App&lt;Msg, Model&gt;, model: Model) -&gt; El&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb4-2" title="2">    <span class="pp">button!</span><span class="op">[</span> </a>
<a class="sourceLine" id="cb4-3" title="3">        simple_ev(<span class="st">&quot;click&quot;</span>, <span class="pp">Msg::</span>Increment), </a>
<a class="sourceLine" id="cb4-4" title="4">        <span class="pp">format!</span>(<span class="st">&quot;Hello, World × {}&quot;</span>, model.val),</a>
<a class="sourceLine" id="cb4-5" title="5">        did_mount(|_| <span class="op">{</span></a>
<a class="sourceLine" id="cb4-6" title="6">            <span class="pp">seed::</span>document().set_title(<span class="st">&quot;New title&quot;</span>)</a>
<a class="sourceLine" id="cb4-7" title="7">        <span class="op">}</span>)</a>
<a class="sourceLine" id="cb4-8" title="8">    <span class="op">]</span></a>
<a class="sourceLine" id="cb4-9" title="9"><span class="op">}</span></a></code></pre></div>
"#.into()
}