# runtest

Build Limo in release mode and run the JavaScript tests concurrently.

```sh
cargo runtest
```

Each `.js` or `.mjs` file directly inside `tests/` runs once as `limo --timeout-ms 5000 <file>` by default. Put shared helpers and fixtures in a subdirectory. A successful test exits with code zero. For example, a module can use the assertion helper to throw on failure.

```js
import { assert } from "./fixtures/assert.mjs";
import { message } from "./fixtures/message.mjs";

assert(message === "hello from module", `Unexpected export: ${message}`);
```

Use `// RUN:` for custom arguments or multiple commands. Each command runs from the repository root in the order it appears in the file. The explicit commands replace the default run. `limo` resolves to the release binary and `%s` expands to the file's absolute path. The runner splits quoted arguments and launches the executable directly. Shell operators such as `|` and `>` are passed as arguments.

```js
// RUN: limo %s
// STDOUT: hello
// STDOUT:
// STDOUT: world

console.log("hello");
console.log("");
console.log("world");
```

```js
// RUN: limo -e 'throw "failure"'
// EXIT: fail
// STDERR: Uncaught failure
```

Group expectations on consecutive lines immediately after their `RUN`.

- `EXIT: ok` is the default. `EXIT: fail` expects a nonzero exit code. A launch error or termination by a signal fails the test. An unexpected exit reports both stdout and stderr.
- `STDOUT:` and `STDERR:` check the declared streams exactly, including a newline for each line. One optional space after the directive is removed and CRLF is normalized to LF.
- For asynchronous tests, an expected final output line can verify that the last assertion ran. A pending top-level `await` alone allows Limo to exit when the page becomes idle.
