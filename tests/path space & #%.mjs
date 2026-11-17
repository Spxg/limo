// RUN: limo --timeout-ms 5000 %s
// STDOUT: entry name: path space & #%.mjs
// STDOUT: relative import: hello from module
// STDOUT: relative fetch: local fixture
// STDOUT: special path: done

import { message } from "./fixtures/message.mjs";

const name = new URL(import.meta.url).pathname.split("/").pop();
console.log("entry name:", decodeURIComponent(name));
console.log("relative import:", message);

const response = await fetch("./fixtures/server-data.json");
console.log("relative fetch:", (await response.json()).message);
console.log("special path: done");
