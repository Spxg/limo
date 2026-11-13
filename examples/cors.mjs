// limo examples/cors.mjs

console.log("origin:", location.origin);

try {
  const response = await fetch("https://baidu.com");
  console.log("status:", response.status);
} catch (error) {
  console.log("fetch failed:", error.message);
}
