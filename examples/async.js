// limo examples/async.js

console.log("start");
queueMicrotask(() => console.log("microtask"));

let ticks = 0;
const interval = setInterval(() => {
  ticks += 1;
  console.log("interval:", ticks);
  if (ticks === 3) clearInterval(interval);
}, 50);

setTimeout(() => {
  console.log("timeout");
  queueMicrotask(() => console.log("last microtask"));
}, 200);

console.log("synchronous end");
