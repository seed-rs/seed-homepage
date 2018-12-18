pub fn text() -> String {
r#"
<h1 id="misc-features">Misc features</h1>
<h2 id="logging-in-the-web-browser">Logging in the web browser</h2>
<p>To output to the web browser's console (ie <code>console.log()</code> in JS), use <code>web_sys::console_log1</code>, or the <code>log</code> macro that wraps it, which is imported in the seed prelude: <code>log!("On the shoulders of", 5, "giants".to_string())</code></p>
<h2 id="routing">Routing</h2>
<p>For now, the only supported routing feature is in setting up how to load the inintial page, based on the entry-point URL. As an example, let's say our site has three pages: a home page, a guide, and a changelog, accessible by <code>http://seed-rs.org/</code>, <code>http://seed-rs.org/guide</code>, and <code>http://seed-rs.org/changelog</code> respectively. First, we need to set up our backend server so that all three of these endpoints point towards our app. We describe the page by a <code>page</code> field in our model, which is an integer: 0 for homepage, 1 for guide, or 2 for changelog. (An enum would work as well). Our update function includes this logic, triggered by clicking a link, or pushing a button:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb1-1" title="1"><span class="kw">match</span> msg <span class="op">{</span></a>
<a class="sourceLine" id="cb1-2" title="2">        <span class="pp">Msg::</span>ChangePage(page) =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb1-3" title="3">            Model <span class="op">{</span>page, ..model<span class="op">}</span></a>
<a class="sourceLine" id="cb1-4" title="4">        <span class="op">}</span>,</a></code></pre></div>
<p>To set up the initial routing, we pass a HashMap&lt;&amp;str, Msg&gt; describing the possible routings as the last parameter of <code>https://docs.rs/seed/0.1.7/seed/fn.run.html</code>:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb2-1" title="1"><span class="at">#[</span>wasm_bindgen<span class="at">]</span></a>
<a class="sourceLine" id="cb2-2" title="2"><span class="kw">pub</span> <span class="kw">fn</span> render() <span class="op">{</span></a>
<a class="sourceLine" id="cb2-3" title="3">    <span class="kw">let</span> <span class="kw">mut</span> route_map = <span class="pp">HashMap::</span>new();</a>
<a class="sourceLine" id="cb2-4" title="4">    route_map.insert(<span class="st">&quot;/guide&quot;</span>, <span class="pp">Msg::</span>ChangePage(<span class="dv">1</span>));</a>
<a class="sourceLine" id="cb2-5" title="5">    route_map.insert(<span class="st">&quot;/changelog&quot;</span>, <span class="pp">Msg::</span>ChangePage(<span class="dv">2</span>));</a>
<a class="sourceLine" id="cb2-6" title="6"></a>
<a class="sourceLine" id="cb2-7" title="7">    <span class="pp">seed::</span>run(<span class="pp">Model::</span><span class="kw">default</span>(), update, view, <span class="st">&quot;main&quot;</span>, <span class="cn">Some</span>(route_map));</a>
<a class="sourceLine" id="cb2-8" title="8"><span class="op">}</span></a></code></pre></div>
<p>Seed searches each of the route_map keys for a matching path name (url suffix). If it finds one, it updates the model based on its associated message. If not, no action will be taken. In our example, we assume the model initialized to page=0, for the homepage.</p>
<h2 id="querying-servers-using-fetch">Querying servers using fetch</h2>
<p>To send and receive data with a server, use <code>wasm-bindgen</code>'s <code>web-sys</code> fetch methods, <a href="https://rustwasm.github.io/wasm-bindgen/examples/fetch.html">described here</a>.</p>
<p>Use the <a href="https://serde.rs/">Serde</a> crate to serialize and deserialize data, eg when sending and receiving data from a REST-etc. It supports most popular formats, including <code>JSON</code>, <code>YAML</code>, and <code>XML</code>.</p>
<p>(Example, and with our integration)</p>
<p>Check out the <code>server_interaction</code> examples for an example of how to send and receive data from the server in JSON.</p>
<p>Seed will implement a high-level fetch API in the future, wrapping web-sys's.</p>
<h2 id="local-storage">Local storage</h2>
<p>You can store page state locally using web_sys's <a href="https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Storage.html">Storage struct</a></p>
<p>Seed provides convenience functions <code>seed::storage::get_storage</code>, which returns the <code>web_sys::storage</code> object, and <code>seed::storage::store_data</code> to store an arbitrary Rust data structure that implements serde's Serialize. Example use:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb3-1" title="1"><span class="kw">extern</span> <span class="kw">crate</span> serde;</a>
<a class="sourceLine" id="cb3-2" title="2"><span class="at">#[</span>macro_use<span class="at">]</span></a>
<a class="sourceLine" id="cb3-3" title="3"><span class="kw">extern</span> <span class="kw">crate</span> serde_derive;</a>
<a class="sourceLine" id="cb3-4" title="4"><span class="kw">extern</span> <span class="kw">crate</span> serde_json;</a>
<a class="sourceLine" id="cb3-5" title="5"></a>
<a class="sourceLine" id="cb3-6" title="6"><span class="co">// ...</span></a>
<a class="sourceLine" id="cb3-7" title="7"><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="at">,</span> Deserialize<span class="at">)]</span></a>
<a class="sourceLine" id="cb3-8" title="8"><span class="kw">struct</span> Data <span class="op">{</span></a>
<a class="sourceLine" id="cb3-9" title="9">    <span class="co">// Arbitrary data (All sub-structs etc must also implement Serialize and Deserialize)</span></a>
<a class="sourceLine" id="cb3-10" title="10"><span class="op">}</span></a>
<a class="sourceLine" id="cb3-11" title="11"></a>
<a class="sourceLine" id="cb3-12" title="12"><span class="kw">let</span> storage = <span class="pp">seed::storage::</span>get_storage();</a>
<a class="sourceLine" id="cb3-13" title="13"><span class="pp">seed::storage::</span>store(storage, <span class="st">&quot;my-data&quot;</span>, <span class="pp">Data::</span>new());</a>
<a class="sourceLine" id="cb3-14" title="14"></a>
<a class="sourceLine" id="cb3-15" title="15"><span class="co">// ...</span></a>
<a class="sourceLine" id="cb3-16" title="16"></a>
<a class="sourceLine" id="cb3-17" title="17"><span class="kw">let</span> loaded_serialized = storage.get_item(<span class="st">&quot;my-data&quot;</span>).unwrap().unwrap();</a>
<a class="sourceLine" id="cb3-18" title="18"><span class="kw">let</span> data = <span class="pp">serde_json::</span>from_str(&amp;loaded_serialized).unwrap();</a></code></pre></div>
<h2 id="display-markdown-and-raw-html">Display markdown and raw HTML</h2>
<p>Seed supports creating elements from markdown text, using <a href="https://github.com/raphlinus/pulldown-cmark">pulldown-cmark</a> internally. Use the <a href="https://docs.rs/seed/0.1.6/seed/dom_types/struct.El.html#method.from_markdown">El::from_markdown()</a> method to create an element that accepts a markdown &amp;str as its only parameter, and displays it normally as html. Note that it does not support syntax highlighting. You can render raw HTML with <code>El::from_html(html)</code>, where <code>html</code> is a &amp;str of HTML.</p>
<p>Example:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb4-1" title="1"><span class="kw">fn</span> view(model: Model) -&gt; El&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb4-2" title="2"></a>
<a class="sourceLine" id="cb4-3" title="3">    <span class="kw">let</span> markdown = </a>
<a class="sourceLine" id="cb4-4" title="4"><span class="st">&quot;</span></a>
<a class="sourceLine" id="cb4-5" title="5"><span class="st">## Hello world</span></a>
<a class="sourceLine" id="cb4-6" title="6"></a>
<a class="sourceLine" id="cb4-7" title="7"><span class="st">Let&#39;s set the existence-of-God issue aside for a later volume,</span></a>
<a class="sourceLine" id="cb4-8" title="8"><span class="st">and just [learn to code](https://play.rust-lang.org/).</span></a>
<a class="sourceLine" id="cb4-9" title="9"><span class="st">&quot;</span></a>
<a class="sourceLine" id="cb4-10" title="10">;</a>
<a class="sourceLine" id="cb4-11" title="11"></a>
<a class="sourceLine" id="cb4-12" title="12">    <span class="kw">let</span> html = </a>
<a class="sourceLine" id="cb4-13" title="13"><span class="st">&quot;</span></a>
<a class="sourceLine" id="cb4-14" title="14"><span class="st">&lt;div&gt;</span></a>
<a class="sourceLine" id="cb4-15" title="15"><span class="st">    &lt;p&gt;It is a truth universally acknowledged, that a single man in </span></a>
<a class="sourceLine" id="cb4-16" title="16"><span class="st">    possession of a good fortune, must be in want of a good time.&lt;/p&gt;</span></a>
<a class="sourceLine" id="cb4-17" title="17"><span class="st">&lt;/div&gt;</span></a>
<a class="sourceLine" id="cb4-18" title="18"><span class="st">&quot;</span></a>
<a class="sourceLine" id="cb4-19" title="19">;</a>
<a class="sourceLine" id="cb4-20" title="20">    </a>
<a class="sourceLine" id="cb4-21" title="21">    <span class="pp">div!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb4-22" title="22">        <span class="pp">El::</span>from_markdown(markdown) </a>
<a class="sourceLine" id="cb4-23" title="23">        <span class="pp">El::</span>from_html(html) </a>
<a class="sourceLine" id="cb4-24" title="24">    <span class="op">]</span></a>
<a class="sourceLine" id="cb4-25" title="25"><span class="op">}</span></a>
<a class="sourceLine" id="cb4-26" title="26"></a>
<a class="sourceLine" id="cb4-27" title="27"></a></code></pre></div>
"#.into()
}