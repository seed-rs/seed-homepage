## Building a release version
The configuration in the Building and Running section towards the top are intended
for development: They produce large `.wasm` file sizes, and unoptimized performance.
For your release version, you'll need to append `--release` to the `cargo build` command,
and point your `wasm-bindgen` command to the `release` subdirectory vice `debug`.
Example:

```
cargo build --target wasm32-unknown-unknown --release
```
and 
```
wasm-bindgen target/wasm32-unknown-unknown/release/appname.wasm --no modules --out-dir ./pkg
```

## Debugging
There are two categories of error message you can receive: I'm using a different definition than
 used in [this section of the Rust book](https://doc.rust-lang.org/book/ch09-00-error-handling.html).
Compiler errors, and panics. 

1: Errors while building, which will be displayed in the terminal 
where you ran `cargo build`, or the build script. Rust's compiler usually provides
helpful messages, so try to work through these using the information available. Examples include
syntax errors, passing a func/struct etc the wrong type of item, and running afoul of the 
borrow checker.

2: Runtime [panics](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html). 
These show up as console errors in the web browser. Example:
`panicked at 'assertion failed`, and provide a traceback. (For example, a problem while using `unwrap()`). 
 They're often associated with`unwrap()` or `expect()` calls. Try to use expect(), with a useful
 error message instead of unwrap(): It your message will show in the console.