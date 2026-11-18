// RUN: limo --timeout-ms 10000 %s
// STDOUT: sqlite: done

import { assert } from "./fixtures/assert.mjs";

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
  assert(
    first.length === 1 && first[0] === "1 | Hello from limo",
    `Unexpected first query: ${JSON.stringify(first)}`,
  );

  // A fresh Worker creates another WASM instance and reopens the OPFS database.
  const second = await query();
  assert(
    second.length === 2 && second[0] === first[0] && second[1] === "2 | Hello from limo",
    `Unexpected rows after reopening: ${JSON.stringify(second)}`,
  );
  console.log("sqlite: done");
} finally {
  clearInterval(keepAlive);
}
