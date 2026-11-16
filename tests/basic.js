// RUN: limo --timeout-ms 5000 %s
// OUTPUT: hello
// OUTPUT:
// OUTPUT: world
// RUN: limo --timeout-ms 5000 -e 'console.log(1 + 2)'
// OUTPUT: 3

console.log("hello");
console.log("");
console.log("world");
