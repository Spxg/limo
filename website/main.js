"use strict";

const examples = {
  dom: {
    title: "A DOM, right in your terminal.",
    description: "Create elements, handle events, and query a browser document.",
    files: [{
      name: "dom.js",
      code: `const button = document.createElement("button");
button.textContent = "Click me";
button.addEventListener("click", () => {
  console.log("Hello from the DOM!");
});
document.body.append(button);

button.click();
console.log(document.querySelector("button").outerHTML);`,
    }],
    command: "limo dom.js",
    output: "Hello from the DOM!\n<button>Click me</button>",
  },
  canvas: {
    title: "Draw an image with Canvas.",
    description: "Draw a square and export it as a PNG data URL.",
    files: [{
      name: "canvas.js",
      code: `const canvas = document.createElement("canvas");
canvas.width = canvas.height = 64;

const ctx = canvas.getContext("2d");
ctx.fillStyle = "#417e38";
ctx.fillRect(0, 0, 64, 64);

console.log(canvas.toDataURL("image/png"));`,
    }],
    command: "limo canvas.js",
    output: "data:image/png;base64,...",
  },
  fetch: {
    title: "Fetch. Just like in the browser.",
    description: "Query the GitHub API with the same Fetch API you already use.",
    files: [{
      name: "fetch.mjs",
      code: `const response = await fetch(
  "https://api.github.com/repos/Spxg/limo",
);
const repo = await response.json();

console.log(repo.full_name);
console.log(repo.description);`,
    }],
    command: "limo fetch.mjs",
    output: "Spxg/limo\nRun JavaScript with browser Web APIs",
  },
  cors: {
    title: "Fetch without CORS.",
    description: "Disable CORS checks to read a cross-origin API response.",
    files: [{
      name: "fetch.mjs",
      code: `const response = await fetch(
  "https://api.bilibili.com/x/web-interface/view?bvid=BV1GJ411x7h7",
);
const { code, message, data } = await response.json();
if (code !== 0) throw new Error(message);

console.log(data.bvid);`,
    }],
    command: "limo --disable-web-security fetch.mjs",
    output: "BV1GJ411x7h7",
  },
  workers: {
    title: "Let a Worker handle it.",
    description: "Send data between scripts with standard browser Workers.",
    files: [{
      name: "main.js",
      code: `const worker = new Worker("./worker.js");

worker.onmessage = ({ data }) => {
  console.log("sum:", data);
  worker.terminate();
};

worker.postMessage([1, 2, 3, 4, 5]);`,
    }, {
      name: "worker.js",
      code: `self.onmessage = ({ data }) => {
  const sum = data.reduce((total, value) => total + value, 0);
  self.postMessage(sum);
};`,
    }],
    command: "limo main.js",
    output: "sum: 15",
  },
  indexeddb: {
    title: "Store data with IndexedDB.",
    description: "Write and read data in a persistent browser database.",
    files: [{
      name: "indexed_db.js",
      code: `const request = indexedDB.open("limo-demo", 1);
request.onupgradeneeded = () => {
  request.result.createObjectStore("messages");
};
request.onsuccess = () => {
  const db = request.result;
  const transaction = db.transaction("messages", "readwrite");
  const store = transaction.objectStore("messages");
  store.put("Hello from IndexedDB!", "greeting");

  const saved = store.get("greeting");
  saved.onsuccess = () => console.log(saved.result);
  transaction.oncomplete = () => db.close();
};`,
    }],
    command: "limo --port 3000 --profile-dir .limo-profile indexed_db.js",
    output: "Hello from IndexedDB!",
  },
  opfs: {
    title: "Read and write files with OPFS.",
    description: "Use a synchronous access handle in a Worker to store and read a file.",
    files: [{
      name: "opfs_sah.js",
      code: `const worker = new Worker("./worker.js");

worker.onmessage = ({ data }) => {
  console.log(data);
  worker.terminate();
};`,
    }, {
      name: "worker.js",
      code: `async function run() {
  const root = await navigator.storage.getDirectory();
  const file = await root.getFileHandle("greeting.txt", { create: true });
  const handle = await file.createSyncAccessHandle();

  try {
    const data = new TextEncoder().encode("Hello from OPFS!");
    const size = handle.write(data, { at: 0 });
    handle.truncate(size);
    handle.flush();

    const saved = new Uint8Array(size);
    handle.read(saved, { at: 0 });
    return new TextDecoder().decode(saved);
  } finally {
    handle.close();
  }
}

run().then((message) => self.postMessage(message));`,
    }],
    command: "limo --port 3000 --profile-dir .limo-profile opfs_sah.js",
    output: "Hello from OPFS!",
  },
};

