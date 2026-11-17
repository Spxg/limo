// RUN: limo --timeout-ms 5000 -e 'console.log("script: before"); setTimeout(() => console.log("script: after"), 1000); throw "script failure";'
// STDOUT: script: before
// STDERR: Uncaught script failure
//
// RUN: limo --timeout-ms 5000 tests/fixtures/error-timer.js
// STDOUT: timer: before
// STDERR: Error: timer failure
// STDERR:     at fail (timer-error.js:2:9)
//
// RUN: limo --timeout-ms 5000 -e 'console.log("promise: before"); setTimeout(() => console.log("promise: after"), 1000); Promise.reject("promise failure");'
// STDOUT: promise: before
// STDERR: Uncaught (in promise) promise failure
//
// RUN: limo --timeout-ms 5000 --input-type module -e 'console.log("module: before"); setTimeout(() => console.log("module: after"), 1000); await Promise.reject("module failure");'
// STDOUT: module: before
// STDERR: Uncaught module failure
//
// RUN: limo --timeout-ms 5000 -e 'console.log("stack: before"); const error = new Error("stack unavailable"); error.stack = undefined; throw error;'
// STDOUT: stack: before
// STDERR: Uncaught Error: stack unavailable
//
// RUN: limo --timeout-ms 5000 -e 'console.log("worker: before"); new Worker("./tests/fixtures/worker-error.js");'
// STDOUT: worker: before
// STDERR: Error: worker failure
// STDERR:     at fail (worker-error.js:2:9)
//
// RUN: limo --timeout-ms 5000 -e 'const worker = new Worker("./tests/fixtures/worker-error.js"); worker.onerror = (event) => { event.preventDefault(); console.log("worker: handled"); worker.terminate(); setTimeout(() => console.log("worker: continued"), 0); };'
// STDOUT: worker: handled
// STDOUT: worker: continued
