# Command-line reference

```text
limo [OPTIONS] [FILE]
```

Choose an entry file, `--eval`, or standard input. A file and `--eval` cannot be supplied together.

Run `limo -h` for built-in help.

| Argument or option | Description | Default |
| --- | --- | --- |
| `[FILE]` | JavaScript entry file; use `-` to read standard input. | Read redirected or piped standard input. |
| `-e, --eval <CODE>` | Run JavaScript source supplied on the command line. | |
| `--input-type <TYPE>` | Execute as `script` or `module`. Applies to files, snippets, and standard input. | File extension or syntax detection. |
| `--port <PORT>` | Port for the local server that loads the script and nearby files. | An available port chosen by the OS. |
| `--profile-dir <PATH>` | Directory for persistent browser state. | Temporary storage. |
| `--timeout-ms <MS>` | Execution timeout in milliseconds. | `600000` (10 minutes). |
| `-h, --help` | Print help. | |
| `-V, --version` | Print the version. | |

See [running JavaScript](running-javascript.md) for examples, execution modes, and relative file paths.

## Reuse browser state

Use a fixed port and the same profile directory across runs:

```sh
limo --port 3000 --profile-dir .limo-profile app.js
```

`--profile-dir` keeps browser data on disk. Browser storage is scoped to an origin, so keep the same `--port` to access it again. Without these options, Limo uses a temporary profile and an automatically assigned port.
