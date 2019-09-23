pub fn text() -> String {
r#####"
<h1 id="app-structure">App structure</h1>
<h2 id="model">Model</h2>
<p>Each app must contain a model <a href="https://doc.rust-lang.org/book/ch05-00-structs.html">struct</a>, which contains the app’s state. It must should contain <a href="https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html">owned data</a>. References with a static <a href="https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html">lifetime</a> work, but may be more difficult to work with. Example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb1-1" title="1"><span class="kw">struct</span> Model <span class="op">{</span></a>
<a class="sourceLine" id="cb1-2" title="2">    count: <span class="dt">i32</span>,</a>
<a class="sourceLine" id="cb1-3" title="3">    what_we_count: <span class="dt">String</span></a>
<a class="sourceLine" id="cb1-4" title="4"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-5" title="5"></a>
<a class="sourceLine" id="cb1-6" title="6"><span class="co">// Setup a default here, for initialization later.</span></a>
<a class="sourceLine" id="cb1-7" title="7"><span class="kw">impl</span> <span class="bu">Default</span> <span class="kw">for</span> Model <span class="op">{</span></a>
<a class="sourceLine" id="cb1-8" title="8">    <span class="kw">fn</span> default() -&gt; <span class="kw">Self</span> <span class="op">{</span></a>
<a class="sourceLine" id="cb1-9" title="9">        <span class="kw">Self</span> <span class="op">{</span></a>
<a class="sourceLine" id="cb1-10" title="10">            count: <span class="dv">0</span>,</a>
<a class="sourceLine" id="cb1-11" title="11">            what_we_count: <span class="st">&quot;click&quot;</span>.into()</a>
<a class="sourceLine" id="cb1-12" title="12">        <span class="op">}</span></a>
<a class="sourceLine" id="cb1-13" title="13">    <span class="op">}</span></a>
<a class="sourceLine" id="cb1-14" title="14"><span class="op">}</span></a></code></pre></div>
<p>In this example, we initialize using Rust’s <code>Default</code> trait, in order to keep the initialization code by the model struct. When we call <code>Model.default()</code>, it initializes with these values. We could also initialize it using a constructor method, or a struct literal. Note the use of <code>into()</code> on our <code>&amp;str</code> literal, to convert it into an owned <code>String</code>.</p>
<p>The model holds all data used by the app, and will be replaced with updated versions when the data changes. Use owned data in the model; eg <code>String</code> instead of <code>&amp;'static str</code>. The model may be split into sub-structs to organize it – this is especially useful as the app grows:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb2-1" title="1"><span class="kw">struct</span> FormData <span class="op">{</span></a>
<a class="sourceLine" id="cb2-2" title="2">    name: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb2-3" title="3">    age: <span class="dt">i8</span>,</a>
<a class="sourceLine" id="cb2-4" title="4"><span class="op">}</span></a>
<a class="sourceLine" id="cb2-5" title="5"></a>
<a class="sourceLine" id="cb2-6" title="6"><span class="kw">struct</span> Misc <span class="op">{</span></a>
<a class="sourceLine" id="cb2-7" title="7">    value: <span class="dt">i8</span>,</a>
<a class="sourceLine" id="cb2-8" title="8">    descrip: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb2-9" title="9"><span class="op">}</span></a>
<a class="sourceLine" id="cb2-10" title="10"></a>
<a class="sourceLine" id="cb2-11" title="11"><span class="kw">struct</span> Model <span class="op">{</span></a>
<a class="sourceLine" id="cb2-12" title="12">    form_data: FormData,</a>
<a class="sourceLine" id="cb2-13" title="13">    misc: Misc</a>
<a class="sourceLine" id="cb2-14" title="14"><span class="op">}</span></a></code></pre></div>
<h2 id="update">Update</h2>
<p>The Message is an <a href="https://doc.rust-lang.org/book/ch06-00-enums.html">enum</a> which categorizes each type of interaction with the app. It must implement <code>Clone</code>, and its fields may hold a value, or not. We’ve abbreviated it as <code>Msg</code> here for brevity. If you’re not familiar with enums, think of one as a set of options; in other languages, you might use an integer, or string for this, but an enum is explicitly limited in which values it can take. Example:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb3-1" title="1"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb3-2" title="2"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb3-3" title="3">    Increment,</a>
<a class="sourceLine" id="cb3-4" title="4">    Decrement,</a>
<a class="sourceLine" id="cb3-5" title="5">    ChangeDescrip(<span class="dt">String</span>),  <span class="co">//  We could use &amp;&#39;static str here too.</span></a>
<a class="sourceLine" id="cb3-6" title="6"><span class="op">}</span></a></code></pre></div>
<p>The update <a href="https://doc.rust-lang.org/book/ch03-03-how-functions-work.html">function</a> you pass to <code>seed::App::build(</code> describes how the state should change, upon receiving each type of message. It’s the only place where the model is changed. It accepts a message, and model as parameters, and returns an <code>Update</code> struct. <code>Update</code> contains <code>ShouldRender</code> and <code>Effect</code> enums. <code>ShouldRender</code> and its variants are imported in the prelude, and has variants of <code>Render</code> and <code>Skip</code>. Render triggers a rendering update, and will be used in most cases. <code>Skip</code> updates the model without triggering a render, and is useful in animations. <code>Effect</code> isn’t exposed in the API: it’s used internally to handle async events like fetch requests. See the <code>Http requests</code> section for more info.</p>
<p>Example:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb4-1" title="1"><span class="kw">fn</span> update(msg: Msg, model: &amp;<span class="kw">mut</span> Model, _orders: &amp;<span class="kw">mut</span> <span class="kw">impl</span> Orders&lt;Msg&gt;) <span class="op">{</span></a>
<a class="sourceLine" id="cb4-2" title="2">    <span class="kw">match</span> msg <span class="op">{</span></a>
<a class="sourceLine" id="cb4-3" title="3">        <span class="pp">Msg::</span>Increment =&gt; model.count += <span class="dv">1</span>,</a>
<a class="sourceLine" id="cb4-4" title="4">        <span class="pp">Msg::</span>SetCount(count) =&gt; model.count = count,</a>
<a class="sourceLine" id="cb4-5" title="5">    <span class="op">}</span></a>
<a class="sourceLine" id="cb4-6" title="6"><span class="op">}</span></a></code></pre></div>
<p>While the signature of the update function is fixed, and will usually involve a match pattern with an arm for each message, there are many ways you can structure this function. Some may be easier to write, and others may be more efficient, or appeal to specific aesthetics. While the example above it straightforward, this becomes important with more complex updates.</p>
<p>More detailed example, from the <a href="https://github.com/David-OConnor/seed/tree/master/examples/todomvc">todoMVC example</a>:</p>
<div class="sourceCode" id="cb5"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb5-1" title="1"><span class="kw">fn</span> update(msg: Msg, model: &amp;<span class="kw">mut</span> Model, _orders: &amp;<span class="kw">mut</span> <span class="kw">impl</span> Orders&lt;Msg&gt;) <span class="op">{</span></a>
<a class="sourceLine" id="cb5-2" title="2">    <span class="kw">match</span> msg <span class="op">{</span></a>
<a class="sourceLine" id="cb5-3" title="3">        <span class="pp">Msg::</span>ClearCompleted =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb5-4" title="4">            model.todos = model.todos.into_iter()</a>
<a class="sourceLine" id="cb5-5" title="5">            .filter(|t| !t.completed)</a>
<a class="sourceLine" id="cb5-6" title="6">            .collect();</a>
<a class="sourceLine" id="cb5-7" title="7">        <span class="op">}</span>,</a>
<a class="sourceLine" id="cb5-8" title="8">        <span class="pp">Msg::</span>Destroy(posit) =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb5-9" title="9">            model.todos.remove(posit);</a>
<a class="sourceLine" id="cb5-10" title="10">        <span class="op">}</span>,</a>
<a class="sourceLine" id="cb5-11" title="11">        <span class="pp">Msg::</span>Toggle(posit) =&gt; model.todos<span class="op">[</span>posit<span class="op">]</span>.completed = !model.todos<span class="op">[</span>posit<span class="op">]</span>.completed,</a>
<a class="sourceLine" id="cb5-12" title="12">        <span class="pp">Msg::</span>ToggleAll =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb5-13" title="13">            <span class="kw">let</span> completed = model.active_count() != <span class="dv">0</span>;</a>
<a class="sourceLine" id="cb5-14" title="14">            <span class="kw">for</span> todo <span class="kw">in</span> &amp;<span class="kw">mut</span> model.todos <span class="op">{</span></a>
<a class="sourceLine" id="cb5-15" title="15">                todo.completed = completed;</a>
<a class="sourceLine" id="cb5-16" title="16">            <span class="op">}</span></a>
<a class="sourceLine" id="cb5-17" title="17">        <span class="op">}</span></a>
<a class="sourceLine" id="cb5-18" title="18"><span class="op">}</span></a></code></pre></div>
<p>The third parameter of the update function is an <a href="https://docs.rs/seed/0.4.0/seed/prelude/struct.Orders.html">Orders</a> struct, imported in the prelude. It has four methods, each defining an update behavior:</p>
<ul>
<li><code>render</code>: Rerender the DOM, based on the new model. If <code>orders</code> is not used for a branch, it is used.</li>
<li><code>skip</code>: Update the model without re-rendering</li>
<li><code>send_msg</code>: Update again, with a new message, the only parameter to this method</li>
<li><code>perform_cmd</code>: Perform an asynchronous task, like pulling data from a server. Its parameter is a <code>Future</code>, ie <code>Future&lt;Item = Ms, Error = Ms&gt; + 'static</code>.</li>
</ul>
<p>For an example of how to use orders, see the <a href="https://github.com/David-OConnor/seed/blob/master/examples/orders/src/lib.rs">orders example</a>.</p>
<p>As with the model, only one update function is passed to the app, but it may be split into sub-functions to aid code organization.</p>
<h2 id="view">View</h2>
<p>Visual layout (ie HTML/DOM elements) is described declaratively in Rust, and uses <a href="https://doc.rust-lang.org/book/appendix-04-macros.html">macros</a> to simplify syntax.</p>
<p>The view’s defined by a function that’s passed to <code>seed::run</code>. This takes a <code>Seed::app&lt;Msg, Model&gt;</code>, and Model as parameters, and outputs something that implements the <code>View</code> trait, which is imported in the prelude. Usually, this is a <code>Node</code>, or <code>Vec&lt;Node&gt;</code>, representing all nodes that will be inserted as children on the top-level one. (The top-level <code>Node</code> is in the html file, and specified with <code>seed::App::build.mount()</code>, or as a default, the element with id <code>app</code>). It may composed into sub-functions, which can be thought of like components in other frameworks.</p>
<p>Examples:</p>
<div class="sourceCode" id="cb6"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb6-1" title="1"><span class="kw">fn</span> view(model: &amp;Model) -&gt; Node&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb6-2" title="2">    <span class="pp">h1!</span><span class="op">[</span> <span class="st">&quot;Let there be light&quot;</span> <span class="op">]</span>,</a>
<a class="sourceLine" id="cb6-3" title="3"><span class="op">}</span></a></code></pre></div>
<div class="sourceCode" id="cb7"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb7-1" title="1"><span class="kw">fn</span> view(model: &amp;Model) -&gt; <span class="dt">Vec</span>&lt;Node&lt;Msg&gt;&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb7-2" title="2">    <span class="pp">vec!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb7-3" title="3">        <span class="pp">h1!</span><span class="op">[</span> <span class="st">&quot;Let there be light&quot;</span> <span class="op">]</span>,</a>
<a class="sourceLine" id="cb7-4" title="4">        <span class="pp">h2!</span><span class="op">[</span> <span class="st">&quot;Let it be both a particle and a wave&quot;</span> <span class="op">]</span></a>
<a class="sourceLine" id="cb7-5" title="5">    <span class="op">]</span></a>
<a class="sourceLine" id="cb7-6" title="6"><span class="op">}</span></a></code></pre></div>
<p>In either of those examples, you could use the signature: <code>fn view(model: &amp;Model) -&gt; impl View&lt;Msg&gt;</code> instead. This allows you to change between them without changing the function signature.</p>
<h2 id="the-node-enum">The Node Enum</h2>
<p>The Virtual DOM is represnted by nested <a href="https://docs.rs/seed/0.4.0/seed/dom_types/enum.None.html">Nodes</a>. THe node has 3 variants: - <code>Text</code> holds a <a href="https://docs.rs/seed/0.4.0/seed/dom_types/struct.Text.html">Text</a> struct. Mostly for internal use, but can be created with <code>Node::new_text()</code>. - <code>Element</code> wraps an <a href="https://docs.rs/seed/0.4.0/seed/dom_types/struct.El.html">El</a>, which is the main component of our VDOM. Created using macros, described below. - <code>Empty</code> is a placeholder that doens’t render anything; useful in conditional/ternary logic. Created using the <code>empty![]</code> macro, or <code>seed::empty()</code>.</p>
<h2 id="elements-attributes-styles">Elements, attributes, styles</h2>
<p>Elements are created using macros, named by the lowercase name of each element, and imported into the global namespace. Eg <code>div!</code> above. We use this code to import them:</p>
<div class="sourceCode" id="cb8"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb8-1" title="1"><span class="at">#[</span>macro_use<span class="at">]</span></a>
<a class="sourceLine" id="cb8-2" title="2"><span class="kw">extern</span> <span class="kw">crate</span> seed;</a></code></pre></div>
<p>These macros accept any combination of the following parameters: - One <a href="https://docs.rs/seed/0.4.0/seed/dom_types/struct.Attrs.html">Attrs</a> struct - One <a href="https://docs.rs/seed/0.4.0/seed/dom_types/struct.Style.html">Style</a> struct - One or more <a href="https://docs.rs/seed/0.4.0/seed/dom_types/struct.Listener.html">Listener</a> structs, which handle events - One or more <code>Vec</code>s of <code>Listener</code> structs - One <code>String</code> or <code>&amp;str</code> representing a node text - One or more <a href="https://docs.rs/seed/0.4.0/seed/dom_types/enum.Node.html">Node</a> structs, representing a child - One or more Vecs of <code>Node</code> structs, representing multiple children - A <code>Map</code>, ie the result of <code>map()</code>, yielding <code>Node</code>s or <code>Listener</code>s, without having to explicitly <code>collect</code>.</p>
<p>The parameters can be passed in any order; the compiler knows how to handle them based on their types. Children are rendered in the order passed.</p>
<p>Views are described using <a href="https://docs.rs/seed/0.4.0/seed/dom_types/struct.El.html">El</a> structs, defined in the <a href="https://docs.rs/seed/0.4.0/seed/dom_types/index.html">seed::dom_types</a> module.</p>
<p><code>Attrs</code> and <code>Style</code> are thinly-wrapped hashmaps created with their own macros: <code>attrs!{}</code> and <code>style!{}</code> respectively.</p>
<p>Example:</p>
<div class="sourceCode" id="cb9"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb9-1" title="1"><span class="kw">fn</span> view(model: &amp;Model) -&gt; Node&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb9-2" title="2">    <span class="kw">let</span> things = <span class="pp">vec!</span><span class="op">[</span> <span class="pp">h4!</span><span class="op">[</span> <span class="st">&quot;thing1&quot;</span> <span class="op">]</span>, <span class="pp">h4!</span><span class="op">[</span> <span class="st">&quot;thing2&quot;</span> <span class="op">]</span> <span class="op">]</span>;</a>
<a class="sourceLine" id="cb9-3" title="3">    </a>
<a class="sourceLine" id="cb9-4" title="4">    <span class="kw">let</span> other_things = <span class="pp">vec!</span><span class="op">[</span><span class="dv">1</span>, <span class="dv">2</span><span class="op">]</span>;</a>
<a class="sourceLine" id="cb9-5" title="5"></a>
<a class="sourceLine" id="cb9-6" title="6">    <span class="pp">div!</span><span class="op">[</span> <span class="pp">attrs!</span><span class="op">{</span><span class="pp">At::</span>Class =&gt; <span class="st">&quot;hardly-any&quot;</span><span class="op">}</span>, </a>
<a class="sourceLine" id="cb9-7" title="7">        things,  <span class="co">// Vec&lt;Node&lt;Msg&gt;</span></a>
<a class="sourceLine" id="cb9-8" title="8">        other_things.map(|t| <span class="pp">h4!</span><span class="op">[</span>t.to_string()<span class="op">]</span>),  <span class="co">// Map</span></a>
<a class="sourceLine" id="cb9-9" title="9">        <span class="pp">h4!</span><span class="op">[</span> <span class="st">&quot;thing3?&quot;</span> <span class="op">]</span>,  <span class="co">// El</span></a>
<a class="sourceLine" id="cb9-10" title="10">    <span class="op">]</span></a>
<a class="sourceLine" id="cb9-11" title="11"><span class="op">}</span></a></code></pre></div>
<p>Note that you can create any of the above items inside an element macro, or create it separately, and pass it in. You can separate different items by comma, semicolon, or space.</p>
<p>Keys passed to <code>attrs!</code> can be <code>Seed::At</code>s, <code>String</code>s, or <code>&amp;str</code>s. Keys passed to <code>style!</code> can be <code>Seed::St</code>s, <code>String</code>s, or <code>&amp;str</code>s. Values passed to <code>attrs!</code>, and <code>style!</code> macros can be owned <code>Strings</code>, <code>&amp;str</code>s, or for <code>style!</code>, <code>unit</code>s. Eg: <code>input![ attrs!{At::Disabled =&gt; false]</code> and <code>input![ attrs!{&quot;disabled&quot; =&gt; &quot;false&quot;]</code> are equivalent. You use the <code>unit!</code> macro to apply units. There’s a <code>px</code> function for the special case where the unit is pixels:</p>
<div class="sourceCode" id="cb10"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb10-1" title="1"><span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>Width =&gt; <span class="pp">unit!</span>(<span class="dv">20</span>, px);<span class="op">}</span></a>
<a class="sourceLine" id="cb10-2" title="2"><span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>Width =&gt; px(<span class="dv">20</span>);<span class="op">}</span>  <span class="co">// equivalent</span></a></code></pre></div>
<p>We can set multiple values for an attribute using <code>Attribute.add_multiple</code>. This is useful for setting multiple classes. Note that we must set this up outside of the view macro, since it involves modifying a variable:</p>
<div class="sourceCode" id="cb11"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb11-1" title="1"><span class="kw">fn</span> a_component() -&gt; Node&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb11-2" title="2">    <span class="kw">let</span> <span class="kw">mut</span> attributes = <span class="pp">attrs!</span><span class="op">{}</span>;</a>
<a class="sourceLine" id="cb11-3" title="3">    attributes.add_multiple(<span class="pp">At::</span>Class, <span class="pp">vec!</span><span class="op">[</span><span class="st">&quot;A-modicum-of&quot;</span>, <span class="st">&quot;hardly-any&quot;</span><span class="op">]</span>);</a>
<a class="sourceLine" id="cb11-4" title="4"></a>
<a class="sourceLine" id="cb11-5" title="5">    <span class="pp">div!</span><span class="op">[</span> attributes <span class="op">]</span></a>
<a class="sourceLine" id="cb11-6" title="6"><span class="op">}</span></a></code></pre></div>
<p>Seed validates attributes <a href="https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes">against this list</a>; The <code>At</code> enum includes only these values, and <code>&amp;strs</code> passed are converted into <code>At</code>s. If you wish to use a custom attribute, use <code>At::Custom(name)</code>, where <code>name</code> is a <code>String</code> of your attribute’s name. In <code>attrs!</code> when using <code>&amp;str</code>s, inserting an unrecognized attribute will do the same.</p>
<p>The <code>class!</code> and <code>id!</code> convenience macros allow settings attributes as a list of classes, or a single id, if no other attributes are required. Do not mix and match these with each other, or with attrs!; all but the last-passed will be thrown out.</p>
<div class="sourceCode" id="cb12"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb12-1" title="1"><span class="kw">fn</span> a_component() -&gt; Node&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb12-2" title="2">    <span class="co">// ...</span></a>
<a class="sourceLine" id="cb12-3" title="3">    <span class="pp">span!</span><span class="op">[</span> <span class="pp">class!</span><span class="op">[</span><span class="st">&quot;calculus&quot;</span>, <span class="st">&quot;chemistry&quot;</span>, <span class="st">&quot;literature&quot;</span><span class="op">]</span> <span class="op">]</span>,</a>
<a class="sourceLine" id="cb12-4" title="4">    <span class="pp">span!</span><span class="op">[</span> <span class="pp">id!</span>(<span class="st">&quot;unique-element&quot;</span>) <span class="op">]</span>,</a>
<a class="sourceLine" id="cb12-5" title="5">    <span class="co">// ...</span></a>
<a class="sourceLine" id="cb12-6" title="6"><span class="op">}</span></a></code></pre></div>
<p>You can conditionally add classes with the <code>class!</code> macro:</p>
<div class="sourceCode" id="cb13"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb13-1" title="1"><span class="kw">let</span> active = <span class="cn">true</span>;</a>
<a class="sourceLine" id="cb13-2" title="2"></a>
<a class="sourceLine" id="cb13-3" title="3"><span class="pp">class!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb13-4" title="4">    <span class="st">&quot;blue&quot;</span>,</a>
<a class="sourceLine" id="cb13-5" title="5">    <span class="st">&quot;highlighted&quot;</span> =&gt; active,</a>
<a class="sourceLine" id="cb13-6" title="6">    <span class="st">&quot;confusing&quot;</span> =&gt; <span class="dv">0.99999</span> == <span class="dv">1</span></a>
<a class="sourceLine" id="cb13-7" title="7">    </a>
<a class="sourceLine" id="cb13-8" title="8"><span class="op">]</span></a></code></pre></div>
<p>Styles and Attrs can be passed as refs as well, which is useful if you need to pass the same one more than once:</p>
<div class="sourceCode" id="cb14"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb14-1" title="1"><span class="kw">fn</span> a_component() -&gt; Node&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb14-2" title="2">    <span class="kw">let</span> item_style = <span class="pp">style!</span><span class="op">{</span></a>
<a class="sourceLine" id="cb14-3" title="3">        <span class="pp">St::</span>MarginTop =&gt; px(<span class="dv">10</span>);</a>
<a class="sourceLine" id="cb14-4" title="4">        <span class="pp">St::</span>FontSize =&gt; <span class="pp">unit!</span>(<span class="dv">1.2</span>, em)</a>
<a class="sourceLine" id="cb14-5" title="5">    <span class="op">}</span>;</a>
<a class="sourceLine" id="cb14-6" title="6"></a>
<a class="sourceLine" id="cb14-7" title="7">    <span class="pp">div!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb14-8" title="8">        <span class="pp">ul!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb14-9" title="9">            <span class="pp">li!</span><span class="op">[</span> &amp;item_style, <span class="st">&quot;Item 1&quot;</span>, <span class="op">]</span>,</a>
<a class="sourceLine" id="cb14-10" title="10">            <span class="pp">li!</span><span class="op">[</span> &amp;item_style, <span class="st">&quot;Item 2&quot;</span>, <span class="op">]</span>,</a>
<a class="sourceLine" id="cb14-11" title="11">        <span class="op">]</span></a>
<a class="sourceLine" id="cb14-12" title="12">    <span class="op">]</span></a>
<a class="sourceLine" id="cb14-13" title="13"><span class="op">}</span></a></code></pre></div>
<p>Setting an InputElement’s <code>checked</code>, or <code>autofocus</code> property is done through normal attributes:</p>
<div class="sourceCode" id="cb15"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb15-1" title="1"><span class="kw">fn</span> a_component() -&gt; Node&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb15-2" title="2">    <span class="co">// ...</span></a>
<a class="sourceLine" id="cb15-3" title="3">    <span class="pp">input!</span><span class="op">[</span> <span class="pp">attrs!</span><span class="op">{</span><span class="pp">At::</span>Typed =&gt; <span class="st">&quot;checkbox&quot;</span>; <span class="pp">At::</span>Checked =&gt; <span class="cn">true</span><span class="op">}</span> <span class="op">]</span></a>
<a class="sourceLine" id="cb15-4" title="4">    <span class="pp">input!</span><span class="op">[</span> <span class="pp">attrs!</span><span class="op">{</span><span class="pp">At::</span>Autofocus =&gt; <span class="cn">true</span><span class="op">}</span> <span class="op">]</span></a>
<a class="sourceLine" id="cb15-5" title="5">    <span class="co">// ...</span></a>
<a class="sourceLine" id="cb15-6" title="6"><span class="op">}</span></a></code></pre></div>
<p>To change Attrs or Styles you’ve created, edit their .vals HashMap. To add a new part to them, use their .add method:</p>
<div class="sourceCode" id="cb16"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb16-1" title="1"><span class="kw">let</span> <span class="kw">mut</span> attributes = <span class="pp">attrs!</span><span class="op">{}</span>;</a>
<a class="sourceLine" id="cb16-2" title="2">attributes.add(<span class="pp">At::</span>Class, <span class="st">&quot;truckloads&quot;</span>);</a></code></pre></div>
<p>Example of the style tag, and how you can use pattern-matching in views:</p>
<div class="sourceCode" id="cb17"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb17-1" title="1"><span class="kw">fn</span> view(model: &amp;Model) -&gt; Node&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb17-2" title="2">    <span class="pp">div!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span></a>
<a class="sourceLine" id="cb17-3" title="3">        St:<span class="bu">Display</span> =&gt; <span class="st">&quot;grid&quot;</span>;</a>
<a class="sourceLine" id="cb17-4" title="4">        <span class="pp">St::</span>GridTemplateColumns =&gt; <span class="st">&quot;auto&quot;</span>;</a>
<a class="sourceLine" id="cb17-5" title="5">        <span class="pp">St::</span>GridTemplateRows =&gt; <span class="st">&quot;100px auto 100px&quot;</span></a>
<a class="sourceLine" id="cb17-6" title="6">        <span class="op">}</span>,</a>
<a class="sourceLine" id="cb17-7" title="7">        <span class="pp">section!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>GridRow =&gt; <span class="st">&quot;1 / 2&quot;</span><span class="op">}</span>,</a>
<a class="sourceLine" id="cb17-8" title="8">            header(),</a>
<a class="sourceLine" id="cb17-9" title="9">        <span class="op">]</span>,</a>
<a class="sourceLine" id="cb17-10" title="10">        <span class="pp">section!</span><span class="op">[</span> <span class="pp">attrs!</span><span class="op">{</span><span class="pp">St::</span>GridRow =&gt; <span class="st">&quot;2 / 3&quot;</span><span class="op">}</span>,</a>
<a class="sourceLine" id="cb17-11" title="11">            <span class="kw">match</span> model.page <span class="op">{</span></a>
<a class="sourceLine" id="cb17-12" title="12">                <span class="pp">Page::</span>Guide =&gt; guide(),</a>
<a class="sourceLine" id="cb17-13" title="13">                <span class="pp">Page::</span>Changelog =&gt; changelog(),</a>
<a class="sourceLine" id="cb17-14" title="14">            <span class="op">}</span>,</a>
<a class="sourceLine" id="cb17-15" title="15">        <span class="op">]</span>,</a>
<a class="sourceLine" id="cb17-16" title="16">        <span class="pp">section!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>GridRow =&gt; <span class="st">&quot;3 / 4&quot;</span><span class="op">}</span>,</a>
<a class="sourceLine" id="cb17-17" title="17">            footer()</a>
<a class="sourceLine" id="cb17-18" title="18">        <span class="op">]</span></a>
<a class="sourceLine" id="cb17-19" title="19">    <span class="op">]</span></a>
<a class="sourceLine" id="cb17-20" title="20"><span class="op">}</span></a></code></pre></div>
<p>We can combine Attrs and Style instances using their <code>merge</code> methods, which take an &amp;Attrs and &amp;Style respectively. This can be used to compose styles from reusable parts. Example:</p>
<div class="sourceCode" id="cb18"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb18-1" title="1"><span class="kw">fn</span> a_component() -&gt; Node&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb18-2" title="2">    <span class="kw">let</span> base_style = !style<span class="op">{</span><span class="st">&quot;color&quot;</span> =&gt; <span class="st">&quot;lavender&quot;</span><span class="op">}</span>;</a>
<a class="sourceLine" id="cb18-3" title="3"></a>
<a class="sourceLine" id="cb18-4" title="4">    <span class="pp">div!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb18-5" title="5">        <span class="pp">h1!</span><span class="op">[</span> &amp;base_style.merge(&amp;<span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>GridRow =&gt; <span class="st">&quot;1 / 2&quot;</span><span class="op">}</span>) <span class="st">&quot;First row&quot;</span> <span class="op">]</span>,</a>
<a class="sourceLine" id="cb18-6" title="6">        <span class="pp">h1!</span><span class="op">[</span> &amp;base_style.merge(&amp;<span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>GridRow =&gt; <span class="st">&quot;2 / 3&quot;</span><span class="op">}</span>) <span class="st">&quot;Second row&quot;</span> <span class="op">]</span>,</a>
<a class="sourceLine" id="cb18-7" title="7">    <span class="op">]</span></a>
<a class="sourceLine" id="cb18-8" title="8"><span class="op">}</span></a></code></pre></div>
<p>Overall: we leverage of Rust’s strict type system to flexibly-create the view using normal Rust code.W</p>
<p><code>El</code> has several helper methods which can be chained together:</p>
<div class="sourceCode" id="cb19"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb19-1" title="1"><span class="kw">let</span> my_el = <span class="pp">div!</span><span class="op">[]</span></a>
<a class="sourceLine" id="cb19-2" title="2">    .add_text(<span class="st">&quot;Words&quot;</span>)</a>
<a class="sourceLine" id="cb19-3" title="3">    .add_class(<span class="st">&quot;complete&quot;</span>)</a>
<a class="sourceLine" id="cb19-4" title="4">    .add_attr(<span class="st">&quot;alt&quot;</span>.to_string(), <span class="st">&quot;a description&quot;</span>.to_string())</a>
<a class="sourceLine" id="cb19-5" title="5">    .add_style(<span class="pp">St::</span>Height, <span class="st">&quot;20px&quot;</span>.to_string())</a>
<a class="sourceLine" id="cb19-6" title="6">    .replace_text(<span class="st">&quot;OOps, not complete&quot;</span>);</a></code></pre></div>
<h2 id="initializing">Initializing</h2>
<p>To start your app, call the <code>seed::App::build</code> method, which takes the following parameters:</p>
<ul>
<li>An <code>init</code> function which accepts an initial routing, initial orders, and outputs an initial model</li>
<li>Your update function</li>
<li>Your view function</li>
</ul>
<p>You can can chain the following optional methods:</p>
<ul>
<li><code>.mount()</code> to mount in an element other than the one with id <code>app</code>.</li>
<li><code>.routes(routes)</code> to set a HashMap of landing-page routings, used to initialize your state based on url (See the <code>Routing</code> section)</li>
<li><code>.window_events(window_events)</code>, to set a function describing events on the <code>Window</code>. (See the <code>Events</code> section)</li>
</ul>
<p>And must must complete with these methods: <code>.finish().run()</code>.</p>
<p><code>.mount()</code> takes a single argument, which can be the id of the element you wish to mount in, a <code>web_sys::Element</code>, or a <code>web_sys::HtmlElement</code>. Examples: <code>seed::App::build(|_, _| Model::default(), update, view).mount(seed::body())</code> <code>seed::App::build(|_, _| Model::default(), update, view).mount('a_div_id</code>)`</p>
<pre><code>seed::App::build(|_, _| Model::default(), update, view).mount(
    seed::body().querySelector(&quot;section&quot;).unwrap().unwrap()
)</code></pre>
<p>The <code>seed::App::build</code> call must be wrapped in a function with the <code>#[wasm_bindgen(start)]</code> invocation.</p>
<p>Example, with optional methods:</p>
<div class="sourceCode" id="cb21"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb21-1" title="1"><span class="at">#[</span>wasm_bindgen<span class="at">(</span>start<span class="at">)]</span></a>
<a class="sourceLine" id="cb21-2" title="2"><span class="kw">pub</span> <span class="kw">fn</span> render() <span class="op">{</span></a>
<a class="sourceLine" id="cb21-3" title="3">    <span class="pp">seed::App::</span>build(|_, _| <span class="pp">Model::</span><span class="kw">default</span>(), update, view)</a>
<a class="sourceLine" id="cb21-4" title="4">        .mount(<span class="st">&quot;main&quot;</span>)</a>
<a class="sourceLine" id="cb21-5" title="5">        .routes(routes)</a>
<a class="sourceLine" id="cb21-6" title="6">        .window_events(window_events)</a>
<a class="sourceLine" id="cb21-7" title="7">        .finish()</a>
<a class="sourceLine" id="cb21-8" title="8">        .run();</a>
<a class="sourceLine" id="cb21-9" title="9"><span class="op">}</span></a></code></pre></div>
<p>Example of using a standalone <code>init</code> function:</p>
<div class="sourceCode" id="cb22"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb22-1" title="1"><span class="kw">fn</span> init(url: Url, orders: &amp;<span class="kw">mut</span> <span class="kw">impl</span> Orders&lt;Msg&gt;) -&gt; Model <span class="op">{</span></a>
<a class="sourceLine" id="cb22-2" title="2">    <span class="kw">let</span> <span class="kw">mut</span> model = <span class="pp">Model::</span><span class="kw">default</span>();</a>
<a class="sourceLine" id="cb22-3" title="3">    update(routes(url).unwrap(), &amp;<span class="kw">mut</span> model, orders);</a>
<a class="sourceLine" id="cb22-4" title="4">    model</a>
<a class="sourceLine" id="cb22-5" title="5"><span class="op">}</span></a>
<a class="sourceLine" id="cb22-6" title="6"></a>
<a class="sourceLine" id="cb22-7" title="7"><span class="at">#[</span>wasm_bindgen<span class="at">(</span>start<span class="at">)]</span></a>
<a class="sourceLine" id="cb22-8" title="8"><span class="kw">pub</span> <span class="kw">fn</span> render() <span class="op">{</span></a>
<a class="sourceLine" id="cb22-9" title="9">    <span class="pp">seed::App::</span>build(init, update, view)</a>
<a class="sourceLine" id="cb22-10" title="10">        .finish()</a>
<a class="sourceLine" id="cb22-11" title="11">        .run();</a>
<a class="sourceLine" id="cb22-12" title="12"><span class="op">}</span></a></code></pre></div>
<p>This will render your app to the element holding the id you passed; in the case of this example, “main”. The only part of the web page Seed will interact with is that element, so you can use other HTML not part of Seed, or other JS code/frameworks in the same document.</p>
"#####.into()
}