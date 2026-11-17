// RUN: limo --timeout-ms 10000 %s
// STDOUT: first write:
// STDOUT: 1 | Hello from limo
// STDOUT: reopen in a new worker:
// STDOUT: 1 | Hello from limo
// STDOUT: 2 | Hello from limo
// STDOUT: sqlite: done

function query() {
  return new Promise((resolve, reject) => {
    const worker = new Worker("./fixtures/sqlite-worker.js", { type: "module" });
    worker.onmessage = ({ data }) => {
      worker.terminate();
      if (data.error) reject(new Error(data.error));
      else resolve(data.rows);
    };
    worker.onerror = ({ message }) => {
      worker.terminate();
      reject(new Error(message));
    };
  });
}

const keepAlive = setInterval(() => {}, 100);
try {
  const first = await query();
  console.log("first write:");
  for (const row of first) console.log(row);

  // A fresh Worker creates another WASM instance and reopens the OPFS database.
  const second = await query();
  console.log("reopen in a new worker:");
  for (const row of second) console.log(row);
  console.log("sqlite: done");
} catch (error) {
  console.error(String(error));
} finally {
  clearInterval(keepAlive);
}
