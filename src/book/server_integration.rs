pub fn text() -> String {
r#"
<h1 id="integration-with-rust-backend-servers">Integration with Rust (backend) servers</h1>
<p>If pairing Seed with a Rust backend server, we can simplify passing data between server and frontend by using a layout similar to that in the <a href="https://github.com/David-OConnor/seed/tree/master/examples/server_interaction">server_integration example</a> Here, we demonstrate using a single struct for both client and server, with <a href="https://rocket.rs/">Rocket</a> as the sersver. It doesn't simplify things much in this simple example, but is useful for reducing duplication in more complicated programs. For example, we can use use the same struct which represents a database item on a server in Seed, without redefining or changing it.</p>
<p>Highlights from the example: - We set up the frontend and backend as independent crates, with the frontend folder inside the backend one. Alternatively, we could set them up at the same nest level. - We place the shared data structures in a barebones third crate called <code>shared</code>. We can't access data on the backend crate due to it being incompatible with the <code>wasm32-unknown-unknown</code> target. We can't do the reverse due to being unable to import <code>"cdylib"</code> crates. - With <code>Rocket</code>, we must use the nightly toolchain for the backend. - We set the server and client to use different ports - The Rocket server is set up with CORS, to accept requests from localhost only. - We are unable to share a workspace between backend and frontend due to incompatible compile targets.</p>
<p>To run the example, navigate to the server_integration example, and run <code>cargo +nightly run</code>. In a different terminal, navigate to its <code>frontend</code> subdirectory, and run a build script and dev server as you would normally for a seed app.</p>
<p>Folder structure:</p>
<pre><code>backend: Our server crate, in this case Rocket
 └── frontend: A normal seed crate
 └── shared: Contains data structures shared between frontend and backend
 </code></pre>
<p>Backend Cargo.toml. A normal <code>Rocket</code> one, with the <code>shared</code> dependency, and CORS support:</p>
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
<p>Frontend Cargo.toml. The only difference from a normal Seed crate is relative path for a <code>shared</code> dependency.</p>
<pre class="toml"><code>[package]
name = &quot;frontend&quot;
version = &quot;0.1.0&quot;
authors = [&quot;Your Name &lt;email@address.com&gt;&quot;]
edition = &quot;2018&quot;


[lib]
crate-type = [&quot;cdylib&quot;]


[dependencies]
seed = &quot;^0.2.1&quot;
wasm-bindgen = &quot;^0.2.29&quot;
web-sys = &quot;^0.3.6&quot;

# For serialization, eg sending requests to a server. Otherwise, not required.
serde = { version = &quot;^1.0.80&quot;, features = [&#39;derive&#39;] }
serde_json = &quot;^1.0.33&quot;

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
<a class="sourceLine" id="cb5-3" title="3"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">,</span> Serialize<span class="at">,</span> Deserialize<span class="at">)]</span></a>
<a class="sourceLine" id="cb5-4" title="4"><span class="kw">pub</span> <span class="kw">struct</span> Data <span class="op">{</span></a>
<a class="sourceLine" id="cb5-5" title="5">    <span class="kw">pub</span> val: <span class="dt">i8</span>,</a>
<a class="sourceLine" id="cb5-6" title="6">    <span class="kw">pub</span> text: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb5-7" title="7"><span class="op">}</span></a></code></pre></div>
<p>In the frontend and backend, we import <code>shared</code>, and use these structures normally:</p>
<p>backend: use shared::Data;</p>
<div class="sourceCode" id="cb6"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb6-1" title="1"><span class="at">#[</span>get<span class="at">(</span><span class="st">&quot;/data&quot;</span><span class="at">,</span> format <span class="at">=</span> <span class="st">&quot;application/json&quot;</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb6-2" title="2"><span class="kw">fn</span> data_api() -&gt; <span class="dt">String</span> <span class="op">{</span></a>
<a class="sourceLine" id="cb6-3" title="3">    <span class="kw">let</span> data = Data <span class="op">{</span></a>
<a class="sourceLine" id="cb6-4" title="4">        val: <span class="dv">7</span>,</a>
<a class="sourceLine" id="cb6-5" title="5">        text: <span class="st">&quot;Test data&quot;</span>.into(),</a>
<a class="sourceLine" id="cb6-6" title="6">    <span class="op">}</span>;</a>
<a class="sourceLine" id="cb6-7" title="7"></a>
<a class="sourceLine" id="cb6-8" title="8">    <span class="pp">serde_json::</span>to_string(&amp;data).unwrap()</a>
<a class="sourceLine" id="cb6-9" title="9"><span class="op">}</span></a></code></pre></div>
<p>frontend:</p>
<div class="sourceCode" id="cb7"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb7-1" title="1"><span class="kw">use</span> <span class="pp">shared::</span>Data;</a>
<a class="sourceLine" id="cb7-2" title="2"></a>
<a class="sourceLine" id="cb7-3" title="3"></a>
<a class="sourceLine" id="cb7-4" title="4"><span class="co">// Model</span></a>
<a class="sourceLine" id="cb7-5" title="5"></a>
<a class="sourceLine" id="cb7-6" title="6"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb7-7" title="7"><span class="kw">struct</span> Model <span class="op">{</span></a>
<a class="sourceLine" id="cb7-8" title="8">    <span class="kw">pub</span> data: Data,</a>
<a class="sourceLine" id="cb7-9" title="9"><span class="op">}</span></a>
<a class="sourceLine" id="cb7-10" title="10"></a>
<a class="sourceLine" id="cb7-11" title="11"><span class="kw">fn</span> get_data(state: <span class="pp">seed::</span>App&lt;Msg, Model&gt;) <span class="op">{</span></a>
<a class="sourceLine" id="cb7-12" title="12">    <span class="kw">let</span> url = <span class="st">&quot;http://localhost:8001/data&quot;</span>;</a>
<a class="sourceLine" id="cb7-13" title="13">    <span class="kw">let</span> callback = <span class="kw">move</span> |json: JsValue| <span class="op">{</span></a>
<a class="sourceLine" id="cb7-14" title="14">        <span class="kw">let</span> data: Data = json.into_serde().unwrap();</a>
<a class="sourceLine" id="cb7-15" title="15">        state.update(<span class="pp">Msg::</span>Replace(data));</a>
<a class="sourceLine" id="cb7-16" title="16">    <span class="op">}</span>;</a>
<a class="sourceLine" id="cb7-17" title="17"></a>
<a class="sourceLine" id="cb7-18" title="18">    <span class="pp">seed::</span>get(url, <span class="cn">None</span>, <span class="dt">Box</span>::new(callback));</a>
<a class="sourceLine" id="cb7-19" title="19"><span class="op">}</span></a>
<a class="sourceLine" id="cb7-20" title="20"></a>
<a class="sourceLine" id="cb7-21" title="21"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb7-22" title="22"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb7-23" title="23">    GetData(<span class="pp">seed::</span>App&lt;Msg, Model&gt;),</a>
<a class="sourceLine" id="cb7-24" title="24">    Replace(Data),</a>
<a class="sourceLine" id="cb7-25" title="25"><span class="op">}</span></a>
<a class="sourceLine" id="cb7-26" title="26"></a>
<a class="sourceLine" id="cb7-27" title="27"><span class="kw">fn</span> update(msg: Msg, model: Model) -&gt; Model <span class="op">{</span></a>
<a class="sourceLine" id="cb7-28" title="28">    <span class="kw">match</span> msg <span class="op">{</span></a>
<a class="sourceLine" id="cb7-29" title="29">        <span class="pp">Msg::</span>GetData(state) =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb7-30" title="30">            get_data(state);</a>
<a class="sourceLine" id="cb7-31" title="31">            model</a>
<a class="sourceLine" id="cb7-32" title="32">        <span class="op">}</span>,</a>
<a class="sourceLine" id="cb7-33" title="33">        <span class="pp">Msg::</span>Replace(data) =&gt; Model <span class="op">{</span>data<span class="op">}</span></a>
<a class="sourceLine" id="cb7-34" title="34">    <span class="op">}</span></a>
<a class="sourceLine" id="cb7-35" title="35"><span class="op">}</span></a></code></pre></div>
"#.into()
}