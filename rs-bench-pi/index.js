const rust = require("./wasm_pibench.js"); 

module.exports = function (context, req) {
    const almost_pi = rust.approximate_pi(10000000);
    context.log(almost_pi);

    context.res = {
        status: 200,
        body: `${almost_pi}`
    };
    context.done();
};