# Examples

Run these commands from the repository root:

## Code snippets and standard input

```sh
limo -e 'console.log(1 + 2)'
echo 'console.log(1 + 2)' | limo -
echo 'console.log(1 + 2)' | limo
```

These inputs run as classic scripts. Use dynamic `import()` to load modules, or run an `.mjs` file for static imports and top-level `await`. Relative URLs for `fetch`, Workers, and dynamic imports resolve from the current working directory.

## Files

```sh
limo examples/dom.js
limo examples/modules/main.mjs
```

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
