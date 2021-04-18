# Intro

## Webassembly Facts

* Simple machine model (VM)
* For browser and outside browser
* WAT (WebAssemblyText) -> WASM files (like ELF)
* Linear memory model (flat array of bytes)

## Why Rust?

* Memory safety
* Powerful language
  * Macros
  * Traits
  * Enums
* Fast (zero-cost abstractions, no runtime)

## Rust and WebAssembly

* [Rust and WASM intro](https://rustwasm.github.io/docs/book/introduction.html)
* [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/introduction.html)
* [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/)
* 

## Storyboard

* Empty folder
* `cargo generate --git https://github.com/rustwasm/wasm-pack-template`
  * Discuss generated *Cargo.toml*:
    * `crate-type`
      * Make it a C compatible dynamic library (`cdylib`)
      * Specify `rlib` to make unit testing with `wasm-pack test` possible.
    * `features`
      * Enable/disable debugging and optimization features
      * *default* features in Cargo.toml
      * Can also be changed at compile time; e.g.: `wasm-pack build -- --features wee_alloc`
  * Discuss generated *lib.rs*:
    * Prelude: Useful things imported in a single line
    * `wasm_bindgen`: Make function accessible both in JS and Rust
    * `extern` imports JavaScript function into Rust
    * Note: *extern* blocks are considered unsafe by *rust-analyzer*. Disable warning by adding the following code to *.vscode/settings.json*:

      ```json
      {
          "rust-analyzer.diagnostics.disabled": [ "missing-unsafe" ]
      }
      ```

    * If *wee_alloc* feature is enabled, switch the [global allocator](https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/global-allocators.html)
  * Discuss generated *utils.rx*:
    * If *console_error_panic_hook* feature is enabled, [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook) will care for better error messages for panics in console. Useful during debugging.
    * Add `#[allow(dead_code)]` to get rid of warning.
    * If feature is disabled, `set_panic_hook` will be an empty function and will be optimized away.
  * Out of scope for today: Testing
* Build:
  * `wasm-pack build`
  * `wasm-pack build --release`
  * Tip: [Optimize for code size](https://rustwasm.github.io/docs/book/reference/code-size.html#optimizing-builds-for-code-size)
  * By default: Suitable for bundler like webpack ([options](https://rustwasm.github.io/docs/wasm-pack/commands/build.html#target))
  * Discuss generated npm package in *pkg*
  * Publish - if you want - like a reguar npm package
* `npm init wasm-app www`
  * Discuss generated `index.html`, `bootstrap.js`, and `index.js`
  * `npm install`
  * Add dependency in *package.json*: `"fireworks": "file:../pkg"`
  * `npm run build`
  * `npm start`
  * [http://localhost:8080](http://localhost:8080)
* 