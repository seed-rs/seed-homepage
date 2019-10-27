pub fn text() -> String {
r#####"
<h1 id="view">View</h1>
<p>Visual layout (ie HTML/DOM elements) is described declaratively in Rust, and uses <a href="https://doc.rust-lang.org/book/appendix-04-macros.html">macros</a> to simplify syntax.</p>
<p>The view’s defined by a function that’s passed to <code>seed::run</code>. This takes a <code>&amp;Model</code> as its parameter, and outputs something that implements the <code>View</code> trait, which is imported in the prelude. Usually, this is a <code>Node</code>, or <code>Vec&lt;Node&gt;</code>, representing all nodes that will be inserted as children on the top-level one. (The top-level <code>Node</code> is in the html file, and specified with <code>seed::App::build.mount()</code>, or as a default, the element with id <code>app</code>). It may composed into sub-functions, which can be thought of like components in other frameworks.</p>
<p>Examples:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb1-1" title="1"><span class="kw">fn</span> view(model: &amp;Model) -&gt; Node&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb1-2" title="2">    <span class="pp">h1!</span><span class="op">[</span> <span class="st">&quot;Let there be light&quot;</span> <span class="op">]</span>,</a>
<a class="sourceLine" id="cb1-3" title="3"><span class="op">}</span></a></code></pre></div>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb2-1" title="1"><span class="kw">fn</span> view(model: &amp;Model) -&gt; <span class="dt">Vec</span>&lt;Node&lt;Msg&gt;&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb2-2" title="2">    <span class="pp">vec!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb2-3" title="3">        <span class="pp">h1!</span><span class="op">[</span> <span class="st">&quot;Let there be light&quot;</span> <span class="op">]</span>,</a>
<a class="sourceLine" id="cb2-4" title="4">        <span class="pp">h2!</span><span class="op">[</span> <span class="st">&quot;Let it be both a particle and a wave&quot;</span> <span class="op">]</span></a>
<a class="sourceLine" id="cb2-5" title="5">    <span class="op">]</span></a>
<a class="sourceLine" id="cb2-6" title="6"><span class="op">}</span></a></code></pre></div>
<p>In either of those examples, you could use the signature: <code>fn view(model: &amp;Model) -&gt; impl View&lt;Msg&gt;</code> instead. This allows you to change between them without changing the function signature.</p>
<h2 id="the-node-enum">The Node Enum</h2>
<p>The Virtual DOM is represnted by nested <a href="https://docs.rs/seed/0.4.0/seed/dom_types/enum.None.html">Nodes</a>. <code>Node</code> has 3 variants: - <code>Text</code> holds a <a href="https://docs.rs/seed/0.4.0/seed/dom_types/struct.Text.html">Text</a> struct. Mostly for internal use, but can be created with <code>Node::new_text()</code>. - <code>Element</code> wraps an <a href="https://docs.rs/seed/0.4.0/seed/dom_types/struct.El.html">El</a>, which is the main component of our VDOM. Created using macros, described below. - <code>Empty</code> is a placeholder that doens’t render anything; useful in conditional/ternary logic. Created using the <code>empty![]</code> macro, or <code>seed::empty()</code>.</p>
<h2 id="elements-attributes-styles">Elements, attributes, styles</h2>
<p>Elements are created using macros, named by the lowercase name of each element, and imported into the global namespace. Eg <code>div!</code> above. We use this code to import them:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb3-1" title="1"><span class="at">#[</span>macro_use<span class="at">]</span></a>
<a class="sourceLine" id="cb3-2" title="2"><span class="kw">extern</span> <span class="kw">crate</span> seed;</a></code></pre></div>
<p>These macros accept any combination of the following parameters: - One <a href="https://docs.rs/seed/0.4.0/seed/dom_types/struct.Attrs.html">Attrs</a> struct - One <a href="https://docs.rs/seed/0.4.0/seed/dom_types/struct.Style.html">Style</a> struct - One or more <a href="https://docs.rs/seed/0.4.0/seed/dom_types/struct.Listener.html">Listener</a> structs, which handle events - One or more <code>Vec</code>s of <code>Listener</code> structs - One <code>String</code> or <code>&amp;str</code> representing a node text - One or more <a href="https://docs.rs/seed/0.4.0/seed/dom_types/enum.Node.html">Node</a> structs, representing a child - One or more Vecs of <code>Node</code> structs, representing multiple children - A <code>Map</code>, ie the result of <code>map()</code>, yielding <code>Node</code>s or <code>Listener</code>s, without having to explicitly <code>collect</code>.</p>
<p>The parameters can be passed in any order; the compiler knows how to handle them based on their types. Children are rendered in the order passed.</p>
<p>Views are described using <a href="https://docs.rs/seed/0.4.0/seed/dom_types/struct.El.html">El</a> structs, defined in the <a href="https://docs.rs/seed/0.4.0/seed/dom_types/index.html">seed::dom_types</a> module.</p>
<p><code>Attrs</code> and <code>Style</code> are thinly-wrapped hashmaps created with their own macros: <code>attrs!{}</code> and <code>style!{}</code> respectively.</p>
<p>Example:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb4-1" title="1"><span class="kw">fn</span> view(model: &amp;Model) -&gt; <span class="kw">impl</span> View&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb4-2" title="2">    <span class="kw">let</span> things = <span class="pp">vec!</span><span class="op">[</span> <span class="pp">h4!</span><span class="op">[</span> <span class="st">&quot;thing1&quot;</span> <span class="op">]</span>, <span class="pp">h4!</span><span class="op">[</span> <span class="st">&quot;thing2&quot;</span> <span class="op">]</span> <span class="op">]</span>;</a>
<a class="sourceLine" id="cb4-3" title="3">    </a>
<a class="sourceLine" id="cb4-4" title="4">    <span class="kw">let</span> other_things = <span class="pp">vec!</span><span class="op">[</span><span class="dv">1</span>, <span class="dv">2</span><span class="op">]</span>;</a>
<a class="sourceLine" id="cb4-5" title="5"></a>
<a class="sourceLine" id="cb4-6" title="6">    <span class="pp">div!</span><span class="op">[</span> <span class="pp">attrs!</span><span class="op">{</span><span class="pp">At::</span>Class =&gt; <span class="st">&quot;hardly-any&quot;</span><span class="op">}</span>, </a>
<a class="sourceLine" id="cb4-7" title="7">        things,  <span class="co">// Vec&lt;Node&lt;Msg&gt;</span></a>
<a class="sourceLine" id="cb4-8" title="8">        other_things.map(|t| <span class="pp">h4!</span><span class="op">[</span>t.to_string()<span class="op">]</span>),  <span class="co">// Map</span></a>
<a class="sourceLine" id="cb4-9" title="9">        <span class="pp">h4!</span><span class="op">[</span> <span class="st">&quot;thing3?&quot;</span> <span class="op">]</span>,  <span class="co">// El</span></a>
<a class="sourceLine" id="cb4-10" title="10">    <span class="op">]</span></a>
<a class="sourceLine" id="cb4-11" title="11"><span class="op">}</span></a></code></pre></div>
<p>Note that you can create any of the above items inside an element macro, or create it separately, and pass it in. You can separate different items by comma, semicolon, or space.</p>
<p>Keys passed to <code>attrs!</code> can be <code>Seed::At</code>s, <code>String</code>s, or <code>&amp;str</code>s. Keys passed to <code>style!</code> can be <code>Seed::St</code>s, <code>String</code>s, or <code>&amp;str</code>s. Values passed to <code>attrs!</code>, and <code>style!</code> macros can be owned <code>Strings</code>, <code>&amp;str</code>s, or for <code>style!</code>, <code>unit</code>s. Eg: <code>input![ attrs!{At::Disabled =&gt; false]</code> and <code>input![ attrs!{&quot;disabled&quot; =&gt; &quot;false&quot;]</code> are equivalent. You use the <code>unit!</code> macro to apply units. There’s a <code>px</code> function for the special case where the unit is pixels:</p>
<div class="sourceCode" id="cb5"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb5-1" title="1"><span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>Width =&gt; <span class="pp">unit!</span>(<span class="dv">20</span>, px);<span class="op">}</span></a>
<a class="sourceLine" id="cb5-2" title="2"><span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>Width =&gt; px(<span class="dv">20</span>);<span class="op">}</span>  <span class="co">// equivalent</span></a></code></pre></div>
<p>For boolean attributes that are handled by presense or absense, like <code>disabled</code>, use can use <code>.as_at_value</code>: <code>input![ attrs!{At::Disabled =&gt; false.as_at_value() ]</code></p>
<p>We can set multiple values for an attribute using <code>Attribute.add_multiple</code>. This is useful for setting multiple classes. Note that we must set this up outside of the view macro, since it involves modifying a variable:</p>
<div class="sourceCode" id="cb6"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb6-1" title="1"><span class="kw">fn</span> a_component() -&gt; Node&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb6-2" title="2">    <span class="kw">let</span> <span class="kw">mut</span> attributes = <span class="pp">attrs!</span><span class="op">{}</span>;</a>
<a class="sourceLine" id="cb6-3" title="3">    attributes.add_multiple(<span class="pp">At::</span>Class, <span class="pp">vec!</span><span class="op">[</span><span class="st">&quot;A-modicum-of&quot;</span>, <span class="st">&quot;hardly-any&quot;</span><span class="op">]</span>);</a>
<a class="sourceLine" id="cb6-4" title="4"></a>
<a class="sourceLine" id="cb6-5" title="5">    <span class="pp">div!</span><span class="op">[</span> attributes <span class="op">]</span></a>
<a class="sourceLine" id="cb6-6" title="6"><span class="op">}</span></a></code></pre></div>
<p>Seed validates attributes <a href="https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes">against this list</a>; The <code>At</code> enum includes only these values, and <code>&amp;strs</code> passed are converted into <code>At</code>s. If you wish to use a custom attribute, use <code>At::Custom(name)</code>, where <code>name</code> is a <code>String</code> of your attribute’s name. In <code>attrs!</code> when using <code>&amp;str</code>s, inserting an unrecognized attribute will do the same.</p>
<p>The <code>class!</code> and <code>id!</code> convenience macros allow settings attributes as a list of classes, or a single id, if no other attributes are required. Do not mix and match these with each other, or with attrs!; all but the last-passed will be thrown out.</p>
<div class="sourceCode" id="cb7"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb7-1" title="1"><span class="kw">fn</span> a_component() -&gt; Node&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb7-2" title="2">    <span class="co">// ...</span></a>
<a class="sourceLine" id="cb7-3" title="3">    <span class="pp">span!</span><span class="op">[</span> <span class="pp">class!</span><span class="op">[</span><span class="st">&quot;calculus&quot;</span>, <span class="st">&quot;chemistry&quot;</span>, <span class="st">&quot;literature&quot;</span><span class="op">]</span> <span class="op">]</span>,</a>
<a class="sourceLine" id="cb7-4" title="4">    <span class="pp">span!</span><span class="op">[</span> <span class="pp">id!</span>(<span class="st">&quot;unique-element&quot;</span>) <span class="op">]</span>,</a>
<a class="sourceLine" id="cb7-5" title="5">    <span class="co">// ...</span></a>
<a class="sourceLine" id="cb7-6" title="6"><span class="op">}</span></a></code></pre></div>
<p>You can conditionally add classes with the <code>class!</code> macro:</p>
<div class="sourceCode" id="cb8"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb8-1" title="1"><span class="kw">let</span> active = <span class="cn">true</span>;</a>
<a class="sourceLine" id="cb8-2" title="2"></a>
<a class="sourceLine" id="cb8-3" title="3"><span class="pp">class!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb8-4" title="4">    <span class="st">&quot;blue&quot;</span>,</a>
<a class="sourceLine" id="cb8-5" title="5">    <span class="st">&quot;highlighted&quot;</span> =&gt; active,</a>
<a class="sourceLine" id="cb8-6" title="6">    <span class="st">&quot;confusing&quot;</span> =&gt; <span class="dv">0.99999</span> == <span class="dv">1</span></a>
<a class="sourceLine" id="cb8-7" title="7">    </a>
<a class="sourceLine" id="cb8-8" title="8"><span class="op">]</span></a></code></pre></div>
<p>Styles and Attrs can be passed as refs as well, which is useful if you need to pass the same one more than once:</p>
<div class="sourceCode" id="cb9"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb9-1" title="1"><span class="kw">fn</span> a_component() -&gt; Node&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb9-2" title="2">    <span class="kw">let</span> item_style = <span class="pp">style!</span><span class="op">{</span></a>
<a class="sourceLine" id="cb9-3" title="3">        <span class="pp">St::</span>MarginTop =&gt; px(<span class="dv">10</span>);</a>
<a class="sourceLine" id="cb9-4" title="4">        <span class="pp">St::</span>FontSize =&gt; <span class="pp">unit!</span>(<span class="dv">1.2</span>, em)</a>
<a class="sourceLine" id="cb9-5" title="5">    <span class="op">}</span>;</a>
<a class="sourceLine" id="cb9-6" title="6"></a>
<a class="sourceLine" id="cb9-7" title="7">    <span class="pp">div!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb9-8" title="8">        <span class="pp">ul!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb9-9" title="9">            <span class="pp">li!</span><span class="op">[</span> &amp;item_style, <span class="st">&quot;Item 1&quot;</span>, <span class="op">]</span>,</a>
<a class="sourceLine" id="cb9-10" title="10">            <span class="pp">li!</span><span class="op">[</span> &amp;item_style, <span class="st">&quot;Item 2&quot;</span>, <span class="op">]</span>,</a>
<a class="sourceLine" id="cb9-11" title="11">        <span class="op">]</span></a>
<a class="sourceLine" id="cb9-12" title="12">    <span class="op">]</span></a>
<a class="sourceLine" id="cb9-13" title="13"><span class="op">}</span></a></code></pre></div>
<p>Setting an InputElement’s <code>checked</code>, or <code>autofocus</code> property is done through normal attributes:</p>
<div class="sourceCode" id="cb10"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb10-1" title="1"><span class="kw">fn</span> a_component() -&gt; Node&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb10-2" title="2">    <span class="co">// ...</span></a>
<a class="sourceLine" id="cb10-3" title="3">    <span class="pp">input!</span><span class="op">[</span> <span class="pp">attrs!</span><span class="op">{</span><span class="pp">At::</span>Typed =&gt; <span class="st">&quot;checkbox&quot;</span>; <span class="pp">At::</span>Checked =&gt; <span class="cn">true</span><span class="op">}</span> <span class="op">]</span></a>
<a class="sourceLine" id="cb10-4" title="4">    <span class="pp">input!</span><span class="op">[</span> <span class="pp">attrs!</span><span class="op">{</span><span class="pp">At::</span>Autofocus =&gt; <span class="cn">true</span><span class="op">}</span> <span class="op">]</span></a>
<a class="sourceLine" id="cb10-5" title="5">    <span class="co">// ...</span></a>
<a class="sourceLine" id="cb10-6" title="6"><span class="op">}</span></a></code></pre></div>
<p>To change Attrs or Styles you’ve created, edit their .vals HashMap. To add a new part to them, use their .add method:</p>
<div class="sourceCode" id="cb11"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb11-1" title="1"><span class="kw">let</span> <span class="kw">mut</span> attributes = <span class="pp">attrs!</span><span class="op">{}</span>;</a>
<a class="sourceLine" id="cb11-2" title="2">attributes.add(<span class="pp">At::</span>Class, <span class="st">&quot;truckloads&quot;</span>);</a></code></pre></div>
<p>Example of the style tag, and how you can use pattern-matching in views:</p>
<div class="sourceCode" id="cb12"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb12-1" title="1"><span class="kw">fn</span> view(model: &amp;Model) -&gt; <span class="kw">impl</span> View&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb12-2" title="2">    <span class="pp">div!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span></a>
<a class="sourceLine" id="cb12-3" title="3">        St:<span class="bu">Display</span> =&gt; <span class="st">&quot;grid&quot;</span>;</a>
<a class="sourceLine" id="cb12-4" title="4">        <span class="pp">St::</span>GridTemplateColumns =&gt; <span class="st">&quot;auto&quot;</span>;</a>
<a class="sourceLine" id="cb12-5" title="5">        <span class="pp">St::</span>GridTemplateRows =&gt; <span class="st">&quot;100px auto 100px&quot;</span></a>
<a class="sourceLine" id="cb12-6" title="6">        <span class="op">}</span>,</a>
<a class="sourceLine" id="cb12-7" title="7">        <span class="pp">section!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>GridRow =&gt; <span class="st">&quot;1 / 2&quot;</span><span class="op">}</span>,</a>
<a class="sourceLine" id="cb12-8" title="8">            header(),</a>
<a class="sourceLine" id="cb12-9" title="9">        <span class="op">]</span>,</a>
<a class="sourceLine" id="cb12-10" title="10">        <span class="pp">section!</span><span class="op">[</span> <span class="pp">attrs!</span><span class="op">{</span><span class="pp">St::</span>GridRow =&gt; <span class="st">&quot;2 / 3&quot;</span><span class="op">}</span>,</a>
<a class="sourceLine" id="cb12-11" title="11">            <span class="kw">match</span> model.page <span class="op">{</span></a>
<a class="sourceLine" id="cb12-12" title="12">                <span class="pp">Page::</span>Guide =&gt; guide(),</a>
<a class="sourceLine" id="cb12-13" title="13">                <span class="pp">Page::</span>Changelog =&gt; changelog(),</a>
<a class="sourceLine" id="cb12-14" title="14">            <span class="op">}</span>,</a>
<a class="sourceLine" id="cb12-15" title="15">        <span class="op">]</span>,</a>
<a class="sourceLine" id="cb12-16" title="16">        <span class="pp">section!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>GridRow =&gt; <span class="st">&quot;3 / 4&quot;</span><span class="op">}</span>,</a>
<a class="sourceLine" id="cb12-17" title="17">            footer()</a>
<a class="sourceLine" id="cb12-18" title="18">        <span class="op">]</span></a>
<a class="sourceLine" id="cb12-19" title="19">    <span class="op">]</span></a>
<a class="sourceLine" id="cb12-20" title="20"><span class="op">}</span></a></code></pre></div>
<p>We can combine Attrs and Style instances using their <code>merge</code> methods, which take an &amp;Attrs and &amp;Style respectively. This can be used to compose styles from reusable parts. Example:</p>
<div class="sourceCode" id="cb13"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb13-1" title="1"><span class="kw">fn</span> a_component() -&gt; Node&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb13-2" title="2">    <span class="kw">let</span> base_style = !style<span class="op">{</span><span class="st">&quot;color&quot;</span> =&gt; <span class="st">&quot;lavender&quot;</span><span class="op">}</span>;</a>
<a class="sourceLine" id="cb13-3" title="3"></a>
<a class="sourceLine" id="cb13-4" title="4">    <span class="pp">div!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb13-5" title="5">        <span class="pp">h1!</span><span class="op">[</span> &amp;base_style.merge(&amp;<span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>GridRow =&gt; <span class="st">&quot;1 / 2&quot;</span><span class="op">}</span>) <span class="st">&quot;First row&quot;</span> <span class="op">]</span>,</a>
<a class="sourceLine" id="cb13-6" title="6">        <span class="pp">h1!</span><span class="op">[</span> &amp;base_style.merge(&amp;<span class="pp">style!</span><span class="op">{</span><span class="pp">St::</span>GridRow =&gt; <span class="st">&quot;2 / 3&quot;</span><span class="op">}</span>) <span class="st">&quot;Second row&quot;</span> <span class="op">]</span>,</a>
<a class="sourceLine" id="cb13-7" title="7">    <span class="op">]</span></a>
<a class="sourceLine" id="cb13-8" title="8"><span class="op">}</span></a></code></pre></div>
<p>Overall: we leverage of Rust’s strict type system to flexibly-create the view using normal Rust code.W</p>
<p><code>El</code> has several helper methods which can be chained together:</p>
<div class="sourceCode" id="cb14"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb14-1" title="1"><span class="kw">let</span> my_el = <span class="pp">div!</span><span class="op">[]</span></a>
<a class="sourceLine" id="cb14-2" title="2">    .add_text(<span class="st">&quot;Words&quot;</span>)</a>
<a class="sourceLine" id="cb14-3" title="3">    .add_class(<span class="st">&quot;complete&quot;</span>)</a>
<a class="sourceLine" id="cb14-4" title="4">    .add_attr(<span class="st">&quot;alt&quot;</span>.to_string(), <span class="st">&quot;a description&quot;</span>.to_string())</a>
<a class="sourceLine" id="cb14-5" title="5">    .add_style(<span class="pp">St::</span>Height, <span class="st">&quot;20px&quot;</span>.to_string())</a>
<a class="sourceLine" id="cb14-6" title="6">    .replace_text(<span class="st">&quot;Oops, not complete&quot;</span>);oo</a></code></pre></div>
<h2 id="svg">Svg</h2>
<p>You can create <code>SVG</code> elements in the same way as normal <code>Html</code> elements. Setting the <code>xmlns</code> attribute isn’t required; it’s set automatically when using the macro.</p>
<p>Example using macros:</p>
<div class="sourceCode" id="cb15"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb15-1" title="1"><span class="pp">svg!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb15-2" title="2">    <span class="pp">rect!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb15-3" title="3">        <span class="pp">attrs!</span><span class="op">{</span></a>
<a class="sourceLine" id="cb15-4" title="4">            <span class="pp">At::</span>X =&gt; <span class="st">&quot;5&quot;</span>,</a>
<a class="sourceLine" id="cb15-5" title="5">            <span class="pp">At::</span>Y =&gt;<span class="st">&quot;5&quot;</span>,</a>
<a class="sourceLine" id="cb15-6" title="6">            <span class="pp">At::</span>Width =&gt; <span class="st">&quot;20&quot;</span>,</a>
<a class="sourceLine" id="cb15-7" title="7">            <span class="pp">At::</span>Height =&gt; <span class="st">&quot;20&quot;</span>,</a>
<a class="sourceLine" id="cb15-8" title="8">            <span class="pp">At::</span>Stroke =&gt; <span class="st">&quot;green&quot;</span>,</a>
<a class="sourceLine" id="cb15-9" title="9">            <span class="pp">At::</span>StrokeWidth =&gt; <span class="st">&quot;4&quot;</span>,</a>
<a class="sourceLine" id="cb15-10" title="10">        <span class="op">}</span></a>
<a class="sourceLine" id="cb15-11" title="11">    <span class="op">]</span></a>
<a class="sourceLine" id="cb15-12" title="12"><span class="op">]</span></a></code></pre></div>
<p>The same exmaple using <code>from_html</code>:</p>
<div class="sourceCode" id="cb16"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb16-1" title="1"><span class="pp">Node::</span>from_html(</a>
<a class="sourceLine" id="cb16-2" title="2"><span class="st">r#&quot;</span></a>
<a class="sourceLine" id="cb16-3" title="3"><span class="st">&lt;svg&gt;</span></a>
<a class="sourceLine" id="cb16-4" title="4"><span class="st">    &lt;rect x=&quot;#</span><span class="dv">5</span><span class="st">&quot; y=&quot;</span><span class="dv">5</span><span class="st">&quot; width=&quot;</span><span class="dv">20</span><span class="st">&quot; height=&quot;</span><span class="dv">20</span><span class="st">&quot; stroke=&quot;</span>green<span class="st">&quot; stroke-width=&quot;</span><span class="dv">4</span><span class="st">&quot; /&gt;</span></a>
<a class="sourceLine" id="cb16-5" title="5"><span class="st">&lt;/svg&gt;</span></a>
<a class="sourceLine" id="cb16-6" title="6"><span class="st">&quot;</span>#)</a></code></pre></div>
<p>Another example, showing it in the <code>View</code> fn:</p>
<div class="sourceCode" id="cb17"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb17-1" title="1"><span class="kw">fn</span> view(model: &amp;Model) -&gt; <span class="dt">Vec</span>&lt;Node&lt;Msg&gt;&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb17-2" title="2">    <span class="pp">vec!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb17-3" title="3">        <span class="pp">svg!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb17-4" title="4">            <span class="pp">attrs!</span><span class="op">{</span></a>
<a class="sourceLine" id="cb17-5" title="5">                <span class="pp">At::</span>Width =&gt; <span class="st">&quot;100%&quot;</span>;</a>
<a class="sourceLine" id="cb17-6" title="6">                <span class="pp">At::</span>Height =&gt; <span class="st">&quot;100%&quot;</span>;</a>
<a class="sourceLine" id="cb17-7" title="7">                <span class="pp">At::</span>ViewBox =&gt; <span class="st">&quot;0 0 512 512&quot;</span>;</a>
<a class="sourceLine" id="cb17-8" title="8">            <span class="op">}</span>,</a>
<a class="sourceLine" id="cb17-9" title="9">            <span class="pp">path!</span><span class="op">[</span> </a>
<a class="sourceLine" id="cb17-10" title="10">                <span class="pp">attrs!</span><span class="op">{</span></a>
<a class="sourceLine" id="cb17-11" title="11">                    <span class="pp">At::</span>Fill =&gt; <span class="st">&quot;lightgrey&quot;</span>;</a>
<a class="sourceLine" id="cb17-12" title="12">                    <span class="pp">At::</span>D =&gt; <span class="st">&quot;M345.863,281.853c19.152-8.872,38.221-15.344,56.1&quot;</span>  <span class="co">// etc</span></a>
<a class="sourceLine" id="cb17-13" title="13">                <span class="op">}</span></a>
<a class="sourceLine" id="cb17-14" title="14">            <span class="op">]</span>,</a>
<a class="sourceLine" id="cb17-15" title="15">            <span class="co">// More elements as required, eg mesh, polyline, circle</span></a>
<a class="sourceLine" id="cb17-16" title="16">        <span class="op">]</span></a>
<a class="sourceLine" id="cb17-17" title="17">    <span class="op">]</span></a>
<a class="sourceLine" id="cb17-18" title="18"><span class="op">}</span></a></code></pre></div>
"#####.into()
}