pub fn text() -> String {
r#####"
<h1 id="view">View</h1>
<p>Visual layout (ie HTML/DOM elements) is described declaratively in Rust, and uses <a href="https://doc.rust-lang.org/book/appendix-04-macros.html">macros</a> to simplify syntax. Each element is represented by a macro, eg <code>div![]</code>. These act as functions that accept an arbitrary number of parameters, in any order: They handle the parameters based exclusively on type.</p>
<p>The view’s defined by a function that’s passed to <a href="https://docs.rs/seed/0.4.2/seed/struct.App.html#method.build">seed::App::build</a>. This takes a <code>&amp;Model</code> as its parameter, and outputs something that implements the <code>View</code> trait, which is imported in the prelude. Usually, this is a <code>Node</code>, or <code>Vec&lt;Node&gt;</code>, representing all nodes that will be inserted as children on the top-level one. (The top-level <code>Node</code> is in the html file, and specified with <a href="https://docs.rs/seed/0.4.2/seed/struct.AppBuilder.html#method.mount">seed::App::build.mount()</a>, or as a default, the element with id <code>app</code>). It may composed into sub-functions, which can be thought of like components in other frameworks.</p>
<p>Examples:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb1-1"><a href="#cb1-1"></a><span class="kw">fn</span> view(model: &amp;Model) -&gt; Node&lt;Msg&gt; <span class="op">{</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>    <span class="pp">h1!</span><span class="op">[</span> <span class="st">&quot;Let there be light&quot;</span> <span class="op">]</span>,</span>
<span id="cb1-3"><a href="#cb1-3"></a><span class="op">}</span></span></code></pre></div>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb2-1"><a href="#cb2-1"></a><span class="kw">fn</span> view(model: &amp;Model) -&gt; <span class="dt">Vec</span>&lt;Node&lt;Msg&gt;&gt; <span class="op">{</span></span>
<span id="cb2-2"><a href="#cb2-2"></a>    <span class="pp">vec!</span><span class="op">[</span></span>
<span id="cb2-3"><a href="#cb2-3"></a>        <span class="pp">h1!</span><span class="op">[</span> <span class="st">&quot;Let there be light&quot;</span> <span class="op">]</span>,</span>
<span id="cb2-4"><a href="#cb2-4"></a>        <span class="pp">h2!</span><span class="op">[</span> <span class="st">&quot;Let it be both a particle and a wave&quot;</span> <span class="op">]</span></span>
<span id="cb2-5"><a href="#cb2-5"></a>    <span class="op">]</span></span>
<span id="cb2-6"><a href="#cb2-6"></a><span class="op">}</span></span></code></pre></div>
<p>In either of those examples, you could use the signature: <code>fn view(model: &amp;Model) -&gt; impl View&lt;Msg&gt;</code> instead. This allows you to change between them without changing the function signature.</p>
<h2 id="the-node-enum">The Node Enum</h2>
<p>The Virtual DOM is represnted by nested <a href="https://docs.rs/seed/0.4.2/seed/dom_types/enum.Node.html">Nodes</a>. <code>Node</code> has 3 variants:</p>
<ul>
<li><code>Text</code> holds a <a href="https://docs.rs/seed/0.4.2/seed/dom_types/struct.Text.html">Text</a> struct. Mostly for internal use, but can be created with <code>Node::new_text()</code>.</li>
<li><code>Element</code> wraps an <a href="https://docs.rs/seed/0.4.2/seed/dom_types/struct.El.html">El</a>, which is the main component of our VDOM. Created using macros, described below.</li>
<li><code>Empty</code> is a placeholder that doens’t render anything; useful in conditional/ternary logic. Created using the <code>empty![]</code> macro, or <code>seed::empty()</code>.</li>
</ul>
<h2 id="elements-attributes-styles">Elements, attributes, styles</h2>
<p>Elements are created using macros, named by the lowercase name of each element, and imported into the global namespace. Eg <code>div!</code> above. We use this code to import them:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb3-1"><a href="#cb3-1"></a><span class="at">#[</span>macro_use<span class="at">]</span></span>
<span id="cb3-2"><a href="#cb3-2"></a><span class="kw">extern</span> <span class="kw">crate</span> seed;</span></code></pre></div>
<p>These macros accept any combination of the following parameters: - One <a href="https://docs.rs/seed/0.4.2/seed/dom_types/struct.Attrs.html">Attrs</a> struct - One <a href="https://docs.rs/seed/0.4.2/seed/dom_types/struct.Style.html">Style</a> struct - One or more <a href="https://docs.rs/seed/0.4.2/seed/dom_types/struct.Listener.html">Listener</a> structs, which handle events - One or more <code>Vec</code>s of <code>Listener</code> structs - One <code>String</code> or <code>&amp;str</code> representing a node text - One or more <a href="https://docs.rs/seed/0.4.2/seed/dom_types/enum.Node.html">Node</a> structs, representing a child - One or more Vecs of <code>Node</code> structs, representing multiple children - A <code>Map</code>, ie the result of <code>map()</code>, yielding <code>Node</code>s or <code>Listener</code>s, without having to explicitly <code>collect</code>.</p>
<p>The parameters can be passed in any order; the compiler knows how to handle them based on their types. Children are rendered in the order passed.</p>
<p>Views are described using <a href="https://docs.rs/seed/0.4.2/seed/dom_types/struct.El.html">El</a> structs, defined in the <a href="https://docs.rs/seed/0.4.2/seed/dom_types/index.html">seed::dom_types</a> module.</p>
<p><code>Attrs</code> and <code>Style</code> are thinly-wrapped hashmaps created with their own macros: <code>attrs!{}</code> and <code>style!{}</code> respectively.</p>
<p>Example:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb4-1"><a href="#cb4-1"></a><span class="kw">fn</span> view(model: &amp;Model) -&gt; <span class="kw">impl</span> View&lt;Msg&gt; <span class="op">{</span></span>
<span id="cb4-2"><a href="#cb4-2"></a>    <span class="kw">let</span> things = <span class="pp">vec!</span><span class="op">[</span> <span class="pp">h4!</span><span class="op">[</span> <span class="st">&quot;thing1&quot;</span> <span class="op">]</span>, <span class="pp">h4!</span><span class="op">[</span> <span class="st">&quot;thing2&quot;</span> <span class="op">]</span> <span class="op">]</span>;</span>
<span id="cb4-3"><a href="#cb4-3"></a>    </span>
<span id="cb4-4"><a href="#cb4-4"></a>    <span class="kw">let</span> other_things = <span class="pp">vec!</span><span class="op">[</span><span class="dv">1</span>, <span class="dv">2</span><span class="op">]</span>;</span>
<span id="cb4-5"><a href="#cb4-5"></a></span>
<span id="cb4-6"><a href="#cb4-6"></a>    <span class="pp">div!</span><span class="op">[</span> <span class="pp">attrs!</span><span class="op">{</span><span class="pp">At::</span>Class =&gt; <span class="st">&quot;hardly-any&quot;</span><span class="op">}</span>, </span>
<span id="cb4-7"><a href="#cb4-7"></a>        things,  <span class="co">// Vec&lt;Node&lt;Msg&gt;</span></span>
<span id="cb4-8"><a href="#cb4-8"></a>        other_things.map(|t| <span class="pp">h4!</span><span class="op">[</span>t.to_string()<span class="op">]</span>),  <span class="co">// Map</span></span>
<span id="cb4-9"><a href="#cb4-9"></a>        <span class="pp">h4!</span><span class="op">[</span> <span class="st">&quot;thing3?&quot;</span> <span class="op">]</span>,  <span class="co">// El</span></span>
<span id="cb4-10"><a href="#cb4-10"></a>    <span class="op">]</span></span>
<span id="cb4-11"><a href="#cb4-11"></a><span class="op">}</span></span></code></pre></div>
<p>Note that you can create any of the above items inside an element macro, or create it separately, and pass it in. You can separate different items by comma, semicolon, or space.</p>
<p>Keys passed to <code>attrs!</code> can be <code>Seed::At</code>s, <code>String</code>s, or <code>&amp;str</code>s. Keys passed to <code>style!</code> can be <code>Seed::St</code>s, <code>String</code>s, or <code>&amp;str</code>s. Values passed to <code>attrs!</code>, and <code>style!</code> macros can be owned <code>Strings</code>, <code>&amp;str</code>s, or for <code>style!</code>, <code>unit</code>s.</p>
<p>You use the <code>unit!</code> macro to apply units. There’s a <code>px</code> function for the special case where the unit is pixels:</p>
<div class="sourceCode" id="cb5"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb5-1"><a href="#cb5-1"></a><span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>Width =&gt; <span class="pp">unit!</span>(<span class="dv">20</span>, px);<span class="op">}</span></span>
<span id="cb5-2"><a href="#cb5-2"></a><span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>Width =&gt; px(<span class="dv">20</span>);<span class="op">}</span>  <span class="co">// equivalent</span></span></code></pre></div>
<p>Some types, like <code>Option</code>s, implement a trait allowing them to be used directly in <code>style!</code>:</p>
<div class="sourceCode" id="cb6"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb6-1"><a href="#cb6-1"></a><span class="kw">let</span> display: &amp;<span class="dt">str</span> = <span class="st">&quot;flex&quot;</span>;</span>
<span id="cb6-2"><a href="#cb6-2"></a><span class="kw">let</span> direction: <span class="dt">String</span> = <span class="st">&quot;column&quot;</span>.to_string();</span>
<span id="cb6-3"><a href="#cb6-3"></a><span class="kw">let</span> order: <span class="dt">Option</span>&lt;<span class="dt">u32</span>&gt; = <span class="cn">None</span>;</span>
<span id="cb6-4"><a href="#cb6-4"></a><span class="kw">let</span> gap: <span class="dt">Option</span>&lt;&amp;<span class="dt">str</span>&gt; = <span class="cn">Some</span>(<span class="st">&quot;8px&quot;</span>);</span>
<span id="cb6-5"><a href="#cb6-5"></a></span>
<span id="cb6-6"><a href="#cb6-6"></a><span class="kw">let</span> style = <span class="pp">style!</span><span class="op">[</span></span>
<span id="cb6-7"><a href="#cb6-7"></a>    <span class="pp">St::</span><span class="bu">Display</span> =&gt; display,</span>
<span id="cb6-8"><a href="#cb6-8"></a>    <span class="pp">St::</span>FlexDirection =&gt; direction,</span>
<span id="cb6-9"><a href="#cb6-9"></a>    <span class="pp">St::</span>Order =&gt; order,</span>
<span id="cb6-10"><a href="#cb6-10"></a>    <span class="pp">St::</span>Gap =&gt; gap,</span>
<span id="cb6-11"><a href="#cb6-11"></a><span class="op">]</span>;</span></code></pre></div>
<p>We can set multiple values for an attribute using <a href="https://docs.rs/seed/0.4.2/seed/dom_types/struct.Attrs.html#method.add_multiple">Attribute.add_multiple</a>. This is useful for setting multiple classes. Note that we must set this up outside of the view macro, since it involves modifying a variable:</p>
<div class="sourceCode" id="cb7"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb7-1"><a href="#cb7-1"></a><span class="kw">fn</span> a_component() -&gt; Node&lt;Msg&gt; <span class="op">{</span></span>
<span id="cb7-2"><a href="#cb7-2"></a>    <span class="kw">let</span> <span class="kw">mut</span> attributes = <span class="pp">attrs!</span><span class="op">{}</span>;</span>
<span id="cb7-3"><a href="#cb7-3"></a>    attributes.add_multiple(<span class="pp">At::</span>Class, <span class="pp">vec!</span><span class="op">[</span><span class="st">&quot;A-modicum-of&quot;</span>, <span class="st">&quot;hardly-any&quot;</span><span class="op">]</span>);</span>
<span id="cb7-4"><a href="#cb7-4"></a></span>
<span id="cb7-5"><a href="#cb7-5"></a>    <span class="pp">div!</span><span class="op">[</span> attributes <span class="op">]</span></span>
<span id="cb7-6"><a href="#cb7-6"></a><span class="op">}</span></span></code></pre></div>
<p>Seed validates attributes <a href="https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes">against this list</a>; The <a href="https://docs.rs/seed/0.4.2/seed/dom_types/enum.At.html">At</a> enum includes only these values, and <code>&amp;strs</code> passed are converted into <code>At</code>s. If you wish to use a custom attribute, use <a href="https://docs.rs/seed/0.4.2/seed/dom_types/enum.At.html#variant.Custom">At::Custom</a> , eg <code>At::Custom(name)</code>, where <code>name</code> is a <code>String</code> of your attribute’s name. In <code>attrs!</code> when using <code>&amp;str</code>s, inserting an unrecognized attribute will do the same. Similar <code>Custom</code> methods exist for <a href="https://docs.rs/seed/0.4.2/seed/dom_entity_names/styles/enum.St.html#variant.Custom">Style</a>, <a href="https://docs.rs/seed/0.4.2/seed/dom_types/enum.Namespace.html#variant.Custom">Namespace</a>, <a href="https://docs.rs/seed/0.4.2/seed/dom_types/enum.Tag.html#variant.Custom">Tag</a>, and <a href="https://docs.rs/seed/0.4.2/seed/events/enum.Category.html#variant.Custom">Category</a>.</p>
<p>The <code>class!</code> and <code>id!</code> convenience macros allow settings attributes as a list of classes, or a single id, if no other attributes are required. Do not mix and match these with each other, or with attrs!; all but the last-passed will be thrown out.</p>
<div class="sourceCode" id="cb8"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb8-1"><a href="#cb8-1"></a><span class="kw">fn</span> a_component() -&gt; Node&lt;Msg&gt; <span class="op">{</span></span>
<span id="cb8-2"><a href="#cb8-2"></a>    <span class="co">// ...</span></span>
<span id="cb8-3"><a href="#cb8-3"></a>    <span class="pp">span!</span><span class="op">[</span> <span class="pp">class!</span><span class="op">[</span><span class="st">&quot;calculus&quot;</span>, <span class="st">&quot;chemistry&quot;</span>, <span class="st">&quot;literature&quot;</span><span class="op">]</span> <span class="op">]</span>,</span>
<span id="cb8-4"><a href="#cb8-4"></a>    <span class="pp">span!</span><span class="op">[</span> <span class="pp">id!</span>(<span class="st">&quot;unique-element&quot;</span>) <span class="op">]</span>,</span>
<span id="cb8-5"><a href="#cb8-5"></a>    <span class="co">// ...</span></span>
<span id="cb8-6"><a href="#cb8-6"></a><span class="op">}</span></span></code></pre></div>
<p>You can conditionally add classes with the <code>class!</code> macro:</p>
<div class="sourceCode" id="cb9"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb9-1"><a href="#cb9-1"></a><span class="kw">let</span> active = <span class="cn">true</span>;</span>
<span id="cb9-2"><a href="#cb9-2"></a></span>
<span id="cb9-3"><a href="#cb9-3"></a><span class="pp">class!</span><span class="op">[</span></span>
<span id="cb9-4"><a href="#cb9-4"></a>    <span class="st">&quot;blue&quot;</span>,</span>
<span id="cb9-5"><a href="#cb9-5"></a>    <span class="st">&quot;highlighted&quot;</span> =&gt; active,</span>
<span id="cb9-6"><a href="#cb9-6"></a>    <span class="st">&quot;confusing&quot;</span> =&gt; <span class="dv">0.99999</span> == <span class="dv">1</span></span>
<span id="cb9-7"><a href="#cb9-7"></a>    </span>
<span id="cb9-8"><a href="#cb9-8"></a><span class="op">]</span></span></code></pre></div>
<p>Styles and Attrs can be passed as refs as well, which is useful if you need to pass the same one more than once:</p>
<div class="sourceCode" id="cb10"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb10-1"><a href="#cb10-1"></a><span class="kw">fn</span> a_component() -&gt; Node&lt;Msg&gt; <span class="op">{</span></span>
<span id="cb10-2"><a href="#cb10-2"></a>    <span class="kw">let</span> item_style = <span class="pp">style!</span><span class="op">{</span></span>
<span id="cb10-3"><a href="#cb10-3"></a>        <span class="pp">St::</span>MarginTop =&gt; px(<span class="dv">10</span>);</span>
<span id="cb10-4"><a href="#cb10-4"></a>        <span class="pp">St::</span>FontSize =&gt; <span class="pp">unit!</span>(<span class="dv">1.2</span>, em)</span>
<span id="cb10-5"><a href="#cb10-5"></a>    <span class="op">}</span>;</span>
<span id="cb10-6"><a href="#cb10-6"></a></span>
<span id="cb10-7"><a href="#cb10-7"></a>    <span class="pp">div!</span><span class="op">[</span></span>
<span id="cb10-8"><a href="#cb10-8"></a>        <span class="pp">ul!</span><span class="op">[</span></span>
<span id="cb10-9"><a href="#cb10-9"></a>            <span class="pp">li!</span><span class="op">[</span> &amp;item_style, <span class="st">&quot;Item 1&quot;</span>, <span class="op">]</span>,</span>
<span id="cb10-10"><a href="#cb10-10"></a>            <span class="pp">li!</span><span class="op">[</span> &amp;item_style, <span class="st">&quot;Item 2&quot;</span>, <span class="op">]</span>,</span>
<span id="cb10-11"><a href="#cb10-11"></a>        <span class="op">]</span></span>
<span id="cb10-12"><a href="#cb10-12"></a>    <span class="op">]</span></span>
<span id="cb10-13"><a href="#cb10-13"></a><span class="op">}</span></span></code></pre></div>
<p>For boolean attributes that are handled by presense or absense, like <code>disabled</code>, <code>checked</code>, <code>autofocus</code> etc, use <a href="https://docs.rs/seed/0.4.2/seed/dom_types/values/trait.AsAtValue.html#tymethod.as_at_value">.as_at_value</a>: <code>input![ attrs!{At::Disabled =&gt; false.as_at_value() ]</code>:</p>
<div class="sourceCode" id="cb11"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb11-1"><a href="#cb11-1"></a><span class="kw">fn</span> a_component() -&gt; Node&lt;Msg&gt; <span class="op">{</span></span>
<span id="cb11-2"><a href="#cb11-2"></a>    <span class="co">// ...</span></span>
<span id="cb11-3"><a href="#cb11-3"></a>    <span class="pp">input!</span><span class="op">[</span> <span class="pp">attrs!</span><span class="op">{</span><span class="pp">At::</span>Typed =&gt; <span class="st">&quot;checkbox&quot;</span>; <span class="pp">At::</span>Checked =&gt; <span class="cn">true</span>.as_at_value()<span class="op">}</span> <span class="op">]</span></span>
<span id="cb11-4"><a href="#cb11-4"></a>    <span class="pp">input!</span><span class="op">[</span> <span class="pp">attrs!</span><span class="op">{</span><span class="pp">At::</span>Autofocus =&gt; <span class="cn">true</span>.as_at_value()<span class="op">}</span> <span class="op">]</span></span>
<span id="cb11-5"><a href="#cb11-5"></a>    <span class="co">// ...</span></span>
<span id="cb11-6"><a href="#cb11-6"></a><span class="op">}</span></span></code></pre></div>
<p><code>At::Checked =&gt; true.as_at_value()</code> is equivalent to the presense of a <code>checked</code> attribute, and <code>At::Checked =&gt; false.as_at_value()</code> is equivalent to ommitting it.</p>
<p>To change Attrs or Styles you’ve created, edit their .vals HashMap. To add a new part to them, use their .add method:</p>
<div class="sourceCode" id="cb12"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb12-1"><a href="#cb12-1"></a><span class="kw">let</span> <span class="kw">mut</span> attributes = <span class="pp">attrs!</span><span class="op">{}</span>;</span>
<span id="cb12-2"><a href="#cb12-2"></a>attributes.add(<span class="pp">At::</span>Class, <span class="st">&quot;truckloads&quot;</span>);</span></code></pre></div>
<p>Example of the style tag, and how you can use pattern-matching in views:</p>
<div class="sourceCode" id="cb13"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb13-1"><a href="#cb13-1"></a><span class="kw">fn</span> view(model: &amp;Model) -&gt; <span class="kw">impl</span> View&lt;Msg&gt; <span class="op">{</span></span>
<span id="cb13-2"><a href="#cb13-2"></a>    <span class="pp">div!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span></span>
<span id="cb13-3"><a href="#cb13-3"></a>        <span class="pp">St::</span><span class="bu">Display</span> =&gt; <span class="st">&quot;grid&quot;</span>;</span>
<span id="cb13-4"><a href="#cb13-4"></a>        <span class="pp">St::</span>GridTemplateColumns =&gt; <span class="st">&quot;auto&quot;</span>;</span>
<span id="cb13-5"><a href="#cb13-5"></a>        <span class="pp">St::</span>GridTemplateRows =&gt; <span class="st">&quot;100px auto 100px&quot;</span></span>
<span id="cb13-6"><a href="#cb13-6"></a>        <span class="op">}</span>,</span>
<span id="cb13-7"><a href="#cb13-7"></a>        <span class="pp">section!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>GridRow =&gt; <span class="st">&quot;1 / 2&quot;</span><span class="op">}</span>,</span>
<span id="cb13-8"><a href="#cb13-8"></a>            header(),</span>
<span id="cb13-9"><a href="#cb13-9"></a>        <span class="op">]</span>,</span>
<span id="cb13-10"><a href="#cb13-10"></a>        <span class="pp">section!</span><span class="op">[</span> <span class="pp">attrs!</span><span class="op">{</span><span class="pp">St::</span>GridRow =&gt; <span class="st">&quot;2 / 3&quot;</span><span class="op">}</span>,</span>
<span id="cb13-11"><a href="#cb13-11"></a>            <span class="kw">match</span> model.page <span class="op">{</span></span>
<span id="cb13-12"><a href="#cb13-12"></a>                <span class="pp">Page::</span>Guide =&gt; guide(),</span>
<span id="cb13-13"><a href="#cb13-13"></a>                <span class="pp">Page::</span>Changelog =&gt; changelog(),</span>
<span id="cb13-14"><a href="#cb13-14"></a>            <span class="op">}</span>,</span>
<span id="cb13-15"><a href="#cb13-15"></a>        <span class="op">]</span>,</span>
<span id="cb13-16"><a href="#cb13-16"></a>        <span class="pp">section!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>GridRow =&gt; <span class="st">&quot;3 / 4&quot;</span><span class="op">}</span>,</span>
<span id="cb13-17"><a href="#cb13-17"></a>            footer()</span>
<span id="cb13-18"><a href="#cb13-18"></a>        <span class="op">]</span></span>
<span id="cb13-19"><a href="#cb13-19"></a>    <span class="op">]</span></span>
<span id="cb13-20"><a href="#cb13-20"></a><span class="op">}</span></span></code></pre></div>
<p>We can combine Attrs and <code>Style</code> instances using their <a href="https://docs.rs/seed/0.4.2/seed/dom_types/struct.Attrs.html#method.merge">merge</a> methods, which take an <code>&amp;Attrs</code> and <code>&amp;Style</code> respectively. This can be used to compose styles from reusable parts. Example:</p>
<div class="sourceCode" id="cb14"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb14-1"><a href="#cb14-1"></a><span class="kw">fn</span> a_component() -&gt; Node&lt;Msg&gt; <span class="op">{</span></span>
<span id="cb14-2"><a href="#cb14-2"></a>    <span class="kw">let</span> base_style = <span class="pp">style!</span><span class="op">{</span><span class="st">&quot;color&quot;</span> =&gt; <span class="st">&quot;lavender&quot;</span><span class="op">}</span>;</span>
<span id="cb14-3"><a href="#cb14-3"></a></span>
<span id="cb14-4"><a href="#cb14-4"></a>    <span class="pp">div!</span><span class="op">[</span></span>
<span id="cb14-5"><a href="#cb14-5"></a>        <span class="pp">h1!</span><span class="op">[</span> &amp;base_style.merge(&amp;<span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>GridRow =&gt; <span class="st">&quot;1 / 2&quot;</span><span class="op">}</span>) <span class="st">&quot;First row&quot;</span> <span class="op">]</span>,</span>
<span id="cb14-6"><a href="#cb14-6"></a>        <span class="pp">h1!</span><span class="op">[</span> &amp;base_style.merge(&amp;<span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>GridRow =&gt; <span class="st">&quot;2 / 3&quot;</span><span class="op">}</span>) <span class="st">&quot;Second row&quot;</span> <span class="op">]</span>,</span>
<span id="cb14-7"><a href="#cb14-7"></a>    <span class="op">]</span></span>
<span id="cb14-8"><a href="#cb14-8"></a><span class="op">}</span></span></code></pre></div>
<p>Perhaps more cleanly, we can use multiple <code>Style</code>s together, to merge their entries:</p>
<div class="sourceCode" id="cb15"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb15-1"><a href="#cb15-1"></a><span class="kw">fn</span> a_component() -&gt; Node&lt;Msg&gt; <span class="op">{</span></span>
<span id="cb15-2"><a href="#cb15-2"></a>    <span class="kw">let</span> base_style = <span class="pp">style!</span><span class="op">{</span><span class="st">&quot;color&quot;</span> =&gt; <span class="st">&quot;lavender&quot;</span><span class="op">}</span>;</span>
<span id="cb15-3"><a href="#cb15-3"></a></span>
<span id="cb15-4"><a href="#cb15-4"></a>    <span class="pp">div!</span><span class="op">[</span></span>
<span id="cb15-5"><a href="#cb15-5"></a>        <span class="pp">h1!</span><span class="op">[</span> </span>
<span id="cb15-6"><a href="#cb15-6"></a>            &amp;base_style, </span>
<span id="cb15-7"><a href="#cb15-7"></a>            <span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>GridRow =&gt; <span class="st">&quot;1 / 2&quot;</span><span class="op">}</span>,</span>
<span id="cb15-8"><a href="#cb15-8"></a>            <span class="st">&quot;First row&quot;</span> <span class="op">]</span>,</span>
<span id="cb15-9"><a href="#cb15-9"></a>        <span class="pp">h1!</span><span class="op">[</span> </span>
<span id="cb15-10"><a href="#cb15-10"></a>            &amp;base_style, </span>
<span id="cb15-11"><a href="#cb15-11"></a>            <span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>GridRow =&gt; <span class="st">&quot;2 / 3&quot;</span><span class="op">}</span>, </span>
<span id="cb15-12"><a href="#cb15-12"></a>            <span class="st">&quot;Second row&quot;</span> <span class="op">]</span>,</span>
<span id="cb15-13"><a href="#cb15-13"></a>    <span class="op">]</span></span>
<span id="cb15-14"><a href="#cb15-14"></a><span class="op">}</span></span></code></pre></div>
<p>Overall: we leverage of Rust’s strict type system to flexibly-create the view using normal Rust code.W</p>
<p><code>El</code> has several helper methods which can be chained together:</p>
<div class="sourceCode" id="cb16"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb16-1"><a href="#cb16-1"></a><span class="kw">let</span> my_el = <span class="pp">div!</span><span class="op">[]</span></span>
<span id="cb16-2"><a href="#cb16-2"></a>    .add_text(<span class="st">&quot;Words&quot;</span>)</span>
<span id="cb16-3"><a href="#cb16-3"></a>    .add_class(<span class="st">&quot;complete&quot;</span>)</span>
<span id="cb16-4"><a href="#cb16-4"></a>    .add_attr(<span class="st">&quot;alt&quot;</span>.to_string(), <span class="st">&quot;a description&quot;</span>.to_string())</span>
<span id="cb16-5"><a href="#cb16-5"></a>    .add_style(<span class="pp">St::</span>Height, <span class="st">&quot;20px&quot;</span>.to_string())</span>
<span id="cb16-6"><a href="#cb16-6"></a>    .replace_text(<span class="st">&quot;Oops, not complete&quot;</span>);oo</span></code></pre></div>
<h2 id="svg">Svg</h2>
<p>You can create <code>SVG</code> elements in the same way as normal <code>Html</code> elements. Setting the <code>xmlns</code> attribute isn’t required; it’s set automatically when using the macro.</p>
<p>Example using macros:</p>
<div class="sourceCode" id="cb17"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb17-1"><a href="#cb17-1"></a><span class="pp">svg!</span><span class="op">[</span></span>
<span id="cb17-2"><a href="#cb17-2"></a>    <span class="pp">rect!</span><span class="op">[</span></span>
<span id="cb17-3"><a href="#cb17-3"></a>        <span class="pp">attrs!</span><span class="op">{</span></span>
<span id="cb17-4"><a href="#cb17-4"></a>            <span class="pp">At::</span>X =&gt; <span class="st">&quot;5&quot;</span>,</span>
<span id="cb17-5"><a href="#cb17-5"></a>            <span class="pp">At::</span>Y =&gt;<span class="st">&quot;5&quot;</span>,</span>
<span id="cb17-6"><a href="#cb17-6"></a>            <span class="pp">At::</span>Width =&gt; <span class="st">&quot;20&quot;</span>,</span>
<span id="cb17-7"><a href="#cb17-7"></a>            <span class="pp">At::</span>Height =&gt; <span class="st">&quot;20&quot;</span>,</span>
<span id="cb17-8"><a href="#cb17-8"></a>            <span class="pp">At::</span>Stroke =&gt; <span class="st">&quot;green&quot;</span>,</span>
<span id="cb17-9"><a href="#cb17-9"></a>            <span class="pp">At::</span>StrokeWidth =&gt; <span class="st">&quot;4&quot;</span>,</span>
<span id="cb17-10"><a href="#cb17-10"></a>        <span class="op">}</span></span>
<span id="cb17-11"><a href="#cb17-11"></a>    <span class="op">]</span></span>
<span id="cb17-12"><a href="#cb17-12"></a><span class="op">]</span></span></code></pre></div>
<p>The same exmaple using <a href="https://docs.rs/seed/0.4.2/seed/dom_types/enum.Node.html#method.from_html">from_html</a>:</p>
<div class="sourceCode" id="cb18"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb18-1"><a href="#cb18-1"></a><span class="pp">Node::</span>from_html(</span>
<span id="cb18-2"><a href="#cb18-2"></a><span class="st">r#&quot;</span></span>
<span id="cb18-3"><a href="#cb18-3"></a><span class="st">&lt;svg&gt;</span></span>
<span id="cb18-4"><a href="#cb18-4"></a><span class="st">    &lt;rect x=&quot;#</span><span class="dv">5</span><span class="st">&quot; y=&quot;</span><span class="dv">5</span><span class="st">&quot; width=&quot;</span><span class="dv">20</span><span class="st">&quot; height=&quot;</span><span class="dv">20</span><span class="st">&quot; stroke=&quot;</span>green<span class="st">&quot; stroke-width=&quot;</span><span class="dv">4</span><span class="st">&quot; /&gt;</span></span>
<span id="cb18-5"><a href="#cb18-5"></a><span class="st">&lt;/svg&gt;</span></span>
<span id="cb18-6"><a href="#cb18-6"></a><span class="st">&quot;</span>#)</span></code></pre></div>
<p>Another example, showing it in the <code>View</code> fn:</p>
<div class="sourceCode" id="cb19"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb19-1"><a href="#cb19-1"></a><span class="kw">fn</span> view(model: &amp;Model) -&gt; <span class="dt">Vec</span>&lt;Node&lt;Msg&gt;&gt; <span class="op">{</span></span>
<span id="cb19-2"><a href="#cb19-2"></a>    <span class="pp">vec!</span><span class="op">[</span></span>
<span id="cb19-3"><a href="#cb19-3"></a>        <span class="pp">svg!</span><span class="op">[</span></span>
<span id="cb19-4"><a href="#cb19-4"></a>            <span class="pp">attrs!</span><span class="op">{</span></span>
<span id="cb19-5"><a href="#cb19-5"></a>                <span class="pp">At::</span>Width =&gt; <span class="st">&quot;100%&quot;</span>;</span>
<span id="cb19-6"><a href="#cb19-6"></a>                <span class="pp">At::</span>Height =&gt; <span class="st">&quot;100%&quot;</span>;</span>
<span id="cb19-7"><a href="#cb19-7"></a>                <span class="pp">At::</span>ViewBox =&gt; <span class="st">&quot;0 0 512 512&quot;</span>;</span>
<span id="cb19-8"><a href="#cb19-8"></a>            <span class="op">}</span>,</span>
<span id="cb19-9"><a href="#cb19-9"></a>            <span class="pp">path!</span><span class="op">[</span> </span>
<span id="cb19-10"><a href="#cb19-10"></a>                <span class="pp">attrs!</span><span class="op">{</span></span>
<span id="cb19-11"><a href="#cb19-11"></a>                    <span class="pp">At::</span>Fill =&gt; <span class="st">&quot;lightgrey&quot;</span>;</span>
<span id="cb19-12"><a href="#cb19-12"></a>                    <span class="pp">At::</span>D =&gt; <span class="st">&quot;M345.863,281.853c19.152-8.872,38.221-15.344,56.1&quot;</span>  <span class="co">// etc</span></span>
<span id="cb19-13"><a href="#cb19-13"></a>                <span class="op">}</span></span>
<span id="cb19-14"><a href="#cb19-14"></a>            <span class="op">]</span>,</span>
<span id="cb19-15"><a href="#cb19-15"></a>            <span class="co">// More elements as required, eg mesh, polyline, circle</span></span>
<span id="cb19-16"><a href="#cb19-16"></a>        <span class="op">]</span></span>
<span id="cb19-17"><a href="#cb19-17"></a>    <span class="op">]</span></span>
<span id="cb19-18"><a href="#cb19-18"></a><span class="op">}</span></span></code></pre></div>
<h2 id="canvas-unreleased-for-now-you-can-use-web_sys-directly.">Canvas (unreleased; for now, you can use <code>web_sys</code> directly.</h2>
<p>Seed provides helper functions for use with <code>Canvas</code>:</p>
<div class="sourceCode" id="cb20"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb20-1"><a href="#cb20-1"></a><span class="kw">fn</span> draw() <span class="op">{</span></span>
<span id="cb20-2"><a href="#cb20-2"></a>    <span class="kw">let</span> canvas = <span class="pp">seed::</span>canvas(<span class="st">&quot;canvas&quot;</span>).unwrap();</span>
<span id="cb20-3"><a href="#cb20-3"></a>    <span class="kw">let</span> ctx = <span class="pp">seed::</span>canvas_context_2d(&amp;canvas);</span>
<span id="cb20-4"><a href="#cb20-4"></a></span>
<span id="cb20-5"><a href="#cb20-5"></a>    ctx.move_to(<span class="dv">0</span>., <span class="dv">0</span>.);</span>
<span id="cb20-6"><a href="#cb20-6"></a>    ctx.line_to(<span class="dv">200</span>., <span class="dv">100</span>.);</span>
<span id="cb20-7"><a href="#cb20-7"></a>    ctx.stroke();</span>
<span id="cb20-8"><a href="#cb20-8"></a><span class="op">}</span></span></code></pre></div>
<p>#[wasm_bindgen(start)] pub fn render() { seed::App::build(|<em>, </em>| Init::new(Model {}), update, view).build_and_start(); draw(); }</p>
<h2 id="components">Components</h2>
<p>The analog of components in frameworks like React are normal Rust functions that that return <a href="https://docs.rs/seed/0.4.2/seed/dom_types/enum.Node.html">Node</a> s. These functions take parameters that are not treated in a way equivalent to attributes on native DOM elements; they just provide a way to organize your code. In practice, they’re used in a way similar to components in React.</p>
<p>For example, you could organize one of the examples in the Structure section of the guide like this:</p>
<div class="sourceCode" id="cb21"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb21-1"><a href="#cb21-1"></a>    <span class="kw">fn</span> text_display(text: &amp;<span class="dt">str</span>) -&gt; Node&lt;Msg&gt; <span class="op">{</span></span>
<span id="cb21-2"><a href="#cb21-2"></a>        <span class="pp">h3!</span><span class="op">[</span> text <span class="op">]</span></span>
<span id="cb21-3"><a href="#cb21-3"></a>    <span class="op">}</span>  </span>
<span id="cb21-4"><a href="#cb21-4"></a>    </span>
<span id="cb21-5"><a href="#cb21-5"></a>    <span class="pp">div!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span><span class="bu">Display</span> =&gt; <span class="st">&quot;flex&quot;</span>; <span class="pp">St::</span>FlexDirection =&gt; <span class="st">&quot;column&quot;</span><span class="op">}</span>,</span>
<span id="cb21-6"><a href="#cb21-6"></a>        text_display(<span class="st">&quot;Some things&quot;</span>),</span>
<span id="cb21-7"><a href="#cb21-7"></a>        <span class="pp">button!</span><span class="op">[</span> simple_ev(<span class="st">&quot;click&quot;</span>, <span class="pp">Msg::</span>SayHi), <span class="st">&quot;Click me!&quot;</span> <span class="op">]</span></span>
<span id="cb21-8"><a href="#cb21-8"></a>    <span class="op">]</span></span></code></pre></div>
<p>The text_display component returns a single <code>Node</code> that is inserted into its parents’ <code>children</code> Vec; you can use this in patterns as you would in React. You can also use functions that return <code>Vec</code>s of<code>Node</code>s, which you can incorporate into other <code>Node</code>s using normal Rust code. See the Fragments section below. Rust’s type system ensures that only <code>Node</code>s can end up as children, so if your app compiles, you haven’t violated any rules.</p>
<p>Unlike in JSX, there’s a clear syntax delineation between natural DOM elements (element macros), and custom components (function calls): We called text_display above as <code>text_display("Some things")</code>, not <code>text_display![ "Some things" ]</code>.</p>
<h2 id="fragments">Fragments</h2>
<p>Fragments (<code>&lt;&gt;...&lt;/&gt;</code> syntax in React and Yew) are components that represent multiple elements without a parent. They’re useful to avoid unecessary divs, which clutter teh DOM, and breaks things like tables and CSS-grid. There’s no special fragment syntax: have your component return a <code>Vec</code> of <code>Node</code>s instead of one. Add it to the parent’s element macro:</p>
<div class="sourceCode" id="cb22"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb22-1"><a href="#cb22-1"></a><span class="kw">fn</span> cols() -&gt; <span class="dt">Vec</span>&lt;Node&lt;Msg&gt;&gt; <span class="op">{</span></span>
<span id="cb22-2"><a href="#cb22-2"></a>    <span class="pp">vec!</span><span class="op">[</span></span>
<span id="cb22-3"><a href="#cb22-3"></a>        <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;1&quot;</span> <span class="op">]</span>,</span>
<span id="cb22-4"><a href="#cb22-4"></a>        <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;2&quot;</span> <span class="op">]</span>,</span>
<span id="cb22-5"><a href="#cb22-5"></a>        <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;3&quot;</span> <span class="op">]</span></span>
<span id="cb22-6"><a href="#cb22-6"></a>    <span class="op">]</span></span>
<span id="cb22-7"><a href="#cb22-7"></a><span class="op">}</span></span>
<span id="cb22-8"><a href="#cb22-8"></a></span>
<span id="cb22-9"><a href="#cb22-9"></a><span class="kw">fn</span> items() -&gt; Node&lt;Msg&gt; <span class="op">{</span></span>
<span id="cb22-10"><a href="#cb22-10"></a>    <span class="pp">table!</span><span class="op">[</span></span>
<span id="cb22-11"><a href="#cb22-11"></a>        <span class="pp">tr!</span><span class="op">[</span> cols() <span class="op">]</span></span>
<span id="cb22-12"><a href="#cb22-12"></a>    <span class="op">]</span></span>
<span id="cb22-13"><a href="#cb22-13"></a><span class="op">}</span></span></code></pre></div>
<p>You can mix <code>Node</code> <code>Vec</code>s with <code>Node</code>s in macros:</p>
<div class="sourceCode" id="cb23"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb23-1"><a href="#cb23-1"></a><span class="kw">fn</span> items() -&gt; Node&lt;Msg&gt; <span class="op">{</span></span>
<span id="cb23-2"><a href="#cb23-2"></a>    <span class="co">// You may wish to keep complicated or dynamic logic separate.</span></span>
<span id="cb23-3"><a href="#cb23-3"></a>    <span class="kw">let</span> <span class="kw">mut</span> more_cols = <span class="pp">vec!</span><span class="op">[</span> <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;another col&quot;</span> <span class="op">]</span>, <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;and another&quot;</span> <span class="op">]</span> <span class="op">]</span>;</span>
<span id="cb23-4"><a href="#cb23-4"></a>    more_cols.push(<span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;yet another&quot;</span> <span class="op">]</span>);</span>
<span id="cb23-5"><a href="#cb23-5"></a></span>
<span id="cb23-6"><a href="#cb23-6"></a>    <span class="pp">table!</span><span class="op">[</span></span>
<span id="cb23-7"><a href="#cb23-7"></a>        <span class="pp">tr!</span><span class="op">[</span></span>
<span id="cb23-8"><a href="#cb23-8"></a>            <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;first col&quot;</span> <span class="op">]</span>,  <span class="co">// A lone element</span></span>
<span id="cb23-9"><a href="#cb23-9"></a>            cols(),  <span class="co">// A &quot;fragment&quot; component.</span></span>
<span id="cb23-10"><a href="#cb23-10"></a>            <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;an extra col&quot;</span> <span class="op">]</span>, <span class="co">// A element after the fragment</span></span>
<span id="cb23-11"><a href="#cb23-11"></a>            <span class="co">// A Vec of Els, not in a separate func</span></span>
<span id="cb23-12"><a href="#cb23-12"></a>            <span class="pp">vec!</span><span class="op">[</span> <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;another col&quot;</span> <span class="op">]</span>, <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;and another&quot;</span> <span class="op">]</span> <span class="op">]</span>,</span>
<span id="cb23-13"><a href="#cb23-13"></a>            more_cols  <span class="co">// A vec of Els created separately.</span></span>
<span id="cb23-14"><a href="#cb23-14"></a>        <span class="op">]</span></span>
<span id="cb23-15"><a href="#cb23-15"></a>    <span class="op">]</span></span>
<span id="cb23-16"><a href="#cb23-16"></a><span class="op">}</span></span></code></pre></div>
<h2 id="dummy-elements">Dummy elements</h2>
<p>When performing ternary operations inside an element macro, all branches must return an <code>Node</code> (Or <code>Vec</code> of <code>Node</code>s) to satisfy Rust’s type system. Seed provides the <a href="https://docs.rs/seed/0.4.2/seed/fn.empty.html">empty</a> function, which creates a <code>Node</code> that will not be rendered, and its <code>empty![]</code> macro alias, which is more concise and consistent:</p>
<div class="sourceCode" id="cb24"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb24-1"><a href="#cb24-1"></a><span class="pp">div!</span><span class="op">[</span></span>
<span id="cb24-2"><a href="#cb24-2"></a>    <span class="kw">if</span> model.count &gt;= <span class="dv">10</span> <span class="op">{</span> <span class="pp">h2!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>Padding =&gt; <span class="dv">50</span><span class="op">}</span>, <span class="st">&quot;Nice!&quot;</span> <span class="op">]</span> <span class="op">}</span> <span class="kw">else</span> <span class="op">{</span> <span class="pp">empty!</span><span class="op">[]</span>) <span class="op">}</span></span>
<span id="cb24-3"><a href="#cb24-3"></a><span class="op">]</span></span></code></pre></div>
"#####.into()
}