## Building a release version
The build commands in the Building and Running section are intended
for development: They produce large `.wasm` file sizes, and unoptimized performance.
To build a release version, append `--release` to the `cargo build` command,
and point your `wasm-bindgen` command to the `release` subdirectory vice `debug`.
Example:
```
cargo build --target wasm32-unknown-unknown --release
```
and 
```
wasm-bindgen target/wasm32-unknown-unknown/release/appname.wasm --no modules --out-dir ./pkg --out-name package
```

## Debugging
There are two categories of error message you may receive: Compiler errors, and panics.
I'm using a different definition than the one in [this section of the Rust book](https://doc.rust-lang.org/book/ch09-00-error-handling.html).


1: Compiler errors are occur while building, and will be displayed in the terminal 
after running `cargo build`, or calling build script. Rust's compiler usually provides
helpful messages, so try to work through these using the information available. It attempts to highlight
which part of your code triggered the error, and suggest fixes. Examples include
syntax errors, using the wrong type of parameter, calling something that doesn't exist, and running afoul of the 
borrow checker.

2: Runtime [panics](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html)
show up as console errors in the web browser. Example:
`panicked at 'assertion failed`. They usually provide a traceback. (For example, a problem while using `unwrap`). 
 They're often associated with`unwrap` or `expect` calls. When applicable, try to use normal
  pattern matching, or `expect` with a useful
 error message instead of `unwrap`: Your message will show in the console, helping identify where
 the panic triggered.


### Debugging elements
`El` implements the `Debug` trait, so you can view them using `log!`: `log!(format!("{:?}", my_el));`
In order to take advantage of this, you must implement `Debug` for your message type, and 
any sub-types. Example:

```rust
#[derive(Copy, Clone, Debug)]
enum Page {
    Guide,
    Changelog
}

#[derive(Clone, Debug)]
enum Msg {
    RoutePage(Page),
    ChangePage(Page),
}
```
