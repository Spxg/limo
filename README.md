A JavaScript command-line runner powered by [moli](https://github.com/lexmount/moli). Run scripts in a browser environment with Web APIs.

## Usage

Install from source:

```sh
cargo install --locked --path .
```

Run a JavaScript file, code snippet, or code from standard input:

```sh
limo [OPTIONS] [FILE]
```

By default, `.mjs` files run as ES modules, `.cjs` files are rejected because CommonJS is unsupported, and other inputs use automatic detection.

## Options

| Option | Description | Default |
| --- | --- | --- |
| `-e, --eval <CODE>` | Run JavaScript code | |
| `--input-type <TYPE>` | `script` or `module`, for files, `--eval`, and standard input | File extension or automatic detection |
| `--port <PORT>` | Local server port | Random port |
| `--profile-dir <PATH>` | Directory for persistent browser data | Temporary storage |
| `--timeout-ms <MS>` | Execution timeout in milliseconds | `600000` |

Choose either a file or `--eval`. Run `limo -h` for help.

## Examples

See the [examples](examples/README.md).

## Testing

See [runtest](runtest/README.md) for the command and output test runner.
