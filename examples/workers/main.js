// limo examples/workers/main.js
const worker = new Worker("./worker.js");

worker.onmessage = ({ data }) => {
  console.log("sum:", data);
  worker.terminate();
};

worker.postMessage([1, 2, 3, 4, 5]);
