// limo --port 3000 --profile-dir /tmp/limo-profile examples/idb.mjs
// Run again with the same port and profile to increment the stored counter.

const db = await new Promise((resolve, reject) => {
  const request = indexedDB.open("limo-example", 1);
  request.onupgradeneeded = () => request.result.createObjectStore("state");
  request.onsuccess = () => resolve(request.result);
  request.onerror = () => reject(request.error);
});

try {
  let runs;
  await new Promise((resolve, reject) => {
    const transaction = db.transaction("state", "readwrite");
    transaction.oncomplete = resolve;
    transaction.onabort = () => reject(transaction.error ?? new Error("Transaction aborted"));

    const store = transaction.objectStore("state");
    const request = store.get("runs");
    request.onsuccess = () => {
      runs = (request.result ?? 0) + 1;
      store.put(runs, "runs");
    };
  });
  console.log("IndexedDB runs:", runs);
} finally {
  db.close();
}
