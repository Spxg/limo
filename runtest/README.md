# runtest

Build limo in release mode, then run JavaScript tests concurrently:

```sh
cargo runtest
```

Files directly inside the repository's `tests/` directory are scanned for `RUN` directives. Subdirectories can hold fixtures and are not scanned. Files without `RUN` directives are skipped.

```js
// RUN: limo %s
// OUTPUT: hello
// OUTPUT:
// OUTPUT: world

console.log("hello");
console.log("");
console.log("world");
```

- `limo` resolves to the release binary built by runtest; `%s` expands to the current file's absolute path.
- Commands run from the repository root. Quoted arguments are supported; shell pipelines and redirections are not.
- Consecutive `OUTPUT` lines immediately after `RUN` specify exact stdout, including a newline for each line. One optional space after `OUTPUT:` is removed; all other whitespace is preserved. CRLF is normalized to LF.
- Without `OUTPUT`, stdout is not checked. The tested command's exit status is always ignored. Build, parse, and process launch errors fail the run.
- Files run concurrently. Multiple `RUN` commands in the same file run sequentially.
- runtest exits unsuccessfully if any test fails, reporting the command and expected/actual stdout with stderr for output mismatches.
