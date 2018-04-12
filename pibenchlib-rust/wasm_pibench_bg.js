let imports = {};

            const bytes = require('fs').readFileSync('wasm_pibench_bg.wasm');
            const wasmModule = new WebAssembly.Module(bytes);
            const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
            module.exports = wasmInstance.exports;
        