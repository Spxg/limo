import init, { run } from "./pkg/limo_sqlite_example.js";

try {
  await init();
  self.postMessage({ rows: await run() });
} catch (error) {
  self.postMessage({ error: String(error) });
}
