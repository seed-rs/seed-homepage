pub fn text() -> String {
r#"
<h1 id="element-creation-macros-under-the-hood">Element-creation macros, under the hood</h1>
<h1 id="this-page-is-out-of-date.-standby.">This page is out of date. Standby.</h1>
<p>For a better understanding of how views are created, reference the <a href="https://docs.rs/seed/0.4.0/seed/dom_types/struct.El.html">El API docs page</a>. The following code returns an <code>El</code> representing a few nested DOM elements displayed in a flexbox layout:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb1-1"><a href="\#cb1-1"></a>    <span class="pp">div!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span><span class="st">&quot;display&quot;</span> =&gt; <span class="st">&quot;flex&quot;</span>; <span class="st">&quot;flex-direction&quot;</span> =&gt; <span class="st">&quot;column&quot;</span><span class="op">}</span>,</span>
<span id="cb1-2"><a href="\#cb1-2"></a>        <span class="pp">h3!</span><span class="op">[</span> <span class="st">&quot;Some things&quot;</span> <span class="op">]</span>,</span>
<span id="cb1-3"><a href="\#cb1-3"></a>        <span class="pp">button!</span><span class="op">[</span> <span class="st">&quot;Click me!&quot;</span> <span class="op">]</span></span>
<span id="cb1-4"><a href="\#cb1-4"></a>    <span class="op">]</span></span></code></pre></div>
<p>This declarative syntax is created using macros, which constrct <code>El</code>s from the arguments passed: The macros know how to use arguments based solely on their type. If a String or &amp;str is passed, it's stored as the El's <code>text</code> field. <code>Attrs</code> and <code>Style</code> structs are stored as the <code>attrs</code> and <code>style</code> fields respectively. <code>Listeners</code>, and Vecs of them are stored as the <code>listeners</code> field. The same principle applies to <code>Els</code>, for the <code>children</code> field. <code>DidMount</code>, <code>DidUpdate</code>, and <code>WillUnmount</code> are also detected appropriately, and passed into appropriate fields.</p>
<p>Here's an another way to construct the same nested <code>El</code> as above, using constructors instead of macros. Reference the docs page for a full list of modifier methods. These provide conveniet syntax over manually editing fields. (In most cases, you won't edit <code>El</code>s at all; you'll create them declaratively using macros.)</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb2-1"><a href="\#cb2-1"></a><span class="kw">use</span> <span class="pp">seed::dom_types::</span><span class="op">{</span>El, Attrs, Style, Tag<span class="op">}</span>;</span>
<span id="cb2-2"><a href="\#cb2-2"></a></span>
<span id="cb2-3"><a href="\#cb2-3"></a><span class="kw">let</span> <span class="kw">mut</span> heading = <span class="pp">El::</span>empty();</span>
<span id="cb2-4"><a href="\#cb2-4"></a>heading.set_text(<span class="st">&quot;Some things&quot;</span>)</span>
<span id="cb2-5"><a href="\#cb2-5"></a></span>
<span id="cb2-6"><a href="\#cb2-6"></a><span class="kw">let</span> <span class="kw">mut</span> button = <span class="pp">El::</span>empty(<span class="pp">Tag::</span>Button);</span>
<span id="cb2-7"><a href="\#cb2-7"></a>button.set_text(<span class="st">&quot;Click me!&quot;</span>);</span>
<span id="cb2-8"><a href="\#cb2-8"></a></span>
<span id="cb2-9"><a href="\#cb2-9"></a><span class="kw">let</span> <span class="kw">mut</span> elements = <span class="pp">El::</span>empty(<span class="pp">Tag::</span>Div);</span>
<span id="cb2-10"><a href="\#cb2-10"></a>elements.add_style(<span class="st">&quot;display&quot;</span>, <span class="st">&quot;flex&quot;</span>);</span>
<span id="cb2-11"><a href="\#cb2-11"></a>elements.add_style(<span class="st">&quot;flex-direction&quot;</span>, <span class="st">&quot;column&quot;</span>);</span>
<span id="cb2-12"><a href="\#cb2-12"></a>elements.children = <span class="pp">vec!</span><span class="op">[</span>heading, button<span class="op">]</span>;</span>
<span id="cb2-13"><a href="\#cb2-13"></a></span>
<span id="cb2-14"><a href="\#cb2-14"></a>elements</span></code></pre></div>
<p>The following equivalent example shows creating the required structs without constructors, to demonstrate that the macros and constructors above represent normal Rust structs, and provides insight into what abstractions they perform.</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb3-1"><a href="\#cb3-1"></a><span class="co">// We don&\#39;t provide an example of a Listener: These are more complicated to </span></span>
<span id="cb3-2"><a href="\#cb3-2"></a><span class="kw">use</span> <span class="pp">seed::dom_types::</span><span class="op">{</span>El, Attrs, Style, Tag<span class="op">}</span>;</span>
<span id="cb3-3"><a href="\#cb3-3"></a></span>
<span id="cb3-4"><a href="\#cb3-4"></a>El <span class="op">{</span></span>
<span id="cb3-5"><a href="\#cb3-5"></a>    tag: <span class="pp">Tag::</span>Div,</span>
<span id="cb3-6"><a href="\#cb3-6"></a>    attrs: Attrs <span class="op">{</span> vals: <span class="pp">HashMap::</span>new() <span class="op">}</span>,</span>
<span id="cb3-7"><a href="\#cb3-7"></a>    style: Style <span class="op">{</span> </span>
<span id="cb3-8"><a href="\#cb3-8"></a>        vals: <span class="pp">hashmap_string!</span><span class="op">{</span></span>
<span id="cb3-9"><a href="\#cb3-9"></a>            <span class="st">&quot;display&quot;</span> =&gt; <span class="st">&quot;flex&quot;</span>,</span>
<span id="cb3-10"><a href="\#cb3-10"></a>            <span class="st">&quot;flex-direction&quot;</span> =&gt; <span class="st">&quot;column&quot;</span></span>
<span id="cb3-11"><a href="\#cb3-11"></a>        <span class="op">}</span></span>
<span id="cb3-12"><a href="\#cb3-12"></a>    <span class="op">}</span>,</span>
<span id="cb3-13"><a href="\#cb3-13"></a>    events: Events <span class="op">{</span> vals: <span class="dt">Vec</span>::new() <span class="op">}</span>,</span>
<span id="cb3-14"><a href="\#cb3-14"></a>    text: <span class="cn">None</span>,</span>
<span id="cb3-15"><a href="\#cb3-15"></a>    children: <span class="pp">vec!</span><span class="op">[</span></span>
<span id="cb3-16"><a href="\#cb3-16"></a>        El <span class="op">{</span></span>
<span id="cb3-17"><a href="\#cb3-17"></a>            tag: <span class="pp">Tag::</span>H3,</span>
<span id="cb3-18"><a href="\#cb3-18"></a>            attrs: Attrs <span class="op">{</span> vals: <span class="pp">HashMap::</span>new() <span class="op">}</span>,</span>
<span id="cb3-19"><a href="\#cb3-19"></a>            style: Style <span class="op">{</span> vals: <span class="pp">HashMap::</span>new() <span class="op">}</span>,</span>
<span id="cb3-20"><a href="\#cb3-20"></a>            listeners: <span class="dt">Vec</span>::new();</span>
<span id="cb3-21"><a href="\#cb3-21"></a>            text: <span class="cn">Some</span>(<span class="dt">String</span>::from(<span class="st">&quot;Some things&quot;</span>)),</span>
<span id="cb3-22"><a href="\#cb3-22"></a>            children: <span class="dt">Vec</span>::new()</span>
<span id="cb3-23"><a href="\#cb3-23"></a>            id: <span class="cn">None</span>,</span>
<span id="cb3-24"><a href="\#cb3-24"></a>            next_level: <span class="cn">None</span>,</span>
<span id="cb3-25"><a href="\#cb3-25"></a>            el_ws: <span class="cn">None</span>,</span>
<span id="cb3-26"><a href="\#cb3-26"></a>            namespace: <span class="cn">None</span>,</span>
<span id="cb3-27"><a href="\#cb3-27"></a>            did_mount: <span class="cn">None</span>,</span>
<span id="cb3-28"><a href="\#cb3-28"></a>            did_update: <span class="cn">None</span>,</span>
<span id="cb3-29"><a href="\#cb3-29"></a>            will_unmount: <span class="cn">None</span>,</span>
<span id="cb3-30"><a href="\#cb3-30"></a>        <span class="op">}</span>,</span>
<span id="cb3-31"><a href="\#cb3-31"></a>        El <span class="op">{</span></span>
<span id="cb3-32"><a href="\#cb3-32"></a>            tag: <span class="pp">Tag::</span>button,</span>
<span id="cb3-33"><a href="\#cb3-33"></a>            attrs: Attrs <span class="op">{</span> vals: <span class="pp">HashMap::</span>new() <span class="op">}</span>,</span>
<span id="cb3-34"><a href="\#cb3-34"></a>            style: Style <span class="op">{</span> vals: <span class="pp">HashMap::</span>new() <span class="op">}</span>,</span>
<span id="cb3-35"><a href="\#cb3-35"></a>            listeners: <span class="dt">Vec</span>::new();</span>
<span id="cb3-36"><a href="\#cb3-36"></a>            text: <span class="cn">Some</span>(<span class="dt">String</span>::from(<span class="st">&quot;Click me!&quot;</span>)),</span>
<span id="cb3-37"><a href="\#cb3-37"></a>            children: <span class="dt">Vec</span>::new(),</span>
<span id="cb3-38"><a href="\#cb3-38"></a>            id: <span class="cn">None</span>,</span>
<span id="cb3-39"><a href="\#cb3-39"></a>            next_level: <span class="cn">None</span>,</span>
<span id="cb3-40"><a href="\#cb3-40"></a>            el_ws: <span class="cn">None</span>,</span>
<span id="cb3-41"><a href="\#cb3-41"></a>            raw_html: <span class="cn">false</span>,</span>
<span id="cb3-42"><a href="\#cb3-42"></a>            text_node: <span class="cn">false</span>,</span>
<span id="cb3-43"><a href="\#cb3-43"></a>            namespace: <span class="cn">None</span>,</span>
<span id="cb3-44"><a href="\#cb3-44"></a>            did_mount: <span class="cn">None</span>,</span>
<span id="cb3-45"><a href="\#cb3-45"></a>            did_update: <span class="cn">None</span>,</span>
<span id="cb3-46"><a href="\#cb3-46"></a>            will_unmount: <span class="cn">None</span>,</span>
<span id="cb3-47"><a href="\#cb3-47"></a>        <span class="op">}</span> </span>
<span id="cb3-48"><a href="\#cb3-48"></a>    <span class="op">]</span></span>
<span id="cb3-49"><a href="\#cb3-49"></a><span class="op">}</span></span></code></pre></div>
<p>For most uses, the first example (using macros) will be the easiest to read and write. You can mix in constructors in components as needed, depending on your code structure. It's evident that struct literals are too verbose, due to the auxillary fields.</p>
"#.into()
}