pub fn text() -> String {
r#"
<h1 id="http-requests-fetch-and-updating-state">Http requests (fetch), and updating state</h1>
<p>We use the <a href="https://docs.rs/seed/0.2.7/seed/fetch/struct.Request.html">seed::Request</a> struct to make HTTP requests in the browser, wrapping the <a href="https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API">Fetch API</a>. To use this, we need to include <code>futures = "^0.1.20"</code> in <code>Cargo.toml</code>. The <a href="https://docs.rs/seed/0.2.7/seed/fetch/index.html">Fetch module</a> is standalone: It can be used with any wasm-bindgen program.</p>
<p>Example, where we update the state on initial load:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb1-1" title="1"><span class="kw">use</span> <span class="pp">seed::</span><span class="op">{</span>Request, Method, spawn_local<span class="op">}</span></a>
<a class="sourceLine" id="cb1-2" title="2"><span class="kw">use</span> <span class="pp">futures::</span>Future;</a>
<a class="sourceLine" id="cb1-3" title="3"><span class="kw">use</span> <span class="pp">serde::</span><span class="op">{</span>Serialize, Deserialize<span class="op">}</span>;</a>
<a class="sourceLine" id="cb1-4" title="4"></a>
<a class="sourceLine" id="cb1-5" title="5"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">,</span> Serialize<span class="at">,</span> Deserialize<span class="at">)]</span></a>
<a class="sourceLine" id="cb1-6" title="6"><span class="kw">pub</span> <span class="kw">struct</span> Commit <span class="op">{</span></a>
<a class="sourceLine" id="cb1-7" title="7">    <span class="kw">pub</span> sha: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb1-8" title="8"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-9" title="9"></a>
<a class="sourceLine" id="cb1-10" title="10"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">,</span> Serialize<span class="at">,</span> Deserialize<span class="at">)]</span></a>
<a class="sourceLine" id="cb1-11" title="11"><span class="kw">pub</span> <span class="kw">struct</span> Branch <span class="op">{</span></a>
<a class="sourceLine" id="cb1-12" title="12">    <span class="kw">pub</span> name: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb1-13" title="13">    <span class="kw">pub</span> commit: Commit,</a>
<a class="sourceLine" id="cb1-14" title="14"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-15" title="15"></a>
<a class="sourceLine" id="cb1-16" title="16"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb1-17" title="17"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb1-18" title="18">    Replace(Branch),</a>
<a class="sourceLine" id="cb1-19" title="19"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-20" title="20"></a>
<a class="sourceLine" id="cb1-21" title="21"><span class="kw">fn</span> update(msg: Msg, model: Model) -&gt; Update&lt;Model&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb1-22" title="22">    <span class="kw">match</span> msg <span class="op">{</span></a>
<a class="sourceLine" id="cb1-23" title="23">        Render(<span class="pp">Msg::</span>Replace(data) =&gt; Model <span class="op">{</span>data<span class="op">}</span>),</a>
<a class="sourceLine" id="cb1-24" title="24">    <span class="op">}</span></a>
<a class="sourceLine" id="cb1-25" title="25"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-26" title="26"></a>
<a class="sourceLine" id="cb1-27" title="27"><span class="kw">fn</span> get_data(state: <span class="pp">seed::</span>App&lt;Msg, Model&gt;) -&gt; <span class="kw">impl</span> Future&lt;Item = (), Error = JsValue&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb1-28" title="28">    <span class="kw">let</span> url = <span class="st">&quot;https://api.github.com/repos/david-oconnor/seed/branches/master&quot;</span>;</a>
<a class="sourceLine" id="cb1-29" title="29"></a>
<a class="sourceLine" id="cb1-30" title="30">    <span class="pp">Request::</span>new(url)</a>
<a class="sourceLine" id="cb1-31" title="31">        .method(<span class="pp">Method::</span>Get)</a>
<a class="sourceLine" id="cb1-32" title="32">        .fetch_json()</a>
<a class="sourceLine" id="cb1-33" title="33">        .map(<span class="kw">move</span> |json| <span class="op">{</span></a>
<a class="sourceLine" id="cb1-34" title="34">            state.update(<span class="pp">Msg::</span>Replace(json));</a>
<a class="sourceLine" id="cb1-35" title="35">        <span class="op">}</span>)</a>
<a class="sourceLine" id="cb1-36" title="36"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-37" title="37"></a>
<a class="sourceLine" id="cb1-38" title="38"><span class="kw">fn</span> view(state: <span class="pp">seed::</span>App&lt;Msg, Model&gt;, model: &amp;Model) -&gt; El&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb1-39" title="39">    <span class="pp">div!</span><span class="op">[</span> <span class="pp">format!</span>(<span class="st">&quot;name: {}, sha: {}&quot;</span>, model.data.name, model.data.commit.sha),</a>
<a class="sourceLine" id="cb1-40" title="40">        did_mount(<span class="kw">move</span> |_| spawn_local(get_data(state.clone())))</a>
<a class="sourceLine" id="cb1-41" title="41">     <span class="op">]</span></a>
<a class="sourceLine" id="cb1-42" title="42"><span class="op">}</span></a></code></pre></div>
<p>When the top-level element is rendered for the first time (<code>did_mount</code>), we make a <code>get</code> request by passing the url, options like headers (In this example, we don't use any), and a callback to be executed once the data's received. In this case, we update our state by sending a message which contains the data to <code>state.update</code>. Note the signature of our get_data func, and that we always wrap calls to <code>seed::Request</code> with <code>seed::spawn_local</code>.</p>
<p>We've set up nested structs that have fields matching the names of the JSON fields of the response, which <code>Serde</code> deserializes the response into, through the <code>fetch_json</code> method of <code>Request</code>. Note that even though more data than what's contained in our Branch struct is included in the response, Serde automatically applies only the info matching our struct's fields. In order to update our state outside of a normal event, we used <code>did_mount</code>.</p>
<p>If we wish to trigger this update from a normal event instead of on load, we can do something like this:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb2-1" title="1"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb2-2" title="2"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb2-3" title="3">    Replace(Branch),</a>
<a class="sourceLine" id="cb2-4" title="4">    GetData(<span class="pp">seed::</span>App&lt;Msg, Model&gt;),</a>
<a class="sourceLine" id="cb2-5" title="5"><span class="op">}</span></a>
<a class="sourceLine" id="cb2-6" title="6"></a>
<a class="sourceLine" id="cb2-7" title="7"><span class="kw">fn</span> update(msg: Msg, model: Model) -&gt; Update&lt;Model&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb2-8" title="8">    <span class="kw">match</span> msg <span class="op">{</span></a>
<a class="sourceLine" id="cb2-9" title="9">        <span class="pp">Msg::</span>Replace(data) =&gt; Render(Model <span class="op">{</span>data<span class="op">}</span>),</a>
<a class="sourceLine" id="cb2-10" title="10">        <span class="pp">Msg::</span>GetData(state) =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb2-11" title="11">            spawn_local(get_data(state));</a>
<a class="sourceLine" id="cb2-12" title="12">            Render(model)</a>
<a class="sourceLine" id="cb2-13" title="13">        <span class="op">}</span>,</a>
<a class="sourceLine" id="cb2-14" title="14">    <span class="op">}</span></a>
<a class="sourceLine" id="cb2-15" title="15"><span class="op">}</span></a>
<a class="sourceLine" id="cb2-16" title="16"></a>
<a class="sourceLine" id="cb2-17" title="17"><span class="kw">fn</span> view(state: <span class="pp">seed::</span>App&lt;Msg, Model&gt;, model: &amp;Model) -&gt; El&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb2-18" title="18">    <span class="pp">div!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb2-19" title="19">        <span class="pp">div!</span><span class="op">[</span> <span class="pp">format!</span>(<span class="st">&quot;Hello World. name: {}, sha: {}&quot;</span>, model.data.name, model.data.commit.sha) <span class="op">]</span>,</a>
<a class="sourceLine" id="cb2-20" title="20">        <span class="pp">button!</span><span class="op">[</span> raw_ev(<span class="pp">Ev::</span>Click, <span class="kw">move</span> |_| <span class="pp">Msg::</span>GetData(state.clone())), <span class="st">&quot;Update from the internet&quot;</span><span class="op">]</span></a>
<a class="sourceLine" id="cb2-21" title="21">    <span class="op">]</span></a>
<a class="sourceLine" id="cb2-22" title="22"><span class="op">}</span></a></code></pre></div>
<p>Example showing POST, and headers:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb3-1" title="1"><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="at">)]</span></a>
<a class="sourceLine" id="cb3-2" title="2"><span class="kw">struct</span> Message <span class="op">{</span></a>
<a class="sourceLine" id="cb3-3" title="3">    <span class="kw">pub</span> name: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb3-4" title="4">    <span class="kw">pub</span> email: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb3-5" title="5">    <span class="kw">pub</span> message: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb3-6" title="6"><span class="op">}</span></a>
<a class="sourceLine" id="cb3-7" title="7"></a>
<a class="sourceLine" id="cb3-8" title="8"><span class="at">#[</span>derive<span class="at">(</span>Deserialize<span class="at">,</span> <span class="bu">Debug</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb3-9" title="9"><span class="kw">struct</span> ServerResponse <span class="op">{</span></a>
<a class="sourceLine" id="cb3-10" title="10">    <span class="kw">pub</span> success: <span class="dt">bool</span>,</a>
<a class="sourceLine" id="cb3-11" title="11"><span class="op">}</span></a>
<a class="sourceLine" id="cb3-12" title="12"></a>
<a class="sourceLine" id="cb3-13" title="13"><span class="kw">fn</span> send() -&gt; <span class="kw">impl</span> Future&lt;Item = (), Error = JsValue&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb3-14" title="14">    <span class="kw">let</span> url = <span class="st">&quot;https://infinitea.herokuapp.com/api/contact&quot;</span>;</a>
<a class="sourceLine" id="cb3-15" title="15"></a>
<a class="sourceLine" id="cb3-16" title="16">    <span class="kw">let</span> message = Message <span class="op">{</span></a>
<a class="sourceLine" id="cb3-17" title="17">        name: <span class="st">&quot;Mark Watney&quot;</span>.into(),</a>
<a class="sourceLine" id="cb3-18" title="18">        email: <span class="st">&quot;mark@crypt.kk&quot;</span>.into(),</a>
<a class="sourceLine" id="cb3-19" title="19">        message: <span class="st">&quot;I wanna be like Iron Man&quot;</span>.into(),</a>
<a class="sourceLine" id="cb3-20" title="20">    <span class="op">}</span>;</a>
<a class="sourceLine" id="cb3-21" title="21"></a>
<a class="sourceLine" id="cb3-22" title="22">    <span class="pp">Request::</span>new(url)</a>
<a class="sourceLine" id="cb3-23" title="23">        .method(<span class="pp">Method::</span>Post)</a>
<a class="sourceLine" id="cb3-24" title="24">        .header(<span class="st">&quot;Content-Type&quot;</span>, <span class="st">&quot;application/json&quot;</span>)</a>
<a class="sourceLine" id="cb3-25" title="25">        .body_json(&amp;message)</a>
<a class="sourceLine" id="cb3-26" title="26">        .fetch_json()</a>
<a class="sourceLine" id="cb3-27" title="27">        .map(|result: ServerResponse| <span class="op">{</span></a>
<a class="sourceLine" id="cb3-28" title="28">            <span class="pp">log!</span>(<span class="pp">format!</span>(<span class="st">&quot;Response: {:?}&quot;</span>, result));</a>
<a class="sourceLine" id="cb3-29" title="29">        <span class="op">}</span>)</a>
<a class="sourceLine" id="cb3-30" title="30"><span class="op">}</span></a></code></pre></div>
<p>Note how we pass a ref to the struct we wish to serialize (the payload) to the <code>.body_json()</code> method; serialization happens out of sight. Reference the <code>Request</code> API docs (linked above) for a full list of methods available to configure the request, and links to the <code>MDN</code> docs describing them. (eg: <code>credentials</code>, <code>mode</code>, <code>integrity</code>)</p>
<h2 id="updating-state">Updating state</h2>
<p>To update the model outside of the element-based event system, we call <code>update_state</code> on our state var, which is the first parameter in our view func. A consequence of this is that we must pass state to any components that need to update state in this way. This may require calling <code>state.clone()</code>, to use it in multiple places. Note that we also need to prepend our closures with <code>move</code>, as above, any time <code>state</code> is used in one.</p>
<p>Here's an example of using set_interval to update the state once every second. It uses <code>seed::set_interval</code>:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb4-1" title="1"><span class="kw">fn</span> view(state: <span class="pp">seed::</span>App&lt;Msg, Model&gt;, model: &amp;Model) -&gt; El&lt;Msg&gt; <span class="op">{</span>  </a>
<a class="sourceLine" id="cb4-2" title="2">    <span class="pp">div!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb4-3" title="3">        did_mount(<span class="kw">move</span> |_| <span class="op">{</span></a>
<a class="sourceLine" id="cb4-4" title="4">            <span class="kw">let</span> state2 = state.clone();</a>
<a class="sourceLine" id="cb4-5" title="5"></a>
<a class="sourceLine" id="cb4-6" title="6">            <span class="kw">let</span> callback = <span class="kw">move</span> || <span class="op">{</span></a>
<a class="sourceLine" id="cb4-7" title="7">                state2.update(<span class="pp">Msg::</span>Increment);</a>
<a class="sourceLine" id="cb4-8" title="8">            <span class="op">}</span>;</a>
<a class="sourceLine" id="cb4-9" title="9"></a>
<a class="sourceLine" id="cb4-10" title="10">            <span class="pp">seed::</span>set_interval(<span class="dt">Box</span>::new(callback), <span class="dv">1000</span>);</a>
<a class="sourceLine" id="cb4-11" title="11">        <span class="op">}</span>),</a>
<a class="sourceLine" id="cb4-12" title="12">        </a>
<a class="sourceLine" id="cb4-13" title="13">        <span class="pp">button!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb4-14" title="14">            simple_ev(<span class="pp">Ev::</span>Click, <span class="pp">Msg::</span>Increment),</a>
<a class="sourceLine" id="cb4-15" title="15">            <span class="pp">format!</span>(<span class="st">&quot;Hello, World × {}&quot;</span>, model.val)</a>
<a class="sourceLine" id="cb4-16" title="16">        <span class="op">]</span></a>
<a class="sourceLine" id="cb4-17" title="17">    <span class="op">]</span></a>
<a class="sourceLine" id="cb4-18" title="18"><span class="op">}</span></a></code></pre></div>
<p><code>App::build</code> returns an instance of <code>seed::App</code>, which we can use to updated state from the <code>render</code> function. Example:</p>
<div class="sourceCode" id="cb5"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb5-1" title="1"><span class="kw">fn</span> open_websockets(state: <span class="pp">seed::</span>App&lt;Msg, Model&gt;) <span class="op">{</span></a>
<a class="sourceLine" id="cb5-2" title="2"></a>
<a class="sourceLine" id="cb5-3" title="3">  <span class="co">// setup websockets ...</span></a>
<a class="sourceLine" id="cb5-4" title="4"></a>
<a class="sourceLine" id="cb5-5" title="5">  <span class="kw">let</span> on_message = <span class="dt">Box</span>::new(<span class="kw">move</span>|ev: MessageEvent| <span class="op">{</span></a>
<a class="sourceLine" id="cb5-6" title="6">    <span class="kw">let</span> txt = ev.data().as_string().unwrap();</a>
<a class="sourceLine" id="cb5-7" title="7">    <span class="kw">let</span> json: JsonMsg = <span class="pp">serde_json::</span>from_str(&amp;text).unwrap();</a>
<a class="sourceLine" id="cb5-8" title="8">    state.update(<span class="pp">Msg::</span>Json(json));</a>
<a class="sourceLine" id="cb5-9" title="9">  <span class="op">}</span>);</a>
<a class="sourceLine" id="cb5-10" title="10"><span class="op">}</span></a>
<a class="sourceLine" id="cb5-11" title="11"></a>
<a class="sourceLine" id="cb5-12" title="12"><span class="kw">pub</span> <span class="kw">fn</span> render() <span class="op">{</span></a>
<a class="sourceLine" id="cb5-13" title="13">    <span class="kw">let</span> state = <span class="pp">App::</span>build(<span class="pp">Model::</span><span class="kw">default</span>(), update, view)</a>
<a class="sourceLine" id="cb5-14" title="14">        .finish()</a>
<a class="sourceLine" id="cb5-15" title="15">        .run();</a>
<a class="sourceLine" id="cb5-16" title="16">    open_websockets(state);</a>
<a class="sourceLine" id="cb5-17" title="17"><span class="op">}</span></a></code></pre></div>
<p>Re-examining our initial example, instead of loading the data when the top-level element mounts, we can load it in <code>render</code> like this:</p>
<div class="sourceCode" id="cb6"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb6-1" title="1"><span class="at">#[</span>wasm_bindgen<span class="at">]</span></a>
<a class="sourceLine" id="cb6-2" title="2"><span class="kw">pub</span> <span class="kw">fn</span> render() <span class="op">{</span></a>
<a class="sourceLine" id="cb6-3" title="3">    <span class="kw">let</span> state = <span class="pp">seed::App::</span>build(<span class="pp">Model::</span><span class="kw">default</span>(), update, view)</a>
<a class="sourceLine" id="cb6-4" title="4">        .finish()</a>
<a class="sourceLine" id="cb6-5" title="5">        .run();</a>
<a class="sourceLine" id="cb6-6" title="6"></a>
<a class="sourceLine" id="cb6-7" title="7">    spawn_local(get_data(state));</a>
<a class="sourceLine" id="cb6-8" title="8"><span class="op">}</span></a></code></pre></div>
<p>See the <a href="https://github.com/David-OConnor/seed/tree/master/examples/server_interaction">server_interaction example</a> for a full example.</p>
<p>Props to Pauan for writing the Fetch module.</p>
"#.into()
}