pub fn text() -> String {
r#####"
<h1 id="http-requests-fetch">HTTP Requests (fetch)</h1>
<p>We use the <a href="https://docs.rs/seed/0.4.2/seed/fetch/struct.Request.html">seed::Request</a> struct to make HTTP requests in the browser, wrapping the <a href="https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API">Fetch API</a>. To use this, we need to include <code>futures = &quot;^0.1.26&quot;</code> in <code>Cargo.toml</code>. The <a href="https://docs.rs/seed/0.4.2/seed/fetch/index.html">Fetch module</a> is standalone: It can be used with any wasm-bindgen program.</p>
<h2 id="receiving-data">Receiving data</h2>
<p>Example, where we update the state on initial load (similar to the <a href="https://github.com/David-OConnor/seed/tree/master/examples/server_interaction">server_interaction example</a> ) from a server. It demonstrates a <code>GET</code> request, and deserializing JSON data. The <code>server_interaction</code> example contains more sample code.</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb1-1" title="1"><span class="kw">use</span> <span class="pp">futures::</span>Future;</a>
<a class="sourceLine" id="cb1-2" title="2"><span class="kw">use</span> <span class="pp">serde::</span><span class="op">{</span>Serialize, Deserialize<span class="op">}</span>;</a>
<a class="sourceLine" id="cb1-3" title="3"></a>
<a class="sourceLine" id="cb1-4" title="4"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">,</span> Serialize<span class="at">,</span> Deserialize<span class="at">)]</span></a>
<a class="sourceLine" id="cb1-5" title="5"><span class="kw">pub</span> <span class="kw">struct</span> Commit <span class="op">{</span></a>
<a class="sourceLine" id="cb1-6" title="6">    <span class="kw">pub</span> sha: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb1-7" title="7"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-8" title="8"></a>
<a class="sourceLine" id="cb1-9" title="9"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">,</span> Serialize<span class="at">,</span> Deserialize<span class="at">)]</span></a>
<a class="sourceLine" id="cb1-10" title="10"><span class="kw">pub</span> <span class="kw">struct</span> Branch <span class="op">{</span></a>
<a class="sourceLine" id="cb1-11" title="11">    <span class="kw">pub</span> name: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb1-12" title="12">    <span class="kw">pub</span> commit: Commit,</a>
<a class="sourceLine" id="cb1-13" title="13"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-14" title="14"></a>
<a class="sourceLine" id="cb1-15" title="15"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb1-16" title="16"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb1-17" title="17">    DataFetched(<span class="pp">seed::fetch::</span>ResponseDataResult&lt;Branch&gt;),</a>
<a class="sourceLine" id="cb1-18" title="18"></a>
<a class="sourceLine" id="cb1-19" title="19"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-20" title="20"></a>
<a class="sourceLine" id="cb1-21" title="21"><span class="kw">fn</span> fetch_data() -&gt; <span class="kw">impl</span> Future&lt;Item = Msg, Error = Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb1-22" title="22">    <span class="kw">let</span> url = <span class="st">&quot;https://api.github.com/repos/david-oconnor/seed/branches/master&quot;</span>;</a>
<a class="sourceLine" id="cb1-23" title="23">    <span class="pp">Request::</span>new(url.into()).fetch_json_data(<span class="pp">Msg::</span>DataFetched)</a>
<a class="sourceLine" id="cb1-24" title="24"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-25" title="25"></a>
<a class="sourceLine" id="cb1-26" title="26"><span class="kw">fn</span> update(msg: Msg, model: &amp;<span class="kw">mut</span> Model, orders: &amp;<span class="kw">mut</span> <span class="kw">impl</span> Orders&lt;Msg&gt;) <span class="op">{</span></a>
<a class="sourceLine" id="cb1-27" title="27">    <span class="kw">match</span> msg <span class="op">{</span></a>
<a class="sourceLine" id="cb1-28" title="28">        <span class="pp">Msg::</span>DataFetched(<span class="cn">Ok</span>(branch)) =&gt; model.branch = branch,</a>
<a class="sourceLine" id="cb1-29" title="29"></a>
<a class="sourceLine" id="cb1-30" title="30">        <span class="pp">Msg::</span>DataFetched(<span class="cn">Err</span>(fail_reason)) =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb1-31" title="31">            <span class="pp">error!</span>(<span class="pp">format!</span>(</a>
<a class="sourceLine" id="cb1-32" title="32">                <span class="st">&quot;Fetch error - Fetching repository info failed - {:#?}&quot;</span>,</a>
<a class="sourceLine" id="cb1-33" title="33">                fail_reason</a>
<a class="sourceLine" id="cb1-34" title="34">            ));</a>
<a class="sourceLine" id="cb1-35" title="35">            orders.skip();</a>
<a class="sourceLine" id="cb1-36" title="36">        <span class="op">}</span></a>
<a class="sourceLine" id="cb1-37" title="37">    <span class="op">}</span></a>
<a class="sourceLine" id="cb1-38" title="38"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-39" title="39"></a>
<a class="sourceLine" id="cb1-40" title="40"><span class="kw">fn</span> view(model: &amp;Model) -&gt; Node&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb1-41" title="41">    <span class="pp">div!</span><span class="op">[</span><span class="pp">format!</span>(</a>
<a class="sourceLine" id="cb1-42" title="42">        <span class="st">&quot;Repo info: Name: {}, SHA: {}&quot;</span>,</a>
<a class="sourceLine" id="cb1-43" title="43">        model.branch.name, model.branch.commit.sha</a>
<a class="sourceLine" id="cb1-44" title="44">    )<span class="op">]</span></a>
<a class="sourceLine" id="cb1-45" title="45"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-46" title="46"></a>
<a class="sourceLine" id="cb1-47" title="47"><span class="kw">fn</span> init(_: Url, orders: &amp;<span class="kw">mut</span> <span class="kw">impl</span> Orders&lt;Msg&gt;) -&gt; Init&lt;Model&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb1-48" title="48">    orders.perform_cmd(fetch_data());</a>
<a class="sourceLine" id="cb1-49" title="49">    <span class="pp">Init::</span>new(<span class="pp">Model::</span><span class="kw">default</span>())</a>
<a class="sourceLine" id="cb1-50" title="50"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-51" title="51"></a>
<a class="sourceLine" id="cb1-52" title="52"><span class="at">#[</span>wasm_bindgen<span class="at">]</span></a>
<a class="sourceLine" id="cb1-53" title="53"><span class="kw">pub</span> <span class="kw">fn</span> render() <span class="op">{</span></a>
<a class="sourceLine" id="cb1-54" title="54">    <span class="kw">let</span> app = <span class="pp">seed::App::</span>build(init, update, view)</a>
<a class="sourceLine" id="cb1-55" title="55">        .finish()</a>
<a class="sourceLine" id="cb1-56" title="56">        .run();</a>
<a class="sourceLine" id="cb1-57" title="57"></a>
<a class="sourceLine" id="cb1-58" title="58">    app.update(<span class="pp">Msg::</span>FetchData);</a>
<a class="sourceLine" id="cb1-59" title="59"><span class="op">}</span></a></code></pre></div>
<p>On page load, we trigger an update in the <code>init</code> function using <code>Msg::FetchData</code>, which points the <code>update</code> via <code>orders.perform_cmd</code> and a function we’ve created called <code>fetch_data</code>. This allows state to be update asynchronosly, when the request is complete. <code>skip()</code> is a convenience method that sets <code>Update::ShouldRender</code> to <code>Skip</code>; sending the request doesn’t trigger a render. We pattern-match the response in the <code>update</code> function’s<code>DataFetched</code> arm: If successful, we update the model. If not, we display an error in the console using the <code>error!</code> macro.</p>
<p>We’ve set up nested structs that have fields matching the names of the JSON fields of the response, which <code>Serde</code> deserializes the response into, through the <code>fetch_json</code> method of <code>Request</code>. Note that even though more data than what’s contained in our Branch struct is included in the response, Serde automatically applies only the info matching our struct’s fields.</p>
<p>If we wish to trigger this update from a normal event instead of on load, we can do something like this:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb2-1" title="1"><span class="kw">fn</span> view(model: &amp;Model) -&gt; <span class="dt">Vec</span>&lt;Node&lt;Msg&gt;&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb2-2" title="2">    <span class="pp">vec!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb2-3" title="3">        <span class="pp">div!</span><span class="op">[</span><span class="pp">format!</span>(</a>
<a class="sourceLine" id="cb2-4" title="4">            <span class="st">&quot;Repo info: Name: {}, SHA: {}&quot;</span>,</a>
<a class="sourceLine" id="cb2-5" title="5">            model.branch.name, model.branch.commit.sha</a>
<a class="sourceLine" id="cb2-6" title="6">        )<span class="op">]</span>,</a>
<a class="sourceLine" id="cb2-7" title="7">        <span class="pp">button!</span><span class="op">[</span> raw_ev(<span class="pp">Ev::</span>Click, <span class="kw">move</span> |_| <span class="pp">Msg::</span>FetchData), <span class="st">&quot;Update&quot;</span><span class="op">]</span></a>
<a class="sourceLine" id="cb2-8" title="8">    <span class="op">]</span></a>
<a class="sourceLine" id="cb2-9" title="9"><span class="op">}</span></a></code></pre></div>
<h2 id="sending-data">Sending data</h2>
<p>Example showing a POST request where we send data to a server and receive the response, and a header:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb3-1" title="1"><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="at">)]</span></a>
<a class="sourceLine" id="cb3-2" title="2"><span class="kw">struct</span> RequestBody <span class="op">{</span></a>
<a class="sourceLine" id="cb3-3" title="3">    <span class="kw">pub</span> name: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb3-4" title="4">    <span class="kw">pub</span> email: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb3-5" title="5">    <span class="kw">pub</span> message: <span class="dt">String</span>,</a>
<a class="sourceLine" id="cb3-6" title="6"><span class="op">}</span></a>
<a class="sourceLine" id="cb3-7" title="7"></a>
<a class="sourceLine" id="cb3-8" title="8"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Debug</span><span class="at">,</span> <span class="bu">Clone</span><span class="at">,</span> Deserialize<span class="at">)]</span></a>
<a class="sourceLine" id="cb3-9" title="9"><span class="kw">struct</span> ResponseBody <span class="op">{</span></a>
<a class="sourceLine" id="cb3-10" title="10">    <span class="kw">pub</span> success: <span class="dt">bool</span>,</a>
<a class="sourceLine" id="cb3-11" title="11"><span class="op">}</span></a>
<a class="sourceLine" id="cb3-12" title="12"></a>
<a class="sourceLine" id="cb3-13" title="13"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb3-14" title="14"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb3-15" title="15">    SendMessage,</a>
<a class="sourceLine" id="cb3-16" title="16">    MessageSent(<span class="pp">seed::fetch::</span>ResponseDataResult&lt;ResponseBody&gt;),</a>
<a class="sourceLine" id="cb3-17" title="17"><span class="op">}</span></a>
<a class="sourceLine" id="cb3-18" title="18"></a>
<a class="sourceLine" id="cb3-19" title="19"><span class="kw">fn</span> update(msg: Msg, model: &amp;<span class="kw">mut</span> Model, orders: &amp;<span class="kw">mut</span> <span class="kw">impl</span> Orders&lt;Msg&gt;) <span class="op">{</span></a>
<a class="sourceLine" id="cb3-20" title="20">    <span class="kw">match</span> msg <span class="op">{</span></a>
<a class="sourceLine" id="cb3-21" title="21">        <span class="pp">Msg::</span>SendMessage =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb3-22" title="22">            orders.skip().perform_cmd(send_message());</a>
<a class="sourceLine" id="cb3-23" title="23">        <span class="op">}</span></a>
<a class="sourceLine" id="cb3-24" title="24"></a>
<a class="sourceLine" id="cb3-25" title="25">        <span class="pp">Msg::</span>MessageSent(<span class="cn">Ok</span>(response_data)) =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb3-26" title="26">            <span class="pp">log!</span>(<span class="pp">format!</span>(<span class="st">&quot;Response data: {:#?}&quot;</span>, response_data));</a>
<a class="sourceLine" id="cb3-27" title="27">            orders.skip();</a>
<a class="sourceLine" id="cb3-28" title="28">        <span class="op">}</span></a>
<a class="sourceLine" id="cb3-29" title="29"></a>
<a class="sourceLine" id="cb3-30" title="30">        <span class="pp">Msg::</span>MessageSent(<span class="cn">Err</span>(fail_reason)) =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb3-31" title="31">            <span class="pp">error!</span>(<span class="pp">format!</span>(</a>
<a class="sourceLine" id="cb3-32" title="32">                <span class="st">&quot;Fetch error - Sending message failed - {:#?}&quot;</span>,</a>
<a class="sourceLine" id="cb3-33" title="33">                fail_reason</a>
<a class="sourceLine" id="cb3-34" title="34">            ));</a>
<a class="sourceLine" id="cb3-35" title="35">            orders.skip();</a>
<a class="sourceLine" id="cb3-36" title="36">        <span class="op">}</span></a>
<a class="sourceLine" id="cb3-37" title="37">    <span class="op">}</span></a>
<a class="sourceLine" id="cb3-38" title="38"><span class="op">}</span></a>
<a class="sourceLine" id="cb3-39" title="39"></a>
<a class="sourceLine" id="cb3-40" title="40"><span class="kw">fn</span> send_message() -&gt; <span class="kw">impl</span> Future&lt;Item = Msg, Error = Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb3-41" title="41">    <span class="kw">let</span> message = RequestBody <span class="op">{</span></a>
<a class="sourceLine" id="cb3-42" title="42">        name: <span class="st">&quot;Mark Watney&quot;</span>.into(),</a>
<a class="sourceLine" id="cb3-43" title="43">        email: <span class="st">&quot;mark@crypt.kk&quot;</span>.into(),</a>
<a class="sourceLine" id="cb3-44" title="44">        message: <span class="st">&quot;I wanna be like Iron Man&quot;</span>.into(),</a>
<a class="sourceLine" id="cb3-45" title="45">    <span class="op">}</span>;</a>
<a class="sourceLine" id="cb3-46" title="46"></a>
<a class="sourceLine" id="cb3-47" title="47">    <span class="pp">Request::</span>new(CONTACT_URL)</a>
<a class="sourceLine" id="cb3-48" title="48">        .method(<span class="pp">Method::</span>Post)</a>
<a class="sourceLine" id="cb3-49" title="49">        .send_json(&amp;message)</a>
<a class="sourceLine" id="cb3-50" title="50">        .fetch_json_data(<span class="pp">Msg::</span>MessageSent)</a>
<a class="sourceLine" id="cb3-51" title="51"><span class="op">}</span></a>
<a class="sourceLine" id="cb3-52" title="52"></a>
<a class="sourceLine" id="cb3-53" title="53"><span class="kw">fn</span> view(model: &amp;Model) -&gt; Node&lt;Msg&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb3-54" title="54">    <span class="pp">button!</span><span class="op">[</span></a>
<a class="sourceLine" id="cb3-55" title="55">        simple_ev(<span class="pp">Ev::</span>Click, <span class="pp">Msg::</span>SendMessage),</a>
<a class="sourceLine" id="cb3-56" title="56">        <span class="st">&quot;Send an urgent message (see console log)&quot;</span></a>
<a class="sourceLine" id="cb3-57" title="57">    <span class="op">]</span></a>
<a class="sourceLine" id="cb3-58" title="58"><span class="op">}</span></a></code></pre></div>
<p>Reference the <code>Request</code> API docs (linked above) for a full list of methods available to configure the request, and links to the <code>MDN</code> docs describing them. (eg: <code>credentials</code>, <code>mode</code>, <code>integrity</code>)</p>
"#####.into()
}