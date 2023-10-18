const rustModule = require('.');

let result = rustModule.add2numbers(23, 45);
console.log(`23 + 45 = ${result}`);

function fibonacci(n) {
    if (n <= 1) {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
  
  // Example usage
  const result2 = rustModule.fibonacci(100);
  console.log(result2); // Output: 55
  