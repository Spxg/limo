// RUN: limo --timeout-ms 5000 %s
// OUTPUT: worker log: preserved
// OUTPUT: worker warn: preserved
// OUTPUT:
// OUTPUT: worker: done

const worker = new Worker("./fixtures/worker-console.js");
worker.onmessage = ({ data }) => {
  console.log(`worker: ${data}`);
  worker.terminate();
};
worker.postMessage("start");
