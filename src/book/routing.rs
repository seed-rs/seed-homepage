pub fn text() -> String {
r#"
<h1 id="routing">Routing</h1>
<p>Seed supports basic routings: You can trigger state changes that update the address bar, and can be nagivated to/from using forward and back buttons. This works for custom landing-page routing as well, provided your server is configured to support this. (eg sub-paths will redirect to the main path instead of throwing errors.)</p>
<p>As an example, let's say our site has three pages: a home page, a guide, and a changelog, accessible by <code>http://seed-rs.org/</code>, <code>http://seed-rs.org/guide</code>, and <code>http://seed-rs.org/changelog</code> respectively. First, we need to set up our backend server so that all three of these endpoints point towards our app. We describe the page by a <code>page</code> field in our model, which is an integer: 0 for homepage, 1 for guide, or 2 for changelog. (An enum would work as well).</p>
<p>To set up the initial routing, we pass a HashMap&lt;&amp;str, Msg&gt; describing the possible routings as the last parameter of <a href="https://docs.rs/seed/0.1.7/seed/fn.run.html">Seed::run</a>:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb1-1" title="1"><span class="at">#[</span>wasm_bindgen<span class="at">]</span></a>
<a class="sourceLine" id="cb1-2" title="2"><span class="kw">pub</span> <span class="kw">fn</span> render() <span class="op">{</span></a>
<a class="sourceLine" id="cb1-3" title="3">    <span class="kw">let</span> <span class="kw">mut</span> route_map = <span class="pp">HashMap::</span>new();</a>
<a class="sourceLine" id="cb1-4" title="4">    route_map.insert(<span class="st">&quot;guide&quot;</span>, <span class="pp">Msg::</span>RoutePage(<span class="dv">1</span>));</a>
<a class="sourceLine" id="cb1-5" title="5">    route_map.insert(<span class="st">&quot;changelog&quot;</span>, <span class="pp">Msg::</span>RoutePage(<span class="dv">2</span>));</a>
<a class="sourceLine" id="cb1-6" title="6"></a>
<a class="sourceLine" id="cb1-7" title="7">    <span class="pp">seed::</span>run(<span class="pp">Model::</span><span class="kw">default</span>(), update, view, <span class="st">&quot;main&quot;</span>, <span class="cn">Some</span>(route_map));</a>
<a class="sourceLine" id="cb1-8" title="8"><span class="op">}</span></a></code></pre></div>
<p>Once this is configured, intial routing on page load will work as expected: The page will load with the default state, then immediately trigger the update prescribed by the RoutePage message.</p>
<p>In order to trigger our route change through an event (eg clicking a link or pushing a button), our update function includes the following logic:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb2-1" title="1"><span class="kw">match</span> msg <span class="op">{</span></a>
<a class="sourceLine" id="cb2-2" title="2">    <span class="pp">Msg::</span>ChangePage(page) =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb2-3" title="3">        <span class="pp">seed::</span>push_route(&amp;page.to_string());</a>
<a class="sourceLine" id="cb2-4" title="4">        update(<span class="pp">Msg::</span>RoutePage(page), model)</a>
<a class="sourceLine" id="cb2-5" title="5">    <span class="op">}</span>,</a>
<a class="sourceLine" id="cb2-6" title="6"></a>
<a class="sourceLine" id="cb2-7" title="7">    <span class="co">// This is separate, because nagivating the route triggers state updates, which would</span></a>
<a class="sourceLine" id="cb2-8" title="8">    <span class="co">// trigger an additional push state.</span></a>
<a class="sourceLine" id="cb2-9" title="9">    <span class="pp">Msg::</span>RoutePage(page) =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb2-10" title="10">        Model <span class="op">{</span>page, ..model<span class="op">}</span></a>
<a class="sourceLine" id="cb2-11" title="11">    <span class="op">}</span></a>
<a class="sourceLine" id="cb2-12" title="12"><span class="op">}</span></a></code></pre></div>
<p><a href="https://docs.rs/seed/0.1.7/seed/fn.push_route.html">seed::push_route</a> accepts a single parameter: a path string corresponding to what will be appended to the url. Currently, it must match one of the keys in the route map.</p>
<p>When a page is loaded, or naviation occurs (eg back button), Seed searches each of the route_map keys for a matching path name (url suffix). If it finds one, it updates the model based on its associated message. If not, no action will be taken. In our example, we assume the model initialized to page=0, for the homepage.</p>
<p>Notice how we keep ChangePage and RoutePage separate in our example: RoutePage performs the action associated with routing, while ChangePage updates our route history, then recursively calls RoutePage. If you were to attempt this in the same message, each navigation event would add a redundant route history entry, interfering with navigation.</p>
<p>Dynamic routes are not yet supported.</p>
"#.into()
}