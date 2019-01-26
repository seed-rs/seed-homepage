pub fn text() -> String {
r#"
<h1 id="element-creation-macros-under-the-hood">Element-creation macros, under the hood</h1>
<p>For a better understanding of how views are created, reference the <a href="https://docs.rs/seed/0.2.4/seed/dom_types/struct.El.html">El API docs page</a>. The following code returns an <code>El</code> representing a few nested DOM elements displayed in a flexbox layout:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb1-1" title="1">    <span class="pp">div!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span><span class="st">&quot;display&quot;</span> =&gt; <span class="st">&quot;flex&quot;</span>; <span class="st">&quot;flex-direction&quot;</span> =&gt; <span class="st">&quot;column&quot;</span><span class="op">}</span>,</a>
<a class="sourceLine" id="cb1-2" title="2">        <span class="pp">h3!</span><span class="op">[</span> <span class="st">&quot;Some things&quot;</span> <span class="op">]</span>,</a>
<a class="sourceLine" id="cb1-3" title="3">        <span class="pp">button!</span><span class="op">[</span> <span class="st">&quot;Click me!&quot;</span> <span class="op">]</span></a>
<a class="sourceLine" id="cb1-4" title="4">    <span class="op">]</span></a></code></pre></div>
<p>This declarative syntax is created using macros, which constrct <code>El</code>s from the arguments passed: The macros know how to use arguments based solely on their type. If a String or &amp;str is passed, it's stored as the El's <code>text</code> field. <code>Attrs</code> and <code>Style</code> structs are stored as the <code>attrs</code> and <code>style</code> fields respectively. <code>Listeners</code>, and Vecs of them are stored as the <code>listeners</code> field. The same principle applies to <code>Els</code>, for the <code>children</code> field. <code>DidMount</code>, <code>DidUpdate</code>, and <code>WillUnmount</code> are also detected appropriately, and passed into appropriate fields.</p>
<p>Here's an another way to construct the same nested <code>El</code> as above, using constructors instead of macros. Reference the docs page for a full list of modifier methods. These provide conveniet syntax over manually editing fields. (In most cases, you won't edit <code>El</code>s at all; you'll create them declaratively using macros.)</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb2-1" title="1"><span class="kw">use</span> <span class="pp">seed::dom_types::</span><span class="op">{</span>El, Attrs, Style, Tag<span class="op">}</span>;</a>
<a class="sourceLine" id="cb2-2" title="2"></a>
<a class="sourceLine" id="cb2-3" title="3"><span class="kw">let</span> <span class="kw">mut</span> heading = <span class="pp">El::</span>empty();</a>
<a class="sourceLine" id="cb2-4" title="4">heading.set_text(<span class="st">&quot;Some things&quot;</span>)</a>
<a class="sourceLine" id="cb2-5" title="5"></a>
<a class="sourceLine" id="cb2-6" title="6"><span class="kw">let</span> <span class="kw">mut</span> button = <span class="pp">El::</span>empty(<span class="pp">Tag::</span>Button);</a>
<a class="sourceLine" id="cb2-7" title="7">button.set_text(<span class="st">&quot;Click me!&quot;</span>);</a>
<a class="sourceLine" id="cb2-8" title="8"></a>
<a class="sourceLine" id="cb2-9" title="9"><span class="kw">let</span> <span class="kw">mut</span> elements = <span class="pp">El::</span>empty(<span class="pp">Tag::</span>Div);</a>
<a class="sourceLine" id="cb2-10" title="10">elements.add_style(<span class="st">&quot;display&quot;</span>, <span class="st">&quot;flex&quot;</span>);</a>
<a class="sourceLine" id="cb2-11" title="11">elements.add_style(<span class="st">&quot;flex-direction&quot;</span>, <span class="st">&quot;column&quot;</span>);</a>
<a class="sourceLine" id="cb2-12" title="12">elements.children = <span class="pp">vec!</span><span class="op">[</span>heading, button<span class="op">]</span>;</a>
<a class="sourceLine" id="cb2-13" title="13"></a>
<a class="sourceLine" id="cb2-14" title="14">elements</a></code></pre></div>
<p>The following equivalent example shows creating the required structs without constructors, to demonstrate that the macros and constructors above represent normal Rust structs, and provides insight into what abstractions they perform.</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb3-1" title="1"><span class="co">// We don&#39;t provide an example of a Listener: These are more complicated to </span></a>
<a class="sourceLine" id="cb3-2" title="2"><span class="kw">use</span> <span class="pp">seed::dom_types::</span><span class="op">{</span>El, Attrs, Style, Tag<span class="op">}</span>;</a>
<a class="sourceLine" id="cb3-3" title="3"></a>
<a class="sourceLine" id="cb3-4" title="4">El <span class="op">{</span></a>
<a class="sourceLine" id="cb3-5" title="5">    tag: <span class="pp">Tag::</span>Div,</a>
<a class="sourceLine" id="cb3-6" title="6">    attrs: Attrs <span class="op">{</span> vals: <span class="pp">HashMap::</span>new() <span class="op">}</span>,</a>
<a class="sourceLine" id="cb3-7" title="7">    style: Style <span class="op">{</span> </a>
<a class="sourceLine" id="cb3-8" title="8">        vals: <span class="pp">hashmap_string!</span><span class="op">{</span></a>
<a class="sourceLine" id="cb3-9" title="9">            <span class="st">&quot;display&quot;</span> =&gt; <span class="st">&quot;flex&quot;</span>,</a>
<a class="sourceLine" id="cb3-10" title="10">            <span class="st">&quot;flex-direction&quot;</span> =&gt; <span class="st">&quot;column&quot;</span></a>
<a class="sourceLine" id="cb3-11" title="11">        <span class="op">}</span></a>
<a class="sourceLine" id="cb3-12" title="12">    <span class="op">}</span>,</a>
<a class="sourceLine" id="cb3-13" title="13">    events: Events <span class="op">{</span> vals: <span class="dt">Vec</span>::new() <span class="op">}</span>,</a>
<a class="sourceLine" id="cb3-14" title="14">    text: <span class="cn">None</span>,</a>
<a class="sourceLine" id="cb3-15" title="15">    children: <span class="pp">vec!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb3-16" title="16">        El <span class="op">{</span></a>
<a class="sourceLine" id="cb3-17" title="17">            tag: <span class="pp">Tag::</span>H3,</a>
<a class="sourceLine" id="cb3-18" title="18">            attrs: Attrs <span class="op">{</span> vals: <span class="pp">HashMap::</span>new() <span class="op">}</span>,</a>
<a class="sourceLine" id="cb3-19" title="19">            style: Style <span class="op">{</span> vals: <span class="pp">HashMap::</span>new() <span class="op">}</span>,</a>
<a class="sourceLine" id="cb3-20" title="20">            listeners: <span class="dt">Vec</span>::new();</a>
<a class="sourceLine" id="cb3-21" title="21">            text: <span class="cn">Some</span>(<span class="dt">String</span>::from(<span class="st">&quot;Some things&quot;</span>)),</a>
<a class="sourceLine" id="cb3-22" title="22">            children: <span class="dt">Vec</span>::new()</a>
<a class="sourceLine" id="cb3-23" title="23">            id: <span class="cn">None</span>,</a>
<a class="sourceLine" id="cb3-24" title="24">            next_level: <span class="cn">None</span>,</a>
<a class="sourceLine" id="cb3-25" title="25">            el_ws: <span class="cn">None</span>,</a>
<a class="sourceLine" id="cb3-26" title="26">            raw_html: <span class="cn">false</span>,</a>
<a class="sourceLine" id="cb3-27" title="27">            text_node: <span class="cn">false</span>,</a>
<a class="sourceLine" id="cb3-28" title="28">            namespace: <span class="cn">None</span>,</a>
<a class="sourceLine" id="cb3-29" title="29">            did_mount: <span class="cn">None</span>,</a>
<a class="sourceLine" id="cb3-30" title="30">            did_update: <span class="cn">None</span>,</a>
<a class="sourceLine" id="cb3-31" title="31">            will_unmount: <span class="cn">None</span>,</a>
<a class="sourceLine" id="cb3-32" title="32">        <span class="op">}</span>,</a>
<a class="sourceLine" id="cb3-33" title="33">        El <span class="op">{</span></a>
<a class="sourceLine" id="cb3-34" title="34">            tag: <span class="pp">Tag::</span>button,</a>
<a class="sourceLine" id="cb3-35" title="35">            attrs: Attrs <span class="op">{</span> vals: <span class="pp">HashMap::</span>new() <span class="op">}</span>,</a>
<a class="sourceLine" id="cb3-36" title="36">            style: Style <span class="op">{</span> vals: <span class="pp">HashMap::</span>new() <span class="op">}</span>,</a>
<a class="sourceLine" id="cb3-37" title="37">            listeners: <span class="dt">Vec</span>::new();</a>
<a class="sourceLine" id="cb3-38" title="38">            text: <span class="cn">Some</span>(<span class="dt">String</span>::from(<span class="st">&quot;Click me!&quot;</span>)),</a>
<a class="sourceLine" id="cb3-39" title="39">            children: <span class="dt">Vec</span>::new(),</a>
<a class="sourceLine" id="cb3-40" title="40">            id: <span class="cn">None</span>,</a>
<a class="sourceLine" id="cb3-41" title="41">            next_level: <span class="cn">None</span>,</a>
<a class="sourceLine" id="cb3-42" title="42">            el_ws: <span class="cn">None</span>,</a>
<a class="sourceLine" id="cb3-43" title="43">            raw_html: <span class="cn">false</span>,</a>
<a class="sourceLine" id="cb3-44" title="44">            text_node: <span class="cn">false</span>,</a>
<a class="sourceLine" id="cb3-45" title="45">            namespace: <span class="cn">None</span>,</a>
<a class="sourceLine" id="cb3-46" title="46">            did_mount: <span class="cn">None</span>,</a>
<a class="sourceLine" id="cb3-47" title="47">            did_update: <span class="cn">None</span>,</a>
<a class="sourceLine" id="cb3-48" title="48">            will_unmount: <span class="cn">None</span>,</a>
<a class="sourceLine" id="cb3-49" title="49">        <span class="op">}</span> </a>
<a class="sourceLine" id="cb3-50" title="50">    <span class="op">]</span></a>
<a class="sourceLine" id="cb3-51" title="51"><span class="op">}</span></a></code></pre></div>
<p>For most uses, the first example (using macros) will be the easiest to read and write. You can mix in constructors in components as needed, depending on your code structure. It's evident that struct literals are too verbose, due to the auxillary fields.</p>
"#.into()
}