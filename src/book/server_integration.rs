pub fn text() -> String {
r#####"
<h1 id="integration-with-rust-backend-servers">Integration with Rust (backend) servers</h1>
<p>If pairing Seed with a Rust backend server, we can simplify passing data between server and frontend using a layout like that in the <a href="https://github.com/David-OConnor/seed/tree/master/examples/server_integration">server_integration example</a></p>
<p>Additionally, the <a href="https://erwabook.com/">Engineering Rust Web Applications book</a> is a great resource showing a more detailed layout including a database using <a href="https://diesel.rs">Diesel</a>, as a step-by-step tutorial.</p>
<p>A key advantage of this approach is that you can reuse data structures, and code that operates on them on both client and server. We use <code>Serde</code> to elegantly, and mostly transparently, handle [de]serialization. For example, we can use use the same struct which represents a database model on a server in Seed, without redefining or changing it. This includes keeping the same methods on both server and client.</p>
<p>Highlights from the example:</p>
<ul>
<li>We set up three crates, each with its own <code>Cargo.toml</code>: One each for server, client, and shared code.</li>
<li>We place the shared data structures in a barebones third crate called <code>shared</code>.</li>
<li>We set the server and client to use different ports</li>
</ul>
<p>Folder structure:</p>
<pre><code>project folder: 
 └── server: Our Rust server crate, in this case Rocket
 └── frontend: A normal Seed crate
 └── shared: Contains data structures shared between frontend and backend</code></pre>
<p>The top-level project folder contains a <code>Cargo.toml</code> that may look like this:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb2-1" title="1"><span class="op">[</span>workspace<span class="op">]</span></a>
<a class="sourceLine" id="cb2-2" title="2"></a>
<a class="sourceLine" id="cb2-3" title="3">members = <span class="op">[</span></a>
<a class="sourceLine" id="cb2-4" title="4">    <span class="st">&quot;client&quot;</span>,</a>
<a class="sourceLine" id="cb2-5" title="5">    <span class="st">&quot;server&quot;</span>,</a>
<a class="sourceLine" id="cb2-6" title="6"><span class="op">]</span></a></code></pre></div>
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
<div class="sourceCode" id="cb5"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb5-1" title="1"><span class="kw">use</span> <span class="pp">serde::</span><span class="op">{</span>Serialize, Deserialize<span class="op">}</span>;</a>
<a class="sourceLine" id="cb5-2" title="2"></a>
<a class="sourceLine" id="cb5-3" title="3"><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="at">,</span> Deserialize<span class="at">)]</span></a>
<a class="sourceLine" id="cb5-4" title="4"><span class="kw">pub</span> <span class="kw">struct</span> Data <span class="op">{</span></a>
<a class="sourceLine" id="cb5-5" title="5">    <span class="kw">pub</span> val: <span class="dt">i8</span>,</a>
<a class="sourceLine" id="cb5-6" title="6">    <span class="kw">pub</span> text: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb5-7" title="7"><span class="op">}</span></a></code></pre></div>
<p>In the server and client, we import <code>shared</code>, and use these structures normally:</p>
<p>Eg server using <code>Rocket</code>:</p>
<div class="sourceCode" id="cb6"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb6-1" title="1"><span class="kw">use</span> <span class="pp">shared::</span>Data;</a>
<a class="sourceLine" id="cb6-2" title="2"></a>
<a class="sourceLine" id="cb6-3" title="3"><span class="at">#[</span>get<span class="at">(</span><span class="st">&quot;/data&quot;</span><span class="at">,</span> format <span class="at">=</span> <span class="st">&quot;application/json&quot;</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb6-4" title="4"><span class="kw">fn</span> data_api() -&gt; <span class="dt">String</span> <span class="op">{</span></a>
<a class="sourceLine" id="cb6-5" title="5">    <span class="kw">let</span> data = Data <span class="op">{</span></a>
<a class="sourceLine" id="cb6-6" title="6">        val: <span class="dv">7</span>,</a>
<a class="sourceLine" id="cb6-7" title="7">        text: <span class="st">&quot;Test data&quot;</span>.into(),</a>
<a class="sourceLine" id="cb6-8" title="8">    <span class="op">}</span>;</a>
<a class="sourceLine" id="cb6-9" title="9"></a>
<a class="sourceLine" id="cb6-10" title="10">    <span class="pp">serde_json::</span>to_string(&amp;data).unwrap()</a>
<a class="sourceLine" id="cb6-11" title="11"><span class="op">}</span></a></code></pre></div>
<p>Client, showing how you might use the same struct as part of the model, and update it from the backend:</p>
<div class="sourceCode" id="cb7"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb7-1" title="1"><span class="kw">use</span> <span class="pp">shared::</span>Data;</a>
<a class="sourceLine" id="cb7-2" title="2"></a>
<a class="sourceLine" id="cb7-3" title="3"><span class="kw">struct</span> Model <span class="op">{</span></a>
<a class="sourceLine" id="cb7-4" title="4">    <span class="kw">pub</span> data: Data,</a>
<a class="sourceLine" id="cb7-5" title="5"><span class="op">}</span></a>
<a class="sourceLine" id="cb7-6" title="6"></a>
<a class="sourceLine" id="cb7-7" title="7"><span class="kw">fn</span> get_data() -&gt; <span class="kw">impl</span> Future&lt;Item = Msg, Error = Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb7-8" title="8">    <span class="kw">let</span> url = <span class="st">&quot;https://localhost:8001/get_data&quot;</span>;</a>
<a class="sourceLine" id="cb7-9" title="9"></a>
<a class="sourceLine" id="cb7-10" title="10">    <span class="pp">Request::</span>new(url)</a>
<a class="sourceLine" id="cb7-11" title="11">        .method(<span class="pp">Method::</span>Get)</a>
<a class="sourceLine" id="cb7-12" title="12">        .fetch_json()</a>
<a class="sourceLine" id="cb7-13" title="13">        .map(<span class="pp">Msg::</span>Replace)</a>
<a class="sourceLine" id="cb7-14" title="14">        .map_err(<span class="pp">Msg::</span>OnFetchErr)</a>
<a class="sourceLine" id="cb7-15" title="15"><span class="op">}</span></a>
<a class="sourceLine" id="cb7-16" title="16"></a>
<a class="sourceLine" id="cb7-17" title="17"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb7-18" title="18"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb7-19" title="19">    GetData,</a>
<a class="sourceLine" id="cb7-20" title="20">    Replace(Data),</a>
<a class="sourceLine" id="cb7-21" title="21">    OnServerResponse(ServerResponse),</a>
<a class="sourceLine" id="cb7-22" title="22">    OnFetchErr(JsValue),</a>
<a class="sourceLine" id="cb7-23" title="23"><span class="op">}</span></a>
<a class="sourceLine" id="cb7-24" title="24"></a>
<a class="sourceLine" id="cb7-25" title="25"><span class="kw">fn</span> update(msg: Msg, model: &amp;<span class="kw">mut</span> Model, orders: &amp;<span class="kw">mut</span> <span class="kw">impl</span> Orders&lt;Msg&gt;) <span class="op">{</span></a>
<a class="sourceLine" id="cb7-26" title="26">    <span class="kw">match</span> msg <span class="op">{</span></a>
<a class="sourceLine" id="cb7-27" title="27">        <span class="pp">Msg::</span>Replace(data) =&gt; model.data = data,</a>
<a class="sourceLine" id="cb7-28" title="28"></a>
<a class="sourceLine" id="cb7-29" title="29">        <span class="pp">Msg::</span>GetData =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb7-30" title="30">            orders.skip().perform_cmd(get_data());</a>
<a class="sourceLine" id="cb7-31" title="31">        <span class="op">}</span></a>
<a class="sourceLine" id="cb7-32" title="32"></a>
<a class="sourceLine" id="cb7-33" title="33">        <span class="pp">Msg::</span>OnServerResponse(result) =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb7-34" title="34">            <span class="pp">log!</span>(<span class="pp">format!</span>(<span class="st">&quot;Response: {:?}&quot;</span>, result));</a>
<a class="sourceLine" id="cb7-35" title="35">            orders.skip();</a>
<a class="sourceLine" id="cb7-36" title="36">        <span class="op">}</span></a>
<a class="sourceLine" id="cb7-37" title="37"></a>
<a class="sourceLine" id="cb7-38" title="38">        <span class="pp">Msg::</span>OnFetchErr(err) =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb7-39" title="39">            <span class="pp">error!</span>(<span class="pp">format!</span>(<span class="st">&quot;Fetch error: {:?}&quot;</span>, err));</a>
<a class="sourceLine" id="cb7-40" title="40">            orders.skip();</a>
<a class="sourceLine" id="cb7-41" title="41">        <span class="op">}</span></a>
<a class="sourceLine" id="cb7-42" title="42">    <span class="op">}</span></a>
<a class="sourceLine" id="cb7-43" title="43"><span class="op">}</span></a></code></pre></div>
"#####.into()
}