pub fn text() -> String {
r#"
<h1 id="integration-with-rust-backend-servers">Integration with Rust (backend) servers</h1>
<p>If pairing Seed with a Rust backend server, we can simplify passing data between server and frontend using a layout like that in the <a href="https://github.com/David-OConnor/seed/tree/master/examples/server_integration">server_integration example</a> Here, we demonstrate using a single struct for both frontend and server, with <a href="https://rocket.rs/">Rocket</a>. as the server. This is useful for reducing duplication of data structures, and allows <code>Serde</code> to elegantly handle [de]serialization. For example, we can use use the same struct which represents a database item on a server in Seed, without redefining or changing it. This includes keeping the same methods on both server and client.</p>
<p>Highlights from the example:</p>
<ul>
<li>We set up the frontend and backend as independent crates, with the frontend folder inside the backend one. Alternatively, we could set them up at the same nest level.</li>
<li>We place the shared data structures in a barebones third crate called <code>shared</code>. We can't access data on the backend crate due to it being incompatible with the <code>wasm32-unknown-unknown</code> target. We can't do the reverse due to being unable to import <code>"cdylib"</code> crates.</li>
<li>With <code>Rocket</code>, we must use the nightly toolchain for the backend.</li>
<li>We set the server and client to use different ports</li>
<li>The Rocket server is set up with CORS, to accept requests from localhost only.</li>
<li>We are unable to share a workspace between backend and frontend due to incompatible compile targets.</li>
</ul>
<p>To run the example, navigate to the server_integration example, and run <code>cargo +nightly run</code>. In a different terminal, navigate to its <code>frontend</code> subdirectory, and run a build script and dev server as you would normally for a seed app.</p>
<p>Folder structure:</p>
<pre><code>backend: Our server crate, in this case Rocket
 └── frontend: A normal Seed crate
 └── shared: Contains data structures shared between frontend and backend
 </code></pre>
<p>Backend Cargo.toml. A normal <code>Rocket</code> one, with a relative-path <code>shared</code> dependency, and CORS support. Notice how we don't use workspaces:</p>
<pre class="toml"><code>[package]
name = &quot;backend&quot;
version = &quot;0.1.0&quot;
authors = [&quot;Your Name &lt;email@address.com&gt;&quot;]
edition = &quot;2018&quot;

[dependencies]
rocket = &quot;^0.4.0-rc.1&quot;
serde_json = &quot;^1.0.33&quot;
rocket_cors = &quot;^0.4.0&quot;
shared = { path = &quot;shared&quot; }</code></pre>
<p>Frontend Cargo.toml. The only difference from a normal Seed crate is the <code>shared</code> dependency. Note that we don't need to import <code>Serde</code> directly, in this case.</p>
<pre class="toml"><code>[package]
name = &quot;frontend&quot;
version = &quot;0.1.0&quot;
authors = [&quot;Your Name &lt;email@address.com&gt;&quot;]
edition = &quot;2018&quot;

[lib]
crate-type = [&quot;cdylib&quot;]

[dependencies]
futures = &quot;^0.1.20&quot;
seed = &quot;^0.2.1&quot;
wasm-bindgen = &quot;^0.2.29&quot;
web-sys = &quot;^0.3.6&quot;
shared = { path = &quot;../shared&quot;}</code></pre>
<p>Shared Cargo.toml:</p>
<pre class="toml"><code>[package]
name = &quot;shared&quot;
version = &quot;0.1.0&quot;
authors = [&quot;Your Name &lt;email@address.com&gt;&quot;]
edition = &quot;2018&quot;

[dependencies]
serde = { version = &quot;^1.0.80&quot;, features = [&#39;derive&#39;] }</code></pre>
<p>In <code>shared/lib.rs</code>, we set up serializable data structures:</p>
<div class="sourceCode" id="cb5"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb5-1" title="1"><span class="kw">use</span> <span class="pp">serde::</span><span class="op">{</span>Serialize, Deserialize<span class="op">}</span>;</a>
<a class="sourceLine" id="cb5-2" title="2"></a>
<a class="sourceLine" id="cb5-3" title="3"><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="at">,</span> Deserialize<span class="at">)]</span></a>
<a class="sourceLine" id="cb5-4" title="4"><span class="kw">pub</span> <span class="kw">struct</span> Data <span class="op">{</span></a>
<a class="sourceLine" id="cb5-5" title="5">    <span class="kw">pub</span> val: <span class="dt">i8</span>,</a>
<a class="sourceLine" id="cb5-6" title="6">    <span class="kw">pub</span> text: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb5-7" title="7"><span class="op">}</span></a></code></pre></div>
<p>In the frontend and backend, we import <code>shared</code>, and use these structures normally:</p>
<p>Backend:</p>
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
<p>Frontend, showing how you might use the same data Struct as part of the model, and update it from the backend:</p>
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
<a class="sourceLine" id="cb7-25" title="25"><span class="kw">fn</span> update(msg: Msg, model: &amp;<span class="kw">mut</span> Model, orders: &amp;<span class="kw">mut</span> Orders&lt;Msg&gt;) <span class="op">{</span></a>
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
"#.into()
}