# Examples

Run these commands from the repository root:

## Code snippets and standard input

```sh
limo -e 'console.log(1 + 2)'
limo -e 'console.log(await Promise.resolve(3))'
limo --input-type module -e 'import { greet } from "./examples/modules/greet.mjs"; console.log(greet("limo"))'
echo 'console.log(1 + 2)' | limo -
echo 'console.log(1 + 2)' | limo
```

Code snippets and standard input load through a local entry script, just like files. limo first tries to compile the code as a classic script. If it has a syntax error, it tries an ES module, allowing static imports and top-level `await`. Use `--input-type script` or `--input-type module` to select explicitly. Relative URLs for `fetch`, Workers, and imports resolve from the current working directory.

Execution follows moli's page behavior: some uncaught exceptions do not yet produce a nonzero exit code, and a pending top-level `await` alone does not keep the process running.

## Files

```sh
limo examples/dom.js
limo examples/modules/main.mjs
limo --input-type module examples/dom.js
```

An explicit `--input-type script` or `--input-type module` takes precedence over the file extension. Otherwise, `.mjs` files run as ES modules and `.cjs` files are rejected because CommonJS is unsupported. All other entry files, including `.js`, first try classic script compilation, then ES module compilation if the script has a syntax error.

| File | Description |
| --- | --- |
| [dom.js](dom.js) | DOM manipulation and events |
| [async.js](async.js) | Timers and microtasks |
| [fetch.mjs](fetch.mjs) | Cross-origin GitHub and Bilibili API requests |
| [modules/main.mjs](modules/main.mjs) | ES module imports |
| [workers/main.js](workers/main.js) | Worker messaging |
| [opfs-sah/main.js](opfs-sah/main.js) | Read and write OPFS through a sync access handle in a Worker |
| [sqlite/main.js](sqlite/main.js) | Persist SQLite data through the SAH VFS and query rows (build steps below) |
| [idb.mjs](idb.mjs) | IndexedDB reads and writes |

## Persistent storage

To reuse stored data across runs, use a fixed port and the same profile directory:

```sh
limo --port 3000 --profile-dir /tmp/limo-profile examples/opfs-sah/main.js
limo --port 3000 --profile-dir /tmp/limo-profile examples/idb.mjs
```

Each example increments its own persisted counter on every run.

## SQLite with the SAH VFS

The [SQLite example](sqlite/src/lib.rs) uses [sqlite-wasm-rs](https://github.com/Spxg/sqlite-wasm-rs) and `sqlite-wasm-vfs` with its `sahpool` feature. A module Worker loads the generated WebAssembly, opens `limo.db` in OPFS, inserts a row into `visits`, and queries all rows. It closes the database and releases the sync access handles before returning the result.

Build the example from the repository root. A C compiler with WebAssembly support, such as Clang, is required:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.130 --locked
cargo build --locked --manifest-path examples/sqlite/Cargo.toml --target wasm32-unknown-unknown --release
wasm-bindgen examples/sqlite/target/wasm32-unknown-unknown/release/limo_sqlite_example.wasm --target web --out-dir examples/sqlite/pkg
```

Run it twice with the same port and profile directory to see the earlier rows persist:

```sh
limo --port 3000 --profile-dir /tmp/limo-profile examples/sqlite/main.js
limo --port 3000 --profile-dir /tmp/limo-profile examples/sqlite/main.js
```

Each run appends one row and prints the table. The database lives in the profile's OPFS storage. Generated build files are ignored by Git.
