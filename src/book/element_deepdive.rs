pub fn text() -> String {
r#"
<h1 id="element-creation-macros-under-the-hood">Element-creation macros, under the hood</h1>
<p>The following code returns an <code>El</code> representing a few DOM elements displayed in a flexbox layout:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb1-1" title="1">    <span class="pp">div!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span><span class="st">&quot;display&quot;</span> =&gt; <span class="st">&quot;flex&quot;</span>; <span class="st">&quot;flex-direction&quot;</span> =&gt; <span class="st">&quot;column&quot;</span><span class="op">}</span>,</a>
<a class="sourceLine" id="cb1-2" title="2">        <span class="pp">h3!</span><span class="op">[</span> <span class="st">&quot;Some things&quot;</span> <span class="op">]</span>,</a>
<a class="sourceLine" id="cb1-3" title="3">        <span class="pp">button!</span><span class="op">[</span> <span class="st">&quot;Click me!&quot;</span> <span class="op">]</span></a>
<a class="sourceLine" id="cb1-4" title="4">    <span class="op">]</span></a></code></pre></div>
<p>The only magic parts of this are the macros used to simplify syntax for creating these things: text are <a href="https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values">Options</a> of Rust borrowed Strings; <code>Listeners</code> are stored in Vecs; children are elements and/or Vecs of; <code>Attr</code>s and <code>Style</code> are thinly-wrapped HashMaps. They can be created independently, and passed to the macros separately. The following code is equivalent; it uses constructors from the El struct. Note that <code>El</code> type is imported with the Prelude.</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb2-1" title="1">    <span class="kw">use</span> <span class="pp">seed::dom_types::</span><span class="op">{</span>El, Attrs, Style, Tag<span class="op">}</span>;</a>
<a class="sourceLine" id="cb2-2" title="2">    </a>
<a class="sourceLine" id="cb2-3" title="3">    <span class="co">// heading and button here show two types of element constructors</span></a>
<a class="sourceLine" id="cb2-4" title="4">    <span class="kw">let</span> <span class="kw">mut</span> heading = <span class="pp">El::</span>new(</a>
<a class="sourceLine" id="cb2-5" title="5">        <span class="pp">Tag::</span>H2, </a>
<a class="sourceLine" id="cb2-6" title="6">        <span class="pp">Attrs::</span>empty(), </a>
<a class="sourceLine" id="cb2-7" title="7">        <span class="pp">Style::</span>empty(), </a>
<a class="sourceLine" id="cb2-8" title="8">        <span class="dt">Vec</span>::new(),</a>
<a class="sourceLine" id="cb2-9" title="9">        <span class="st">&quot;Some things&quot;</span>,</a>
<a class="sourceLine" id="cb2-10" title="10">        <span class="dt">Vec</span>::New()</a>
<a class="sourceLine" id="cb2-11" title="11">    );  </a>
<a class="sourceLine" id="cb2-12" title="12">    </a>
<a class="sourceLine" id="cb2-13" title="13">    <span class="kw">let</span> <span class="kw">mut</span> button = <span class="pp">El::</span>empty(<span class="pp">Tag::</span>Button);</a>
<a class="sourceLine" id="cb2-14" title="14">    <span class="kw">let</span> children = <span class="pp">vec!</span><span class="op">[</span>heading, button<span class="op">]</span>;</a>
<a class="sourceLine" id="cb2-15" title="15">    </a>
<a class="sourceLine" id="cb2-16" title="16">    <span class="kw">let</span> <span class="kw">mut</span> elements = <span class="pp">El::</span>empty(<span class="pp">Tag::</span>Div);</a>
<a class="sourceLine" id="cb2-17" title="17">    elements.add_style(<span class="st">&quot;display&quot;</span>, <span class="st">&quot;flex&quot;</span>);</a>
<a class="sourceLine" id="cb2-18" title="18">    elements.add_style(<span class="st">&quot;flex-direction&quot;</span>, <span class="st">&quot;column&quot;</span>);</a>
<a class="sourceLine" id="cb2-19" title="19">    elements.children = children;</a>
<a class="sourceLine" id="cb2-20" title="20">    </a>
<a class="sourceLine" id="cb2-21" title="21">    elements</a></code></pre></div>
<p>The following equivalent example shows creating the required structs without constructors, to demonstrate that the macros and constructors above represent normal Rust structs, and provides insight into what abstractions they perform:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb3-1" title="1"><span class="co">// We didn&#39;t provide an example of a Listener/style: These are</span></a>
<a class="sourceLine" id="cb3-2" title="2"><span class="co">// more complicated to show using literals.</span></a>
<a class="sourceLine" id="cb3-3" title="3"><span class="kw">use</span> <span class="pp">seed::dom_types::</span><span class="op">{</span>El, Attrs, Style, Tag<span class="op">}</span>;</a>
<a class="sourceLine" id="cb3-4" title="4"></a>
<a class="sourceLine" id="cb3-5" title="5"><span class="co">// Rust has no built-in HashMap literal syntax.</span></a>
<a class="sourceLine" id="cb3-6" title="6"><span class="kw">let</span> <span class="kw">mut</span> style = <span class="pp">HashMap::</span>new();</a>
<a class="sourceLine" id="cb3-7" title="7">style.insert(<span class="st">&quot;display&quot;</span>, <span class="st">&quot;flex&quot;</span>);  </a>
<a class="sourceLine" id="cb3-8" title="8">style.insert(<span class="st">&quot;flex-direction&quot;</span>, <span class="st">&quot;column&quot;</span>);  </a>
<a class="sourceLine" id="cb3-9" title="9"></a>
<a class="sourceLine" id="cb3-10" title="10">El <span class="op">{</span></a>
<a class="sourceLine" id="cb3-11" title="11">    tag: <span class="pp">Tag::</span>Div,</a>
<a class="sourceLine" id="cb3-12" title="12">    attrs: Attrs <span class="op">{</span> vals: <span class="pp">HashMap::</span>new() <span class="op">}</span>,</a>
<a class="sourceLine" id="cb3-13" title="13">    style: Style <span class="op">{</span> vals: style <span class="op">}</span>,</a>
<a class="sourceLine" id="cb3-14" title="14">    events: Events <span class="op">{</span> vals: <span class="dt">Vec</span>::new() <span class="op">}</span>,</a>
<a class="sourceLine" id="cb3-15" title="15">    text: <span class="cn">None</span>,</a>
<a class="sourceLine" id="cb3-16" title="16">    children: <span class="pp">vec!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb3-17" title="17">        El <span class="op">{</span></a>
<a class="sourceLine" id="cb3-18" title="18">            tag: <span class="pp">Tag::</span>H2,</a>
<a class="sourceLine" id="cb3-19" title="19">            attrs: Attrs <span class="op">{</span> vals: <span class="pp">HashMap::</span>new() <span class="op">}</span>,</a>
<a class="sourceLine" id="cb3-20" title="20">            style: Style <span class="op">{</span> vals: <span class="pp">HashMap::</span>new() <span class="op">}</span>,</a>
<a class="sourceLine" id="cb3-21" title="21">            listeners: <span class="dt">Vec</span>::new();</a>
<a class="sourceLine" id="cb3-22" title="22">            text: <span class="cn">Some</span>(<span class="dt">String</span>::from(<span class="st">&quot;Some Things&quot;</span>)),</a>
<a class="sourceLine" id="cb3-23" title="23">            children: <span class="dt">Vec</span>::new()</a>
<a class="sourceLine" id="cb3-24" title="24">        <span class="op">}</span>,</a>
<a class="sourceLine" id="cb3-25" title="25">        El <span class="op">{</span></a>
<a class="sourceLine" id="cb3-26" title="26">            tag: <span class="pp">Tag::</span>button,</a>
<a class="sourceLine" id="cb3-27" title="27">            attrs: Attrs <span class="op">{</span> vals: <span class="pp">HashMap::</span>new() <span class="op">}</span>,</a>
<a class="sourceLine" id="cb3-28" title="28">            style: Style <span class="op">{</span> vals: <span class="pp">HashMap::</span>new() <span class="op">}</span>,</a>
<a class="sourceLine" id="cb3-29" title="29">            listeners: <span class="dt">Vec</span>::new();</a>
<a class="sourceLine" id="cb3-30" title="30">            text: <span class="cn">None</span>,</a>
<a class="sourceLine" id="cb3-31" title="31">            children: <span class="dt">Vec</span>::new(),</a>
<a class="sourceLine" id="cb3-32" title="32">        <span class="op">}</span> </a>
<a class="sourceLine" id="cb3-33" title="33">    <span class="op">]</span></a>
<a class="sourceLine" id="cb3-34" title="34"><span class="op">}</span></a></code></pre></div>
<p>For most uses, the first example (using macros) will be the easiest to read and write. You can mix in constructors (or struct literals) in components as needed, depending on your code structure.</p>
"#.into()
}