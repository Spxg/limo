async function increment() {
  const root = await navigator.storage.getDirectory();
  const file = await root.getFileHandle("runs.txt", { create: true });
  const handle = await file.createSyncAccessHandle();

  try {
    const buffer = new Uint8Array(handle.getSize());
    handle.read(buffer, { at: 0 });
    const runs = Number(new TextDecoder().decode(buffer)) + 1;

    const data = new TextEncoder().encode(String(runs));
    handle.write(data, { at: 0 });
    handle.truncate(data.length);
    handle.flush();
    return runs;
  } finally {
    handle.close();
  }
}

increment().then(
  (runs) => self.postMessage({ runs }),
  (error) => self.postMessage({ error: String(error) }),
);
