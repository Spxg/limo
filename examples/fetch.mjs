// limo examples/fetch.mjs

const response = await fetch("./data.json");
if (!response.ok) throw new Error(`HTTP ${response.status}`);

const data = await response.json();
console.log("message:", data.message);
console.log("items:", data.items.join(", "));
