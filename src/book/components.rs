pub fn text() -> String {
r#"
<h1 id="components">Components</h1>
<p>The analog of components in frameworks like React are normal Rust functions that that return <a href="https://docs.rs/seed/0.3.7/seed/dom_types/struct.El.html">El</a>s. These functions take parameters that are not treated in a way equivalent to attributes on native DOM elements; they just provide a way to organize your code. In practice, they're used in a way similar to components in React.</p>
<p>For example, you could organize one of the examples in the Structure section of the guide like this:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb1-1"><a href="#cb1-1"></a>    <span class="kw">fn</span> text_display(text: &amp;<span class="dt">str</span>) -&gt; El&lt;Msg&gt; <span class="op">{</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>        <span class="pp">h3!</span><span class="op">[</span> text <span class="op">]</span></span>
<span id="cb1-3"><a href="#cb1-3"></a>    <span class="op">}</span>  </span>
<span id="cb1-4"><a href="#cb1-4"></a>    </span>
<span id="cb1-5"><a href="#cb1-5"></a>    <span class="pp">div!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span><span class="st">&quot;display&quot;</span> =&gt; <span class="st">&quot;flex&quot;</span>; <span class="st">&quot;flex-direction&quot;</span> =&gt; <span class="st">&quot;column&quot;</span><span class="op">}</span>,</span>
<span id="cb1-6"><a href="#cb1-6"></a>        text_display(<span class="st">&quot;Some things&quot;</span>),</span>
<span id="cb1-7"><a href="#cb1-7"></a>        <span class="pp">button!</span><span class="op">[</span> simple_ev(<span class="st">&quot;click&quot;</span>, <span class="pp">Msg::</span>SayHi), <span class="st">&quot;Click me!&quot;</span> <span class="op">]</span></span>
<span id="cb1-8"><a href="#cb1-8"></a>    <span class="op">]</span></span></code></pre></div>
<p>The text_display component returns a single <code>El</code> that is inserted into its parents' <code>children</code> Vec; you can use this in patterns as you would in React. You can also use functions that return <code>Vec</code>s of<code>El</code>s, which you can incorporate into other <code>El</code>s using normal Rust code. See the Fragments section below. Rust's type system ensures that only <code>El</code>s can end up as children, so if your app compiles, you haven't violated any rules.</p>
<p>Unlike in JSX, there's a clear syntax delineation between natural DOM elements (element macros), and custom components (function calls): We called text_display above as <code>text_display("Some things")</code>, not <code>text_display![ "Some things" ]</code>.</p>
<h2 id="fragments">Fragments</h2>
<p>Fragments (<code>&lt;&gt;...&lt;/&gt;</code> syntax in React and Yew) are components that represent multiple elements without a parent. They're useful to avoid unecessary divs, which clutter teh DOM, and breaks things like tables and CSS-grid. There's no special fragment syntax: have your component return a <code>Vec</code> of <code>El</code>s instead of one. Add it to the parent's element macro:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb2-1"><a href="#cb2-1"></a><span class="kw">fn</span> cols() -&gt; <span class="dt">Vec</span>&lt;El&lt;Msg&gt;&gt; <span class="op">{</span></span>
<span id="cb2-2"><a href="#cb2-2"></a>    <span class="pp">vec!</span><span class="op">[</span></span>
<span id="cb2-3"><a href="#cb2-3"></a>        <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;1&quot;</span> <span class="op">]</span>,</span>
<span id="cb2-4"><a href="#cb2-4"></a>        <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;2&quot;</span> <span class="op">]</span>,</span>
<span id="cb2-5"><a href="#cb2-5"></a>        <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;3&quot;</span> <span class="op">]</span></span>
<span id="cb2-6"><a href="#cb2-6"></a>    <span class="op">]</span></span>
<span id="cb2-7"><a href="#cb2-7"></a><span class="op">}</span></span>
<span id="cb2-8"><a href="#cb2-8"></a></span>
<span id="cb2-9"><a href="#cb2-9"></a><span class="kw">fn</span> items() -&gt; El&lt;Msg&gt; <span class="op">{</span></span>
<span id="cb2-10"><a href="#cb2-10"></a>    <span class="pp">table!</span><span class="op">[</span></span>
<span id="cb2-11"><a href="#cb2-11"></a>        <span class="pp">tr!</span><span class="op">[</span> cols() <span class="op">]</span></span>
<span id="cb2-12"><a href="#cb2-12"></a>    <span class="op">]</span></span>
<span id="cb2-13"><a href="#cb2-13"></a><span class="op">}</span></span></code></pre></div>
<p>You can mix <code>El</code> <code>Vec</code>s with <code>Els</code> in macros:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb3-1"><a href="#cb3-1"></a><span class="kw">fn</span> items() -&gt; El&lt;Msg&gt; <span class="op">{</span></span>
<span id="cb3-2"><a href="#cb3-2"></a>    <span class="co">// You may wish to keep complicated or dynamic logic separate.</span></span>
<span id="cb3-3"><a href="#cb3-3"></a>    <span class="kw">let</span> <span class="kw">mut</span> more_cols = <span class="pp">vec!</span><span class="op">[</span> <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;another col&quot;</span> <span class="op">]</span>, <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;and another&quot;</span> <span class="op">]</span> <span class="op">]</span>;</span>
<span id="cb3-4"><a href="#cb3-4"></a>    more_cols.push(<span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;yet another&quot;</span> <span class="op">]</span>);</span>
<span id="cb3-5"><a href="#cb3-5"></a></span>
<span id="cb3-6"><a href="#cb3-6"></a>    <span class="pp">table!</span><span class="op">[</span></span>
<span id="cb3-7"><a href="#cb3-7"></a>        <span class="pp">tr!</span><span class="op">[</span></span>
<span id="cb3-8"><a href="#cb3-8"></a>            <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;first col&quot;</span> <span class="op">]</span>,  <span class="co">// A lone element</span></span>
<span id="cb3-9"><a href="#cb3-9"></a>            cols(),  <span class="co">// A &quot;fragment&quot; component.</span></span>
<span id="cb3-10"><a href="#cb3-10"></a>            <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;an extra col&quot;</span> <span class="op">]</span>, <span class="co">// A element after the fragment</span></span>
<span id="cb3-11"><a href="#cb3-11"></a>            <span class="co">// A Vec of Els, not in a separate func</span></span>
<span id="cb3-12"><a href="#cb3-12"></a>            <span class="pp">vec!</span><span class="op">[</span> <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;another col&quot;</span> <span class="op">]</span>, <span class="pp">td!</span><span class="op">[</span> <span class="st">&quot;and another&quot;</span> <span class="op">]</span> <span class="op">]</span>,</span>
<span id="cb3-13"><a href="#cb3-13"></a>            more_cols  <span class="co">// A vec of Els created separately.</span></span>
<span id="cb3-14"><a href="#cb3-14"></a>        <span class="op">]</span></span>
<span id="cb3-15"><a href="#cb3-15"></a>    <span class="op">]</span></span>
<span id="cb3-16"><a href="#cb3-16"></a><span class="op">}</span></span></code></pre></div>
<h2 id="dummy-elements">Dummy elements</h2>
<p>When performing ternary operations inside an element macro, all branches must return an <code>El</code> (Or <code>Vec</code> of <code>El</code>s) to satisfy Rust's type system. Seed provides the <a href="https://docs.rs/seed/0.3.7/seed/fn.empty.html">empty</a> function, which creates an element that will not be rendered, and its <code>empty![]</code> macro alias, which is more concise and consistent:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb4-1"><a href="#cb4-1"></a><span class="pp">div!</span><span class="op">[</span></span>
<span id="cb4-2"><a href="#cb4-2"></a>    <span class="kw">if</span> model.count &gt;= <span class="dv">10</span> <span class="op">{</span> <span class="pp">h2!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span><span class="st">&quot;padding&quot;</span> =&gt; <span class="dv">50</span><span class="op">}</span>, <span class="st">&quot;Nice!&quot;</span> <span class="op">]</span> <span class="op">}</span> <span class="kw">else</span> <span class="op">{</span> <span class="pp">empty!</span><span class="op">[]</span>) <span class="op">}</span></span>
<span id="cb4-3"><a href="#cb4-3"></a><span class="op">]</span></span></code></pre></div>
"#.into()
}