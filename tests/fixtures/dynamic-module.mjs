globalThis.moduleLoads = (globalThis.moduleLoads ?? 0) + 1;
console.log("module:", globalThis.moduleLoads);
