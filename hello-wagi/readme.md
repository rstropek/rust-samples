# WAGI Intro

## Installation

This project uses the following tools  development:

* [Browsersync](https://browsersync.io/) for auto-reloading a website whenever the source (Wasm, *Handlebar* templates) changes
* [*just*](https://github.com/casey/just) to make it easier to run various development commands
* [*watchexec*](https://github.com/casey/just) to trigger *Cargo* builds whenever source (rs) changes
* [*wasm-to-oci*](https://github.com/engineerd/wasm-to-oci) for pushing Wasm modules to OCI registry
* [*WAGI Fileserver*](https://github.com/deislabs/wagi-fileserver) for serving static files

For that reason, you have to install the tools mentioned above:

* Install Browsersync: `npm install`
* Install [*just*](https://github.com/casey/just): `cargo install just`
* Install [*watchexec*](https://github.com/watchexec/watchexec): `cargo install watchexec-cl`
* [Install *wasm-to-oci*](https://github.com/engineerd/wasm-to-oci#installation)
  * In order to try the OCI-related sample, you will need an [*Azure Container Registry*](https://azure.microsoft.com/en-us/services/container-registry/) (Docker Hub is at the time of writing not supported). Login to ACR using `az azr login`. In this documentation, we assume that your Container Registry is called `rustlinzwasm`. Change it accordingly if neccessary.
* Download [*WAGI Fileserver*](https://github.com/deislabs/wagi-fileserver) into the root folder or the sample: `curl -OL https://github.com/deislabs/wagi-fileserver/releases/download/v0.6.0/fileserver.gr.wasm`

## Development

* Start *Browsersync* proxy in one terminal: `just serve`
* Start watch process building Wasm module from Rust source: `just watch <sample>` (where sample is e.g. *level1*, *level2*)
* Open a browser window at [http://localhost:8080](http://localhost:8080) (append path as needed). For those samples that generate HTML, it will auto-reload whenever you change the Rust sourcecode or the [*Handlebar* templates](templates). For samples that generate plain text, you have to reload manually.
* Note that you can access the *Browsersync* UI at [http://localhost:8081](http://localhost:8081)

## Samples

### [Level 1](level1)

The purpose of this sample is to demonstrate how to write the simplest possible HTTP handler using WAGI. Note that there are not dependencies at all.

* [http://localhost:8080/level1-ping](http://localhost:8080/level1-ping)

### [Level 1](level1) with OCI

* Push the Wasm module from the previous sample to Container Registry: `wasm-to-oci push target/wasm32-wasi/debug/level1.wasm rustlinzwasm.azurecr.io/wagi-level1-oci:latest`
* Reference Wasm module in OCI in *modules.toml*:

  ```toml
  [[module]]
  route = "/ping-oci"
  module = "oci:rustlinzwasm.azurecr.io/wagi-level1-oci:latest"
  ```

Note that this examples assumes that your *Azure Container Registry* allows anonymous pull requests. [Configure ACR](https://docs.microsoft.com/en-us/azure/container-registry/anonymous-pull-access) accordingly.

### [Level 2](level2)

This sample demonstrates how WAGI receives HTTP data (e.g. path, parameters, headers). The code is based on a [sample from the WAGI team](https://github.com/deislabs/env_wagi/blob/main/src/main.rs). Take a look at their code as it contains a lot of interesting comments.

* [http://localhost:8080/level2](http://localhost:8080/level2)
* POST some JSON to [http://localhost:8080/level2](http://localhost:8080/level2):

  ```txt
  POST {{host}}/level2?foo=bar
  Content-Type: application/json

  {
      "foo": "Bar"
  }
  ```

### [Level 3](level3)

This sample uses the [*Handlebars*](https://docs.rs/crate/handlebars/latest) template language to generate HTML. Additionally, it uses the [*WAGI Fileserver*](https://github.com/deislabs/wagi-fileserver) for serving static files.

* [Add subroutes](https://github.com/deislabs/wagi/blob/main/docs/writing_modules.md#advanced-declaring-sub-routes-in-the-module) to WAGI
  * [http://localhost:8080/level3](http://localhost:8080/level3)
* Inline template
  * [http://localhost:8080/level3/inline?foo=bar&answer=42](http://localhost:8080/level3/inline?foo=bar&answer=42)
* Template file
  * Enable file access from WAGI (`templates`)

    ```toml
    [[module]]
    route = "/level3"
    module = "./target/wasm32-wasi/debug/level3.wasm"
    volumes = {"/templates" = "/home/rainer/github/rust-samples/hello-wagi/level3/templates"}
    ```

  * [http://localhost:8080/level3/file](http://localhost:8080/level3/file)
* Static files with [*WAGI Fileserver*](https://github.com/deislabs/wagi-fileserver)
  * Add fileserver to *modules.toml*

    ```toml
    [[module]]
    route = "/static/..."
    module = "./fileserver.gr.wasm"
    volumes = {"/" = "/home/rainer/github/rust-samples/hello-wagi/static"}
    ```

  * [http://localhost:8080/level3/file](http://localhost:8080/level3/file)
  * POST JSON

    ```txt
    POST {{host}}/level3/file?foo=bar
    Content-Type: application/json

    {
        "foo": "Bar"
    }
    ```

### [Level 4](level4)

This sample demonstrates how to make HTTP requests using [*wasi-experimental-http*](https://github.com/deislabs/wasi-experimental-http). It implements a solving hack for the popular [*Wordle* game](https://www.nytimes.com/games/wordle/index.html).

The Wordle "solver" algorithm is based on a TypeScript sample written by @blaumeise20 (https://github.com/blaumeise20/blaumeise20). Thank you for allowing me to use it for this demo.


* Set *allowed_hosts*.

  ```toml
  [[module]]
  route = "/level4"
  module = "./target/wasm32-wasi/debug/level4.wasm"
  allowed_hosts = ["https://www.nytimes.com"]
  http_max_concurrency = 2
  ```

* [http://localhost:8080/static/wordle.html](http://localhost:8080/static/wordle.html)
