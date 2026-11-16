console.log(await Promise.resolve("awaited value"));
console.log(this === undefined);
