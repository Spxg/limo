self.addEventListener("install", event => event.waitUntil(self.skipWaiting()));
self.addEventListener("activate", event => event.waitUntil(self.clients.claim()));
self.addEventListener("message", ({ ports: [port] }) => {
  console.log("service log: preserved");
  console.warn("service warn: preserved");
  console.log();
  port.postMessage("done");
  port.close();
});
