const rust = require("./wasm_pibench.js"); 

const fibonacci = rust.run_fibonacci_test(43);
console.log(fibonacci);
