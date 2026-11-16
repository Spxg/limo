# Running JavaScript

Run a file, a code snippet, or source from standard input. Each runs in a browser page with access to globals such as `document` and `fetch`.

## Files

Pass a JavaScript file as the entry point:

```sh
limo app.js
limo app.mjs
```

## Code snippets

Use `-e` or `--eval` to run source code from the command line:

```sh
limo -e 'console.log(1 + 2)'
limo -e 'console.log(await Promise.resolve(3))'
```

The snippet loads as an entry script, just like a file. Use `console.log()` to print a value.

## Standard input

Use `-` to read source code from standard input:

```sh
echo 'console.log(1 + 2)' | limo -
limo - < app.js
```

When standard input is redirected or piped, you can omit `-`:

```sh
echo 'console.log(1 + 2)' | limo
```

Limo reads the complete input before running it. Running `limo` without an input does not start an interactive REPL.

## Scripts and modules

By default, Limo chooses the execution mode from the input:

| Input | Execution mode |
| --- | --- |
| `.mjs` file | ES module |
| `.cjs` file | Rejected; CommonJS is unsupported |
| Other files, `--eval`, or standard input | Detected from the source syntax |

Static `import`, `export`, and top-level `await` select module execution during syntax detection.

Use `--input-type` to select a mode explicitly. It takes priority over the file extension and syntax detection:

```sh
limo --input-type script app.js
limo --input-type module app.js
limo --input-type module -e 'console.log(import.meta.url)'
```

Limo uses browser ES modules. CommonJS globals such as `require` and `module.exports`, and Node.js built-in imports such as `node:fs`, are unavailable.

## Relative files

Limo serves local files from a directory determined by the input:

| Input | Local root |
| --- | --- |
| JavaScript file | The entry file's directory |
| `--eval` or standard input | The current working directory |

In page scripts, relative URLs passed to `fetch()` or `new Worker()` resolve from this root. Keep local resources inside it. ES module imports resolve relative to the importing module.

For example, `limo scripts/main.mjs` loads `fetch("./data.json")` from `scripts/data.json`. Piping the same code from the repository root loads `data.json` from the repository root instead.

Requests to external sites follow browser CORS rules.

## Output and completion

Console messages from the page and its Workers are forwarded to standard output, including `console.warn` and `console.error`.

Limo waits for browser work to become idle before exiting. Clear repeating timers and terminate Workers when their work is done. A pending top-level `await` alone does not keep the process running.

The default execution timeout is 10 minutes. Set a different limit in milliseconds:

```sh
limo --timeout-ms 5000 app.js
```

Some uncaught JavaScript exceptions currently do not produce a nonzero exit code.
