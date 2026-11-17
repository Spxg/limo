// RUN: limo --timeout-ms 5000 %s
// STDOUT: worker log: preserved
// STDOUT: worker warn: preserved
// STDOUT:
// STDOUT: worker: done

const worker = new Worker("./fixtures/worker-console.js");
worker.onmessage = ({ data }) => {
  console.log(`worker: ${data}`);
  worker.terminate();
};
worker.postMessage("start");
