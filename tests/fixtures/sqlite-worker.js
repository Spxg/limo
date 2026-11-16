import init, { run } from "./sqlite/limo_sqlite_example.js";

try {
  await init();
  const table = await run();
  // Compare the queried IDs and messages; created_at varies between runs.
  const rows = table.trimEnd().split("\n").slice(1)
    .map(row => row.split(" | ").slice(0, 2).join(" | "));
  self.postMessage({ rows });
} catch (error) {
  self.postMessage({ error: String(error) });
}
