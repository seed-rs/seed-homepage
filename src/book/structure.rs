pub fn text() -> String {
r#####"
<h1 id="app-structure">App structure</h1>
<h2 id="model">Model</h2>
<p>Each app must contain a model <a href="https://doc.rust-lang.org/book/ch05-00-structs.html">struct</a>, which contains the app’s state. It must should contain <a href="https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html">owned data</a>. References with a static <a href="https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html">lifetime</a> work, but may be more difficult to work with. Example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb1-1"><a href="#cb1-1"></a><span class="kw">struct</span> Model <span class="op">{</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>    count: <span class="dt">i32</span>,</span>
<span id="cb1-3"><a href="#cb1-3"></a>    what_we_count: <span class="dt">String</span></span>
<span id="cb1-4"><a href="#cb1-4"></a><span class="op">}</span></span>
<span id="cb1-5"><a href="#cb1-5"></a></span>
<span id="cb1-6"><a href="#cb1-6"></a><span class="co">// Setup a default here, for initialization later.</span></span>
<span id="cb1-7"><a href="#cb1-7"></a><span class="kw">impl</span> <span class="bu">Default</span> <span class="kw">for</span> Model <span class="op">{</span></span>
<span id="cb1-8"><a href="#cb1-8"></a>    <span class="kw">fn</span> default() -&gt; <span class="kw">Self</span> <span class="op">{</span></span>
<span id="cb1-9"><a href="#cb1-9"></a>        <span class="kw">Self</span> <span class="op">{</span></span>
<span id="cb1-10"><a href="#cb1-10"></a>            count: <span class="dv">0</span>,</span>
<span id="cb1-11"><a href="#cb1-11"></a>            what_we_count: <span class="st">&quot;click&quot;</span>.into()</span>
<span id="cb1-12"><a href="#cb1-12"></a>        <span class="op">}</span></span>
<span id="cb1-13"><a href="#cb1-13"></a>    <span class="op">}</span></span>
<span id="cb1-14"><a href="#cb1-14"></a><span class="op">}</span></span></code></pre></div>
<p>In this example, we initialize using Rust’s <code>Default</code> trait, in order to keep the initialization code by the model struct. When we call <code>Model.default()</code>, it initializes with these values. We could also initialize it using a constructor method, or a struct literal. Note the use of <code>into()</code> on our <code>&amp;str</code> literal, to convert it into an owned <code>String</code>.</p>
<p>The model holds all data used by the app, and will be replaced with updated versions when the data changes. Use owned data in the model; eg <code>String</code> instead of <code>&amp;'static str</code>. The model may be split into sub-structs to organize it – this is especially useful as the app grows:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb2-1"><a href="#cb2-1"></a><span class="kw">struct</span> FormData <span class="op">{</span></span>
<span id="cb2-2"><a href="#cb2-2"></a>    name: <span class="dt">String</span>,</span>
<span id="cb2-3"><a href="#cb2-3"></a>    age: <span class="dt">i8</span>,</span>
<span id="cb2-4"><a href="#cb2-4"></a><span class="op">}</span></span>
<span id="cb2-5"><a href="#cb2-5"></a></span>
<span id="cb2-6"><a href="#cb2-6"></a><span class="kw">struct</span> Misc <span class="op">{</span></span>
<span id="cb2-7"><a href="#cb2-7"></a>    value: <span class="dt">i8</span>,</span>
<span id="cb2-8"><a href="#cb2-8"></a>    descrip: <span class="dt">String</span>,</span>
<span id="cb2-9"><a href="#cb2-9"></a><span class="op">}</span></span>
<span id="cb2-10"><a href="#cb2-10"></a></span>
<span id="cb2-11"><a href="#cb2-11"></a><span class="kw">struct</span> Model <span class="op">{</span></span>
<span id="cb2-12"><a href="#cb2-12"></a>    form_data: FormData,</span>
<span id="cb2-13"><a href="#cb2-13"></a>    misc: Misc</span>
<span id="cb2-14"><a href="#cb2-14"></a><span class="op">}</span></span></code></pre></div>
<h2 id="update">Update</h2>
<p>The Message is an <a href="https://doc.rust-lang.org/book/ch06-00-enums.html">enum</a> which categorizes each type of interaction with the app. It must implement <code>Clone</code>, and its fields may hold a value, or not. We’ve abbreviated it as <code>Msg</code> here for brevity. If you’re not familiar with enums, think of one as a set of options; in other languages, you might use an integer, or string for this, but an enum is explicitly limited in which values it can take. Example:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb3-1"><a href="#cb3-1"></a><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></span>
<span id="cb3-2"><a href="#cb3-2"></a><span class="kw">enum</span> Msg <span class="op">{</span></span>
<span id="cb3-3"><a href="#cb3-3"></a>    Increment,</span>
<span id="cb3-4"><a href="#cb3-4"></a>    Decrement,</span>
<span id="cb3-5"><a href="#cb3-5"></a>    ChangeDescrip(<span class="dt">String</span>),  <span class="co">//  We could use &amp;&#39;static str here too.</span></span>
<span id="cb3-6"><a href="#cb3-6"></a><span class="op">}</span></span></code></pre></div>
<p>The update <a href="https://doc.rust-lang.org/book/ch03-03-how-functions-work.html">function</a> you pass to <code>seed::App::build(</code> describes how the state should change, upon receiving each type of message. It’s the only place where the model is changed. It accepts a message, and model as parameters, and returns an <code>Update</code> struct. <code>Update</code> contains <code>ShouldRender</code> and <code>Effect</code> enums. <code>ShouldRender</code> and its variants are imported in the prelude, and has variants of <code>Render</code> and <code>Skip</code>. Render triggers a rendering update, and will be used in most cases. <code>Skip</code> updates the model without triggering a render, and is useful in animations. <code>Effect</code> isn’t exposed in the API: it’s used internally to handle async events like fetch requests. See the <code>Http requests</code> section for more info.</p>
<p>Example:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb4-1"><a href="#cb4-1"></a><span class="kw">fn</span> update(msg: Msg, model: &amp;<span class="kw">mut</span> Model, _orders: &amp;<span class="kw">mut</span> <span class="kw">impl</span> Orders&lt;Msg&gt;) <span class="op">{</span></span>
<span id="cb4-2"><a href="#cb4-2"></a>    <span class="kw">match</span> msg <span class="op">{</span></span>
<span id="cb4-3"><a href="#cb4-3"></a>        <span class="pp">Msg::</span>Increment =&gt; model.count += <span class="dv">1</span>,</span>
<span id="cb4-4"><a href="#cb4-4"></a>        <span class="pp">Msg::</span>SetCount(count) =&gt; model.count = count,</span>
<span id="cb4-5"><a href="#cb4-5"></a>    <span class="op">}</span></span>
<span id="cb4-6"><a href="#cb4-6"></a><span class="op">}</span></span></code></pre></div>
<p>While the signature of the update function is fixed, and will usually involve a match pattern with an arm for each message, there are many ways you can structure this function. Some may be easier to write, and others may be more efficient, or appeal to specific aesthetics. While the example above it straightforward, this becomes important with more complex updates.</p>
<p>More detailed example, from the <a href="https://github.com/David-OConnor/seed/tree/master/examples/todomvc">todoMVC example</a>:</p>
<div class="sourceCode" id="cb5"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb5-1"><a href="#cb5-1"></a><span class="kw">fn</span> update(msg: Msg, model: &amp;<span class="kw">mut</span> Model, _orders: &amp;<span class="kw">mut</span> <span class="kw">impl</span> Orders&lt;Msg&gt;) <span class="op">{</span></span>
<span id="cb5-2"><a href="#cb5-2"></a>    <span class="kw">match</span> msg <span class="op">{</span></span>
<span id="cb5-3"><a href="#cb5-3"></a>        <span class="pp">Msg::</span>ClearCompleted =&gt; <span class="op">{</span></span>
<span id="cb5-4"><a href="#cb5-4"></a>            model.todos = model.todos.into_iter()</span>
<span id="cb5-5"><a href="#cb5-5"></a>            .filter(|t| !t.completed)</span>
<span id="cb5-6"><a href="#cb5-6"></a>            .collect();</span>
<span id="cb5-7"><a href="#cb5-7"></a>        <span class="op">}</span>,</span>
<span id="cb5-8"><a href="#cb5-8"></a>        <span class="pp">Msg::</span>Destroy(posit) =&gt; <span class="op">{</span></span>
<span id="cb5-9"><a href="#cb5-9"></a>            model.todos.remove(posit);</span>
<span id="cb5-10"><a href="#cb5-10"></a>        <span class="op">}</span>,</span>
<span id="cb5-11"><a href="#cb5-11"></a>        <span class="pp">Msg::</span>Toggle(posit) =&gt; model.todos<span class="op">[</span>posit<span class="op">]</span>.completed = !model.todos<span class="op">[</span>posit<span class="op">]</span>.completed,</span>
<span id="cb5-12"><a href="#cb5-12"></a>        <span class="pp">Msg::</span>ToggleAll =&gt; <span class="op">{</span></span>
<span id="cb5-13"><a href="#cb5-13"></a>            <span class="kw">let</span> completed = model.active_count() != <span class="dv">0</span>;</span>
<span id="cb5-14"><a href="#cb5-14"></a>            <span class="kw">for</span> todo <span class="kw">in</span> &amp;<span class="kw">mut</span> model.todos <span class="op">{</span></span>
<span id="cb5-15"><a href="#cb5-15"></a>                todo.completed = completed;</span>
<span id="cb5-16"><a href="#cb5-16"></a>            <span class="op">}</span></span>
<span id="cb5-17"><a href="#cb5-17"></a>        <span class="op">}</span></span>
<span id="cb5-18"><a href="#cb5-18"></a><span class="op">}</span></span></code></pre></div>
<p>The third parameter of the update function is an <a href="https://docs.rs/seed/0.4.2/seed/prelude/struct.Orders.html">Orders</a> struct, imported in the prelude. It has four methods, each defining an update behavior:</p>
<ul>
<li><code>render</code>: Rerender the DOM, based on the new model. If <code>orders</code> is not used for a branch, it is used.</li>
<li><code>skip</code>: Update the model without re-rendering</li>
<li><code>send_msg</code>: Update again, with a new message, the only parameter to this method</li>
<li><code>perform_cmd</code>: Perform an asynchronous task, like pulling data from a server. Its parameter is a <code>Future</code>, ie <code>Future&lt;Item = Ms, Error = Ms&gt; + 'static</code>.</li>
</ul>
<p>For an example of how to use orders, see the <a href="https://github.com/David-OConnor/seed/blob/master/examples/orders/src/lib.rs">orders example</a>.</p>
<p>As with the model, only one update function is passed to the app, but it may be split into sub-functions to aid code organization.</p>
<h2 id="view">View</h2>
<p>See the <a href="https://seed-rs.org/guide/view">view section</a> for details.</p>
<h2 id="initializing">Initializing</h2>
<p>To start your app, call the <code>seed::App::build</code> method, which takes the following parameters:</p>
<ul>
<li>An <code>init</code> function which accepts an initial routing, initial orders, and outputs an <a href="https://docs.rs/seed/0.4.2/seed/struct.Init.html">Init struct</a> (imported in the prelude), wrapping the initial model.</li>
<li>Your update function</li>
<li>Your view function</li>
</ul>
<p>You can can chain the following optional methods:</p>
<ul>
<li><code>.mount()</code> to mount in an element other than the one with id <code>app</code>.</li>
<li><code>.routes(routes)</code> to set a HashMap of landing-page routings, used to initialize your state based on url (See the <code>Routing</code> section)</li>
<li><code>.window_events(window_events)</code>, to set a function describing events on the <code>Window</code>. (See the <code>Events</code> section)</li>
</ul>
<p>And must must complete with the method <code>.build_and_start();</code>.</p>
<p><code>.mount()</code> takes a single argument, which can be the id of the element you wish to mount in, a <code>web_sys::Element</code>, or a <code>web_sys::HtmlElement</code>. Examples: <code>seed::App::build(|_, _| Model::default(), update, view).mount(seed::body())</code> <code>seed::App::build(|_, _| Model::default(), update, view).mount('a_div_id</code>)`</p>
<pre><code>seed::App::build(|_, _| Model::default(), update, view).mount(
    seed::body().querySelector(&quot;section&quot;).unwrap().unwrap()
)</code></pre>
<p>The <code>seed::App::build</code> call must be wrapped in a function with the <code>#[wasm_bindgen(start)]</code> invocation.</p>
<p>This will render your app to the element holding the id you passed; in the case of this example, “main”. The only part of the web page Seed will interact with is that element, so you can use other HTML not part of Seed, or other JS code/frameworks in the same document.</p>
<p>Example, with optional methods:</p>
<div class="sourceCode" id="cb7"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb7-1"><a href="#cb7-1"></a><span class="at">#[</span>wasm_bindgen<span class="at">(</span>start<span class="at">)]</span></span>
<span id="cb7-2"><a href="#cb7-2"></a><span class="kw">pub</span> <span class="kw">fn</span> render() <span class="op">{</span></span>
<span id="cb7-3"><a href="#cb7-3"></a>    <span class="pp">seed::App::</span>build(|_, _| <span class="pp">Init::</span>new(<span class="pp">Model::</span><span class="kw">default</span>()), update, view)</span>
<span id="cb7-4"><a href="#cb7-4"></a>        .mount(<span class="st">&quot;main&quot;</span>)</span>
<span id="cb7-5"><a href="#cb7-5"></a>        .routes(routes)</span>
<span id="cb7-6"><a href="#cb7-6"></a>        .window_events(window_events)</span>
<span id="cb7-7"><a href="#cb7-7"></a>        .build_and_start();</span>
<span id="cb7-8"><a href="#cb7-8"></a><span class="op">}</span></span></code></pre></div>
<p>Example of using a standalone <code>init</code> function:</p>
<div class="sourceCode" id="cb8"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb8-1"><a href="#cb8-1"></a><span class="kw">fn</span> init(url: Url, orders: &amp;<span class="kw">mut</span> <span class="kw">impl</span> Orders&lt;Msg&gt;) -&gt; Init&lt;Model&gt; <span class="op">{</span></span>
<span id="cb8-2"><a href="#cb8-2"></a>    <span class="pp">Init::</span>new(<span class="pp">Model::</span><span class="kw">default</span>())</span>
<span id="cb8-3"><a href="#cb8-3"></a><span class="op">}</span></span>
<span id="cb8-4"><a href="#cb8-4"></a></span>
<span id="cb8-5"><a href="#cb8-5"></a><span class="at">#[</span>wasm_bindgen<span class="at">(</span>start<span class="at">)]</span></span>
<span id="cb8-6"><a href="#cb8-6"></a><span class="kw">pub</span> <span class="kw">fn</span> render() <span class="op">{</span></span>
<span id="cb8-7"><a href="#cb8-7"></a>    <span class="pp">seed::App::</span>build(init, update, view)</span>
<span id="cb8-8"><a href="#cb8-8"></a>        .build_and_start();</span>
<span id="cb8-9"><a href="#cb8-9"></a><span class="op">}</span></span></code></pre></div>
<p><code>Init</code> has the following fields: - <code>model</code>: The initial model - <code>url_handling</code>: A <a href="https://docs.rs/seed/0.4.2/seed/enum.UrlHandling.html">Urlhandling</a> enum, which has variants <code>PassToRoutes</code>: default with <code>Init::new()</code>), and <code>None</code> - <code>mount_type</code>: A <a href="https://docs.rs/seed/0.4.2/seed/enum.MountType.html">MountType</a> enum, which has variants <code>Append</code>: default with <code>Init::new()</code>, Leave the previously existing elements in the mount alone. This does not make guarantees of elements added after the <code>App</code> has been mounted), and <code>Takeover</code>: Take control of previously existing elements in the mount. This does not make guarantees of elements added after the <code>App</code> has been mounted. Note that existing elements in the DOM will be recreated. This can be dangerous for script tags and other, similar tags.</p>
<p><code>Init::new()</code> covers the most common use-cases of the <code>Init</code>, but pass an <code>Init</code> literal if you’d like to use <code>url_handling</code> or <code>mount_type</code>. <code>UrlHandling</code> and <code>MountType</code> are imported in the prelude.</p>
"#####.into()
}