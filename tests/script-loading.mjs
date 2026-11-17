// RUN: limo --timeout-ms 5000 %s
// STDOUT: classic: 1
// STDOUT: classic: 2
// STDOUT: moved classic: 2
// STDOUT: module: 1
// STDOUT: module loads: 1
// STDOUT: missing script: error
// STDOUT: classic: 3
// STDOUT: script loading: done

function loadScript(path, type = "text/javascript") {
  return new Promise((resolve, reject) => {
    const script = document.createElement("script");
    script.type = type;
    script.src = new URL(path, import.meta.url);
    script.onload = () => resolve(script);
    script.onerror = () => reject(new Error("script load failed"));
    document.body.appendChild(script);
  });
}

const first = await loadScript("./fixtures/dynamic-script.js");
await loadScript("./fixtures/dynamic-script.js");

const container = document.createElement("div");
document.body.appendChild(container);
container.appendChild(first);
await new Promise((resolve) => setTimeout(resolve, 0));
console.log("moved classic:", globalThis.classicLoads);

await loadScript("./fixtures/dynamic-module.mjs", "module");
await loadScript("./fixtures/dynamic-module.mjs", "module");
console.log("module loads:", globalThis.moduleLoads);

await loadScript("./fixtures/missing-script.js").catch(() => {
  console.log("missing script: error");
});
await loadScript("./fixtures/dynamic-script.js");
console.log("script loading: done");
