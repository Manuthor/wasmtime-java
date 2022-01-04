# Notes about `wit_lb`

The crate `wit_lib` is a wit-bindgen-wasm test module (not yet running with the `wasmtime-java`!):

- this crate `wit_lib` is a wit_bindgen-example for building a WASI-WebAssembly file
- the WIT file `my.wit` gives an interface which contains basic functions to be implemented
- src/bindings.rs comes from the command `wit-bindgen rust-wasm -e my.wit --out-dir src/`
- src/lib.rs: the 2 functions `__alloc_bytes` and `__dealloc_bytes` have been artificially added to make the WASM loading possible by `wasmtime-java`
