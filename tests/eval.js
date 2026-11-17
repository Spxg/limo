// RUN: limo --timeout-ms 5000 -e 'console.log(this === globalThis); console.log("hello world"); console.log("line\nnext"); console.log("C:\\tmp\\file")'
// STDOUT: true
// STDOUT: hello world
// STDOUT: line
// STDOUT: next
// STDOUT: C:\tmp\file
//
// RUN: limo --timeout-ms 5000 --input-type script -e 'console.log(this === globalThis); console.log((function () { return this; })() === globalThis)'
// STDOUT: true
// STDOUT: true
//
// RUN: limo --timeout-ms 5000 --input-type module -e 'console.log(this === undefined); console.log((function () { return this; })() === undefined)'
// STDOUT: true
// STDOUT: true
//
// RUN: limo --timeout-ms 5000 -e 'console.log(await Promise.resolve("await result")); console.log(this === undefined)'
// STDOUT: await result
// STDOUT: true
//
// RUN: limo --timeout-ms 5000 -e 'import { value } from "./tests/fixtures/eval-value.mjs"; console.log(value); console.log(this === undefined)'
// STDOUT: eval import from cwd
// STDOUT: true
