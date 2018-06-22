const rust = require("./wasm_pibench.js"); 

module.exports = function (context, req) {
    const fibonacci = rust.run_fibonacci_test(43);
    console.log(fibonacci);
    context.res = {
        status: 200,
        body: `${fibonacci}`
    };
    context.done();
};