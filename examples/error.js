function fail() {
  throw new Error("Example error");
}

console.log("Throwing from a timer...");
setTimeout(fail, 1000);
