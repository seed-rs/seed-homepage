pub fn text() -> String {
r#####"
<h1 id="interaction-with-javascript">Interaction with Javascript</h1>
<h2 id="calling-javascript-functions-from-seed.">Calling Javascript functions from seed.</h2>
<p>If you have Javascript functions in your app, you can call them from seed code using <code>wasm_bindgen</code>. For a detailed example, see the <a href="https://github.com/seed-rs/seed/tree/master/examples/update_from_js">official exampe</a>.</p>
<p>For example, you might have a Javascript function defined elsewhere in your document, like this:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode html"><code class="sourceCode html"><span id="cb1-1"><a href="#cb1-1"></a><span class="kw">&lt;script</span><span class="ot"> type=</span><span class="st">&quot;text/javascript&quot;</span><span class="kw">&gt;</span></span>
<span id="cb1-2"><a href="#cb1-2"></a>    <span class="kw">function</span> <span class="at">addOne</span>(val) <span class="op">{</span></span>
<span id="cb1-3"><a href="#cb1-3"></a>        <span class="cf">return</span> val <span class="op">+</span> <span class="dv">1</span></span>
<span id="cb1-4"><a href="#cb1-4"></a>    <span class="op">}</span></span>
<span id="cb1-5"><a href="#cb1-5"></a><span class="kw">&lt;/script&gt;</span></span>
<span id="cb1-6"><a href="#cb1-6"></a></span>
<span id="cb1-7"><a href="#cb1-7"></a><span class="kw">&lt;section</span><span class="ot"> id=</span><span class="st">&quot;app&quot;</span><span class="kw">&gt;&lt;/section&gt;</span></span>
<span id="cb1-8"><a href="#cb1-8"></a><span class="kw">&lt;script</span><span class="ot"> type=</span><span class="st">&quot;module&quot;</span><span class="kw">&gt;</span></span>
<span id="cb1-9"><a href="#cb1-9"></a>    <span class="im">import</span> init <span class="im">from</span> <span class="st">&#39;/static/pkg/mypage.js&#39;</span><span class="op">;</span></span>
<span id="cb1-10"><a href="#cb1-10"></a>    <span class="at">init</span>(<span class="st">&#39;/static/pkg/rap_bg.wasm&#39;</span>)<span class="op">;</span></span>
<span id="cb1-11"><a href="#cb1-11"></a><span class="kw">&lt;/script&gt;</span></span></code></pre></div>
<p>Define a function like this in your app, where <code>addOne</code> here is the same name as the javascript function you wish to call.</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb2-1"><a href="#cb2-1"></a><span class="co">/// ALlows calling the JS function getCookie, for CSRF tokens.</span></span>
<span id="cb2-2"><a href="#cb2-2"></a><span class="at">#[</span>wasm_bindgen<span class="at">]</span></span>
<span id="cb2-3"><a href="#cb2-3"></a><span class="kw">extern</span> <span class="st">&quot;C&quot;</span> <span class="op">{</span></span>
<span id="cb2-4"><a href="#cb2-4"></a>    <span class="kw">fn</span> addOne(val: &amp;<span class="dt">i32</span>) -&gt; <span class="dt">i32</span>;</span>
<span id="cb2-5"><a href="#cb2-5"></a><span class="op">}</span></span></code></pre></div>
<p>You can then call this anywhere in your app, eg:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb3-1"><a href="#cb3-1"></a><span class="pp">h1!</span><span class="op">[</span> <span class="pp">format!</span>(<span class="st">&quot;Two plus one equals {}&quot;</span>, addOne(<span class="dv">2</span>)) <span class="op">]</span></span></code></pre></div>
"#####.into()
}