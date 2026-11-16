self.onconnect = ({ ports: [port] }) => {
  port.onmessage = () => {
    console.log("shared log: preserved");
    console.warn("shared warn: preserved");
    console.log();
    port.postMessage("done");
    port.close();
    self.close();
  };
  port.start();
};
