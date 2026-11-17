# runtest

Build limo in release mode, then run JavaScript tests concurrently:

```sh
cargo runtest
```

Files directly inside the repository's `tests/` directory are scanned for `RUN` directives. Subdirectories can hold fixtures and are not scanned. Files without `RUN` directives are skipped.

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
// STDERR: Uncaught failure
```

- `limo` resolves to the release binary built by runtest; `%s` expands to the current file's absolute path.
- Commands run from the repository root. Quoted arguments are supported; shell pipelines and redirections are not.
- Consecutive `STDOUT` and `STDERR` lines immediately after `RUN` specify exact output for each stream, including a newline for each line. One optional space after the directive is removed; all other whitespace is preserved. CRLF is normalized to LF.
- Streams without an expectation are not checked. The tested command's exit status is always ignored. Build, parse, and process launch errors fail the run.
- Files run concurrently. Multiple `RUN` commands in the same file run sequentially.
- runtest exits unsuccessfully if any test fails, reporting the command and expected/actual output with stderr for output mismatches.
