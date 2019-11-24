pub fn text() -> String {
r#####"
<h1 id="integration-with-rust-backend-servers">Integration with Rust (backend) servers</h1>
<p>If pairing Seed with a Rust backend server, we can simplify passing data between server and frontend using a layout like that in the <a href="https://github.com/David-OConnor/seed/tree/master/examples/server_integration">server_integration example</a></p>
<p>A key advantage of this approach is that you can reuse data structures, and code that operates on them on both client and server. We use <code>Serde</code> to elegantly, and mostly transparently, handle [de]serialization. For example, we can use use the same struct which represents a database model on a server in Seed, without redefining or changing it. This includes keeping the same methods on both server and client.</p>
<p>The <a href="https://erwabook.com/">Engineering Rust Web Applications book</a> is an excellent resource showing a more detailed layout including a database using <a href="https://diesel.rs">Diesel</a>, as a step-by-step tutorial. You may wish to stop reading this page now, and skip directly to reading this book.</p>
<p>Highlights:</p>
<ul>
<li>We set up three crates, each with its own <code>Cargo.toml</code>: One each for server, client, and shared code.</li>
<li>We place the shared data structures in a barebones third crate called <code>shared</code>.</li>
<li>We set the server and client to use different ports</li>
</ul>
<p>Folder structure:</p>
<pre><code>project folder: 
 └── server: Our Rust server crate, in this case Rocket
 └── client: A normal Seed crate
 └── shared: Contains data structures shared between the server and client</code></pre>
<p>The top-level project folder contains a <code>Cargo.toml</code> that may look like this:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb2-1"><a href="#cb2-1"></a><span class="op">[</span>workspace<span class="op">]</span></span>
<span id="cb2-2"><a href="#cb2-2"></a></span>
<span id="cb2-3"><a href="#cb2-3"></a>members = <span class="op">[</span></span>
<span id="cb2-4"><a href="#cb2-4"></a>    <span class="st">&quot;client&quot;</span>,</span>
<span id="cb2-5"><a href="#cb2-5"></a>    <span class="st">&quot;server&quot;</span>,</span>
<span id="cb2-6"><a href="#cb2-6"></a><span class="op">]</span></span></code></pre></div>
<p>A makefile, which will may additional scripts from those included in the quickstart for running the server, client etc.</p>
<p>Server <code>Cargo.toml</code>: A normal one for <code>Rocket</code>/<code>Actix</code> etc, with a relative-path <code>shared</code> dependency</p>
<pre class="toml"><code>[package]
name = &quot;server&quot;
version = &quot;0.1.0&quot;
authors = [&quot;Your Name &lt;email@address.com&gt;&quot;]
edition = &quot;2018&quot;

[dependencies]
actix = &quot;0.8.3&quot;
actix-web = &quot;1.0.0&quot;
actix-files = &quot;0.1.1&quot;
actix-multipart = &quot;0.1.2&quot;
tokio-timer = &quot;0.2.11&quot;

shared = { path = &quot;../shared&quot; }</code></pre>
<p>The client’s <code>cargo.toml</code> is a standard Seed one. The shared <code>Cargo.toml</code> includes whatever you need for your shared data structures and code; it will usually include <code>serde</code> for serializing and deserializing, and may include database code, since this crate is a good place for databse models and schema.</p>
<pre class="toml"><code>[package]
name = &quot;shared&quot;
version = &quot;0.1.0&quot;
authors = [&quot;Your Name &lt;email@address.com&gt;&quot;]
edition = &quot;2018&quot;

[dependencies]
serde = { version = &quot;^1.0.80&quot;, features = [&#39;derive&#39;] }
diesel = { version = &quot;^1.4.2&quot;, features = [&quot;postgres&quot;] }</code></pre>
<p>In <code>shared/lib.rs</code>, we set up serializable data structures:</p>
<div class="sourceCode" id="cb5"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb5-1"><a href="#cb5-1"></a><span class="kw">use</span> <span class="pp">serde::</span><span class="op">{</span>Serialize, Deserialize<span class="op">}</span>;</span>
<span id="cb5-2"><a href="#cb5-2"></a></span>
<span id="cb5-3"><a href="#cb5-3"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="at">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb5-4"><a href="#cb5-4"></a><span class="kw">pub</span> <span class="kw">struct</span> Data <span class="op">{</span></span>
<span id="cb5-5"><a href="#cb5-5"></a>    <span class="kw">pub</span> val: <span class="dt">i8</span>,</span>
<span id="cb5-6"><a href="#cb5-6"></a>    <span class="kw">pub</span> text: <span class="dt">String</span>,</span>
<span id="cb5-7"><a href="#cb5-7"></a><span class="op">}</span></span></code></pre></div>
<p>In the server and client, we import <code>shared</code>, and use these structures normally:</p>
<p>Eg server using <code>Rocket</code>:</p>
<div class="sourceCode" id="cb6"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb6-1"><a href="#cb6-1"></a><span class="kw">use</span> <span class="pp">shared::</span>Data;</span>
<span id="cb6-2"><a href="#cb6-2"></a></span>
<span id="cb6-3"><a href="#cb6-3"></a><span class="at">#[</span>get<span class="at">(</span><span class="st">&quot;/data&quot;</span><span class="at">,</span> format <span class="at">=</span> <span class="st">&quot;application/json&quot;</span><span class="at">)]</span></span>
<span id="cb6-4"><a href="#cb6-4"></a><span class="kw">fn</span> data_api() -&gt; <span class="dt">String</span> <span class="op">{</span></span>
<span id="cb6-5"><a href="#cb6-5"></a>    <span class="kw">let</span> data = Data <span class="op">{</span></span>
<span id="cb6-6"><a href="#cb6-6"></a>        val: <span class="dv">7</span>,</span>
<span id="cb6-7"><a href="#cb6-7"></a>        text: <span class="st">&quot;Test data&quot;</span>.into(),</span>
<span id="cb6-8"><a href="#cb6-8"></a>    <span class="op">}</span>;</span>
<span id="cb6-9"><a href="#cb6-9"></a></span>
<span id="cb6-10"><a href="#cb6-10"></a>    <span class="pp">serde_json::</span>to_string(&amp;data).unwrap()</span>
<span id="cb6-11"><a href="#cb6-11"></a><span class="op">}</span></span></code></pre></div>
<p>Client, showing how you might use the same struct as part of the model, and update it from the server:</p>
<div class="sourceCode" id="cb7"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb7-1"><a href="#cb7-1"></a><span class="kw">use</span> <span class="pp">shared::</span>Data;</span>
<span id="cb7-2"><a href="#cb7-2"></a></span>
<span id="cb7-3"><a href="#cb7-3"></a><span class="kw">struct</span> Model <span class="op">{</span></span>
<span id="cb7-4"><a href="#cb7-4"></a>    <span class="kw">pub</span> data: Data,</span>
<span id="cb7-5"><a href="#cb7-5"></a><span class="op">}</span></span>
<span id="cb7-6"><a href="#cb7-6"></a></span>
<span id="cb7-7"><a href="#cb7-7"></a><span class="kw">fn</span> get_data() -&gt; <span class="kw">impl</span> Future&lt;Item = Msg, Error = Msg&gt; <span class="op">{</span></span>
<span id="cb7-8"><a href="#cb7-8"></a>    <span class="kw">let</span> url = <span class="st">&quot;https://localhost:8001/get_data&quot;</span>;</span>
<span id="cb7-9"><a href="#cb7-9"></a></span>
<span id="cb7-10"><a href="#cb7-10"></a>    <span class="pp">Request::</span>new(url)</span>
<span id="cb7-11"><a href="#cb7-11"></a>        .method(<span class="pp">Method::</span>Get)</span>
<span id="cb7-12"><a href="#cb7-12"></a>        .fetch_json()</span>
<span id="cb7-13"><a href="#cb7-13"></a>        .map(<span class="pp">Msg::</span>Replace)</span>
<span id="cb7-14"><a href="#cb7-14"></a>        .map_err(<span class="pp">Msg::</span>OnFetchErr)</span>
<span id="cb7-15"><a href="#cb7-15"></a><span class="op">}</span></span>
<span id="cb7-16"><a href="#cb7-16"></a></span>
<span id="cb7-17"><a href="#cb7-17"></a><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></span>
<span id="cb7-18"><a href="#cb7-18"></a><span class="kw">enum</span> Msg <span class="op">{</span></span>
<span id="cb7-19"><a href="#cb7-19"></a>    GetData,</span>
<span id="cb7-20"><a href="#cb7-20"></a>    Replace(Data),</span>
<span id="cb7-21"><a href="#cb7-21"></a>    OnServerResponse(ServerResponse),</span>
<span id="cb7-22"><a href="#cb7-22"></a>    OnFetchErr(JsValue),</span>
<span id="cb7-23"><a href="#cb7-23"></a><span class="op">}</span></span>
<span id="cb7-24"><a href="#cb7-24"></a></span>
<span id="cb7-25"><a href="#cb7-25"></a><span class="kw">fn</span> update(msg: Msg, model: &amp;<span class="kw">mut</span> Model, orders: &amp;<span class="kw">mut</span> <span class="kw">impl</span> Orders&lt;Msg&gt;) <span class="op">{</span></span>
<span id="cb7-26"><a href="#cb7-26"></a>    <span class="kw">match</span> msg <span class="op">{</span></span>
<span id="cb7-27"><a href="#cb7-27"></a>        <span class="pp">Msg::</span>Replace(data) =&gt; model.data = data,</span>
<span id="cb7-28"><a href="#cb7-28"></a></span>
<span id="cb7-29"><a href="#cb7-29"></a>        <span class="pp">Msg::</span>GetData =&gt; <span class="op">{</span></span>
<span id="cb7-30"><a href="#cb7-30"></a>            orders.skip().perform_cmd(get_data());</span>
<span id="cb7-31"><a href="#cb7-31"></a>        <span class="op">}</span></span>
<span id="cb7-32"><a href="#cb7-32"></a></span>
<span id="cb7-33"><a href="#cb7-33"></a>        <span class="pp">Msg::</span>OnServerResponse(result) =&gt; <span class="op">{</span></span>
<span id="cb7-34"><a href="#cb7-34"></a>            <span class="pp">log!</span>(<span class="pp">format!</span>(<span class="st">&quot;Response: {:?}&quot;</span>, result));</span>
<span id="cb7-35"><a href="#cb7-35"></a>            orders.skip();</span>
<span id="cb7-36"><a href="#cb7-36"></a>        <span class="op">}</span></span>
<span id="cb7-37"><a href="#cb7-37"></a></span>
<span id="cb7-38"><a href="#cb7-38"></a>        <span class="pp">Msg::</span>OnFetchErr(err) =&gt; <span class="op">{</span></span>
<span id="cb7-39"><a href="#cb7-39"></a>            <span class="pp">error!</span>(<span class="pp">format!</span>(<span class="st">&quot;Fetch error: {:?}&quot;</span>, err));</span>
<span id="cb7-40"><a href="#cb7-40"></a>            orders.skip();</span>
<span id="cb7-41"><a href="#cb7-41"></a>        <span class="op">}</span></span>
<span id="cb7-42"><a href="#cb7-42"></a>    <span class="op">}</span></span>
<span id="cb7-43"><a href="#cb7-43"></a><span class="op">}</span></span></code></pre></div>
"#####.into()
}