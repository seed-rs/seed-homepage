pub fn text() -> String {
r#"
<h1 id="quickstart">Quickstart</h1>
<h2 id="setup">Setup</h2>
<p>This framework requires you to first install <a href="https://www.rust-lang.org/tools/install">Rust</a>.</p>
<p>You'll need a recent version of Rust: <code>rustup update</code></p>
<p>The wasm32-unknown-unknown target: <code>rustup target add wasm32-unknown-unknown</code></p>
<p>And wasm-bindgen: <code>cargo install wasm-bindgen-cli</code></p>
<h2 id="the-theoretical-minimum">The theoretical minimum</h2>
<p>To start, clone <a href="https://github.com/David-OConnor/seed-quickstart">This quickstart repo</a>, run <code>build.sh</code> or <code>build.ps1</code> in a terminal, then start a dev server that supports WASM. For example, with <a href="https://www.python.org/downloads/">Python</a> installed, run <code>python serve.py</code>. (Linux users may need to run <code>python3 serve.py</code>.) Once you change your package name, you'll need to tweak the html file and build script, as described below.</p>
<h2 id="a-little-deeper">A little deeper</h2>
<p>Alternatively, create a new lib with Cargo: <code>cargo new --lib appname</code>. Here and everywhere it appears in this guide, <code>appname</code> should be replaced with the name of your app.</p>
If not using the quickstart repo, create an Html file with a body that contains this: ```html
<section id="main">
</section>
<script src='./pkg/appname.js'></script>
<script>
    const { render } = wasm_bindgen;
    function run() {
        render();
    }
    wasm_bindgen('./pkg/appname_bg.wasm')
        .then(run)
        .catch(console.error);
</script>
<p>``` The first line above is an empty element with id: It's where your app will render. The subsequent ones load your app's wasm modules.</p>
<p>The quickstart repo includes this file, but you will need to rename the two occurances of <code>appname</code>. (If your project name has a hyphen, use an underscore instead here) You will eventually need to modify this file to change the page's title, add a description, favicon, stylesheet etc.</p>
<p><code>Cargo.toml</code>, which is a file created by Cargo that describes your app, needs <code>wasm-bindgen</code>, <code>web-sys</code>, and <code>seed</code> as depdendencies, and crate-type of <code>"cdylib"</code>. The version in the quickstart repo has these set up already. Example:</p>
<pre class="toml"><code>[package]
name = &quot;appname&quot;
version = &quot;0.1.0&quot;
authors = [&quot;Your Name &lt;email@address.com&gt;&quot;]
edition = &quot;2018&quot;

[lib]
crate-type = [&quot;cdylib&quot;]

[dependencies]
seed = &quot;^0.1.6&quot;
wasm-bindgen = &quot;^0.2.29&quot;
web-sys = &quot;^0.3.6&quot;

# For serialization, eg sending requests to a server. Otherwise, not required.
serde = &quot;^1.0.80&quot;
serde_derive = &quot;^1.0.80&quot;
serde_json = &quot;1.0.33&quot;</code></pre>
<h2 id="a-short-example">A short example</h2>
<p>Here's an example demonstrating structure and syntax; it can be found in working form under <code>examples/counter</code>. Descriptions of its parts are in the Guide section below. Its structure follows <a href="https://guide.elm-lang.org/architecture/">The Elm Architecture</a>.</p>
<p><em>lib.rs</em>:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb2-1" title="1"></a>
<a class="sourceLine" id="cb2-2" title="2"><span class="at">#[</span>macro_use<span class="at">]</span></a>
<a class="sourceLine" id="cb2-3" title="3"><span class="kw">extern</span> <span class="kw">crate</span> seed;</a>
<a class="sourceLine" id="cb2-4" title="4"><span class="kw">use</span> <span class="pp">seed::prelude::</span>*;</a>
<a class="sourceLine" id="cb2-5" title="5"><span class="kw">use</span> <span class="pp">wasm_bindgen::prelude::</span>*;</a>
<a class="sourceLine" id="cb2-6" title="6"></a>
<a class="sourceLine" id="cb2-7" title="7"></a>
<a class="sourceLine" id="cb2-8" title="8"><span class="co">// Model</span></a>
<a class="sourceLine" id="cb2-9" title="9"></a>
<a class="sourceLine" id="cb2-10" title="10"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb2-11" title="11"><span class="kw">struct</span> Model <span class="op">{</span></a>
<a class="sourceLine" id="cb2-12" title="12">    count: <span class="dt">i32</span>,</a>
<a class="sourceLine" id="cb2-13" title="13">    what_we_count: <span class="dt">String</span></a>
<a class="sourceLine" id="cb2-14" title="14"><span class="op">}</span></a>
<a class="sourceLine" id="cb2-15" title="15"></a>
<a class="sourceLine" id="cb2-16" title="16"><span class="co">// Setup a default here, for initialization later.</span></a>
<a class="sourceLine" id="cb2-17" title="17"><span class="kw">impl</span> <span class="bu">Default</span> <span class="kw">for</span> Model <span class="op">{</span></a>
<a class="sourceLine" id="cb2-18" title="18">    <span class="kw">fn</span> default() -&gt; <span class="kw">Self</span> <span class="op">{</span></a>
<a class="sourceLine" id="cb2-19" title="19">        <span class="kw">Self</span> <span class="op">{</span></a>
<a class="sourceLine" id="cb2-20" title="20">            count: <span class="dv">0</span>,</a>
<a class="sourceLine" id="cb2-21" title="21">            what_we_count: <span class="st">&quot;click&quot;</span>.into()</a>
<a class="sourceLine" id="cb2-22" title="22">        <span class="op">}</span></a>
<a class="sourceLine" id="cb2-23" title="23">    <span class="op">}</span></a>
<a class="sourceLine" id="cb2-24" title="24"><span class="op">}</span></a>
<a class="sourceLine" id="cb2-25" title="25"></a>
<a class="sourceLine" id="cb2-26" title="26"></a>
<a class="sourceLine" id="cb2-27" title="27"><span class="co">// Update</span></a>
<a class="sourceLine" id="cb2-28" title="28"></a>
<a class="sourceLine" id="cb2-29" title="29"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb2-30" title="30"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb2-31" title="31">    Increment,</a>
<a class="sourceLine" id="cb2-32" title="32">    Decrement,</a>
<a class="sourceLine" id="cb2-33" title="33">    ChangeWWC(<span class="dt">String</span>),</a>
<a class="sourceLine" id="cb2-34" title="34"><span class="op">}</span></a>
<a class="sourceLine" id="cb2-35" title="35"></a>
<a class="sourceLine" id="cb2-36" title="36"><span class="co">/// The sole source of updating the model; returns a fresh one.</span></a>
<a class="sourceLine" id="cb2-37" title="37"><span class="kw">fn</span> update(msg: Msg, model: Model) -&gt; Model <span class="op">{</span></a>
<a class="sourceLine" id="cb2-38" title="38">    <span class="kw">match</span> msg <span class="op">{</span></a>
<a class="sourceLine" id="cb2-39" title="39">        <span class="pp">Msg::</span>Increment =&gt; Model <span class="op">{</span>count: model.count + <span class="dv">1</span>, ..model<span class="op">}</span>,</a>
<a class="sourceLine" id="cb2-40" title="40">        <span class="pp">Msg::</span>Decrement =&gt; Model <span class="op">{</span>count: model.count - <span class="dv">1</span>, ..model<span class="op">}</span>,</a>
<a class="sourceLine" id="cb2-41" title="41">        <span class="pp">Msg::</span>ChangeWWC(what_we_count) =&gt; Model <span class="op">{</span>what_we_count, ..model <span class="op">}</span></a>
<a class="sourceLine" id="cb2-42" title="42">    <span class="op">}</span></a>
<a class="sourceLine" id="cb2-43" title="43"><span class="op">}</span></a>
<a class="sourceLine" id="cb2-44" title="44"></a>
<a class="sourceLine" id="cb2-45" title="45"></a>
<a class="sourceLine" id="cb2-46" title="46"><span class="co">// View</span></a>
<a class="sourceLine" id="cb2-47" title="47"></a>
<a class="sourceLine" id="cb2-48" title="48"><span class="co">/// A simple component.</span></a>
<a class="sourceLine" id="cb2-49" title="49"><span class="kw">fn</span> success_level(clicks: <span class="dt">i32</span>) -&gt; El&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb2-50" title="50">    <span class="kw">let</span> descrip = <span class="kw">match</span> clicks <span class="op">{</span></a>
<a class="sourceLine" id="cb2-51" title="51">        <span class="dv">0</span> ... <span class="dv">3</span> =&gt; <span class="st">&quot;Not very many 🙁&quot;</span>,</a>
<a class="sourceLine" id="cb2-52" title="52">        <span class="dv">4</span> ... <span class="dv">7</span> =&gt; <span class="st">&quot;An OK amount 😐&quot;</span>,</a>
<a class="sourceLine" id="cb2-53" title="53">        <span class="dv">8</span> ... <span class="dv">999</span> =&gt; <span class="st">&quot;Good job! 🙂&quot;</span>,</a>
<a class="sourceLine" id="cb2-54" title="54">        _ =&gt; <span class="st">&quot;You broke it 🙃&quot;</span></a>
<a class="sourceLine" id="cb2-55" title="55">    <span class="op">}</span>;</a>
<a class="sourceLine" id="cb2-56" title="56">    <span class="pp">p!</span><span class="op">[</span> descrip <span class="op">]</span></a>
<a class="sourceLine" id="cb2-57" title="57"><span class="op">}</span></a>
<a class="sourceLine" id="cb2-58" title="58"></a>
<a class="sourceLine" id="cb2-59" title="59"><span class="co">/// The top-level component we pass to the virtual dom. Must accept a ref to the model as its</span></a>
<a class="sourceLine" id="cb2-60" title="60"><span class="co">/// only argument, and output a single El.</span></a>
<a class="sourceLine" id="cb2-61" title="61"><span class="kw">fn</span> view(model: Model) -&gt; El&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb2-62" title="62">    <span class="kw">let</span> plural = <span class="kw">if</span> model.count == <span class="dv">1</span> <span class="op">{</span><span class="st">&quot;&quot;</span><span class="op">}</span> <span class="kw">else</span> <span class="op">{</span><span class="st">&quot;s&quot;</span><span class="op">}</span>;</a>
<a class="sourceLine" id="cb2-63" title="63"></a>
<a class="sourceLine" id="cb2-64" title="64">    <span class="co">// Attrs, Style, Events, and children may be defined separately.</span></a>
<a class="sourceLine" id="cb2-65" title="65">    <span class="kw">let</span> outer_style = <span class="pp">style!</span><span class="op">{</span></a>
<a class="sourceLine" id="cb2-66" title="66">            <span class="st">&quot;display&quot;</span> =&gt; <span class="st">&quot;flex&quot;</span>;</a>
<a class="sourceLine" id="cb2-67" title="67">            <span class="st">&quot;flex-direction&quot;</span> =&gt; <span class="st">&quot;column&quot;</span>;</a>
<a class="sourceLine" id="cb2-68" title="68">            <span class="st">&quot;text-align&quot;</span> =&gt; <span class="st">&quot;center&quot;</span></a>
<a class="sourceLine" id="cb2-69" title="69">    <span class="op">}</span>;</a>
<a class="sourceLine" id="cb2-70" title="70"></a>
<a class="sourceLine" id="cb2-71" title="71">     <span class="pp">div!</span><span class="op">[</span> outer_style,</a>
<a class="sourceLine" id="cb2-72" title="72">        <span class="pp">h1!</span><span class="op">[</span> <span class="st">&quot;The Grand Total&quot;</span> <span class="op">]</span>,</a>
<a class="sourceLine" id="cb2-73" title="73">        <span class="pp">div!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb2-74" title="74">            <span class="pp">style!</span><span class="op">{</span></a>
<a class="sourceLine" id="cb2-75" title="75">                <span class="co">// Example of conditional logic in a style.</span></a>
<a class="sourceLine" id="cb2-76" title="76">                <span class="st">&quot;color&quot;</span> =&gt; <span class="kw">if</span> model.count &gt; <span class="dv">4</span> <span class="op">{</span><span class="st">&quot;purple&quot;</span><span class="op">}</span> <span class="kw">else</span> <span class="op">{</span><span class="st">&quot;gray&quot;</span><span class="op">}</span>;</a>
<a class="sourceLine" id="cb2-77" title="77">                <span class="co">// When passing numerical values to style!, &quot;px&quot; is implied.</span></a>
<a class="sourceLine" id="cb2-78" title="78">                <span class="st">&quot;border&quot;</span> =&gt; <span class="st">&quot;2px solid #004422&quot;</span>; <span class="st">&quot;padding&quot;</span> =&gt; <span class="dv">20</span></a>
<a class="sourceLine" id="cb2-79" title="79">            <span class="op">}</span>,</a>
<a class="sourceLine" id="cb2-80" title="80">            <span class="co">// We can use normal Rust code and comments in the view.</span></a>
<a class="sourceLine" id="cb2-81" title="81">            <span class="pp">h3!</span><span class="op">[</span> <span class="pp">format!</span>(<span class="st">&quot;{} {}{} so far&quot;</span>, model.count, model.what_we_count, plural) <span class="op">]</span>,</a>
<a class="sourceLine" id="cb2-82" title="82">            <span class="pp">button!</span><span class="op">[</span> simple_ev(<span class="st">&quot;click&quot;</span>, <span class="pp">Msg::</span>Increment), <span class="st">&quot;+&quot;</span> <span class="op">]</span>,</a>
<a class="sourceLine" id="cb2-83" title="83">            <span class="pp">button!</span><span class="op">[</span> simple_ev(<span class="st">&quot;click&quot;</span>, <span class="pp">Msg::</span>Decrement), <span class="st">&quot;-&quot;</span> <span class="op">]</span>,</a>
<a class="sourceLine" id="cb2-84" title="84"></a>
<a class="sourceLine" id="cb2-85" title="85">            <span class="co">// Optionally-displaying an element</span></a>
<a class="sourceLine" id="cb2-86" title="86">            <span class="kw">if</span> model.count &gt;= <span class="dv">10</span> <span class="op">{</span> <span class="pp">h2!</span><span class="op">[</span> <span class="pp">style!</span><span class="op">{</span><span class="st">&quot;padding&quot;</span> =&gt; <span class="dv">50</span><span class="op">}</span>, <span class="st">&quot;Nice!&quot;</span> <span class="op">]</span> <span class="op">}</span> <span class="kw">else</span> <span class="op">{</span> <span class="pp">seed::</span>empty() <span class="op">}</span></a>
<a class="sourceLine" id="cb2-87" title="87"></a>
<a class="sourceLine" id="cb2-88" title="88">            <span class="op">]</span>,</a>
<a class="sourceLine" id="cb2-89" title="89">        success_level(model.count),  <span class="co">// Incorporating a separate component</span></a>
<a class="sourceLine" id="cb2-90" title="90"></a>
<a class="sourceLine" id="cb2-91" title="91">        <span class="pp">h3!</span><span class="op">[</span> <span class="st">&quot;What precisely is it we&#39;re counting?&quot;</span> <span class="op">]</span>,</a>
<a class="sourceLine" id="cb2-92" title="92">        <span class="pp">input!</span><span class="op">[</span> <span class="pp">attrs!</span><span class="op">{</span><span class="st">&quot;value&quot;</span> =&gt; model.what_we_count<span class="op">}</span>, input_ev(<span class="st">&quot;input&quot;</span>, <span class="pp">Msg::</span>ChangeWWC) <span class="op">]</span></a>
<a class="sourceLine" id="cb2-93" title="93">    <span class="op">]</span></a>
<a class="sourceLine" id="cb2-94" title="94"><span class="op">}</span></a>
<a class="sourceLine" id="cb2-95" title="95"></a>
<a class="sourceLine" id="cb2-96" title="96"></a>
<a class="sourceLine" id="cb2-97" title="97"><span class="at">#[</span>wasm_bindgen<span class="at">]</span></a>
<a class="sourceLine" id="cb2-98" title="98"><span class="kw">pub</span> <span class="kw">fn</span> render() <span class="op">{</span></a>
<a class="sourceLine" id="cb2-99" title="99">    <span class="co">// The final parameter is an optional routing map.</span></a>
<a class="sourceLine" id="cb2-100" title="100">    <span class="pp">seed::</span>run(<span class="pp">Model::</span><span class="kw">default</span>(), update, view, <span class="st">&quot;main&quot;</span>, <span class="cn">None</span>);</a>
<a class="sourceLine" id="cb2-101" title="101"><span class="op">}</span></a></code></pre></div>
<p>For truly minimimal example, see <a href="https://github.com/David-OConnor/seed-quickstart/blob/master/src/lib.rs">lib.rs in the quickstart repo</a></p>
<h2 id="building-and-running">Building and running</h2>
<p>To build your app, create a <code>pkg</code> subdirectory, and run the following two commands:</p>
<pre><code>cargo build --target wasm32-unknown-unknown</code></pre>
<p>and</p>
<pre><code>wasm-bindgen target/wasm32-unknown-unknown/debug/appname.wasm --no modules --out-dir ./pkg</code></pre>
<p>where <code>appname</code> is replaced with your app's name. This compiles your code in the target folder, and populates the pkg folder with your WASM module, a Typescript definitions file, and a JS file used to link your module from HTML.</p>
<p>You may wish to create a build script with these two lines. (<code>build.sh</code> for Linux; <code>build.ps1</code> for Windows). The quickstart repo includes these, but you'll still need to do the rename. You can then use <code>./build.sh</code> or <code>.\build.ps1</code> If you run into permission errors on <code>build.sh</code>, try this command to allow executing the file:<code>chmod +x build.sh</code>.</p>
<p>For development, you can view your app using a shimmed Python dev server, as described above. (Set up <a href="https://github.com/David-OConnor/seed-quickstart/blob/master/serve.py">this mime-type shim</a> from the quickstart repo, and run <code>python serve.py</code>).</p>
<p>In the future, the build script and commands above may be replaced by <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</p>
<h2 id="running-included-examples">Running included examples</h2>
<p>To run an example located in the <a href="https://github.com/David-OConnor/seed/tree/master/examples">examples folder</a>, navigate to that folder in a terminal, run the build script for your system (<code>build.sh</code> or <code>build.ps1</code>), then start a dev server as described above. Note that if you copy an example to a separate folder, you'll need to edit its <code>Cargo.toml</code> to point to the package on <a href="https://crates.io">crates.io</a> instead of locally: Ie replace <code>seed = { path = "../../"</code> with <code>seed = "^0.1.8"</code>, and in the build script, remove the leading <code>../../</code> on the second line.</p>
"#.into()
}