// RUN: limo --timeout-ms 5000 %s
// STDOUT: service log: preserved
// STDOUT: service warn: preserved
// STDOUT:
// STDOUT: service: done

const keepAlive = setInterval(() => {}, 100);
const registration = await navigator.serviceWorker.register("./fixtures/service-console.js");
const worker = registration.installing ?? registration.waiting ?? registration.active;
if (worker.state !== "activated") {
  await new Promise(resolve => {
    worker.addEventListener("statechange", () => {
      if (worker.state === "activated") resolve();
    });
  });
}
const channel = new MessageChannel();
const result = await new Promise(resolve => {
  channel.port1.onmessage = ({ data }) => {
    resolve(data);
  };
  channel.port1.start();
  worker.postMessage("start", [channel.port2]);
});
channel.port1.close();
await registration.unregister();
clearInterval(keepAlive);
console.log(`service: ${result}`);
