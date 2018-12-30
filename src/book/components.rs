pub fn text() -> String {
r#"
<h1 id="components">Components</h1>
<p>The analog of components in frameworks like React are normal Rust functions that that return <a href="https://docs.rs/seed/0.2.0/seed/dom_types/struct.El.html">El</a>s. These functions take parameters that are not treated in a way equivalent to attributes on native DOM elements; they just provide a way to organize your code. In practice, they're used in a way similar to components in React.</p>
<p>For example, you could organize one of the examples in the Structure section of the guide like this:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb1-1" title="1">    <span class="kw">fn</span> text_display(text: &amp;<span class="dt">str</span>) -&gt; El&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb1-2" title="2">        <span class="pp">h3!</span><span class="op">[</span> text <span class="op">]</span></a>
<a class="sourceLine" id="cb1-3" title="3">    <span class="op">}</span>  </a>
<a class="sourceLine" id="cb1-4" title="4">    </a>
<a class="sourceLine" id="cb1-5" title="5">    <span class="pp">div!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span><span class="st">&quot;display&quot;</span> =&gt; <span class="st">&quot;flex&quot;</span>; <span class="st">&quot;flex-direction&quot;</span> =&gt; <span class="st">&quot;column&quot;</span><span class="op">}</span>,</a>
<a class="sourceLine" id="cb1-6" title="6">        text_display(<span class="st">&quot;Some things&quot;</span>),</a>
<a class="sourceLine" id="cb1-7" title="7">        <span class="pp">button!</span><span class="op">[</span> simple_ev(<span class="st">&quot;click&quot;</span>, <span class="pp">Msg::</span>SayHi), <span class="st">&quot;Click me!&quot;</span> <span class="op">]</span></a>
<a class="sourceLine" id="cb1-8" title="8">    <span class="op">]</span></a></code></pre></div>
<p>The text_display component returns a single <code>El</code> that is inserted into its parents' <code>children</code> Vec; you can use this in patterns as you would in React. You can also use functions that return <code>Vec</code>s of<code>El</code>s, which you can incorporate into other <code>El</code>s using normal Rust code. See the Fragments section below. Rust's type system ensures that only <code>El</code>s can end up as children, so if your app compiles, you haven't violated any rules.</p>
<p>Unlike in JSX, there's a clear syntax delineation between natural DOM elements (element macros), and custom components (function calls): We called text_display above as <code>text_display("Some things")</code>, not <code>text_display![ "Some things" ]</code>.</p>
<h2 id="fragments">Fragments</h2>
<p>Fragments (<code>&lt;&gt;...&lt;/&gt;</code> syntax in React and Yew) are components that represent multiple elements without a parent. They're useful to avoid unecessary divs, which clutter teh DOM, and breaks things like tables and CSS-grid. There's no special fragment syntax: have your component return a <code>Vec</code> of <code>El</code>s instead of one. Add it to the parent's element macro:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb2-1" title="1"><span class="kw">fn</span> cols() -&gt; <span class="dt">Vec</span>&lt;El&lt;Msg&gt;&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb2-2" title="2">    <span class="pp">vec!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb2-3" title="3">        <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;1&quot;</span> <span class="op">]</span>,</a>
<a class="sourceLine" id="cb2-4" title="4">        <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;2&quot;</span> <span class="op">]</span>,</a>
<a class="sourceLine" id="cb2-5" title="5">        <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;3&quot;</span> <span class="op">]</span></a>
<a class="sourceLine" id="cb2-6" title="6">    <span class="op">]</span></a>
<a class="sourceLine" id="cb2-7" title="7"><span class="op">}</span></a>
<a class="sourceLine" id="cb2-8" title="8"></a>
<a class="sourceLine" id="cb2-9" title="9"><span class="kw">fn</span> items() -&gt; El&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb2-10" title="10">    <span class="pp">table!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb2-11" title="11">        <span class="pp">tr!</span><span class="op">[</span> cols() <span class="op">]</span></a>
<a class="sourceLine" id="cb2-12" title="12">    <span class="op">]</span></a>
<a class="sourceLine" id="cb2-13" title="13"><span class="op">}</span></a></code></pre></div>
<p>You can mix <code>El</code> <code>Vec</code>s with <code>Els</code> in macros:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb3-1" title="1"><span class="kw">fn</span> items() -&gt; El&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb3-2" title="2">    <span class="co">// You may wish to keep complicated or dynamic logic separate.</span></a>
<a class="sourceLine" id="cb3-3" title="3">    <span class="kw">let</span> <span class="kw">mut</span> more_cols = <span class="pp">vec!</span><span class="op">[</span> <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;another col&quot;</span> <span class="op">]</span>, <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;and another&quot;</span> <span class="op">]</span> <span class="op">]</span>;</a>
<a class="sourceLine" id="cb3-4" title="4">    more_cols.push(<span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;yet another&quot;</span> <span class="op">]</span>);</a>
<a class="sourceLine" id="cb3-5" title="5"></a>
<a class="sourceLine" id="cb3-6" title="6">    <span class="pp">table!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb3-7" title="7">        <span class="pp">tr!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb3-8" title="8">            <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;first col&quot;</span> <span class="op">]</span>,  <span class="co">// A lone element</span></a>
<a class="sourceLine" id="cb3-9" title="9">            cols(),  <span class="co">// A &quot;fragment&quot; component.</span></a>
<a class="sourceLine" id="cb3-10" title="10">            <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;an extra col&quot;</span> <span class="op">]</span>, <span class="co">// A element after the fragment</span></a>
<a class="sourceLine" id="cb3-11" title="11">            <span class="co">// A Vec of Els, not in a separate func</span></a>
<a class="sourceLine" id="cb3-12" title="12">            <span class="pp">vec!</span><span class="op">[</span> <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;another col&quot;</span> <span class="op">]</span>, <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;and another&quot;</span> <span class="op">]</span> <span class="op">]</span>,</a>
<a class="sourceLine" id="cb3-13" title="13">            more_cols  <span class="co">// A vec of Els created separately.</span></a>
<a class="sourceLine" id="cb3-14" title="14">        <span class="op">]</span></a>
<a class="sourceLine" id="cb3-15" title="15">    <span class="op">]</span></a>
<a class="sourceLine" id="cb3-16" title="16"><span class="op">}</span></a></code></pre></div>
<h2 id="dummy-elements">Dummy elements</h2>
<p>When performing ternary operations inside an element macro, all branches must return an <code>El</code> (Or <code>Vec</code> of <code>El</code>s) to satisfy Rust's type system. Seed provides the <a href="https://docs.rs/seed/0.2.0/seed/fn.empty.html">empty</a> function, which creates an element that will not be rendered:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb4-1" title="1"><span class="pp">div!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb4-2" title="2">    <span class="kw">if</span> model.count &gt;= <span class="dv">10</span> <span class="op">{</span> <span class="pp">h2!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span><span class="st">&quot;padding&quot;</span> =&gt; <span class="dv">50</span><span class="op">}</span>, <span class="st">&quot;Nice!&quot;</span> <span class="op">]</span> <span class="op">}</span> <span class="kw">else</span> <span class="op">{</span> <span class="pp">seed::</span>empty() <span class="op">}</span></a>
<a class="sourceLine" id="cb4-3" title="3"><span class="op">]</span></a></code></pre></div>
"#.into()
}