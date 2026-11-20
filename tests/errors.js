// RUN: limo --timeout-ms 5000 -e 'console.log("script: before"); setTimeout(() => console.log("script: after"), 1000); throw "script failure"; //# sourceURL=script-error.js'
// EXIT: fail
// STDOUT: script: before
// STDERR: Uncaught script failure
// STDERR:     thrown at script-error.js:1:86
//
// RUN: limo --timeout-ms 5000 tests/fixtures/error-timer.js
// EXIT: fail
// STDOUT: timer: before
// STDERR: Uncaught Error: timer failure
// STDERR:     thrown at timer-error.js:2:3
// STDERR:     at fail
//
// RUN: limo --timeout-ms 5000 -e 'console.log("promise: before"); setTimeout(() => console.log("promise: after"), 1000); Promise.reject("promise failure");'
// EXIT: fail
// STDOUT: promise: before
// STDERR: Uncaught promise failure
//
// RUN: limo --timeout-ms 5000 --input-type module -e 'console.log("module: before"); setTimeout(() => console.log("module: after"), 1000); await Promise.reject("module failure");'
// EXIT: fail
// STDOUT: module: before
// STDERR: Uncaught module failure
//
// RUN: limo --timeout-ms 5000 tests/fixtures/error-without-stack.js
// EXIT: fail
// STDOUT: stack: before
// STDERR: Uncaught Error: stack unavailable
// STDERR:     thrown at error-without-stack.js:7:1
// STDERR:     at buildError
// STDERR:     at <anonymous>
//
// RUN: limo --timeout-ms 5000 -e 'console.log("worker: before"); const worker = new Worker("./tests/fixtures/worker-error.js"); worker.onerror = () => console.log("worker: onerror"); window.onerror = () => { console.log("window: onerror"); setTimeout(() => console.log("worker: after"), 1000); };'
// EXIT: fail
// STDOUT: worker: before
// STDOUT: worker: onerror
// STDOUT: window: onerror
// STDERR: Uncaught Error: worker failure
// STDERR:     thrown at worker-error.js:2:3
// STDERR:     at fail
//
// RUN: limo --timeout-ms 5000 -e 'const worker = new Worker("./tests/fixtures/worker-error.js"); window.onerror = () => console.log("unexpected Window error"); worker.onerror = event => { event.preventDefault(); console.log("worker: handled on page"); worker.terminate(); setTimeout(() => console.log("page: continued"), 0); };'
// STDOUT: worker: handled on page
// STDOUT: page: continued
//
// RUN: limo --timeout-ms 5000 -e 'const worker = new Worker("./tests/fixtures/worker-error.js"); worker.onerror = () => console.log("worker: onerror"); window.onerror = () => { console.log("window: handled"); worker.terminate(); setTimeout(() => console.log("page: continued"), 0); return true; };'
// STDOUT: worker: onerror
// STDOUT: window: handled
// STDOUT: page: continued
//
// RUN: limo --timeout-ms 5000 -e 'const worker = new Worker("./tests/fixtures/worker-error.js?handled"); worker.onerror = () => console.log("unexpected Worker error"); window.onerror = () => console.log("unexpected Window error"); worker.onmessage = ({data}) => { console.log(data); worker.terminate(); };'
// STDOUT: worker: continued
