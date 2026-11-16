# SQLite fixture

Precompiled from `examples/sqlite/`, using `sqlite-wasm-rs` and the OPFS SAH pool VFS. The test opens the database in two separate Workers, inserts a row in each, and checks that the second Worker reads both rows.

Regenerate from the repository root with Clang, the `wasm32-unknown-unknown` target, and `wasm-bindgen-cli` 0.2.130 installed:

```sh
cargo build --locked --manifest-path examples/sqlite/Cargo.toml --target wasm32-unknown-unknown --release
wasm-bindgen examples/sqlite/target/wasm32-unknown-unknown/release/limo_sqlite_example.wasm --target web --no-typescript --out-dir tests/fixtures/sqlite
```

The generated JavaScript and WebAssembly are included so `cargo runtest` does not need the WebAssembly build toolchain.
