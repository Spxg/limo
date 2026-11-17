// RUN: limo --timeout-ms 5000 %s
// STDOUT: debug: preserved
// STDOUT: info: preserved
// STDOUT: log: preserved
// STDOUT: warn: preserved
// STDOUT: error: preserved
// STDOUT:
// STDOUT: final: preserved

console.debug("debug: preserved");
console.info("info: preserved");
console.log("log: preserved");
console.warn("warn: preserved");
console.error("error: preserved");
console.log();
queueMicrotask(() => console.log("final: preserved"));
