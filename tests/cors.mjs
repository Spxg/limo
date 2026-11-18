// RUN: limo --timeout-ms 5000 %s
// STDOUT: TypeError
//
// RUN: limo --timeout-ms 5000 --disable-web-security %s
// STDOUT: local fixture

const url = new URL("./fixtures/server-data.json", import.meta.url);
// Limo serves the page on 127.0.0.1. Using localhost changes its origin.
url.hostname = "localhost";

try {
  const response = await fetch(url);
  console.log((await response.json()).message);
} catch (error) {
  console.log(error.name);
}
