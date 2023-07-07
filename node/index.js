
const myProject = require('../rust');

// change the number below to change the Fib numbers generated
const fibObj = myProject.fibonacci(80);

// return is a single object with the input number as the key and the Fib number as the value
// values are strings with commas embedded
for (var key in fibObj) {
  const val = fibObj[key];
  console.log(`${key}: ${val}`);
}

const num = 31
console.log(`The Fibonacci number for ${num} is ${fibObj[num]}`);
