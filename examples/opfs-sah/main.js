// limo --port 3000 --profile-dir /tmp/limo-profile examples/opfs-sah/main.js

const worker = new Worker("./worker.js");

worker.onmessage = ({ data }) => {
  if (data.error) console.error(data.error);
  else console.log("OPFS runs:", data.runs);
  worker.terminate();
};

worker.onerror = ({ message }) => {
  console.error(message);
  worker.terminate();
};
