const rust = require("./wasm_pibench.js"); 
const almost_pi = rust.approximate_pi(10000000);
console.log(almost_pi);