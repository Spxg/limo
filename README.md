A JavaScript command-line runner powered by [moli](https://github.com/lexmount/moli). Run scripts in a browser environment with Web APIs.

## Usage

Install from source:

```sh
cargo install --locked --path .
```

Run a JavaScript file:

```sh
limo script.js
limo script.mjs
```

`.js` files run as classic scripts, and `.mjs` files run as ES modules. All `console` output is forwarded to stdout in real time. limo exits when moli considers the page idle, with a default timeout of ten minutes.

## Options

```sh
limo [OPTIONS] <FILE>
```

| Option | Description | Default |
| --- | --- | --- |
| `--port <PORT>` | Local server port | Random port |
| `--profile-dir <PATH>` | Directory for persistent browser data | Temporary storage |
| `--timeout-ms <MS>` | Execution timeout in milliseconds | `600000` |

Run `limo -h` for help.

## Examples

See the [examples](examples/README.md).
