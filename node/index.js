
const myProject = require('../rust');
const ar = myProject.fibonacci(80);

// console.log ar
for (var key in ar) {
  const val = ar[key];
  console.log(`${key}: ${val}`);
}
