// RUN: limo --timeout-ms 5000 %s
// STDOUT: worker: ping

const worker = new Worker("./fixtures/worker.js");
worker.onmessage = ({ data }) => {
  console.log(data);
  worker.terminate();
};
worker.postMessage("ping");
