// RUN: limo --timeout-ms 5000 %s
// STDOUT: script
// STDOUT: true
//
// RUN: limo --timeout-ms 5000 --input-type module %s
// STDOUT: module
// STDOUT: false
//
// RUN: limo --timeout-ms 5000 tests/fixtures/input-import.js
// STDOUT: imported value
// STDOUT: true
//
// RUN: limo --timeout-ms 5000 tests/fixtures/input-await.js
// STDOUT: awaited value
// STDOUT: true
//
// RUN: limo --timeout-ms 5000 tests/fixtures/input-plain.mjs
// STDOUT: module
//
// RUN: limo --timeout-ms 5000 --input-type script tests/fixtures/input-plain.mjs
// STDOUT: script
//
// RUN: limo --timeout-ms 5000 --input-type script tests/fixtures/input-plain.cjs
// STDOUT: script
//
// RUN: limo --timeout-ms 5000 --input-type module tests/fixtures/input-plain.cjs
// STDOUT: module

var entryBinding = "entry";
console.log(this === globalThis ? "script" : "module");
console.log(Object.hasOwn(globalThis, "entryBinding"));
