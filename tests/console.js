// RUN: limo --timeout-ms 5000 %s
// OUTPUT: debug: preserved
// OUTPUT: info: preserved
// OUTPUT: log: preserved
// OUTPUT: warn: preserved
// OUTPUT: error: preserved
// OUTPUT:
// OUTPUT: final: preserved

console.debug("debug: preserved");
console.info("info: preserved");
console.log("log: preserved");
console.warn("warn: preserved");
console.error("error: preserved");
console.log();
queueMicrotask(() => console.log("final: preserved"));
