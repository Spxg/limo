// RUN: limo --timeout-ms 5000 %s
// OUTPUT: hello from module

import { message } from "./fixtures/message.mjs";

console.log(message);
