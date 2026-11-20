function fail() {
  throw new Error("worker failure");
}

setTimeout(fail, 0);
if (new URL(location.href).searchParams.has("handled")) {
  self.addEventListener("error", event => event.preventDefault());
  setTimeout(() => postMessage("worker: continued"), 10);
}
//# sourceURL=worker-error.js
