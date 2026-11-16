// RUN: limo --timeout-ms 5000 %s
// OUTPUT: sync: start
// OUTPUT: sync: end
// OUTPUT: promise
// OUTPUT: microtask
// OUTPUT: timer
// OUTPUT: interval: 1
// OUTPUT: interval: 2
// OUTPUT: async: done

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
