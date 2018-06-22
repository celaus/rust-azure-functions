let imports = {};
imports['./wasm_pibench'] = require('./wasm_pibench');

            const join = require('path').join;
            const bytes = require('fs').readFileSync(join(__dirname, 'wasm_pibench_bg.wasm'));
            const wasmModule = new WebAssembly.Module(bytes);
            const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
            module.exports = wasmInstance.exports;
        