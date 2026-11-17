// RUN: limo --timeout-ms 5000 %s
// STDOUT: shared log: preserved
// STDOUT: shared warn: preserved
// STDOUT:
// STDOUT: shared: done

const worker = new SharedWorker("./fixtures/shared-console.js");
worker.port.onmessage = ({ data }) => {
  console.log(`shared: ${data}`);
  worker.port.close();
};
worker.port.start();
worker.port.postMessage("start");
