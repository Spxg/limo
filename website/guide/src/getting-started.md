# Getting started

Limo runs JavaScript in a browser environment, right from your terminal.

## Install

Linux and macOS:

```sh
curl -fsSL https://github.com/Spxg/limo/releases/latest/download/limo-installer.sh | sh
```

Windows PowerShell:

```powershell
irm https://github.com/Spxg/limo/releases/latest/download/limo-installer.ps1 | iex
```

Releases support Linux (glibc), macOS 13+, and Windows on x86_64 and ARM64.

Limo installs to `~/.local/bin` on Linux and macOS, or `%LOCALAPPDATA%\Limo\bin` on Windows. Follow the printed instructions to add that directory to your `PATH` if needed.

## Run your first script

Create `dom.js`:

```js
const title = document.createElement("h1");
title.textContent = "Hello, Limo!";
document.body.append(title);

console.log(document.querySelector("h1").textContent);
```

Run it from the terminal:

```sh
limo dom.js
```

```text
Hello, Limo!
```

Continue with [running JavaScript](running-javascript.md) or the [command-line reference](cli.md).
