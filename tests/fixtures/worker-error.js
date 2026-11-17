function fail() {
  throw new Error("worker failure");
}

setTimeout(fail, 0);
//# sourceURL=worker-error.js
