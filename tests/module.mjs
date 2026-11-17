// RUN: limo --timeout-ms 5000 %s
// STDOUT: hello from module

import { message } from "./fixtures/message.mjs";

console.log(message);
