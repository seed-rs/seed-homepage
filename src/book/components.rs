pub fn text() -> String {
r#"
<h1 id="components">Components</h1>
<p>The analog of components in frameworks like React are normal Rust functions that that return Els. The parameters these functions take are not treated in a way equivalent to attributes on native DOM elements; they just provide a way to organize your code. In practice, they feel similar to components in React, but are just functions used to create elements that end up in the <code>children</code> property of parent elements.</p>
<p>For example, you could break up one of the above examples like this:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb1-1" title="1">    <span class="kw">fn</span> text_display(text: &amp;<span class="dt">str</span>) -&gt; El&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb1-2" title="2">        <span class="pp">h3!</span><span class="op">[</span> text <span class="op">]</span></a>
<a class="sourceLine" id="cb1-3" title="3">    <span class="op">}</span>  </a>
<a class="sourceLine" id="cb1-4" title="4">    </a>
<a class="sourceLine" id="cb1-5" title="5">    <span class="pp">div!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span><span class="st">&quot;display&quot;</span> =&gt; <span class="st">&quot;flex&quot;</span>; flex-direction: <span class="st">&quot;column&quot;</span><span class="op">}</span>,</a>
<a class="sourceLine" id="cb1-6" title="6">        text_display(<span class="st">&quot;Some things&quot;</span>),</a>
<a class="sourceLine" id="cb1-7" title="7">        <span class="pp">button!</span><span class="op">[</span> simple_ev(<span class="st">&quot;click&quot;</span>, <span class="pp">Msg::</span>SayHi), <span class="st">&quot;Click me!&quot;</span> <span class="op">]</span></a>
<a class="sourceLine" id="cb1-8" title="8">    <span class="op">]</span></a></code></pre></div>
<p>The text_display() component returns a single El that is inserted into its parents' <code>children</code> Vec; you can use this in patterns as you would in React. You can also use functions that return Vecs of Els, which you can incorporate into other components using normal Rust code. See Fragments section below. Rust's type system ensures that only <code>El</code>s can end up as children, so if your app compiles, you haven't violated any rules.</p>
<p>Note that unlike in JSX, there's a clear syntax delineation here between natural HTML elements (element macros), and custom components (function calls).</p>
<h2 id="fragments">Fragments</h2>
<p>Fragments (<code>&lt;&gt;...&lt;/&gt;</code> syntax in React and Yew) are components that represent multiple elements without a parent. This is useful to avoid unecessary divs, which may be undesirable on their own, and breaks things like tables and CSS-grid. There's no special syntax; just have your component return a Vec of <code>El</code>s instead of one, and add it to the parent's element macro; on its own like in the example below, or with other children, or Vecs of children.</p>
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
<h2 id="dummy-elements">Dummy elements</h2>
<p>When performing ternary and related operations instead an element macro, all branches must return <code>El</code>s to satisfy Rust's type system. Seed provides the <code>empty()</code> function, which creates a VDOM element that will not be rendered:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb3-1" title="1"><span class="pp">div!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb3-2" title="2">    <span class="kw">if</span> model.count &gt;= <span class="dv">10</span> <span class="op">{</span> <span class="pp">h2!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span><span class="st">&quot;padding&quot;</span> =&gt; <span class="dv">50</span><span class="op">}</span>, <span class="st">&quot;Nice!&quot;</span> <span class="op">]</span> <span class="op">}</span> <span class="kw">else</span> <span class="op">{</span> <span class="pp">seed::</span>empty() <span class="op">}</span></a>
<a class="sourceLine" id="cb3-3" title="3"><span class="op">]</span></a></code></pre></div>
<p>For more complicated construsts, you may wish to create the <code>children</code> Vec separately, push what components are needed, and pass it into the element macro.</p>
"#.into()
}