// RUN: limo --timeout-ms 5000 %s
// OUTPUT: entry name: path space & #%.mjs
// OUTPUT: relative import: hello from module
// OUTPUT: relative fetch: local fixture
// OUTPUT: special path: done

import { message } from "./fixtures/message.mjs";

const name = new URL(import.meta.url).pathname.split("/").pop();
console.log("entry name:", decodeURIComponent(name));
console.log("relative import:", message);

const response = await fetch("./fixtures/server-data.json");
console.log("relative fetch:", (await response.json()).message);
console.log("special path: done");
