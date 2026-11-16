# Limo

Limo runs JavaScript in a browser environment with full Web API support, right from your terminal. Powered by [moli](https://github.com/lexmount/moli).

See the [guide](https://spxg.github.io/limo/) for installation and usage.

## Example

```js
// dom.js
const button = document.createElement("button");
button.textContent = "Click me";
button.addEventListener("click", () => {
  console.log("Hello from the DOM!");
});
document.body.append(button);

button.click();
console.log(document.querySelector("button").outerHTML);
```

```sh
limo dom.js
```

```text
Hello from the DOM!
<button>Click me</button>
```
