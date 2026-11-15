// limo --port 3000 --profile-dir /tmp/limo-profile examples/sqlite/main.js

const worker = new Worker("./worker.js", { type: "module" });

worker.onmessage = ({ data }) => {
  if (data.error) console.error(data.error);
  else console.log(data.rows);
  worker.terminate();
};

worker.onerror = ({ message }) => {
  console.error(message);
  worker.terminate();
};
