pub fn text() -> String {
r#"
<h1 id="lifecycle-hooks">Lifecycle hooks</h1>
<p>You can use lifecycle hooks, like those in React, to introduce side effects on DOM elements when an element is rendered for the first time, upates, or de-renders. We do this by passing one of the following structs to the element macro:</p>
<ul>
<li><a href="https://docs.rs/seed/0.1.11/seed/dom_types/struct.DidMount.html">DidMount</a></li>
<li><a href="https://docs.rs/seed/0.1.11/seed/dom_types/struct.DidUpdate.html">DidUpdate</a></li>
<li><a href="https://docs.rs/seed/0.1.11/seed/dom_types/struct.WillUnmount.html">WillUnmount</a></li>
</ul>
<p>These are inspired by, and act similar to <a href="https://reactjs.org/docs/react-component.html#componentdidmount">functions of similar names</a> in React. Each of these is a thin-wrapper for a closure that takes the <a href="https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html">web_sys element</a> as its only parameter, and doesn't return anything. We use them to perform side-effects (eg actions that don't change state), like setup and teardown operations on DOM elements.</p>
<p>We create them using the following functions respectively, imported in the prelude:</p>
<ul>
<li><a href="https://docs.rs/seed/0.1.11/seed/fn.did_mount.html">did_mount</a></li>
<li><a href="https://docs.rs/seed/0.1.11/seed/fn.did_update.html">did_update</a></li>
<li><a href="https://docs.rs/seed/0.1.11/seed/fn.will_unmount.html">will_unmount</a></li>
</ul>
<p>Each of these takes a single parameter: the closure described above.</p>
<p>Example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb1-1" title="1"><span class="pp">h3!</span><span class="op">[</span> num_clicks, did_update(|_| <span class="pp">log!</span>(<span class="st">&quot;This shows when we increment&quot;</span>)) <span class="op">]</span>,</a>
<a class="sourceLine" id="cb1-2" title="2"></a>
<a class="sourceLine" id="cb1-3" title="3"><span class="co">// ...</span></a>
<a class="sourceLine" id="cb1-4" title="4"></a>
<a class="sourceLine" id="cb1-5" title="5"><span class="kw">if</span> model.count &gt;= <span class="dv">10</span> <span class="op">{</span></a>
<a class="sourceLine" id="cb1-6" title="6">    <span class="pp">h2!</span><span class="op">[</span> <span class="st">&quot;Nice!&quot;</span>,</a>
<a class="sourceLine" id="cb1-7" title="7">         did_mount(|_| <span class="pp">log!</span>(<span class="st">&quot;This shows when clicks reach 10&quot;</span>)),</a>
<a class="sourceLine" id="cb1-8" title="8">         will_unmount(|_| <span class="pp">log!</span>(<span class="st">&quot;This shows when clicks drop below 10&quot;</span>)),</a>
<a class="sourceLine" id="cb1-9" title="9">    <span class="op">]</span></a>
<a class="sourceLine" id="cb1-10" title="10"><span class="op">}</span> <span class="kw">else</span> <span class="op">{</span> <span class="pp">seed::</span>empty() <span class="op">}</span></a></code></pre></div>
<p>An example of updating the associated DOM element:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb2-1" title="1"><span class="pp">button!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb2-2" title="2">    <span class="st">&quot;Autofocuses on load&quot;</span>,</a>
<a class="sourceLine" id="cb2-3" title="3">    did_mount(|el| <span class="op">{</span></a>
<a class="sourceLine" id="cb2-4" title="4">        <span class="kw">let</span> html_el = <span class="pp">seed::</span>to_html_el(&amp;el);</a>
<a class="sourceLine" id="cb2-5" title="5">        html_el.focus().unwrap();</a>
<a class="sourceLine" id="cb2-6" title="6">    <span class="op">}</span>)</a>
<a class="sourceLine" id="cb2-7" title="7"><span class="op">]</span>,</a></code></pre></div>
"#.into()
}