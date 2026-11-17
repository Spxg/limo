function fail() {
  throw new Error("timer failure");
}

console.log("timer: before");
setTimeout(fail, 0);
setTimeout(() => console.log("timer: after"), 1000);
//# sourceURL=timer-error.js
