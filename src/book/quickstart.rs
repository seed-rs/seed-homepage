pub fn text() -> String {
r#"
<h1 id="quickstart">Quickstart</h1>
<h2 id="setup">Setup</h2>
<p>This framework requires you to install <a href="https://www.rust-lang.org/tools/install">Rust</a>.</p>
<p>You'll need a recent version of Rust: <code>rustup update</code></p>
<p>The wasm32-unknown-unknown target: <code>rustup target add wasm32-unknown-unknown</code></p>
<p>And cargo-make: <code>cargo install --force cargo-make</code></p>
<h2 id="the-theoretical-minimum">The theoretical minimum</h2>
<p>To start, clone <a href="https://github.com/David-OConnor/seed-quickstart">the quickstart repo</a>: <code>git clone https://github.com/david-oconnor/seed-quickstart.git</code>, run <code>cargo make all</code> in a terminal to build the app, and <code>cargo make serve</code> to start a dev server on <code>127.0.0.0:8000</code>. If you'd like the compiler automatically check and recompile when you make changes, run <code>cargo make watch</code> instead of <code>cargo make all</code>.</p>
<h2 id="a-little-deeper">A little deeper</h2>
<p>Alternatively, create a new lib with Cargo: <code>cargo new --lib appname</code>. Here and everywhere it appears in this guide, <code>appname</code> should be replaced with the name of your app.</p>
<p>If not using the quickstart repo, create an Html file with a body that contains this:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode html"><code class="sourceCode html"><span id="cb1-1"><a href="\#cb1-1"></a><span class="kw">&lt;section</span><span class="ot"> id=</span><span class="st">&quot;app&quot;</span><span class="kw">&gt;&lt;/section&gt;</span></span>
<span id="cb1-2"><a href="\#cb1-2"></a></span>
<span id="cb1-3"><a href="\#cb1-3"></a><span class="kw">&lt;script</span><span class="ot"> src=</span><span class="st">&quot;/pkg/package.js&quot;</span><span class="kw">&gt;&lt;/script&gt;</span></span>
<span id="cb1-4"><a href="\#cb1-4"></a></span>
<span id="cb1-5"><a href="\#cb1-5"></a><span class="kw">&lt;script&gt;</span></span>
<span id="cb1-6"><a href="\#cb1-6"></a>  <span class="kw">const</span> <span class="op">{</span> render <span class="op">}</span> <span class="op">=</span> wasm_bindgen<span class="op">;</span></span>
<span id="cb1-7"><a href="\#cb1-7"></a>  <span class="kw">function</span> <span class="at">run</span>() <span class="op">{</span></span>
<span id="cb1-8"><a href="\#cb1-8"></a>    <span class="at">render</span>()<span class="op">;</span></span>
<span id="cb1-9"><a href="\#cb1-9"></a>  <span class="op">}</span></span>
<span id="cb1-10"><a href="\#cb1-10"></a>  <span class="at">wasm_bindgen</span>(<span class="st">&quot;/pkg/package_bg.wasm&quot;</span>)</span>
<span id="cb1-11"><a href="\#cb1-11"></a>    .<span class="at">then</span>(run)</span>
<span id="cb1-12"><a href="\#cb1-12"></a>    .<span class="at">catch</span>(<span class="va">console</span>.<span class="at">error</span>)<span class="op">;</span></span>
<span id="cb1-13"><a href="\#cb1-13"></a><span class="kw">&lt;/script&gt;</span></span></code></pre></div>
<p>The first line above is an empty element with id: It's where your app will render. The subsequent ones load your app's wasm modules.</p>
<p>The quickstart repo includes this file. You will eventually need to modify it to change the page's title, add a description, favicon, stylesheet etc.</p>
<p><code>Cargo.toml</code>, which is a file created by Cargo that describes your app, needs <code>wasm-bindgen</code>, <code>web-sys</code>, and <code>seed</code> as depdendencies, and crate-type of <code>"cdylib"</code>. The version in the quickstart repo has these set up already. Example:</p>
<pre class="toml"><code>[package]
name = &quot;appname&quot;
version = &quot;0.1.0&quot;
authors = [&quot;Your Name &lt;email@address.com&gt;&quot;]
edition = &quot;2018&quot;

[lib]
crate-type = [&quot;cdylib&quot;]

[dependencies]
seed = &quot;^0.3.4&quot;
wasm-bindgen = &quot;^0.2.38&quot;
web-sys = &quot;^0.3.6&quot;</code></pre>
<h2 id="a-short-example">A short example</h2>
<p>Here's an example demonstrating structure and syntax; it can be found in working form in the <a href="https://github.com/David-OConnor/seed/tree/master/examples/counter">counter example</a> Descriptions of its parts are in the Guide section below. Its structure follows <a href="https://guide.elm-lang.org/architecture/">The Elm Architecture</a>.</p>
<p><em>lib.rs</em>:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb3-1"><a href="\#cb3-1"></a><span class="at">\#[</span>macro_use<span class="at">]</span></span>
<span id="cb3-2"><a href="\#cb3-2"></a><span class="kw">extern</span> <span class="kw">crate</span> seed;</span>
<span id="cb3-3"><a href="\#cb3-3"></a><span class="kw">use</span> <span class="pp">seed::prelude::</span>*;</span>
<span id="cb3-4"><a href="\#cb3-4"></a></span>
<span id="cb3-5"><a href="\#cb3-5"></a></span>
<span id="cb3-6"><a href="\#cb3-6"></a><span class="co">// Model</span></span>
<span id="cb3-7"><a href="\#cb3-7"></a></span>
<span id="cb3-8"><a href="\#cb3-8"></a><span class="kw">struct</span> Model <span class="op">{</span></span>
<span id="cb3-9"><a href="\#cb3-9"></a>    count: <span class="dt">i32</span>,</span>
<span id="cb3-10"><a href="\#cb3-10"></a>    what_we_count: <span class="dt">String</span></span>
<span id="cb3-11"><a href="\#cb3-11"></a><span class="op">}</span></span>
<span id="cb3-12"><a href="\#cb3-12"></a></span>
<span id="cb3-13"><a href="\#cb3-13"></a><span class="co">// Setup a default here, for initialization later.</span></span>
<span id="cb3-14"><a href="\#cb3-14"></a><span class="kw">impl</span> <span class="bu">Default</span> <span class="kw">for</span> Model <span class="op">{</span></span>
<span id="cb3-15"><a href="\#cb3-15"></a>    <span class="kw">fn</span> default() -&gt; <span class="kw">Self</span> <span class="op">{</span></span>
<span id="cb3-16"><a href="\#cb3-16"></a>        <span class="kw">Self</span> <span class="op">{</span></span>
<span id="cb3-17"><a href="\#cb3-17"></a>            count: <span class="dv">0</span>,</span>
<span id="cb3-18"><a href="\#cb3-18"></a>            what_we_count: <span class="st">&quot;click&quot;</span>.into()</span>
<span id="cb3-19"><a href="\#cb3-19"></a>        <span class="op">}</span></span>
<span id="cb3-20"><a href="\#cb3-20"></a>    <span class="op">}</span></span>
<span id="cb3-21"><a href="\#cb3-21"></a><span class="op">}</span></span>
<span id="cb3-22"><a href="\#cb3-22"></a></span>
<span id="cb3-23"><a href="\#cb3-23"></a></span>
<span id="cb3-24"><a href="\#cb3-24"></a><span class="co">// Update</span></span>
<span id="cb3-25"><a href="\#cb3-25"></a></span>
<span id="cb3-26"><a href="\#cb3-26"></a><span class="at">\#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></span>
<span id="cb3-27"><a href="\#cb3-27"></a><span class="kw">enum</span> Msg <span class="op">{</span></span>
<span id="cb3-28"><a href="\#cb3-28"></a>    Increment,</span>
<span id="cb3-29"><a href="\#cb3-29"></a>    Decrement,</span>
<span id="cb3-30"><a href="\#cb3-30"></a>    ChangeWWC(<span class="dt">String</span>),</span>
<span id="cb3-31"><a href="\#cb3-31"></a><span class="op">}</span></span>
<span id="cb3-32"><a href="\#cb3-32"></a></span>
<span id="cb3-33"><a href="\#cb3-33"></a><span class="co">/// How we update the model</span></span>
<span id="cb3-34"><a href="\#cb3-34"></a><span class="kw">fn</span> update(msg: Msg, model: &amp;<span class="kw">mut</span> Model, _orders: &amp;<span class="kw">mut</span> Orders&lt;Msg&gt;) <span class="op">{</span></span>
<span id="cb3-35"><a href="\#cb3-35"></a>    <span class="kw">match</span> msg <span class="op">{</span></span>
<span id="cb3-36"><a href="\#cb3-36"></a>        <span class="pp">Msg::</span>Increment =&gt; model.count += <span class="dv">1</span>,</span>
<span id="cb3-37"><a href="\#cb3-37"></a>        <span class="pp">Msg::</span>Decrement =&gt; model.count -= <span class="dv">1</span>,</span>
<span id="cb3-38"><a href="\#cb3-38"></a>        <span class="pp">Msg::</span>ChangeWWC(what_we_count) =&gt; model.what_we_count = what_we_count,</span>
<span id="cb3-39"><a href="\#cb3-39"></a>    <span class="op">}</span></span>
<span id="cb3-40"><a href="\#cb3-40"></a><span class="op">}</span></span>
<span id="cb3-41"><a href="\#cb3-41"></a></span>
<span id="cb3-42"><a href="\#cb3-42"></a></span>
<span id="cb3-43"><a href="\#cb3-43"></a><span class="co">// View</span></span>
<span id="cb3-44"><a href="\#cb3-44"></a></span>
<span id="cb3-45"><a href="\#cb3-45"></a><span class="co">/// A simple component.</span></span>
<span id="cb3-46"><a href="\#cb3-46"></a><span class="kw">fn</span> success_level(clicks: <span class="dt">i32</span>) -&gt; Node&lt;Msg&gt; <span class="op">{</span></span>
<span id="cb3-47"><a href="\#cb3-47"></a>    <span class="kw">let</span> descrip = <span class="kw">match</span> clicks <span class="op">{</span></span>
<span id="cb3-48"><a href="\#cb3-48"></a>        <span class="dv">0</span> ... <span class="dv">5</span> =&gt; <span class="st">&quot;Not very many 🙁&quot;</span>,</span>
<span id="cb3-49"><a href="\#cb3-49"></a>        <span class="dv">6</span> ... <span class="dv">9</span> =&gt; <span class="st">&quot;I got my first real six-string 😐&quot;</span>,</span>
<span id="cb3-50"><a href="\#cb3-50"></a>        <span class="dv">10</span> ... <span class="dv">11</span> =&gt; <span class="st">&quot;Spinal Tap 🙂&quot;</span>,</span>
<span id="cb3-51"><a href="\#cb3-51"></a>        _ =&gt; <span class="st">&quot;Double pendulum 🙃&quot;</span></span>
<span id="cb3-52"><a href="\#cb3-52"></a>    <span class="op">}</span>;</span>
<span id="cb3-53"><a href="\#cb3-53"></a>    <span class="pp">p!</span><span class="op">[</span> descrip <span class="op">]</span></span>
<span id="cb3-54"><a href="\#cb3-54"></a><span class="op">}</span></span>
<span id="cb3-55"><a href="\#cb3-55"></a></span>
<span id="cb3-56"><a href="\#cb3-56"></a><span class="co">/// The top-level component we pass to the virtual dom.</span></span>
<span id="cb3-57"><a href="\#cb3-57"></a><span class="kw">fn</span> view(model: &amp;Model) -&gt; Node&lt;Msg&gt; <span class="op">{</span></span>
<span id="cb3-58"><a href="\#cb3-58"></a>    <span class="kw">let</span> plural = <span class="kw">if</span> model.count == <span class="dv">1</span> <span class="op">{</span><span class="st">&quot;&quot;</span><span class="op">}</span> <span class="kw">else</span> <span class="op">{</span><span class="st">&quot;s&quot;</span><span class="op">}</span>;</span>
<span id="cb3-59"><a href="\#cb3-59"></a></span>
<span id="cb3-60"><a href="\#cb3-60"></a>    <span class="co">// Attrs, Style, Events, and children may be defined separately.</span></span>
<span id="cb3-61"><a href="\#cb3-61"></a>    <span class="kw">let</span> outer_style = <span class="pp">style!</span><span class="op">{</span></span>
<span id="cb3-62"><a href="\#cb3-62"></a>            <span class="st">&quot;display&quot;</span> =&gt; <span class="st">&quot;flex&quot;</span>;</span>
<span id="cb3-63"><a href="\#cb3-63"></a>            <span class="st">&quot;flex-direction&quot;</span> =&gt; <span class="st">&quot;column&quot;</span>;</span>
<span id="cb3-64"><a href="\#cb3-64"></a>            <span class="st">&quot;text-align&quot;</span> =&gt; <span class="st">&quot;center&quot;</span></span>
<span id="cb3-65"><a href="\#cb3-65"></a>    <span class="op">}</span>;</span>
<span id="cb3-66"><a href="\#cb3-66"></a></span>
<span id="cb3-67"><a href="\#cb3-67"></a>    <span class="pp">div!</span><span class="op">[</span> outer_style,</span>
<span id="cb3-68"><a href="\#cb3-68"></a>        <span class="pp">h1!</span><span class="op">[</span> <span class="st">&quot;The Grand Total&quot;</span> <span class="op">]</span>,</span>
<span id="cb3-69"><a href="\#cb3-69"></a>        <span class="pp">div!</span><span class="op">[</span></span>
<span id="cb3-70"><a href="\#cb3-70"></a>            <span class="pp">style!</span><span class="op">{</span></span>
<span id="cb3-71"><a href="\#cb3-71"></a>                <span class="co">// Example of conditional logic in a style.</span></span>
<span id="cb3-72"><a href="\#cb3-72"></a>                <span class="st">&quot;color&quot;</span> =&gt; <span class="kw">if</span> model.count &gt; <span class="dv">4</span> <span class="op">{</span><span class="st">&quot;purple&quot;</span><span class="op">}</span> <span class="kw">else</span> <span class="op">{</span><span class="st">&quot;gray&quot;</span><span class="op">}</span>;</span>
<span id="cb3-73"><a href="\#cb3-73"></a>                <span class="co">// When passing numerical values to style!, &quot;px&quot; is implied.</span></span>
<span id="cb3-74"><a href="\#cb3-74"></a>                <span class="st">&quot;border&quot;</span> =&gt; <span class="st">&quot;2px solid \#004422&quot;</span>; <span class="st">&quot;padding&quot;</span> =&gt; <span class="dv">20</span></span>
<span id="cb3-75"><a href="\#cb3-75"></a>            <span class="op">}</span>,</span>
<span id="cb3-76"><a href="\#cb3-76"></a>            <span class="co">// We can use normal Rust code and comments in the view.</span></span>
<span id="cb3-77"><a href="\#cb3-77"></a>            <span class="pp">h3!</span><span class="op">[</span> <span class="pp">format!</span>(<span class="st">&quot;{} {}{} so far&quot;</span>, model.count, model.what_we_count, plural) <span class="op">]</span>,</span>
<span id="cb3-78"><a href="\#cb3-78"></a>            <span class="pp">button!</span><span class="op">[</span> simple_ev(<span class="pp">Ev::</span>Click, <span class="pp">Msg::</span>Increment), <span class="st">&quot;+&quot;</span> <span class="op">]</span>,</span>
<span id="cb3-79"><a href="\#cb3-79"></a>            <span class="pp">button!</span><span class="op">[</span> simple_ev(<span class="pp">Ev::</span>Click, <span class="pp">Msg::</span>Decrement), <span class="st">&quot;-&quot;</span> <span class="op">]</span>,</span>
<span id="cb3-80"><a href="\#cb3-80"></a></span>
<span id="cb3-81"><a href="\#cb3-81"></a>            <span class="co">// Optionally-displaying an element</span></span>
<span id="cb3-82"><a href="\#cb3-82"></a>            <span class="kw">if</span> model.count &gt;= <span class="dv">10</span> <span class="op">{</span> <span class="pp">h2!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span><span class="st">&quot;padding&quot;</span> =&gt; <span class="dv">50</span><span class="op">}</span>, <span class="st">&quot;Nice!&quot;</span> <span class="op">]</span> <span class="op">}</span> <span class="kw">else</span> <span class="op">{</span> <span class="pp">empty!</span><span class="op">[]</span> <span class="op">}</span></span>
<span id="cb3-83"><a href="\#cb3-83"></a>        <span class="op">]</span>,</span>
<span id="cb3-84"><a href="\#cb3-84"></a>        success_level(model.count),  <span class="co">// Incorporating a separate component</span></span>
<span id="cb3-85"><a href="\#cb3-85"></a></span>
<span id="cb3-86"><a href="\#cb3-86"></a>        <span class="pp">h3!</span><span class="op">[</span> <span class="st">&quot;What are we counting?&quot;</span> <span class="op">]</span>,</span>
<span id="cb3-87"><a href="\#cb3-87"></a>        <span class="pp">input!</span><span class="op">[</span> <span class="pp">attrs!</span><span class="op">{</span><span class="pp">At::</span>Value =&gt; model.what_we_count<span class="op">}</span>, input_ev(<span class="pp">Ev::</span>Input, <span class="pp">Msg::</span>ChangeWWC) <span class="op">]</span></span>
<span id="cb3-88"><a href="\#cb3-88"></a>    <span class="op">]</span></span>
<span id="cb3-89"><a href="\#cb3-89"></a><span class="op">}</span></span>
<span id="cb3-90"><a href="\#cb3-90"></a></span>
<span id="cb3-91"><a href="\#cb3-91"></a></span>
<span id="cb3-92"><a href="\#cb3-92"></a><span class="at">\#[</span>wasm_bindgen<span class="at">]</span></span>
<span id="cb3-93"><a href="\#cb3-93"></a><span class="kw">pub</span> <span class="kw">fn</span> render() <span class="op">{</span></span>
<span id="cb3-94"><a href="\#cb3-94"></a>    <span class="pp">seed::App::</span>build(<span class="pp">Model::</span><span class="kw">default</span>(), update, view)</span>
<span id="cb3-95"><a href="\#cb3-95"></a>        .finish()</span>
<span id="cb3-96"><a href="\#cb3-96"></a>        .run();</span>
<span id="cb3-97"><a href="\#cb3-97"></a><span class="op">}</span></span></code></pre></div>
<p>For a truly minimimal example, see <a href="https://github.com/David-OConnor/seed-quickstart/blob/master/src/lib.rs">lib.rs in the quickstart repo</a></p>
<h2 id="building-and-running">Building and running</h2>
<p>To build your app, run <code>cargo make all</code>, and to host on a dev server, run <code>cargo make serve</code>.</p>
<p>For a more robust starting setup, check out Martin Kavik's <a href="https://github.com/MartinKavik/seed-quickstart-webpack">seed-quickstart-webpack repo</a>.</p>
<h2 id="running-included-examples">Running included examples</h2>
<p>To run an example located in the <a href="https://github.com/David-OConnor/seed/tree/master/examples">examples folder</a>, run <code>cargo make start example_name</code>, where you replace <code>example_name</code> with the example name. Eg: <code>cargo make start counter</code>.</p>
<p>Some examples also require to run API server in another terminal window - <code>cargo make start_server example_name</code>.</p>
<p>When server(s) are running, open <a href="http://127.0.0.1:8000">127.0.0.1:8000</a> in your browser.</p>
"#.into()
}