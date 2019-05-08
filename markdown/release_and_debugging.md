## Building a release version
The build commands in the Building and Running section are intended
for development: They produce large `.wasm` file sizes, and unoptimized performance.
To build a release version, run `cargo make all_release` instead of `cargo make all`.

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


### Logging
You may log things to the JS console using the following 3 functions: `seed::log`, `seed::error`,
and `seed::debug`. The first two accept a single argument which implements `ToString`, eg `String`,
`&str` etc. The third accepts one that implements `Debug`. Each has a corresponding macro: `log!`,
`error!`, and `debug!`, which work in a similar way, but accept multiple arguments, which will
be displayed separated by spaces.


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


### Tests
To run tests, you may either use `wasm-pack` test commands, or simplified ones from the 
`Makefile.toml` included in the quickstart repo. 

To run all tests with the makefile:
`cargo make test firefox`
Where `firefox` may be replaced with `chrome` or `safari`.

With `wasm-pack` directly, or to run individual tests, use commands similar to this, with 
`module_name` and `test_name` replaced
with the module which contains the test, and the test name repectively.:
```bash
wasm-pack test --firefox --headless -- --module_name test_name
```