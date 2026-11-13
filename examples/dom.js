// limo examples/dom.js

console.log("Hello from limo!");

const button = document.createElement("button");
button.textContent = "Click me";
button.addEventListener("click", () => {
  console.log("clicked:", button.textContent);
});
document.body.appendChild(button);

button.click();
console.log("DOM:", document.body.innerHTML);
