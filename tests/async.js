// RUN: limo --timeout-ms 5000 %s
// STDOUT: sync: start
// STDOUT: sync: end
// STDOUT: promise
// STDOUT: microtask
// STDOUT: timer
// STDOUT: interval: 1
// STDOUT: interval: 2
// STDOUT: async: done

console.log("sync: start");
Promise.resolve().then(() => console.log("promise"));
queueMicrotask(() => console.log("microtask"));
setTimeout(() => {
  console.log("timer");
  let ticks = 0;
  const interval = setInterval(() => {
    console.log("interval:", ++ticks);
    if (ticks === 2) {
      clearInterval(interval);
      queueMicrotask(() => console.log("async: done"));
    }
  }, 10);
}, 10);
console.log("sync: end");
