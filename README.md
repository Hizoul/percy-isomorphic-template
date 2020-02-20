# Percy Empty Isomorphic Template
This repository contains a stripped-down version of the [official isomorphic percy example](https://github.com/chinedufn/percy/tree/master/examples/isomorphic).
Linux bash scripts were converted to `cargo-make` shell commands so that they are [runnable across platforms](https://sagiegurari.github.io/cargo-make/#usage-task-command-script-task-exampleshell2batch).
Furthermore, a `prerender` script has been added to enable client-only applications. Thus everything can be prerendered with data for faster load times and search indexability and the `server`-crate becomes optional.
Also, a [TODO](https://github.com/chinedufn/percy/blob/e41857e0d002ade865a4c0b9f1c052f75dc06d0b/examples/isomorphic/client/src/lib.rs#L49) item in the example for pure Rust rerendering has been fixed, which reduces the amount of JS code in the `index.html`.

# Getting Started
```bash
cargo install wasm-pack cargo-make watchexec
cargo make dev
```

Why do I need these dependencies?
- `wasm-pack` is required to build the web client
- `cargo-make` lets you use the [scripts](#Scripts)
- `watchexec` allows the `dev` script to automatically restart

# Scripts

Scripts are invoked via `cargo make <scriptName>` or `makers <scriptName>`

| Command  | Description |
|---|---|
| `dev` | Automatically rebuild client and start the server on any change to the sources |
| `build` | Builds the release version of the client and the server |
| `release` | Run the release version of the server and builds release client |
| `prerender` | Prerenders all routes and build the production version of the client. The result from `prerender` can be hosted through any static file host (e.g. github pages). |

# Prerendering
Prerendering doesn't happen automagically like in `preact-cli`, `gatsby` or `react-static`.
Make sure to end your percy routes in the `app` in .html.
Modify `prerender/src/main.rs` so that necessary routes are rendered.

# Why?
Coming from the React / Webpack ecosystem, I was looking for something similar to the [isomorphic SSR setups](https://github.com/preactjs/preact-cli) available and percy provides exactly that :)
However, the official example was lacking an auto-restart on changes.
Cargo doesn't come with something like `npm run <script>` or `nodemon` but `cargo-make` and `watchexec` fix that.

# Pitfalls
- `dev` doesn't automatically reload the browser window with the client as `webpack-dev-server` would

# Credits
Thanks go out to [percy devs](https://github.com/chinedufn/percy/graphs/contributors), [cargo-make devs](https://github.com/sagiegurari/cargo-make/graphs/contributors), [watchexec devs](https://github.com/watchexec/watchexec/graphs/contributors) as well as all other devs of the libraries in the import tree and of course the guys over at [Rust](https://www.rust-lang.org/governance) and [wasm-pack](https://github.com/rustwasm/wasm-pack/graphs/contributors)!

# License
MIT License
@percy devs should you deem this set up a good idea then feel free to merge this into your example folder or link to it :)