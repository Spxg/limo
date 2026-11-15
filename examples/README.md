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
| [fetch.mjs](fetch.mjs) | Fetch a local JSON file |
| [modules/main.mjs](modules/main.mjs) | ES module imports |
| [workers/main.js](workers/main.js) | Worker messaging |
| [opfs-sah/main.js](opfs-sah/main.js) | Read and write OPFS through a sync access handle in a Worker |
| [idb.mjs](idb.mjs) | IndexedDB reads and writes |
| [cors.mjs](cors.mjs) | Cross-origin requests and CORS restrictions |

## Persistent storage

To reuse stored data across runs, use a fixed port and the same profile directory:

```sh
limo --port 3000 --profile-dir /tmp/limo-profile examples/opfs-sah/main.js
limo --port 3000 --profile-dir /tmp/limo-profile examples/idb.mjs
```

Each example increments its own persisted counter on every run.
