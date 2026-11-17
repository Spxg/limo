// RUN: limo --timeout-ms 5000 %s
// STDOUT: onerror: handled
// STDOUT: error event: handled
// STDOUT: rejection: handled
// STDOUT: handled errors: done

window.onerror = () => {
  window.onerror = null;
  console.log("onerror: handled");
  setTimeout(preventErrorEvent, 10);
  return true;
};

setTimeout(() => {
  throw new Error("handled by window.onerror");
}, 0);

function preventErrorEvent() {
  addEventListener("error", (event) => {
    event.preventDefault();
    console.log("error event: handled");
    setTimeout(preventRejection, 10);
  }, { once: true });
  throw new Error("handled by the error event");
}

function preventRejection() {
  addEventListener("unhandledrejection", (event) => {
    event.preventDefault();
    console.log("rejection: handled");
    setTimeout(() => console.log("handled errors: done"), 10);
  }, { once: true });
  Promise.reject(new Error("handled by the rejection event"));
}
