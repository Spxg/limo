// RUN: limo --timeout-ms 5000 %s
// OUTPUT: script
// OUTPUT: true
// RUN: limo --timeout-ms 5000 --input-type module %s
// OUTPUT: module
// OUTPUT: false
// RUN: limo --timeout-ms 5000 tests/fixtures/input-import.js
// OUTPUT: imported value
// OUTPUT: true
// RUN: limo --timeout-ms 5000 tests/fixtures/input-await.js
// OUTPUT: awaited value
// OUTPUT: true
// RUN: limo --timeout-ms 5000 tests/fixtures/input-plain.mjs
// OUTPUT: module
// RUN: limo --timeout-ms 5000 --input-type script tests/fixtures/input-plain.mjs
// OUTPUT: script
// RUN: limo --timeout-ms 5000 --input-type script tests/fixtures/input-plain.cjs
// OUTPUT: script
// RUN: limo --timeout-ms 5000 --input-type module tests/fixtures/input-plain.cjs
// OUTPUT: module

var entryBinding = "entry";
console.log(this === globalThis ? "script" : "module");
console.log(Object.hasOwn(globalThis, "entryBinding"));
