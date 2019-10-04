# Percy Empty Isomorphic Template
This is a stripped down version of the [official isomorphic percy example](https://github.com/chinedufn/percy/tree/master/examples/isomorphic).
Linux bash scripts were converted to `cargo-make` shell commands so that they are [runnable across platforms](https://sagiegurari.github.io/cargo-make/#usage-task-command-script-task-exampleshell2batch).
Furthermore a `prerender` script has been added so that client only applications can be prerendered with data for faster load times and search indexability and the `server`-crate becomes optional.

# Getting Started
```bash
cargo install wasm-pack cargo-make watchexec
cargo make dev
```

Why do I need these dependencies?
- `wasm-pack` is required to build the web client
- `cargo-make` lets you use the [scripts](#Scripts)

# Scripts
Scripts are invoked via `cargo make <scriptName>` or `makers <scriptName>`
|   |   |
|---|---|
| `dev` | Automatically rebuilds client and starts the server on any change to the sources |
| `build` | Builds the release version of the client and the server |
| `release` | Run the release version of the server after |
| `prerender` | Prerenders all routes and build the production version of the client. Result from `prerender` can be hosted through any static file host. |

# Why?
Coming from the React / Webpack ecosystem I was looking for something similar to the [isomorphic ssr setups](https://github.com/preactjs/preact-cli) available and percy provides exactly that :)
However the official example was lacking auto restart functionality offered by the `dev` script.
Cargo doesn't come with something like `npm run <script>` or `nodemon` but `cargo-make` provides both that.

# Pitfalls
- `dev` doesn't automatically reload the browser window with the client like `webpack-dev-server` would

# Credits
Thanks go out to: [percy devs](https://github.com/chinedufn/percy/graphs/contributors), [cargo-make devs](https://github.com/sagiegurari/cargo-make/graphs/contributors), [watchexec devs](https://github.com/watchexec/watchexec/graphs/contributors) as well as all other devs of the libraries in the import tree and obviously the guys over at [Rust](https://www.rust-lang.org/governance) and [wasm-pack](https://github.com/rustwasm/wasm-pack/graphs/contributors)!

# License
MIT License
@percy devs should you deem this setup a good idea then feel free to merge this into your example folder or link to it :)