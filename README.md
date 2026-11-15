A JavaScript command-line runner powered by [moli](https://github.com/lexmount/moli). Run scripts in a browser environment with Web APIs.

## Usage

Install from source:

```sh
cargo install --locked --path .
```

Run a JavaScript file, evaluate code, or read code from standard input:

```sh
limo [OPTIONS] [FILE]
```

`.js` files run as classic scripts, and `.mjs` files run as ES modules.

## Options

| Option | Description | Default |
| --- | --- | --- |
| `-e, --eval <CODE>` | Evaluate JavaScript code | |
| `--port <PORT>` | Local server port | Random port |
| `--profile-dir <PATH>` | Directory for persistent browser data | Temporary storage |
| `--timeout-ms <MS>` | Execution timeout in milliseconds | `600000` |

Choose either a file or `--eval`. Run `limo -h` for help.

## Examples

See the [examples](examples/README.md).
