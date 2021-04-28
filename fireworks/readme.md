# Intro

## Webassembly Facts

* Simple machine model (VM)
* For browser and outside browser
* WAT (WebAssemblyText) -> WASM files (like ELF)
* Linear memory model (flat array of bytes)

## Why Rust in the Browser?

* Memory safety
* Powerful language
  * Macros
  * Traits
  * Enums
* Fast (zero-cost abstractions, no runtime)
* Reuse code on server and client
* Share code between platforms (e.g. desktop, mobile)

## Rust and WebAssembly

* [Rust and WASM intro](https://rustwasm.github.io/docs/book/introduction.html)
* [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/introduction.html)
* [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/)

## Demo

https://cddataexchange.blob.core.windows.net/data-exchange/fireworks/index.html

## Storyboard

### Generate Rust WASM Library

* Empty folder (*live/fireworks-sample*)
* `cargo generate --git https://github.com/rustwasm/wasm-pack-template`
  * Remove files we do not need:
    * `rm .appveyor.yml .travis.yml LICENSE_APACHE LICENSE_MIT README.md`
    * `rm -rf tests`
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
    * Add `#[allow(dead_code)]` to get rid of warning (snippet *01-allow-dead-code*)
    * If feature is disabled, `set_panic_hook` will be an empty function and will be optimized away.
  * Out of scope for today: Testing
* Build:
  * `wasm-pack build`
  * `wasm-pack build --release`
  * Tip: [Optimize for code size](https://rustwasm.github.io/docs/book/reference/code-size.html#optimizing-builds-for-code-size)
  * By default: Suitable for bundler like webpack ([options](https://rustwasm.github.io/docs/wasm-pack/commands/build.html#target))
  * Discuss generated npm package in *pkg*
  * Publish - if you want - like a reguar npm package
* Add dependencies (*0015-dependencies*)
* Add usings (*0017-usings*)
* Add constants (*0020-constants*)
* Add a struct (*0030-rocket-struct*)
* Add an enum (*0040-rocket-state*)
  * Discuss the fact that enums can implement traits, contain functions, etc.
  * Discuss `Default` trait
* Add custom type (*0050-vec-rocket-states*)
  * Discuss traits
  * Discuss how we can ad method to `Vec`
* Add another enum (*0060-particle-state*)
  * Discuss `derive` macro
* Add 2D vector (*0070-vector-2d*)
  * Discuss traits in combination with operators
* Add some additional parameter structs (*0080-some-parameters*)
* Add main struct `Firework` (*0090-firework-struct*)
* Add "constructor" (*0100-constructor*)
  * Discuss `wasm_bindgen` for structs (view *.d.ts* file)
  * Note default values based on `Default` trait
* Add method for adding rockets (*0110-add-rocket-low-level*)
  * Show generated JSDoc in *.d.ts* (*wasm-pack build*)
* Add method for adding rocket using JS object (*0120-add-rocket-json*)
  * Discuss `JsValue`

### Generate JS Client

* `npm init wasm-app www`
  * Remove files we do not need:
    * `rm .travis.yml LICENSE-APACHE LICENSE-MIT README.md`
  * Discuss generated `index.html`, `bootstrap.js`, and `index.js`
  * Note: [Sample with TypeScript and Webpack 5](fireworks/www)
  * `npm install`
  * Add dependency in *package.json*: `"fireworks": "file:../pkg"`
  * `npm run build`
  * `npm start`
  * [http://localhost:8080](http://localhost:8080)
* Create firework (*0130-js-fireworks-new*)
* Add rockets (*0140-js-p5-draw*)
  * Discuss different functions for adding rockets

### Exchanging data

* Add functions for getting pointers in WASM linear memory (*0150-get-buffer-ptrs*)
* Get pointers in JS (*0160-js-access-buffers*)
* Implement logic for firework in Rust
  * *0170-apply-force*
  * *0180-explosions* (outside of `wasm_bindgen`)
* Draw in JS based on memory from WASM (*0190-js-draw-impl*)
