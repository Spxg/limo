self.onmessage = () => {
  console.log("worker log: preserved");
  console.warn("worker warn: preserved");
  console.log();
  self.postMessage("done");
};