const exampleTabs = [...document.querySelectorAll("[data-example]")];
const fileTabs = document.getElementById("file-tabs");
const sourceCode = document.getElementById("source-code");
const sourcePre = document.getElementById("source-pre");
const copyCode = document.getElementById("copy-code");
const copyCommand = document.getElementById("copy-command");
const copyStatus = document.getElementById("copy-status");
let currentExample = "dom";
let currentFile = 0;

function escapeHtml(value) {
  return value.replace(/[&<>"']/g, (character) => ({
    "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;",
  })[character]);
}

function highlight(code) {
  const pattern = /("(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'|`(?:\\.|[^`\\])*`)|(\/\/[^\n]*|\/\*[\s\S]*?\*\/)|\b(const|let|var|new|return|await|async|throw|if|else|try|catch|finally|true|false|null)\b|\b(\d+)\b|\b(document|console|self|indexedDB|Worker|Number|String)\b|\b([a-zA-Z_$][\w$]*)(?=\s*\()/g;
  let result = "";
  let position = 0;
  for (const match of code.matchAll(pattern)) {
    result += escapeHtml(code.slice(position, match.index));
    const kind = ["string", "comment", "keyword", "number", "builtin", "function"]
      .find((_, index) => match[index + 1] !== undefined);
    result += `<span class="token-${kind}">${escapeHtml(match[0])}</span>`;
    position = match.index + match[0].length;
  }
  return result + escapeHtml(code.slice(position));
}

function showFile(index) {
  currentFile = index;
  const file = examples[currentExample].files[index];
  sourceCode.innerHTML = file.code.split("\n").map((line, lineNumber) =>
    `<span class="code-line"><span class="line-number" aria-hidden="true">${lineNumber + 1}</span><span>${highlight(line)}</span></span>`,
  ).join("\n");
  sourcePre.setAttribute("aria-label", `${file.name} source code`);
  document.getElementById("code-language").textContent = file.language === "json" ? "JSON" : "JavaScript";
  sourcePre.scrollLeft = 0;
  sourcePre.scrollTop = 0;
  copyCode.setAttribute("aria-label", `Copy ${file.name} code`);
  fileTabs.querySelectorAll("button").forEach((button, buttonIndex) => {
    button.setAttribute("aria-pressed", String(buttonIndex === index));
  });
  resetCopyButton(copyCode);
}

function showExample(name) {
  currentExample = name;
  const example = examples[name];
  exampleTabs.forEach((tab) => {
    const selected = tab.dataset.example === name;
    tab.setAttribute("aria-selected", String(selected));
    tab.tabIndex = selected ? 0 : -1;
  });
  document.getElementById("example-panel").setAttribute("aria-labelledby", `tab-${name}`);
  document.getElementById("example-title").textContent = example.title;
  document.getElementById("example-description").textContent = example.description;
  document.getElementById("run-command").textContent = example.command;
  document.getElementById("run-command").scrollLeft = 0;
  document.getElementById("terminal-output").textContent = example.output;
  fileTabs.replaceChildren(...example.files.map((file, index) => {
    const button = document.createElement("button");
    button.className = "file-tab";
    button.type = "button";
    button.textContent = file.name;
    button.dataset.language = file.language || "javascript";
    button.addEventListener("click", () => showFile(index));
    return button;
  }));
  showFile(0);
  resetCopyButton(copyCommand);
}

exampleTabs.forEach((tab, index) => {
  tab.addEventListener("click", () => showExample(tab.dataset.example));
  tab.addEventListener("keydown", (event) => {
    let next;
    if (event.key === "ArrowRight") next = (index + 1) % exampleTabs.length;
    if (event.key === "ArrowLeft") next = (index - 1 + exampleTabs.length) % exampleTabs.length;
    if (event.key === "Home") next = 0;
    if (event.key === "End") next = exampleTabs.length - 1;
    if (next === undefined) return;
    event.preventDefault();
    showExample(exampleTabs[next].dataset.example);
    exampleTabs[next].focus();
  });
});

const copyTimers = new Map();

function resetCopyButton(button) {
  clearTimeout(copyTimers.get(button));
  copyTimers.delete(button);
  button.querySelector("span").textContent = button.id === "copy-command" ? "Copy" : "Copy to clipboard";
  button.querySelector("use").setAttribute("href", "#icon-copy");
  button.removeAttribute("data-copied");
}

async function copyText(text, button) {
  resetCopyButton(button);
  copyStatus.textContent = "";
  let copied = false;
  try {
    await navigator.clipboard.writeText(text);
    copied = true;
  } catch {
    const textarea = document.createElement("textarea");
    textarea.value = text;
    textarea.style.cssText = "position:fixed;top:0;left:0;opacity:0;pointer-events:none";
    (button.closest("dialog") || document.body).append(textarea);
    textarea.select();
    copied = document.execCommand("copy");
    textarea.remove();
    button.focus({ preventScroll: true });
  }
  clearTimeout(copyTimers.get(button));
  if (copied) {
    button.querySelector("span").textContent = "Copied!";
    button.querySelector("use").setAttribute("href", "#icon-check");
    button.dataset.copied = "true";
    copyStatus.textContent = "Copied to clipboard.";
  } else {
    button.querySelector("span").textContent = "Try again";
    copyStatus.textContent = "Your browser could not copy this text. You can select it to copy manually.";
  }
  copyTimers.set(button, setTimeout(() => resetCopyButton(button), 2200));
}

copyCode.addEventListener("click", () => {
  copyText(examples[currentExample].files[currentFile].code, copyCode);
});
copyCommand.addEventListener("click", () => {
  copyText(examples[currentExample].command, copyCommand);
});
document.querySelectorAll("[data-copy-install]").forEach((button) => {
  button.addEventListener("click", () => {
    copyText(button.closest(".install-panel").querySelector("code").textContent, button);
  });
});

showExample("dom");

const themeToggle = document.getElementById("theme-toggle");
const systemTheme = matchMedia("(prefers-color-scheme: dark)");
let chosenTheme;
try { chosenTheme = localStorage.getItem("limo-theme"); } catch {}

function setTheme(theme) {
  document.documentElement.dataset.theme = theme;
  themeToggle.setAttribute("aria-label", `Switch to ${theme === "dark" ? "light" : "dark"} theme`);
  document.querySelector('meta[name="theme-color"]').content = theme === "dark" ? "#0d121c" : "#ffffff";
}

setTheme(chosenTheme || (systemTheme.matches ? "dark" : "light"));
themeToggle.addEventListener("click", () => {
  chosenTheme = document.documentElement.dataset.theme === "dark" ? "light" : "dark";
  setTheme(chosenTheme);
  try { localStorage.setItem("limo-theme", chosenTheme); } catch {}
});
systemTheme.addEventListener("change", (event) => {
  if (!chosenTheme) setTheme(event.matches ? "dark" : "light");
});

const installDialog = document.getElementById("get-started");
document.querySelectorAll("[data-install]").forEach((link) => {
  link.addEventListener("click", (event) => {
    event.preventDefault();
    installDialog.showModal();
  });
});
document.getElementById("close-install").addEventListener("click", () => installDialog.close());
installDialog.addEventListener("click", (event) => {
  if (event.target !== installDialog) return;
  const bounds = installDialog.getBoundingClientRect();
  if (event.clientX < bounds.left || event.clientX > bounds.right || event.clientY < bounds.top || event.clientY > bounds.bottom) installDialog.close();
});
