// RUN: limo --timeout-ms 5000 %s
// STDOUT: entry MIME: text/javascript
// STDOUT: entry cache: no-store
// STDOUT: relative fetch: local fixture
// STDOUT: resource cache: no-store
// STDOUT: last-modified: true
// STDOUT: conditional status: 200
// STDOUT: conditional body: local fixture
// STDOUT: server: done
//
// RUN: limo --timeout-ms 5000 --input-type module tests/fixtures/entry-source
// STDOUT: extensionless MIME: text/javascript
// STDOUT: extensionless module: done

const entry = await fetch(import.meta.url);
console.log("entry MIME:", entry.headers.get("content-type"));
console.log("entry cache:", entry.headers.get("cache-control"));

const resource = new URL("./fixtures/server-data.json", import.meta.url);
const response = await fetch(resource);
console.log("relative fetch:", (await response.json()).message);
console.log("resource cache:", response.headers.get("cache-control"));
console.log("last-modified:", response.headers.has("last-modified"));

const conditional = await fetch(resource, {
  headers: { "If-Modified-Since": response.headers.get("last-modified") },
});
console.log("conditional status:", conditional.status);
console.log("conditional body:", (await conditional.json()).message);
console.log("server: done");
