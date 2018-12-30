pub fn text() -> String {
r#"
<h1 id="http-requests-fetch-and-updating-state">Http requests (fetch), and updating state</h1>
<p>We use the <a href="https://docs.rs/seed/0.2.0/seed/fetch/fn.fetch.html">seed::fetch</a> function to make HTTP requests in the browser, wrapping the <a href="https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API">Fetch API</a>. <code>fetch</code> takes 3 parameters:</p>
<ul>
<li>The <a href="https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods">request method</a>: a <a href="https://docs.rs/seed/0.2.0/seed/fetch/enum.Method.html">seed::Method</a></li>
<li>The url, an <code>&amp;str</code></li>
<li>An optional <a href="https://docs.rs/seed/0.2.0/seed/fetch/struct.RequestOpts.html">seed::RequstOpts</a> struct, where you can set things like headers, payload, and credentials.</li>
<li>A callback that performs actions once the request is complete. It accepts a <a href="https://docs.rs/wasm-bindgen/0.2.29/wasm_bindgen/">JsValue</a>, and returns nothing.</li>
</ul>
<p>The convenience functions <a href="https://docs.rs/seed/0.2.0/seed/fetch/fn.get.html">seed::get</a> and <a href="https://docs.rs/seed/0.2.0/seed/fetch/fn.post.html">seed::post</a> are also available; these are the same as <code>fetch</code>, but ommit the method parameter. Additionally, <code>seed::post</code> uses a non-serialized payload as a second parameter: This is any Rust struct which implements <code>serde::Serialize</code>. It overrides the payload defined in <code>RequestOpts</code>.</p>
<p>Example, where we update the state on initial app load:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb1-1" title="1"><span class="kw">use</span> <span class="pp">serde::</span><span class="op">{</span>Serialize, Deserialize<span class="op">}</span>;</a>
<a class="sourceLine" id="cb1-2" title="2"></a>
<a class="sourceLine" id="cb1-3" title="3"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">,</span> Serialize<span class="at">,</span> Deserialize<span class="at">)]</span></a>
<a class="sourceLine" id="cb1-4" title="4"><span class="kw">pub</span> <span class="kw">struct</span> Commit <span class="op">{</span></a>
<a class="sourceLine" id="cb1-5" title="5">    <span class="kw">pub</span> sha: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb1-6" title="6"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-7" title="7"></a>
<a class="sourceLine" id="cb1-8" title="8"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">,</span> Serialize<span class="at">,</span> Deserialize<span class="at">)]</span></a>
<a class="sourceLine" id="cb1-9" title="9"><span class="kw">pub</span> <span class="kw">struct</span> Branch <span class="op">{</span></a>
<a class="sourceLine" id="cb1-10" title="10">    <span class="kw">pub</span> name: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb1-11" title="11">    <span class="kw">pub</span> commit: Commit,</a>
<a class="sourceLine" id="cb1-12" title="12"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-13" title="13"></a>
<a class="sourceLine" id="cb1-14" title="14"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb1-15" title="15"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb1-16" title="16">    Replace(Branch),</a>
<a class="sourceLine" id="cb1-17" title="17"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-18" title="18"></a>
<a class="sourceLine" id="cb1-19" title="19"><span class="kw">fn</span> update(msg: Msg, model: Model) -&gt; Model <span class="op">{</span></a>
<a class="sourceLine" id="cb1-20" title="20">    <span class="kw">match</span> msg <span class="op">{</span></a>
<a class="sourceLine" id="cb1-21" title="21">        <span class="pp">Msg::</span>Replace(data) =&gt; Model <span class="op">{</span>data<span class="op">}</span>,</a>
<a class="sourceLine" id="cb1-22" title="22">    <span class="op">}</span></a>
<a class="sourceLine" id="cb1-23" title="23"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-24" title="24"></a>
<a class="sourceLine" id="cb1-25" title="25"><span class="kw">fn</span> get_data(state: <span class="pp">seed::</span>App&lt;Msg, Model&gt;) <span class="op">{</span></a>
<a class="sourceLine" id="cb1-26" title="26">    <span class="kw">let</span> url = <span class="st">&quot;https://api.github.com/repos/david-oconnor/seed/branches/master&quot;</span>;</a>
<a class="sourceLine" id="cb1-27" title="27">    <span class="kw">let</span> callback = <span class="kw">move</span> |json: JsValue| <span class="op">{</span></a>
<a class="sourceLine" id="cb1-28" title="28">        <span class="kw">let</span> data: Branch = json.into_serde().unwrap();</a>
<a class="sourceLine" id="cb1-29" title="29">        state.update(<span class="pp">Msg::</span>Replace(data));</a>
<a class="sourceLine" id="cb1-30" title="30">    <span class="op">}</span>;</a>
<a class="sourceLine" id="cb1-31" title="31">    <span class="pp">seed::</span>get(url, <span class="cn">None</span>, <span class="dt">Box</span>::new(callback));</a>
<a class="sourceLine" id="cb1-32" title="32"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-33" title="33"></a>
<a class="sourceLine" id="cb1-34" title="34"><span class="kw">fn</span> view(state: <span class="pp">seed::</span>App&lt;Msg, Model&gt;, model: Model) -&gt; El&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb1-35" title="35">    <span class="pp">div!</span><span class="op">[</span> <span class="pp">format!</span>(<span class="st">&quot;name: {}, sha: {}&quot;</span>, model.data.name, model.data.commit.sha),</a>
<a class="sourceLine" id="cb1-36" title="36">        did_mount(<span class="kw">move</span> |_| get_data(state.clone()))</a>
<a class="sourceLine" id="cb1-37" title="37">     <span class="op">]</span></a>
<a class="sourceLine" id="cb1-38" title="38"><span class="op">}</span></a></code></pre></div>
<p>When the top-level element is rendered for the first time (<code>did_mount</code>), we make a <code>get</code> request by passing the url, options like headers (In this example, we don't use any), and a callback to be executed once the data's received. In this case, we update our state by sending a message which contains the data to <code>state.update</code>.</p>
<p>We've set up nested structs that have fields matching the names of the JSON fields of the response, which Serde deserializes the response into. Note that even though more data than what's contained in our Branch struct is included in the response, Serde automatically applies only the info matching our struct's fields. In order to update our state outside of a normal event, we used <code>did_mount</code>. If we wish to trigger this update from a normal event instead of on load, we can do something like this:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb2-1" title="1"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb2-2" title="2"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb2-3" title="3">    Replace(Branch),</a>
<a class="sourceLine" id="cb2-4" title="4">    GetData(<span class="pp">seed::</span>App&lt;Msg, Model&gt;)</a>
<a class="sourceLine" id="cb2-5" title="5"><span class="op">}</span></a>
<a class="sourceLine" id="cb2-6" title="6"></a>
<a class="sourceLine" id="cb2-7" title="7"><span class="kw">fn</span> update(msg: Msg, model: Model) -&gt; Model <span class="op">{</span></a>
<a class="sourceLine" id="cb2-8" title="8">    <span class="kw">match</span> msg <span class="op">{</span></a>
<a class="sourceLine" id="cb2-9" title="9">        <span class="pp">Msg::</span>Replace(data) =&gt; Model <span class="op">{</span>data<span class="op">}</span>,</a>
<a class="sourceLine" id="cb2-10" title="10">        <span class="pp">Msg::</span>GetData(app) =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb2-11" title="11">            <span class="kw">let</span> url = <span class="st">&quot;https://api.github.com/repos/david-oconnor/seed/branches/master&quot;</span>;</a>
<a class="sourceLine" id="cb2-12" title="12">            <span class="kw">let</span> callback = <span class="kw">move</span> |json: JsValue| <span class="op">{</span></a>
<a class="sourceLine" id="cb2-13" title="13">                <span class="kw">let</span> data: Branch = json.into_serde().unwrap();</a>
<a class="sourceLine" id="cb2-14" title="14">                app.update(<span class="pp">Msg::</span>Replace(data));</a>
<a class="sourceLine" id="cb2-15" title="15">            <span class="op">}</span>;</a>
<a class="sourceLine" id="cb2-16" title="16">            <span class="pp">seed::</span>get(url, <span class="cn">None</span>, <span class="dt">Box</span>::new(callback));</a>
<a class="sourceLine" id="cb2-17" title="17">            model</a>
<a class="sourceLine" id="cb2-18" title="18">        <span class="op">}</span></a>
<a class="sourceLine" id="cb2-19" title="19">    <span class="op">}</span></a>
<a class="sourceLine" id="cb2-20" title="20"><span class="op">}</span></a>
<a class="sourceLine" id="cb2-21" title="21"></a>
<a class="sourceLine" id="cb2-22" title="22"><span class="kw">fn</span> view(state: <span class="pp">seed::</span>App&lt;Msg, Model&gt;, model: Model) -&gt; El&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb2-23" title="23">    <span class="pp">div!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb2-24" title="24">        <span class="pp">div!</span><span class="op">[</span> <span class="pp">format!</span>(<span class="st">&quot;Hello World. name: {}, sha: {}&quot;</span>, model.data.name, model.data.commit.sha) <span class="op">]</span>,</a>
<a class="sourceLine" id="cb2-25" title="25">        <span class="pp">button!</span><span class="op">[</span> raw_ev(<span class="st">&quot;click&quot;</span>, <span class="kw">move</span> |_| <span class="pp">Msg::</span>GetData(state.clone())), <span class="st">&quot;Update from the internet&quot;</span><span class="op">]</span></a>
<a class="sourceLine" id="cb2-26" title="26">    <span class="op">]</span></a>
<a class="sourceLine" id="cb2-27" title="27"><span class="op">}</span></a></code></pre></div>
<h2 id="updating-state">Updating state</h2>
<p>To update the model outside of the element-based event system, we call <code>update_state</code> on our state var, which is the first parameter in our view func. A consequence of this is that we must pass state to any components that need to update state in this way. This may require calling <code>state.clone()</code>, to use it in multiple places. Note that we also need to prepend our closures with <code>move</code>, as above, any time <code>state</code> is used in one.</p>
<p>Example showing POST, and headers:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb3-1" title="1"><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="at">,</span> Deserialize<span class="at">)]</span></a>
<a class="sourceLine" id="cb3-2" title="2"><span class="kw">struct</span> Message <span class="op">{</span></a>
<a class="sourceLine" id="cb3-3" title="3">    <span class="kw">pub</span> name: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb3-4" title="4">    <span class="kw">pub</span> email: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb3-5" title="5">    <span class="kw">pub</span> message: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb3-6" title="6"><span class="op">}</span></a>
<a class="sourceLine" id="cb3-7" title="7"></a>
<a class="sourceLine" id="cb3-8" title="8"><span class="kw">fn</span> post_data() <span class="op">{</span></a>
<a class="sourceLine" id="cb3-9" title="9">    <span class="kw">let</span> message = Message <span class="op">{</span></a>
<a class="sourceLine" id="cb3-10" title="10">        name: <span class="st">&quot;Mark Watney&quot;</span>.into(),</a>
<a class="sourceLine" id="cb3-11" title="11">        email: <span class="st">&quot;mark@crypt.kk&quot;</span>.into(),</a>
<a class="sourceLine" id="cb3-12" title="12">        message: <span class="st">&quot;I wanna be like Iron Man&quot;</span>.into(),</a>
<a class="sourceLine" id="cb3-13" title="13">    <span class="op">}</span>;</a>
<a class="sourceLine" id="cb3-14" title="14">    </a>
<a class="sourceLine" id="cb3-15" title="15">    <span class="kw">let</span> <span class="kw">mut</span> opts = <span class="pp">seed::RequestOpts::</span>new();</a>
<a class="sourceLine" id="cb3-16" title="16">    opts.headers.insert(<span class="st">&quot;Content-Type&quot;</span>.into(), <span class="st">&quot;application/json&quot;</span>.into());</a>
<a class="sourceLine" id="cb3-17" title="17">    </a>
<a class="sourceLine" id="cb3-18" title="18">    <span class="co">// We can handle the server&#39;s response in the callback, as in the Get example.</span></a>
<a class="sourceLine" id="cb3-19" title="19">    <span class="kw">let</span> callback = <span class="kw">move</span> |json: JsValue| <span class="op">{}</span>;</a>
<a class="sourceLine" id="cb3-20" title="20">    <span class="pp">seed::</span>post(url, message, <span class="cn">Some</span>(opts), <span class="dt">Box</span>::new(callback));</a>
<a class="sourceLine" id="cb3-21" title="21"><span class="op">}</span></a></code></pre></div>
<p>Note how we pass the struct we wish to serialize (the payload) as the second parameter to <code>post</code>; serialization happens out of sight. If a payload is included in <code>RequestOpts</code>, it's replaced by this. Alternatively, we could use <code>fetch</code>, and pass an arbitrary payload <code>String</code> in <code>opts</code>. Here's an example, also demonstrating use of the <code>hashmap_string!</code> macro for brevity: a HashMap literal, which converts both key and value to Strings (eg we avoid repetitive <code>insert</code>, and <code>into()</code> as in above):</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb4-1" title="1"><span class="kw">use</span> serde_json;</a>
<a class="sourceLine" id="cb4-2" title="2"></a>
<a class="sourceLine" id="cb4-3" title="3"><span class="co">// ...</span></a>
<a class="sourceLine" id="cb4-4" title="4"></a>
<a class="sourceLine" id="cb4-5" title="5"><span class="kw">fn</span> post_data() <span class="op">{</span></a>
<a class="sourceLine" id="cb4-6" title="6">    <span class="kw">let</span> message = Message <span class="op">{</span></a>
<a class="sourceLine" id="cb4-7" title="7">        name: <span class="st">&quot;Mark Watney&quot;</span>.into(),</a>
<a class="sourceLine" id="cb4-8" title="8">        email: <span class="st">&quot;mark@crypt.kk&quot;</span>.into(),</a>
<a class="sourceLine" id="cb4-9" title="9">        message: <span class="st">&quot;I wanna be like Iron Man&quot;</span>.into(),</a>
<a class="sourceLine" id="cb4-10" title="10">    <span class="op">}</span>;</a>
<a class="sourceLine" id="cb4-11" title="11">    </a>
<a class="sourceLine" id="cb4-12" title="12">    <span class="kw">let</span> <span class="kw">mut</span> opts = <span class="pp">seed::RequestOpts::</span>new();</a>
<a class="sourceLine" id="cb4-13" title="13">    opts.headers = <span class="pp">hashmap_string!</span><span class="op">{</span></a>
<a class="sourceLine" id="cb4-14" title="14">        <span class="st">&quot;Content-Type&quot;</span> =&gt; <span class="st">&quot;application/json&quot;</span>,</a>
<a class="sourceLine" id="cb4-15" title="15">    <span class="op">}</span>;</a>
<a class="sourceLine" id="cb4-16" title="16">    opts.payload = <span class="cn">Some</span>(<span class="pp">serde_json::</span>to_string(&amp;message).unwrap());</a>
<a class="sourceLine" id="cb4-17" title="17">    </a>
<a class="sourceLine" id="cb4-18" title="18">    <span class="kw">let</span> callback = <span class="kw">move</span> |json: JsValue| <span class="op">{}</span>;</a>
<a class="sourceLine" id="cb4-19" title="19">    <span class="pp">seed::</span>fetch(<span class="pp">seed::Method::</span>Post, url, <span class="cn">Some</span>(opts), <span class="dt">Box</span>::new(callback));</a>
<a class="sourceLine" id="cb4-20" title="20"><span class="op">}</span></a></code></pre></div>
<p>See the <a href="https://github.com/David-OConnor/seed/tree/master/examples/server_interaction">server_interaction example</a> for a full example.</p>
"#.into()
}