pub fn text() -> String {
r#####"
<h2 id="building-a-release-version">Building a release version</h2>
<p>The build commands in the Building and Running section are intended for development: They produce large <code>.wasm</code> file sizes, and unoptimized performance. To build a release version, run <code>cargo make build_release</code> instead of <code>cargo make build</code>.</p>
<h2 id="debugging">Debugging</h2>
<p>There are two categories of error message you may receive: Compiler errors, and panics. I’m using a different definition than the one in <a href="https://doc.rust-lang.org/book/ch09-00-error-handling.html">this section of the Rust book</a>.</p>
<p>1: Compiler errors are occur while building, and will be displayed in the terminal after running <code>cargo build</code>, or calling build script. Rust’s compiler usually provides helpful messages, so try to work through these using the information available. It attempts to highlight which part of your code triggered the error, and suggest fixes. Examples include syntax errors, using the wrong type of parameter, calling something that doesn’t exist, and running afoul of the borrow checker.</p>
<p>2: Runtime <a href="https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html">panics</a> show up as console errors in the web browser. Example: <code>panicked at 'assertion failed</code>. They usually provide a traceback. (For example, a problem while using <code>unwrap</code>). They’re often associated with<code>unwrap</code> or <code>expect</code> calls. When applicable, try to use normal pattern matching, or <code>expect</code> with a useful error message instead of <code>unwrap</code>: Your message will show in the console, helping identify where the panic triggered.</p>
<h3 id="logging">Logging</h3>
<p>You may log things to the browser console using the following functions: <code>seed::log</code>, and <code>seed::error</code>. They accept a single argument which implements <code>Debug</code>. Each has a corresponding macro: <code>log!</code>, and <code>error!</code>, which work in a similar way, but accept multiple arguments, which will be displayed separated by spaces. If you’d like to log something which implements <code>ToString</code>, but not <code>Debug</code>, call <code>to_string()</code> on it when using it in the function or macro.</p>
<h3 id="debugging-elements">Debugging elements</h3>
<p><code>El</code> and <code>Node</code> implement the <code>Debug</code> trait, so you can view them using <code>log!</code>: <code>log!(format!("{:?}", my_el));</code> In order to take advantage of this, you must implement <code>Debug</code> for your message type, and any sub-types. Example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><span id="cb1-1"><a href="#cb1-1"></a><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Copy</span><span class="at">,</span> <span class="bu">Clone</span><span class="at">,</span> <span class="bu">Debug</span><span class="at">)]</span></span>
<span id="cb1-2"><a href="#cb1-2"></a><span class="kw">enum</span> Page <span class="op">{</span></span>
<span id="cb1-3"><a href="#cb1-3"></a>    Guide,</span>
<span id="cb1-4"><a href="#cb1-4"></a>    Changelog</span>
<span id="cb1-5"><a href="#cb1-5"></a><span class="op">}</span></span>
<span id="cb1-6"><a href="#cb1-6"></a></span>
<span id="cb1-7"><a href="#cb1-7"></a><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">,</span> <span class="bu">Debug</span><span class="at">)]</span></span>
<span id="cb1-8"><a href="#cb1-8"></a><span class="kw">enum</span> Msg <span class="op">{</span></span>
<span id="cb1-9"><a href="#cb1-9"></a>    RoutePage(Page),</span>
<span id="cb1-10"><a href="#cb1-10"></a>    ChangePage(Page),</span>
<span id="cb1-11"><a href="#cb1-11"></a><span class="op">}</span></span></code></pre></div>
<h2 id="tests">Tests</h2>
<p>To run tests, you may either use <code>wasm-pack</code> test commands, or simplified ones from the <code>Makefile.toml</code> included in the quickstart repo.</p>
<p>To run all tests with the Makefile: <code>cargo make test_h firefox</code> Where <code>firefox</code> may be replaced with <code>chrome</code> or <code>safari</code>.</p>
<p>To run a single test: <code>cargo make test_h test_name</code>. with <code>test_name</code> replaced by the name of the test. It uses Firefox.</p>
<p>With <code>wasm-pack</code> directly, or to run individual tests, use commands similar to this:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode bash"><code class="sourceCode bash"><span id="cb2-1"><a href="#cb2-1"></a><span class="ex">wasm-pack</span> test --firefox --headless -- --lib test_name</span></code></pre></div>
"#####.into()
}