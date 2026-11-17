// RUN: limo --timeout-ms 5000 %s
// STDOUT: entry name: path &copy; 中文.mjs
// STDOUT: relative import: hello from module

import { message } from "./fixtures/message.mjs";

console.log("entry name:", decodeURIComponent(new URL(import.meta.url).pathname.slice(1)));
console.log("relative import:", message);
