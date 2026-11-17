// RUN: limo --timeout-ms 5000 %s
// STDOUT: hello
// STDOUT:
// STDOUT: world
//
// RUN: limo --timeout-ms 5000 -e 'console.log(1 + 2)'
// STDOUT: 3

console.log("hello");
console.log("");
console.log("world");
