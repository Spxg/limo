# Getting started

Limo runs JavaScript in a browser environment, right from your terminal.

## Install from source

You need Git, Rust, and native build tools. Clone the repository and install Limo with Cargo:

```sh
git clone https://github.com/Spxg/limo.git
cd limo
cargo install --locked --path .
```

Cargo installs the `limo` executable in its binary directory. Make sure that directory is on your `PATH`.

## Run your first script

Create `hello.js`:

```js
const title = document.createElement("h1");
title.textContent = "Hello, Limo!";
document.body.append(title);

console.log(document.querySelector("h1").textContent);
```

Run it from the terminal:

```sh
limo hello.js
```

```text
Hello, Limo!
```

Continue with [running JavaScript](running-javascript.md) or the [command-line reference](cli.md).
