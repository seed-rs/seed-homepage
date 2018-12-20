pub fn text() -> String {
r#"
<h2 id="building-a-release-version">Building a release version</h2>
<p>The build commands in the Building and Running section are intended for development: They produce large <code>.wasm</code> file sizes, and unoptimized performance. To build a release version, append <code>--release</code> to the <code>cargo build</code> command, and point your <code>wasm-bindgen</code> command to the <code>release</code> subdirectory vice <code>debug</code>. Example:</p>
<pre><code>cargo build --target wasm32-unknown-unknown --release</code></pre>
<p>and</p>
<pre><code>wasm-bindgen target/wasm32-unknown-unknown/release/appname.wasm --no modules --out-dir ./pkg</code></pre>
<h2 id="debugging">Debugging</h2>
<p>There are two categories of error message you may receive: Compiler errors, and panics. I'm using a different definition than the one in <a href="https://doc.rust-lang.org/book/ch09-00-error-handling.html">this section of the Rust book</a>.</p>
<p>1: Compiler errors are occur while building, and will be displayed in the terminal after running <code>cargo build</code>, or calling build script. Rust's compiler usually provides helpful messages, so try to work through these using the information available. It attempts to highlight which part of your code triggered the error, and suggest fixes. Examples include syntax errors, using the wrong type of parameter, calling something that doesn't exist, and running afoul of the borrow checker.</p>
<p>2: Runtime <a href="https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html">panics</a> show up as console errors in the web browser. Example: <code>panicked at 'assertion failed</code>. They usually provide a traceback. (For example, a problem while using <code>unwrap()</code>). They're often associated with<code>unwrap()</code> or <code>expect()</code> calls. When applicable, try to use expect() with a useful error message instead of unwrap(): Your message will show in the console, helping identify where the panic triggered.</p>
"#.into()
}