// limo examples/fetch.mjs
// limo --disable-web-security examples/fetch.mjs

console.log("origin:", location.origin);

// GitHub allows cross-origin requests by default.
try {
  const response = await fetch("https://api.github.com/repos/Spxg/limo");
  console.log("GitHub status:", response.status);
  const repo = await response.json();
  if (!response.ok) throw new Error(repo.message);
  console.log(JSON.stringify({
    name: repo.full_name,
    description: repo.description,
    stars: repo.stargazers_count,
    forks: repo.forks_count,
  }, null, 2));
} catch (error) {
  console.log("GitHub fetch failed:", error.message);
}

// Bilibili requires --disable-web-security to read the cross-origin response.
try {
  const response = await fetch(
    "https://api.bilibili.com/x/web-interface/view?bvid=BV1GJ411x7h7",
  );
  console.log("Bilibili status:", response.status);
  const { code, message, data } = await response.json();
  if (code !== 0) throw new Error(message);
  console.log(JSON.stringify({
    bvid: data.bvid,
    title: data.title,
    author: data.owner.name,
    views: data.stat.view,
  }, null, 2));
} catch (error) {
  console.log("Bilibili fetch failed:", error.message);
}
