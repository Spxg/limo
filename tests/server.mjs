// RUN: limo --timeout-ms 5000 %s
// STDOUT: server: done
//
// RUN: limo --timeout-ms 5000 --input-type module tests/fixtures/entry-source
// STDOUT: extensionless module: done

import { assert } from "./fixtures/assert.mjs";

const entry = await fetch(import.meta.url);
assert(entry.headers.get("content-type") === "text/javascript", "Incorrect entry MIME type");
assert(entry.headers.get("cache-control") === "no-store", "Entry should disable caching");

const resource = new URL("./fixtures/server-data.json", import.meta.url);
const response = await fetch(resource);
assert((await response.json()).message === "local fixture", "Incorrect relative resource");
assert(response.headers.get("cache-control") === "no-store", "Resource should disable caching");
const lastModified = response.headers.get("last-modified");
assert(lastModified !== null, "Missing Last-Modified header");

const conditional = await fetch(resource, {
  headers: { "If-Modified-Since": lastModified },
});
assert(conditional.status === 200, `Conditional request returned ${conditional.status}`);
assert((await conditional.json()).message === "local fixture", "Incorrect conditional response");
console.log("server: done");
