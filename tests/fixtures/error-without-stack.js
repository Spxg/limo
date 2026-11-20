function buildError() {
  return new Error("stack unavailable");
}
console.log("stack: before");
const error = buildError();
error.stack = undefined;
throw error;
//# sourceURL=error-without-stack.js
