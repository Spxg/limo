// RUN: limo --timeout-ms 5000 -e 'console.log(this === globalThis); console.log("hello world"); console.log("line\nnext"); console.log("C:\\tmp\\file")'
// OUTPUT: true
// OUTPUT: hello world
// OUTPUT: line
// OUTPUT: next
// OUTPUT: C:\tmp\file
// RUN: limo --timeout-ms 5000 --input-type script -e 'console.log(this === globalThis); console.log((function () { return this; })() === globalThis)'
// OUTPUT: true
// OUTPUT: true
// RUN: limo --timeout-ms 5000 --input-type module -e 'console.log(this === undefined); console.log((function () { return this; })() === undefined)'
// OUTPUT: true
// OUTPUT: true
// RUN: limo --timeout-ms 5000 -e 'console.log(await Promise.resolve("await result")); console.log(this === undefined)'
// OUTPUT: await result
// OUTPUT: true
// RUN: limo --timeout-ms 5000 -e 'import { value } from "./tests/fixtures/eval-value.mjs"; console.log(value); console.log(this === undefined)'
// OUTPUT: eval import from cwd
// OUTPUT: true
