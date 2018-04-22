/* tslint:disable */
var wasm;


module.exports.__wbg_f_random_random_n = function () {
        return Math.random();
};
module.exports.approximate_pi = function (arg0) {
        const ret = wasm.approximate_pi(arg0);
        return ret;
};


wasm = require('./wasm_pibench_bg');