pub fn text() -> String {
r#"
<h2 id="building-a-release-version">Building a release version</h2>
<p>The configuration in the Building and Running section towards the top are intended for development: They produce large <code>.wasm</code> file sizes, and unoptimized performance. For your release version, you'll need to append <code>--release</code> to the <code>cargo build</code> command, and point your <code>wasm-bindgen</code> command to the <code>release</code> subdirectory vice <code>debug</code>. Example:</p>
<pre><code>cargo build --target wasm32-unknown-unknown --release</code></pre>
<p>and</p>
<pre><code>wasm-bindgen target/wasm32-unknown-unknown/release/appname.wasm --no modules --out-dir ./pkg</code></pre>
<h2 id="debugging">Debugging</h2>
<p>There are two categories of error message you can receive: I'm using a different definition than used in <a href="https://doc.rust-lang.org/book/ch09-00-error-handling.html">this section of the Rust book</a>. Compiler errors, and panics.</p>
<p>1: Errors while building, which will be displayed in the terminal where you ran <code>cargo build</code>, or the build script. Rust's compiler usually provides helpful messages, so try to work through these using the information available. Examples include syntax errors, passing a func/struct etc the wrong type of item, and running afoul of the borrow checker.</p>
<p>2: Runtime <a href="https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html">panics</a>. These show up as console errors in the web browser. Example: <code>panicked at 'assertion failed</code>, and provide a traceback. (For example, a problem while using <code>unwrap()</code>). They're often associated with<code>unwrap()</code> or <code>expect()</code> calls. Try to use expect(), with a useful error message instead of unwrap(): It your message will show in the console.</p>
"#.into()
}